use super::*;

// [E] MTLBlitOption
#[repr(usize)]
pub enum BlitOption {
    None = 0,
}
impl_encode_for_type!(BlitOption: usize);

// [C] MTLBlitPassDescriptor
declare!(BlitPassDescriptor);
impl BlitPassDescriptor {
    // [M] blitPassDescriptor
    pub fn blit_pass_descriptor() -> Retained<BlitPassDescriptor> {
        unsafe { msg_send_id![class!(MTLBlitPassDescriptor), blitPassDescriptor] }
    }
    // [P] sampleBufferAttachments
    pub fn sample_buffer_attachments(
        &self,
    ) -> Retained<BlitPassSampleBufferAttachmentDescriptorArray> {
        unsafe { msg_send_id![self, sampleBufferAttachments] }
    }
}

// [Pr] MTLBlitCommandEncoder
declare!(BlitCommandEncoder);
impl CommandEncoder for BlitCommandEncoder {}
impl BlitCommandEncoder {
    // [M] copyFromBuffer:sourceOffset:toBuffer:destinationOffset:size:
    pub fn copy_between_buffers(
        &self,
        source: &Buffer,
        source_offset: usize,
        dest: &Buffer,
        dest_offset: usize,
        size: usize,
    ) {
        unsafe {
            msg_send![
                self,
                copyFromBuffer: source,
                sourceOffset: source_offset,
                toBuffer: dest,
                destinationOffset: dest_offset,
                size: size
            ]
        }
    }
    // [M] copyFromBuffer:sourceOffset:sourceBytesPerRow:sourceBytesPerImage:sourceSize:toTexture:destinationSlice:destinationLevel:destinationOrigin:
    pub fn copy_from_buffer_to_texture(
        &self,
        buffer: &Buffer,
        offset: usize,
        bytes_per_row: usize,
        bytes_per_image: usize,
        size: Size,
        texture: &Texture,
        slice: usize,
        level: usize,
        origin: Origin,
    ) {
        unsafe {
            msg_send![
                self,
                copyFromBuffer: buffer,
                sourceOffset: offset,
                sourceBytesPerRow: bytes_per_row,
                sourceBytesPerImage: bytes_per_image,
                sourceSize: size,
                toTexture: texture,
                destinationSlice: slice,
                destinationLevel: level,
                destinationOrigin: origin
            ]
        }
    }
    // [M] copyFromTexture:sourceSlice:sourceLevel:sourceOrigin:sourceSize:toBuffer:destinationOffset:destinationBytesPerRow:destinationBytesPerImage:
    pub fn copy_from_texture_to_buffer(
        &self,
        texture: &Texture,
        slice: usize,
        level: usize,
        origin: Origin,
        size: Size,
        buffer: &Buffer,
        offset: usize,
        bytes_per_row: usize,
        bytes_per_image: usize,
    ) {
        unsafe {
            msg_send![
                self,
                copyFromTexture: texture,
                sourceSlice: slice,
                sourceLevel: level,
                sourceOrigin: origin,
                sourceSize: size,
                toBuffer: buffer,
                destinationOffset: offset,
                destinationBytesPerRow: bytes_per_row,
                destinationBytesPerImage: bytes_per_image
            ]
        }
    }
    // [M] copyFromTexture:sourceSlice:sourceLevel:sourceOrigin:sourceSize:toTexture:destinationSlice:destinationLevel:destinationOrigin:
    pub fn copy_between_textures(
        &self,
        source: &Texture,
        source_slice: usize,
        source_level: usize,
        source_origin: Origin,
        source_size: Size,
        dest: &Texture,
        dest_slice: usize,
        dest_level: usize,
        dest_origin: Origin,
    ) {
        unsafe {
            msg_send![
                self,
                copyFromTexture: source,
                sourceSlice: source_slice,
                sourceLevel: source_level,
                sourceOrigin: source_origin,
                sourceSize: source_size,
                toTexture: dest,
                destinationSlice: dest_slice,
                destinationLevel: dest_level,
                destinationOrigin: dest_origin
            ]
        }
    }
    // [M] copyFromTexture:sourceSlice:sourceLevel:toTexture:destinationSlice:destinationLevel:sliceCount:levelCount:
    pub fn copy_between_texture_slices(
        &self,
        source: &Texture,
        source_slice: usize,
        source_level: usize,
        dest: &Texture,
        dest_slice: usize,
        dest_level: usize,
        slice_count: usize,
        level_count: usize,
    ) {
        unsafe {
            msg_send![
                self,
                copyFromTexture: source,
                sourceSlice: source_slice,
                sourceLevel: source_level,
                toTexture: dest,
                destinationSlice: dest_slice,
                destinationLevel: dest_level,
                sliceCount: slice_count,
                levelCount: level_count
            ]
        }
    }
    // [M] fillBuffer:range:value:
    pub fn fill_buffer(&self, buffer: &Buffer, range: NSRange, value: u8) {
        unsafe { msg_send![self, fillBuffer: buffer, range: range, value: value] }
    }
    // indirect command buffers
    // [M] synchronizeResource:
    pub fn synchronize_managed_resource(&self, resource: &Resource) {
        unsafe { msg_send![self, synchronizeResource: resource] }
    }
    // [M] synchronizeTexture:slice:level:
    pub fn synchronize_managed_texture(&self, texture: &Texture, slice: usize) {
        unsafe { msg_send![self, synchronizeTexture: texture, slice: slice, level: 0] }
    }
    // [M] waitForFence:
    pub fn wait_for_fence(&self, fence: &Fence) {
        unsafe { msg_send![self, waitForFence: fence] }
    }
    // [M] updateFence:
    pub fn update_fence(&self, fence: &Fence) {
        unsafe { msg_send![self, updateFence: fence] }
    }
    // [M] optimizeContentsForGPUAccess:
    pub fn optimize_contents_for_gpu_access(&self, texture: &Texture) {
        unsafe { msg_send![self, optimizeContentsForGPUAccess: texture] }
    }
    // [M] optimizeContentsForGPUAccess:slice:level:
    pub fn optimize_contents_slice_for_gpu_access(&self, texture: &Texture, slice: usize) {
        unsafe { msg_send![self, optimizeContentsForGPUAccess: texture, slice: slice, level: 0] }
    }
    // [M] optimizeContentsForCPUAccess:
    pub fn optimize_contents_for_cpu_access(&self, texture: &Texture) {
        unsafe { msg_send![self, optimizeContentsForCPUAccess: texture] }
    }
    // [M] optimizeContentsForCPUAccess:slice:level:
    pub fn optimize_contents_slice_for_cpu_access(&self, texture: &Texture, slice: usize) {
        unsafe { msg_send![self, optimizeContentsForCPUAccess: texture, slice: slice, level: 0] }
    }
    // [M] sampleCountersInBuffer:atSampleIndex:withBarrier:
    pub fn sample_counter_in_buffer(
        &self,
        sample_buffer: &CounterSampleBuffer,
        sample_index: usize,
        barrier: bool,
    ) {
        unsafe {
            msg_send![
                self,
                sampleCountersInBuffer: sample_buffer,
                atSampleIndex: sample_index,
                withBarrier: Bool::new(barrier)
            ]
        }
    }
    // [M] resolveCounters:inRange:destinationBuffer:destinationOffset:
    // other counter methods
}
