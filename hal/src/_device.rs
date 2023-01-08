use super::*;

pub struct Device {
    inner: lib::Id<lib::Device>,
    texture_desc: lib::Id<lib::TextureDescriptor>,
    heap_desc: lib::Id<lib::HeapDescriptor>,
}
// impl ::std::ops::Deref for Device {
//     type Target = lib::Id<lib::Device>;
//     #[inline]
//     fn deref(&self) -> &Self::Target {
//         &self.inner
//     }
// }

impl Device {
    pub fn new() -> Self {
        let texture_desc = lib::TextureDescriptor::new();
        let heap_desc = lib::HeapDescriptor::new();
        Self {
            inner: lib::create_system_default_device(),
            texture_desc,
            heap_desc,
        }
    }
    // Creating Queues
    pub fn new_queue<'a>(&'a self, label: &'a str, max_command_buffer_count: usize) -> Queue {
        let inner_queue = self
            .inner
            .new_command_queue_with_max_command_buffer_count(max_command_buffer_count);
        Queue::new(inner_queue, label, self)
    }
    // Creating Heaps
    pub fn new_heap<'a>(&'a self, label: &'a str, length: usize) -> Heap {
        let resource_options = lib::ResourceOptions::default(true);
        self.heap_desc.set_size(length);
        self.heap_desc.set_resource_options(resource_options);
        let inner_heap = self.inner.new_heap_with_descriptor(&self.heap_desc);
        Heap::new(inner_heap, label, self, length)
    }
    // Creating Buffers
    pub fn max_buffer_length(&self) -> usize {
        self.inner.max_buffer_length()
    }
    pub fn new_buffer_empty<'a, T>(
        &'a self,
        label: &'a str,
        private: bool,
        count: usize,
    ) -> Buffer<T> {
        let length = count * std::mem::size_of::<T>();
        let resource_options = lib::ResourceOptions::default(private);
        let inner_buffer = self.inner.new_buffer_with_length(length, resource_options);
        Buffer::new(inner_buffer, label, self, private, count, None)
    }
    // the point of the below is to avoid copying when the data arrived from elsewhere, but in that case it's probably not page aligned, so it won't work.
    // pub fn new_buffer_with_ptr<'a, T, F>(
    //     &'a self,
    //     label: &'a str,
    //     private: bool,
    //     count: usize,
    //     ptr: *const T,
    //     deallocator: F,
    // ) -> Buffer<T>
    // where
    //     F: Fn(*const c_void, usize) + 'static,
    // {
    //     let length = count * std::mem::size_of::<T>();
    //     let resource_options = lib::ResourceOptions::default(private);
    //     // let deallocator_block: Option<&lib::Block<(*const std::ffi::c_void, usize), ()>> =
    //     //     match deallocator {
    //     //         Some(deallocator) => {
    //     //             let rc_block = lib::ConcreteBlock::new(deallocator).copy();
    //     //             Some(rc_block)
    //     //         }
    //     //         None => None,
    //     //     };
    //     let deallocator_block = lib::ConcreteBlock::new(deallocator).copy();
    //     let inner_buffer = self.inner.new_buffer_with_bytes_no_copy(
    //         ptr.cast(),
    //         length,
    //         resource_options,
    //         None, //Some(&deallocator_block),
    //     );
    //     Buffer::new(inner_buffer, label, self, private, count, None)
    // }
    pub fn new_buffer_with_ptr<'a, T>(
        &'a self,
        label: &'a str,
        private: bool,
        count: usize,
        ptr: *const T,
    ) -> Buffer<T> {
        let length = count * std::mem::size_of::<T>();
        let resource_options = lib::ResourceOptions::default(private);
        let inner_buffer = self
            .inner
            .new_buffer_with_bytes(ptr.cast(), length, resource_options);
        Buffer::new(inner_buffer, label, self, private, count, None)
    }
    // Creating Textures
    pub fn new_texture<'a>(
        &'a self,
        label: &'a str,
        private: bool,
        usage: TextureUsage,
        format: PixelFormat,
        size: Size,
        array: bool,
    ) -> Texture {
        configure_texture_descriptor(&self.texture_desc, private, usage, format, size, array);
        let inner_texture = self.inner.new_texture_with_descriptor(&self.texture_desc);
        // label
        inner_texture.set_label(&lib::NSString::from_str(label));
        Texture::new(
            inner_texture,
            label,
            self,
            private,
            usage,
            format,
            size,
            array,
            None,
        )
    }
    // Checking size and align
    pub fn heap_buffer_size_and_align(&self, length: usize) -> SizeAndAlign {
        let resource_options = lib::ResourceOptions::default(true);
        self.inner
            .heap_buffer_size_and_align_with_length(length, resource_options)
    }
    pub fn heap_texture_size_and_align(
        &self,
        usage: TextureUsage,
        format: PixelFormat,
        size: Size,
        array: bool,
    ) -> SizeAndAlign {
        configure_texture_descriptor(&self.texture_desc, true, usage, format, size, array);
        self.inner
            .heap_texture_size_and_align_with_descriptor(&self.texture_desc)
    }
}
