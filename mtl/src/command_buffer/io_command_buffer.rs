use super::*;

declare!(IOCommandBuffer);

/// # Loading Assets
impl IOCommandBuffer {
    // [M] loadBuffer:offset:size:sourceHandle:sourceHandleOffset:
    pub fn load_buffer(
        &self,
        buffer: &Buffer,
        offset: usize,
        size: usize,
        source_handle: &IOFileHandler,
        source_handle_offset: usize,
    ) {
        unsafe {
            msg_send![
                self,
                loadBuffer: buffer,
                offset: offset,
                size: size,
                sourceHandle: source_handle,
                sourceHandleOffset: source_handle_offset
            ]
        }
    }
}

//
declare!(IOFileHandler);
