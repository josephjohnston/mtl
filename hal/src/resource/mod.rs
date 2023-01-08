use super::*;

// pub trait Resource {
//     fn allocated_size(&self) -> usize;
//     fn heap_offset(&self) -> usize;
//     fn is_aliasable(&self) -> bool;
//     fn make_aliasable(&self);
// }

// maybe we should copy and try to learn from HAL.

// struct Resource<'a> {
//     pub label: &'a str,
//     pub device: &'a Device,
//     pub private: bool,
//     // heap,
//     // heapOffset
//     inner: lib::Id<lib::Resource>,
// }

// impl<'a> Resource<'a> {
//     pub fn new(
//         label: &'a str,
//         device: &'a Device,
//         private: bool,
//         inner: lib::Id<lib::Resource>,
//     ) -> Self {
//         inner.set_label(&lib::NSString::from_str(label));
//         Self {
//             label,
//             device,
//             private,
//             inner,
//         }
//     }
//     // pub fn allocated_size(&self) -> usize {
//     //     self.inner.allocated_size()
//     // }
//     // pub fn make_aliasable(&self) {
//     //     self.inner.make_aliasable();
//     // }
// }

// mod types;
// use types::*;

// #[cfg(target_os = "ios")]
// #[cfg(target_os = "macos")]

pub(crate) fn get_gpu_resource_options() -> mtl::ResourceOptions {
    mtl::ResourceOptions::new(
        mtl::CPUCacheMode::DefaultCache,
        mtl::StorageMode::Private,
        mtl::HazardTrackingMode::Untracked,
    )
}

pub(crate) trait Wrapper<T> {
    fn get_mtl(&self) -> &T;
}

mod buffer;
pub use buffer::*;

mod heap;
pub use heap::*;

mod texture;
pub use texture::*;

// what's the API?
// create heaps (on gpu), from heap create buffer or texture.

// https://developer.apple.com/documentation/metal/mtlheapdescriptor/1649567-storagemode?language=objc
// can't use shared heaps on osx. therefore we don't use cpu heaps. we use cpu individual resources and gpu heaps.
// https://developer.apple.com/documentation/metal/mtlresourceoptions/mtlresourcestoragemodeshared?language=objc
// can't use shared textures on macos. thereore we don't use cpu textures. we use cpu buffers.

// we could use: managed resources on osx and shared heaps on ios
// reasons not to do this:
// 1. they have different APIs. managed vs shared -- we'd simulate synchronization. individual vs heaps -- we'd simulate allocating individuals.
// 2. using textures on cpu requires copying, and isn't more helpful than using buffers.

// now we move on. to what? to encoding. putting these two together. well, there's also commands to take care of.
