use super::*;

declare!(Heap);

/// # Identifying the Heap
impl Label for Heap {}
impl Heap {
    // [P] device
    pub fn device(&self) -> Retained<Device> {
        unsafe { msg_send_id![self, device] }
    }
}

/// # Querying Heap Properties
impl Heap {
    // read all the properties specified in the descriptor

    // [P] usedSize
    pub fn used_size(&self) -> usize {
        unsafe { msg_send![self, usedSize] }
    }
    // [P] currentAllocatedSize
    pub fn current_allocated_size(&self) -> usize {
        unsafe { msg_send![self, currentAllocatedSize] }
    }
    // [P] maxAvailableSizeWithAlignment:
    pub fn max_available_size_with_alignment(&self, alignment: usize) -> usize {
        unsafe { msg_send![self, maxAvailableSizeWithAlignment: alignment] }
    }
}

/// # Setting the Puregeable State of the Resource
impl Heap {
    // [M] setPuregeableState:
    pub fn set_puregeable_state(&self, state: PurgeableState) {
        unsafe { msg_send![self, setPuregeableState: state] }
    }
}

/// # Creating Resources on the Heap
impl Heap {
    // [M] newBufferWithLength:options:
    pub fn new_buffer_with_length(
        &self,
        length: usize,
        options: ResourceOptions,
    ) -> Option<Retained<Buffer>> {
        unsafe {
            let raw_buffer: *mut Buffer =
                msg_send![self, newBufferWithLength: length, options: options];
            Retained::from_raw(raw_buffer)
        }
    }
    // [M] newBufferWithLength:options:offset:
    pub fn new_buffer_with_length_offset(
        &self,
        length: usize,
        options: ResourceOptions,
        offset: usize,
    ) -> Retained<Buffer> {
        unsafe {
            let raw_buffer: *mut Buffer = msg_send![
                self,
                newBufferWithLength: length,
                options: options,
                offset: offset
            ];
            Retained::from_raw(raw_buffer).expect(ID_NEW_FAILURE)
        }
    }
    // [M] newTextureWithDescriptor:
    pub fn new_texture_with_descriptor(
        &self,
        descriptor: &TextureDescriptor,
    ) -> Option<Retained<Texture>> {
        unsafe {
            let raw_texture: *mut Texture = msg_send![self, newTextureWithDescriptor: descriptor];
            Retained::from_raw(raw_texture)
        }
    }
    // [M] newTextureWithDescriptor:offset:
    pub fn new_texture_with_descriptor_offset(
        &self,
        descriptor: &TextureDescriptor,
        offset: usize,
    ) -> Retained<Texture> {
        unsafe {
            let raw_texture: *mut Texture =
                msg_send![self, newTextureWithDescriptor: descriptor, offset: offset];
            Retained::from_raw(raw_texture).expect(ID_NEW_FAILURE)
        }
    }
}
