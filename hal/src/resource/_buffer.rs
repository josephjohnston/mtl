use super::*;

pub struct Buffer<'a, T> {
    pub(crate) inner: lib::Id<lib::Buffer>,
    pub label: &'a str,
    pub device: &'a Device,
    pub private: bool,
    pub count: usize,
    pub heap: Option<&'a Heap<'a>>,
    ptr: *mut T,
    allocated_size: usize,
}

impl<'a, T> Buffer<'a, T> {
    pub(crate) fn new(
        inner: lib::Id<lib::Buffer>,
        label: String,
        // device: &'a Device,
        private: bool,
        count: usize,
        heap: Option<&'a Heap>,
    ) -> Self {
        inner.set_label(&lib::NSString::from_str(label));
        let ptr: *mut T = inner.contents().cast();
        let allocated_size = inner.allocated_size();
        Self {
            inner,
            label,
            device,
            private,
            count,
            heap,
            ptr,
            allocated_size,
        }
    }
    pub fn as_slice(&self) -> &[T] {
        unsafe {
            let slice = std::slice::from_raw_parts::<T>(self.ptr, self.count);
            slice
        }
    }
    pub fn as_mut_slice(&self) -> &mut [T] {
        unsafe {
            let mut_slice = std::slice::from_raw_parts_mut::<T>(self.ptr, self.count);
            mut_slice
        }
    }
    // pub fn len(&self) -> usize {
    //     self.length
    // }
    pub fn as_ptr(&self) -> *const T {
        // assert_eq!(self.private, false);
        self.ptr
    }
    pub unsafe fn as_mut_ptr(&self) -> *mut T {
        assert_eq!(self.private, false);
        self.ptr
    }
    /// Safety: ensure
    pub unsafe fn cast<U>(self) -> Buffer<'a, U> {
        Buffer {
            inner: self.inner,
            label: self.label,
            device: self.device,
            private: self.private,
            count: self.count,
            heap: self.heap,
            ptr: self.ptr.cast(),
            allocated_size: self.allocated_size,
        }
    }
}

// impl<'a T> std::ops::Index<usize> for Buffer<'a T> {
//     type Output = T;
//     fn index(&self, index: usize) -> &Self::Output {
//         let x = unsafe { self.ptr.offset(index) };
//         unsafe { std::ptr::read(x) }
//     }
// }

impl<'a, T> Resource for Buffer<'a, T> {
    fn allocated_size(&self) -> usize {
        self.inner.allocated_size()
    }
    fn heap_offset(&self) -> usize {
        self.inner.heap_offset()
    }
    fn is_aliasable(&self) -> bool {
        self.inner.is_aliasable()
    }
    fn make_aliasable(&self) {
        self.inner.make_aliasable();
    }
}
