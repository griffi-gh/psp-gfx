use core::ffi::c_void;

use color::Color;
use psp::{
    Align16, BUF_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH,
    sys::{self, DisplayPixelFormat, GuPrimitive, GuState, TexturePixelFormat, VertexType},
    vram_alloc::get_vram_allocator,
};
use rect::Rect;

pub mod color;
pub mod rect;

pub static mut BUFFER: Align16<[u32; 0x40000]> = Align16([0; 0x40000]);

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Vertex {
    pub u: u16,
    pub v: u16,
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

pub struct GraphicsManager {
    fbp0: *mut u8,
    fbp1: *mut u8,
    zbp: *mut u8,
}

impl GraphicsManager {
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

    pub fn clear_color_depth(&mut self, color: Color, depth: u32) {
        unsafe {
            sys::sceGuClearColor(color.0);
            sys::sceGuClearDepth(depth);
            sys::sceGuClear(
                sys::ClearBuffer::COLOR_BUFFER_BIT | sys::ClearBuffer::DEPTH_BUFFER_BIT,
            );
        }
    }

    pub fn begin_frame(&mut self) {
        unsafe {
            sys::sceGuStart(
                psp::sys::GuContextType::Direct,
                BUFFER.0.as_mut_ptr() as *mut _,
            );
        }
    }

    pub fn end_frame(&mut self) {
        unsafe {
            sys::sceGuFinish();
            sys::sceGuSync(sys::GuSyncMode::Finish, sys::GuSyncBehavior::Wait);
            sys::sceDisplayWaitVblankStart();
            sys::sceGuSwapBuffers();
        }
    }

    pub fn draw(&mut self, vtx: &[Vertex], color: Color, primitive: GuPrimitive) {
        let len = vtx.len();
        let len_bytes = core::mem::size_of_val(vtx);
        assert!(len_bytes < i32::MAX as usize);

        let buffer = unsafe { sys::sceGuGetMemory(len_bytes as i32) };
        unsafe {
            core::ptr::copy_nonoverlapping(vtx.as_ptr(), buffer as *mut Vertex, len);
        }

        unsafe {
            sys::sceGuColor(color.0);
            sys::sceGuDrawArray(
                primitive,
                VertexType::TEXTURE_16BIT | VertexType::VERTEX_16BIT | VertexType::TRANSFORM_2D,
                vtx.len() as i32,
                core::ptr::null::<c_void>(),
                buffer,
            );
        }
    }

    pub fn fill_rect(&mut self, rect: Rect, color: Color) {
        self.draw(
            &[
                Vertex {
                    x: rect.x as i16,
                    y: rect.y as i16,
                    ..Default::default()
                },
                Vertex {
                    x: (rect.x + rect.w) as i16,
                    y: (rect.y + rect.h) as i16,
                    ..Default::default()
                },
            ],
            color,
            GuPrimitive::Sprites,
        );
    }
}
