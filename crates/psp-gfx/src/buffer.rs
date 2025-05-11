use core::ffi::c_void;
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};

use psp::sys::VertexType;

pub struct UntypedBuffer<'a> {
    ptr: *mut c_void,
    size: i32,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> UntypedBuffer<'a> {
    /// Get memory from sceGuGetMemory as an untyped buffer
    ///
    /// Use `frame.get_memory_untyped` for a safe alternative
    ///
    /// Safety:
    /// - Must not outlive current frame.
    pub unsafe fn get_memory<T>(data: &[T]) -> Self {
        let len = data.len();
        let len_bytes = core::mem::size_of_val(data);
        assert!(len_bytes < i32::MAX as usize);
        let ptr = unsafe { psp::sys::sceGuGetMemory(len_bytes as i32) };
        unsafe {
            core::ptr::copy_nonoverlapping(data.as_ptr(), ptr as *mut T, len);
        }
        Self {
            ptr,
            size: len_bytes as i32,
            _phantom: PhantomData,
        }
    }

    pub fn size_bytes(&self) -> usize {
        self.size as usize
    }

    pub fn to_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

#[repr(transparent)]
pub struct TypedBuffer<'a, T> {
    inner: UntypedBuffer<'a>,
    _phantom: PhantomData<T>,
}

impl<'a, T> Deref for TypedBuffer<'a, T> {
    type Target = UntypedBuffer<'a>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a, T> DerefMut for TypedBuffer<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'a, T> TypedBuffer<'a, T> {
    /// Get memory from sceGuGetMemory as a typed buffer
    ///
    /// Use `frame.get_memory_typed` for a safe alternative
    ///
    /// Safety:
    /// - Must not outlive current frame.
    pub unsafe fn get_memory(data: &[T]) -> Self {
        Self {
            inner: unsafe { UntypedBuffer::get_memory(data) },
            _phantom: PhantomData,
        }
    }

    /// Returns amount of items in a buffer
    pub fn len(&self) -> usize {
        self.inner.size_bytes() / size_of::<T>()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get a reference to the underlying [`UntypedBuffer`]
    pub fn as_untyped(&self) -> &UntypedBuffer {
        &self.inner
    }

    /// Converts [`TypedBuffer`] into [`UntypedBuffer`]
    pub fn into_untyped(self) -> UntypedBuffer<'a> {
        self.inner
    }
}

impl<'a, T> From<TypedBuffer<'a, T>> for UntypedBuffer<'a> {
    fn from(buffer: TypedBuffer<'a, T>) -> Self {
        buffer.into_untyped()
    }
}

impl<'a> From<UntypedBuffer<'a>> for TypedBuffer<'a, u8> {
    fn from(buffer: UntypedBuffer<'a>) -> Self {
        TypedBuffer {
            inner: buffer,
            _phantom: PhantomData,
        }
    }
}

/// Marker trait used on [`TypedBuffers`] that can be used as Index Buffers for drawing
pub unsafe trait IndexBuffer {
    /// internal implementeation detail. do not rely on this
    fn idx_vtype(&self) -> VertexType;
    /// internal implementeation detail. do not rely on this
    fn idx_len(&self) -> i32;
    /// internal implementeation detail. do not rely on this
    fn idx_buffer(&self) -> &UntypedBuffer;
}

unsafe impl<'a> IndexBuffer for TypedBuffer<'a, u8> {
    fn idx_vtype(&self) -> VertexType {
        VertexType::INDEX_8BIT
    }
    fn idx_len(&self) -> i32 {
        self.len() as i32
    }
    fn idx_buffer(&self) -> &UntypedBuffer {
        self.as_untyped()
    }
}

unsafe impl<'a> IndexBuffer for TypedBuffer<'a, u16> {
    fn idx_vtype(&self) -> VertexType {
        VertexType::INDEX_16BIT
    }
    fn idx_len(&self) -> i32 {
        self.len() as i32
    }
    fn idx_buffer(&self) -> &UntypedBuffer {
        self.as_untyped()
    }
}
