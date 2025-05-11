#![no_std]
#![allow(static_mut_refs)]
#![allow(clippy::missing_safety_doc)]

extern crate alloc;

use core::mem::ManuallyDrop;
use psp::{
    Align16, BUF_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH,
    sys::{self, DisplayPixelFormat, GuPrimitive, GuState, TexturePixelFormat},
    vram_alloc::get_vram_allocator,
};

pub mod buffer;
pub mod color;
pub mod index;
pub mod rect;
pub mod vertex;

use buffer::{Buffer, TransientBuffer};
use color::Color32;
use index::IndexItem;
use rect::Rect;
use vertex::Vertex;

// Re-export
pub use paste;

pub static mut BUFFER: Align16<[u32; 0x40000]> = Align16([0; 0x40000]);

pub struct PspGfx {
    fbp0: *mut u8,
    fbp1: *mut u8,
    zbp: *mut u8,
}

impl PspGfx {
    pub fn init() -> Self {
        let allocator = get_vram_allocator().unwrap();
        let fbp0 = allocator
            .alloc_texture_pixels(BUF_WIDTH, SCREEN_HEIGHT, TexturePixelFormat::Psm8888)
            .as_mut_ptr_from_zero();
        let fbp1 = allocator
            .alloc_texture_pixels(BUF_WIDTH, SCREEN_HEIGHT, TexturePixelFormat::Psm8888)
            .as_mut_ptr_from_zero();
        let zbp = allocator
            .alloc_texture_pixels(BUF_WIDTH, SCREEN_HEIGHT, TexturePixelFormat::Psm4444)
            .as_mut_ptr_from_zero();

        unsafe {
            sys::sceGuInit();
            sys::sceGumLoadIdentity();
            sys::sceGuStart(
                psp::sys::GuContextType::Direct,
                BUFFER.0.as_mut_ptr() as *mut _,
            );
            sys::sceGuDrawBuffer(DisplayPixelFormat::Psm8888, fbp0 as _, BUF_WIDTH as i32);
            sys::sceGuDispBuffer(
                SCREEN_WIDTH as i32,
                SCREEN_HEIGHT as i32,
                fbp1 as _,
                BUF_WIDTH as i32,
            );
            sys::sceGuDepthBuffer(zbp as _, BUF_WIDTH as i32);
            sys::sceGuOffset(2048 - (SCREEN_WIDTH / 2), 2048 - (SCREEN_HEIGHT / 2));
            sys::sceGuViewport(2048, 2048, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
            sys::sceGuDepthRange(65535, 0);
            sys::sceGuScissor(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
            sys::sceGuEnable(GuState::ScissorTest);
            sys::sceGuFinish();
            sys::sceGuSync(sys::GuSyncMode::Finish, sys::GuSyncBehavior::Wait);
            sys::sceDisplayWaitVblankStart();
            sys::sceGuDisplay(true);
        }

        Self { fbp0, fbp1, zbp }
    }

    pub fn start_frame<'a>(&'a self) -> Frame<'a> {
        unsafe {
            sys::sceGuStart(
                psp::sys::GuContextType::Direct,
                BUFFER.0.as_mut_ptr() as *mut _,
            );
        }
        Frame { _gfx: self }
    }
}

pub struct Frame<'gfx> {
    _gfx: &'gfx PspGfx,
}

impl<'gfx> Frame<'gfx> {
    fn finish_non_consuming(&self) {
        unsafe {
            sys::sceGuFinish();
            sys::sceGuSync(sys::GuSyncMode::Finish, sys::GuSyncBehavior::Wait);
            sys::sceDisplayWaitVblankStart();
            sys::sceGuSwapBuffers();
        }
    }

    pub fn finish(self) {
        self.finish_non_consuming();
        // XXX: this could *potentially* leak
        let _ = ManuallyDrop::new(self);
    }

    pub fn clear_color_depth(&self, color: Color32, depth: u32) {
        unsafe {
            sys::sceGuClearColor(color.as_abgr());
            sys::sceGuClearDepth(depth);
            sys::sceGuClear(
                sys::ClearBuffer::COLOR_BUFFER_BIT | sys::ClearBuffer::DEPTH_BUFFER_BIT,
            );
        }
    }

    pub fn set_color(&self, color: Color32) {
        unsafe {
            sys::sceGuColor(color.as_abgr());
        }
    }

    pub fn set_scissor(&self, scissor: Rect) {
        unsafe {
            sys::sceGuScissor(scissor.x, scissor.y, scissor.w, scissor.h);
        }
    }

    /// Get memory from sceGuGetMemory as a [`TransientBuffer`]
    ///
    /// (Safe alternative to [`UntypedBuffer::get_memory_static`])
    pub fn get_memory<'frame, T>(&'frame self, data: &[T]) -> TransientBuffer<'frame, T> {
        unsafe { TransientBuffer::get_memory_static(data) }
    }

    pub fn draw_array<V: Buffer>(&self, primitive: GuPrimitive, vertex_buf: &V)
    where
        V::Item: Vertex,
    {
        unsafe {
            sys::sceGuDrawArray(
                primitive,
                V::Item::vtype(),
                vertex_buf.len() as i32,
                core::ptr::null(),
                vertex_buf.as_ptr(),
            );
        }
    }

    pub fn draw_array_indexed<V: Buffer, I: Buffer>(
        &self,
        primitive: GuPrimitive,
        vertex_buf: &V,
        index_buf: &I,
    ) where
        V::Item: Vertex,
        I::Item: IndexItem + Default,
    {
        // XXX: are indices pointing oob ub?
        unsafe {
            sys::sceGuDrawArray(
                primitive,
                V::Item::vtype() | I::Item::vtype(),
                index_buf.len() as i32,
                index_buf.as_ptr(),
                vertex_buf.as_ptr(),
            );
        }
    }
}

impl<'a> Drop for Frame<'a> {
    fn drop(&mut self) {
        self.finish_non_consuming();
    }
}
