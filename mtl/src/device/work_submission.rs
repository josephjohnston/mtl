use super::*;

impl Device {
    // [M] newCommandQueue
    pub fn new_command_queue(&self) -> Retained<CommandQueue> {
        unsafe {
            // let raw_command_queue: *mut CommandQueue = msg_send![self, newCommandQueue];
            // Id::new(raw_command_queue).expect(ID_NEW_FAILURE)
            // newbie returning, seems we can replace with below
            msg_send_id![self, newCommandQueue]
        }
    }
    // [M] newCommandQueueWithMaxCommandBufferCount:
    pub fn new_command_queue_with_max_command_buffer_count(
        &self,
        max_command_buffer_count: usize,
    ) -> Retained<CommandQueue> {
        unsafe {
            // let raw_command_queue: *mut CommandQueue = msg_send![
            //     self,
            //     newCommandQueueWithMaxCommandBufferCount: max_command_buffer_count
            // ];
            // Id::new(raw_command_queue).expect(ID_NEW_FAILURE)
            msg_send_id![
                self,
                newCommandQueueWithMaxCommandBufferCount: max_command_buffer_count
            ]
        }
    }
    // [M] newIndirectCommandBufferWithDescriptor:maxCommandCount:options:
    pub fn new_indirect_command_buffer(
        &self,
        desc: &IndirectCommandBufferDescriptor,
        max_command_count: usize,
        options: ResourceOptions,
    ) -> Retained<IndirectCommandBuffer> {
        unsafe {
            let raw_indirect_command_buffer: *mut IndirectCommandBuffer = msg_send![
                self,
                newIndirectCommandBufferWithDescriptor: desc,
                maxCommandCount: max_command_count,
                options: options
            ];
            Retained::from_raw(raw_indirect_command_buffer).expect(ID_NEW_FAILURE)
        }
    }
    // Creating I/O Command Queues, iOS only 16.0+
}
