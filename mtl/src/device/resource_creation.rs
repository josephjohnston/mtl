use super::*;

/// # Working With Resource Heaps
impl Device {
    pub fn new_heap_with_descriptor(&self, desc: &HeapDescriptor) -> Id<Heap> {
        unsafe {
            let raw_heap: *mut Heap = msg_send![self, newHeapWithDescriptor: desc];
            Id::new(raw_heap).expect(ID_NEW_FAILURE)
        }
    }
    pub fn heap_buffer_size_and_align_with_length(
        &self,
        length: usize,
        options: ResourceOptions,
    ) -> SizeAndAlign {
        unsafe {
            msg_send![
                self,
                heapBufferSizeAndAlignWithLength: length,
                options: options
            ]
        }
    }
    pub fn heap_texture_size_and_align_with_descriptor(
        &self,
        desc: &TextureDescriptor,
    ) -> SizeAndAlign {
        unsafe { msg_send![self, heapTextureSizeAndAlignWithDescriptor: desc] }
    }
}

/// # Creating Buffers
impl Device {
    // [P] maxBufferLength
    pub fn max_buffer_length(&self) -> usize {
        unsafe { msg_send![self, maxBufferLength] }
    }
    // [M] newBufferWithLength:options:
    pub fn new_buffer_with_length(&self, length: usize, options: ResourceOptions) -> Id<Buffer> {
        unsafe {
            let raw_buffer: *mut Buffer =
                msg_send![self, newBufferWithLength: length, options: options];
            Id::new(raw_buffer).expect(ID_NEW_FAILURE)
        }
    }
    // [M] newBufferWithBytes:length:options:
    pub fn new_buffer_with_bytes(
        &self,
        pointer: *const c_void,
        length: usize,
        options: ResourceOptions,
    ) -> Id<Buffer> {
        unsafe {
            let raw_buffer: *mut Buffer = msg_send![
                self,
                newBufferWithBytes: pointer,
                length: length,
                options: options
            ];
            Id::new(raw_buffer).expect(ID_NEW_FAILURE)
        }
    }
    // [M] newBufferWithBytesNoCopy:length:options:deallocator:
    pub fn new_buffer_with_bytes_no_copy(
        &self,
        pointer: *const c_void,
        length: usize,
        options: ResourceOptions,
        deallocator: Option<&block::Block<(*const c_void, usize), ()>>,
    ) -> Id<Buffer> {
        unsafe {
            let raw_buffer: *mut Buffer = msg_send![
                self,
                newBufferWithBytesNoCopy: pointer,
                length: length,
                options: options,
                deallocator: deallocator
            ];
            Id::new(raw_buffer).expect(ID_NEW_FAILURE)
        }
    }
}

/// # Creating Textures
impl Device {
    // [M] newTextureWithDescriptor:
    pub fn new_texture_with_descriptor(&self, desc: &TextureDescriptor) -> Id<Texture> {
        unsafe {
            let raw_texture: *mut Texture = msg_send![self, newTextureWithDescriptor: desc];
            Id::new(raw_texture).expect(ID_NEW_FAILURE)
        }
    }
}
