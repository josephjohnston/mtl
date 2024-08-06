use super::*;

// [E] MTLIndirectCommandType
#[repr(usize)]
pub enum IndirectCommandType {
    ConcurrentDispatch = 1 << 5,
    ConcurrentDispatchThreads = 1 << 6,
}
impl_encode_for_type!(IndirectCommandType: usize);

// [C] MTLIndirectCommandBufferDescriptor
declare!(IndirectCommandBufferDescriptor);
impl IndirectCommandBufferDescriptor {
    pub fn new() -> Retained<Self> {
        unsafe { msg_send_id![class!(MTLIndirectCommandBufferDescriptor), new] }
    }
    // [P] commandTypes and setCommandTypes
    pub fn command_types(&self) -> IndirectCommandType {
        unsafe { msg_send![self, commandTypes] }
    }
    pub fn set_command_types(&self, command_types: IndirectCommandType) {
        unsafe { msg_send![self, setCommandTypes: command_types] }
    }
    // [P] inheritBuffers and setInheritBuffers
    pub fn inherit_buffers(&self) -> bool {
        unsafe { msg_send![self, inheritBuffers] }
    }
    pub fn set_inherit_buffers(&self, inherit_buffers: Bool) {
        unsafe { msg_send![self, setInheritBuffers: inherit_buffers] }
    }
    // [P] inheritPipelineState and setInheritPipelineState
    pub fn inherit_pipeline_state(&self) -> bool {
        unsafe { msg_send![self, inheritPipelineState] }
    }
    pub fn set_inherit_pipeline_state(&self, inherit_pipeline_state: Bool) {
        unsafe { msg_send![self, setInheritPipelineState: inherit_pipeline_state] }
    }
}

// [Pr] MTLIndirectCommandBuffer
declare!(IndirectCommandBuffer);
impl IndirectCommandBuffer {
    // [P] size
    pub fn size(&self) -> usize {
        unsafe { msg_send![self, size] }
    }
    // [M] indirectComputeCommandAtIndex:
    pub fn indirect_compute_command_at_index(
        &self,
        index: usize,
    ) -> Retained<IndirectComputeCommand> {
        unsafe { msg_send_id![self, indirectComputeCommandAtIndex: index] }
    }
    // [M] resetWithRange:
    pub fn reset_with_range(&self, range: NSRange) {
        unsafe { msg_send![self, resetWithRange: range] }
    }
}
