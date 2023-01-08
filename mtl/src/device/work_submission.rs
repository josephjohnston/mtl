use super::*;

impl Device {
    // [M] newCommandQueue
    pub fn new_command_queue(&self) -> Id<CommandQueue> {
        unsafe {
            let raw_command_queue: *mut CommandQueue = msg_send![self, newCommandQueue];
            Id::new(raw_command_queue).expect(ID_NEW_FAILURE)
        }
    }
    // [M] newCommandQueueWithMaxCommandBufferCount:
    pub fn new_command_queue_with_max_command_buffer_count(
        &self,
        max_command_buffer_count: usize,
    ) -> Id<CommandQueue> {
        unsafe {
            let raw_command_queue: *mut CommandQueue = msg_send![
                self,
                newCommandQueueWithMaxCommandBufferCount: max_command_buffer_count
            ];
            Id::new(raw_command_queue).expect(ID_NEW_FAILURE)
        }
    }
    // [M] newIndirectCommandBufferWithDescriptor:maxCommandCount:options:
    pub fn new_indirect_command_buffer(
        &self,
        desc: &IndirectCommandBufferDescriptor,
        max_command_count: usize,
        options: ResourceOptions,
    ) -> Id<IndirectCommandBuffer> {
        unsafe {
            let raw_indirect_command_buffer: *mut IndirectCommandBuffer = msg_send![
                self,
                newIndirectCommandBufferWithDescriptor: desc,
                maxCommandCount: max_command_count,
                options: options
            ];
            Id::new(raw_indirect_command_buffer).expect(ID_NEW_FAILURE)
        }
    }
    // Creating I/O Command Queues, iOS only 16.0+
}
