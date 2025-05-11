use core::ffi::c_void;
use core::marker::PhantomData;

pub unsafe trait Buffer {
    type Item;

    /// Get raw pointer to the buffer
    fn as_ptr(&self) -> *const c_void;
    /// Get size of the buffer in bytes
    fn byte_size(&self) -> usize;

    /// Get length of the buffer in elements
    fn len(&self) -> usize {
        self.byte_size() / core::mem::size_of::<Self::Item>()
    }
    /// Check if the buffer is empty
    fn is_empty(&self) -> bool {
        self.byte_size() == 0
    }
}

pub struct TransientBuffer<'frame, T> {
    ptr: *mut c_void,
    size: i32,
    _phantom: PhantomData<&'frame T>,
}

impl<'frame, T> TransientBuffer<'frame, T> {
    /// Get memory from sceGuGetMemory as a [`TransientBuffer`]
    ///
    /// Use `frame.get_memory` for a safe alternative
    ///
    /// Safety:
    /// - Must not outlive current frame.
    pub unsafe fn get_memory_static<'a>(data: &[T]) -> TransientBuffer<'a, T> {
        let len = data.len();
        let len_bytes = core::mem::size_of_val(data);
        assert!(len_bytes < i32::MAX as usize);
        let ptr = unsafe { psp::sys::sceGuGetMemory(len_bytes as i32) };
        unsafe {
            core::ptr::copy_nonoverlapping(data.as_ptr(), ptr as *mut T, len);
        }
        TransientBuffer {
            ptr,
            size: len_bytes as i32,
            _phantom: PhantomData,
        }
    }
}

unsafe impl<'frame, T> Buffer for TransientBuffer<'frame, T> {
    type Item = T;

    fn as_ptr(&self) -> *const c_void {
        self.ptr
    }

    fn byte_size(&self) -> usize {
        self.size as usize
    }
}
