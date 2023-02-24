use super::*;

pub(crate) fn get_gpu_resource_options() -> mtl::ResourceOptions {
    mtl::ResourceOptions::new(
        mtl::CPUCacheMode::WriteCombined,
        mtl::StorageMode::Private,
        mtl::HazardTrackingMode::Untracked,
    )
}

trait Heap {
    fn allocated_size(&self) -> usize;
    fn used_size(&self) -> usize;
    fn max_available_size(&self, alignment: usize) -> usize;
}

pub struct AutoHeap {
    id: Id<mtl::Heap>,
    pub name: String,
    pub length: usize,
    texture_desc: Id<mtl::TextureDescriptor>,
    // device: &'a mtl::Device,
}
impl AutoHeap {
    pub(crate) fn new(
        device: &mtl::Device,
        desc: &mtl::HeapDescriptor,
        name: String,
        length: usize,
    ) -> Self {
        desc.set_heap_type(mtl::HeapType::Automatic);
        desc.set_size(length);
        let id = device.new_heap_with_descriptor(&desc);
        id.set_label(&NSString::from_str(name.as_str()));
        let texture_desc = mtl::TextureDescriptor::new();
        texture_desc.set_resource_options(get_gpu_resource_options());
        Self {
            id,
            name,
            length,
            texture_desc,
            // device,
        }
    }
    pub fn new_buffer<T>(&self, name: String, len: usize) -> Buffer<T, Private> {
        // let length = len;
        let length = len * std::mem::size_of::<T>();
        // let (id, ptr) = autoreleasepool(|_| {
        let buffer_id = self
            .id
            .new_buffer_with_length(length, get_gpu_resource_options())
            .unwrap();
        // (id, id.contents().cast())
        // });
        Buffer::<T, Private>::new(buffer_id, name, len)
    }
    pub fn new_texture(
        &self,
        name: String,
        usage: TextureUsage,
        format: PixelFormat,
        size: Size,
        array: bool,
    ) -> Texture {
        // autoreleasepool(|_| {
        Texture::configure_descriptor(&self.texture_desc, usage, format, size, array);
        let texture_id = self
            .id
            .new_texture_with_descriptor(&self.texture_desc)
            .unwrap();
        Texture::new(texture_id, name, usage, format, size, array)
        // })
    }
}
impl Heap for AutoHeap {
    fn allocated_size(&self) -> usize {
        self.id.current_allocated_size()
    }
    fn used_size(&self) -> usize {
        self.id.used_size()
    }
    fn max_available_size(&self, alignment: usize) -> usize {
        self.id.max_available_size_with_alignment(alignment)
    }
}

pub struct SelfHeap {
    id: Id<mtl::Heap>,
    pub name: String,
    pub length: usize,
    texture_desc: Id<mtl::TextureDescriptor>,
    // device: &'a mtl::Device,
}
impl SelfHeap {
    pub(crate) fn new(
        device: &mtl::Device,
        desc: &mtl::HeapDescriptor,
        name: String,
        length: usize,
    ) -> Self {
        desc.set_heap_type(mtl::HeapType::Placement);
        desc.set_size(length);
        let id = device.new_heap_with_descriptor(&desc);
        id.set_label(&NSString::from_str(name.as_str()));
        let texture_desc = mtl::TextureDescriptor::new();
        Self {
            id,
            name,
            length,
            texture_desc,
            // device,
        }
    }
    pub fn new_buffer<T>(&self, name: String, len: usize, offset: usize) -> Buffer<T, Private> {
        let length = len * std::mem::size_of::<T>();
        let buffer_id =
            self.id
                .new_buffer_with_length_offset(length, get_gpu_resource_options(), offset);
        Buffer::<T, Private>::new(buffer_id, name, len)
    }
    pub fn new_texture(
        &self,
        name: String,
        usage: TextureUsage,
        format: PixelFormat,
        size: Size,
        array: bool,
        offset: usize,
    ) -> Texture {
        Texture::configure_descriptor(&self.texture_desc, usage, format, size, array);
        let texture_id = self
            .id
            .new_texture_with_descriptor_offset(&self.texture_desc, offset);
        Texture::new(texture_id, name, usage, format, size, array)
    }
}
impl Heap for SelfHeap {
    fn allocated_size(&self) -> usize {
        self.id.current_allocated_size()
    }
    fn used_size(&self) -> usize {
        self.id.used_size()
    }
    fn max_available_size(&self, alignment: usize) -> usize {
        self.id.max_available_size_with_alignment(alignment)
    }
}
