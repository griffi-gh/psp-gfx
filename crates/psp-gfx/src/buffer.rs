use core::ffi::c_void;
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};

use psp::sys::VertexType;

use crate::private::SealedTrait;

pub struct UntypedBuffer {
    ptr: *mut c_void,
    size: i32,
    // _phantom: PhantomData<&'a ()>,
}

impl UntypedBuffer {
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
            // _phantom: PhantomData,
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
pub struct TypedBuffer<T> {
    inner: UntypedBuffer,
    _phantom: PhantomData<T>,
}

impl<T> Deref for TypedBuffer<T> {
    type Target = UntypedBuffer;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for TypedBuffer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T> TypedBuffer<T> {
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

    pub fn into_untyped(self) -> UntypedBuffer {
        self.inner
    }
}

impl<T> From<TypedBuffer<T>> for UntypedBuffer {
    fn from(buffer: TypedBuffer<T>) -> Self {
        buffer.into_untyped()
    }
}

// impl From<UntypedBuffer> for TypedBuffer<u8> {
//     fn from(buffer: UntypedBuffer) -> Self {
//         TypedBuffer {
//             inner: buffer,
//             _phantom: PhantomData,
//         }
//     }
// }

pub trait IndexBuffer: SealedTrait {
    fn idx_vtype(&self) -> VertexType;
    fn idx_len(&self) -> i32;
    fn idx_buffer(&self) -> &UntypedBuffer;
}

impl SealedTrait for TypedBuffer<u8> {}
impl IndexBuffer for TypedBuffer<u8> {
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

impl SealedTrait for TypedBuffer<u16> {}
impl IndexBuffer for TypedBuffer<u16> {
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
