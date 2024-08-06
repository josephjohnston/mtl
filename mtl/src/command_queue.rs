use super::*;

// [Pr] MTLCommandQueue
declare!(CommandQueue);
impl Label for CommandQueue {}
impl CommandQueue {
    // [P] device
    pub fn device(&self) -> Retained<Device> {
        unsafe { msg_send_id![self, device] }
    }
    // [M] commandBuffer
    pub fn command_buffer(&self) -> Retained<CommandBuffer> {
        unsafe { msg_send_id![self, commandBuffer] }
    }
    // [M] commandBufferWithDescriptor:
    pub fn command_buffer_with_descriptor(
        &self,
        descriptor: &CommandBufferDescriptor,
    ) -> Retained<CommandBuffer> {
        unsafe { msg_send_id![self, commandBufferWithDescriptor: descriptor] }
    }
    // [M] commandBufferWithUnretainedReferences
    pub fn command_buffer_with_unretained_references(&self) -> Retained<CommandBuffer> {
        unsafe { msg_send_id![self, commandBufferWithUnretainedReferences] }
    }
}

// [C] MTLIOCommandQueueDescriptor
declare!(IOCommandQueueDescriptor);

// [Pr] MTLIOCommandQueue
declare!(IOCommandQueue);
impl Label for IOCommandQueue {}

// /// # Creating an Input/Output Command Buffer
// pub fn command_buffer(&self) -> Retained<IOCommandBuffer> {
// }
