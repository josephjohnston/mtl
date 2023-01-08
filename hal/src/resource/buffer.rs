use super::*;
use core::marker::PhantomData;

fn get_cpu_resource_options() -> mtl::ResourceOptions {
    mtl::ResourceOptions::new(
        mtl::CPUCacheMode::DefaultCache,
        mtl::StorageMode::Shared,
        mtl::HazardTrackingMode::Untracked,
    )
}

struct Never;
pub struct Shared(Never);
pub struct Private(Never);
mod private {
    pub trait Sealed {}
    impl Sealed for super::Shared {}
    impl Sealed for super::Private {}
}

pub trait Env: private::Sealed + 'static + Sized {}
impl Env for Shared {}
impl Env for Private {}

pub struct Buffer<'a, T, E: Env> {
    id: Id<mtl::Buffer>,
    pub name: String,
    slice: &'a mut [T],
    initial_length: usize,
    env: PhantomData<E>,
}
impl<'a, T, E: Env> Buffer<'a, T, E> {
    pub(crate) fn get_mtl(&self) -> &mtl::Buffer {
        &*self.id
    }
    // can we cast differently when on the gpu? i guess, but what's the use there if we can't use it as a slice?
    pub fn cast<U>(self, len: usize) -> Buffer<'a, U, E> {
        assert!(len * std::mem::size_of::<U>() <= self.initial_length);
        let ptr: *mut U = self.slice.as_mut_ptr().cast();
        let slice = unsafe { std::slice::from_raw_parts_mut::<U>(ptr, len) };
        Buffer {
            id: self.id,
            name: self.name,
            slice,
            initial_length: self.initial_length,
            env: PhantomData,
        }
    }
}
impl<'a, T> Buffer<'a, T, Shared> {
    pub(crate) fn new(device: &mtl::Device, name: String, len: usize) -> Self {
        let length = len * std::mem::size_of::<T>();
        // let (id, ptr) = autoreleasepool(|_| {
        let id = device.new_buffer_with_length(length, get_cpu_resource_options());
        id.set_label(&NSString::from_str(name.as_str()));
        let ptr = id.contents().cast();
        // (id, id.contents().cast())
        // });
        let slice = unsafe { std::slice::from_raw_parts_mut::<T>(ptr, len) };
        Self {
            id,
            name,
            slice,
            initial_length: length,
            env: PhantomData,
        }
    }
    pub fn as_slice(&self) -> &[T] {
        self.slice
    }
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.slice
    }
}
impl<'a, T> Buffer<'a, T, Private> {
    pub(crate) fn new(heap: &mtl::Heap, name: String, len: usize) -> Self {
        let length = len;
        let id = heap
            .new_buffer_with_length(length, get_gpu_resource_options())
            .unwrap();
        id.set_label(&NSString::from_str(name.as_str()));
        let slice = unsafe { std::slice::from_raw_parts_mut::<T>(std::ptr::null_mut(), length) };
        Self {
            id,
            name,
            slice,
            initial_length: length,
            env: PhantomData,
        }
    }
}

// pub trait Buffer {
//     fn get_mtl(&self) -> &mtl::Buffer;
// }
// pub struct CPUBuffer<'a, T> {
//     id: Id<mtl::Buffer>,
//     pub name: String,
//     slice: &'a mut [T],
//     initial_length: usize,
// }
// unsafe impl<'a, T> Send for CPUBuffer<'a, T> {}
// // impl<'a, T> Buffer for CPUBuffer<'a, T> {}
// impl<'a, T> Buffer for CPUBuffer<'a, T> {
//     fn get_mtl(&self) -> &mtl::Buffer {
//         &*self.id
//     }
// }
// //     fn get_mtl_buffer(&self) -> &mtl::Buffer {
// //         &*self.id
// //     }
// // }
// impl<'a, T> CPUBuffer<'a, T> {
//     pub(crate) fn new(device: &mtl::Device, name: String, len: usize) -> Self {
//         let length = len * std::mem::size_of::<T>();
//         // let (id, ptr) = autoreleasepool(|_| {
//         let id = device.new_buffer_with_length(length, get_cpu_resource_options());
//         id.set_label(&NSString::from_str(name.as_str()));
//         let ptr = id.contents().cast();
//         // (id, id.contents().cast())
//         // });
//         let slice = unsafe { std::slice::from_raw_parts_mut::<T>(ptr, len) };
//         Self {
//             id,
//             name,
//             slice,
//             initial_length: length,
//         }
//     }
//     pub fn as_slice(&self) -> &[T] {
//         self.slice
//     }
//     pub fn as_mut_slice(&mut self) -> &mut [T] {
//         self.slice
//     }
//     // can we cast differently when on the gpu? i guess, but what's the use there if we can't use it as a slice?
//     pub fn cast<U>(self, len: usize) -> CPUBuffer<'a, U> {
//         assert!(len * std::mem::size_of::<U>() <= self.initial_length);
//         let ptr: *mut U = self.slice.as_mut_ptr().cast();
//         let slice = unsafe { std::slice::from_raw_parts_mut::<U>(ptr, len) };
//         CPUBuffer {
//             id: self.id,
//             name: self.name,
//             slice,
//             initial_length: self.initial_length,
//         }
//     }
//     // pub(crate) fn get_id(&self) -> &mtl::Buffer {
//     //     &*self.id
//     // }
// }

// pub struct GPUBuffer<T> {
//     id: Id<mtl::Buffer>,
//     pub name: String,
//     phantom: std::marker::PhantomData<T>,
// }
// // impl<T> Buffer for GPUBuffer<T> {}
// impl<T> Buffer for GPUBuffer<T> {
//     fn get_mtl(&self) -> &mtl::Buffer {
//         &*self.id
//     }
// }
// impl<T> GPUBuffer<T> {
//     pub(crate) fn new(id: Id<mtl::Buffer>, name: String, len: usize) -> Self {
//         id.set_label(&NSString::from_str(name.as_str()));
//         Self {
//             id,
//             name,
//             // initial_length: len * std::mem::size_of::<T>(),
//             phantom: std::marker::PhantomData,
//         }
//     }
//     pub(crate) fn get_id(&self) -> &mtl::Buffer {
//         &*self.id
//     }
// }
