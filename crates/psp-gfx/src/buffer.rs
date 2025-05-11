use core::ffi::c_void;
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};

use psp::sys::VertexType;

use crate::private::SealedTrait;

pub struct UntypedBuffer<'frame> {
    ptr: *mut c_void,
    size: i32,
    _phantom: PhantomData<&'frame ()>,
}

impl<'frame> UntypedBuffer<'frame> {
    pub unsafe fn new<T>(data: &[T]) -> Self {
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
pub struct TypedBuffer<'frame, T> {
    inner: UntypedBuffer<'frame>,
    _phantom: PhantomData<T>,
}

impl<'frame, T> Deref for TypedBuffer<'frame, T> {
    type Target = UntypedBuffer<'frame>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'frame, T> DerefMut for TypedBuffer<'frame, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'frame, T> TypedBuffer<'frame, T> {
    pub unsafe fn new(data: &[T]) -> Self {
        Self {
            inner: unsafe { UntypedBuffer::new(data) },
            _phantom: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.inner.size_bytes() / size_of::<T>()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn as_untyped(&self) -> &UntypedBuffer {
        &self.inner
    }

    pub fn into_untyped(self) -> UntypedBuffer<'frame> {
        self.inner
    }
}

impl<'frame, T> From<TypedBuffer<'frame, T>> for UntypedBuffer<'frame> {
    fn from(buffer: TypedBuffer<'frame, T>) -> Self {
        buffer.into_untyped()
    }
}

impl<'frame> From<UntypedBuffer<'frame>> for TypedBuffer<'frame, u8> {
    fn from(buffer: UntypedBuffer<'frame>) -> Self {
        TypedBuffer {
            inner: buffer,
            _phantom: PhantomData,
        }
    }
}

pub trait IndexBuffer: SealedTrait {
    fn idx_vtype(&self) -> VertexType;
    fn idx_len(&self) -> i32;
    fn idx_buffer(&self) -> &UntypedBuffer;
}

impl<'frame> SealedTrait for TypedBuffer<'frame, u8> {}
impl<'frame> IndexBuffer for TypedBuffer<'frame, u8> {
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

impl<'frame> SealedTrait for TypedBuffer<'frame, u16> {}
impl<'frame> IndexBuffer for TypedBuffer<'frame, u16> {
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
