use super::*;

// [C] MTLCommandBufferDescriptor
declare!(CommandBufferDescriptor);
impl CommandBufferDescriptor {
    pub fn new() -> Retained<CommandBufferDescriptor> {
        unsafe { msg_send_id![class!(MTLCommandBufferDescriptor), new] }
    }
    // [P] retainedReferences and setRetainedReferences
    pub fn retained_references(&self) -> bool {
        unsafe { Bool::as_bool(msg_send![self, retainedReferences]) }
    }
    pub fn set_retained_references(&self, retain_references: bool) {
        unsafe { msg_send![self, setRetainedReferences: Bool::new(retain_references)] }
    }
    // [P] errorOptions and setErrorOptions
    pub fn error_options(&self) -> CommandBufferErrorOption {
        unsafe { msg_send![self, errorOptions] }
    }
    pub fn set_error_options(&self, error_options: CommandBufferErrorOption) {
        unsafe { msg_send![self, setErrorOptions: error_options] }
    }
}

// [Pr] MTLCommandBuffer
declare!(CommandBuffer);

// [Pr] MTLEvent
declare!(Event);
impl Label for Event {}
impl Event {
    pub fn device(&self) -> Retained<Device> {
        unsafe { msg_send_id![self, device] }
    }
}

// Submitting a Command Buffer
impl CommandBuffer {
    // [M] enqueue
    pub fn enqueue(&self) {
        unsafe { msg_send![self, enqueue] }
    }
    // [M] commit
    pub fn commit(&self) {
        unsafe { msg_send![self, commit] }
    }
}

// Synchronizing Passes With Events
impl CommandBuffer {
    // [M] encodeWaitForEvent:value:
    pub fn encode_wait_for_event(&self, event: &Event, value: u64) {
        unsafe { msg_send![self, encodeWaitForEvent: event, value: value] }
    }
    // [M] encodeSignalEvent:value:
    pub fn encode_signal_event(&self, event: &Event, value: u64) {
        unsafe { msg_send![self, encodeSignalEvent: event, value: value] }
    }
}

// Waiting for State Changes
impl CommandBuffer {
    // [M] waitUntilScheduled
    pub fn wait_until_scheduled(&self) {
        unsafe { msg_send![self, waitUntilScheduled] }
    }
    // [M] waitUntilCompleted
    pub fn wait_until_completed(&self) {
        unsafe { msg_send![self, waitUntilCompleted] }
    }
}

// Registering State Change Handlers
pub type CommandBufferHandler<'a> = block::Block<(&'a CommandBuffer,), ()>;
impl CommandBuffer {
    // [P] addScheduledHandler
    pub fn add_scheduled_handler(&self, block: &CommandBufferHandler) {
        unsafe { msg_send![self, addScheduledHandler: block] }
    }
    // [P] addCompletedHandler:
    pub fn add_completed_handler(&self, block: &CommandBufferHandler) {
        unsafe { msg_send![self, addCompletedHandler: block] }
    }
}

// [E] CommandBufferStatus
#[repr(usize)]
pub enum CommandBufferStatus {
    NotEnqueued = 0,
    Enqueued = 1,
    Committed = 2,
    Scheduled = 3,
    Completed = 4,
    StatusError = 5,
}
impl_encode_for_type!(CommandBufferStatus: usize);

impl CommandBuffer {
    // [P] status
    pub fn status(&self) -> CommandBufferStatus {
        unsafe { msg_send![self, status] }
    }
}

// Creating Command Encoders
mod creating_encoders;
pub use creating_encoders::*;

// Command Buffer Debugging
mod debugging;
pub use debugging::*;

// // [Pr] MTLIOCommandBuffer
// mod io_command_buffer;
// pub use io_command_buffer::*;

// Indirect Command Buffers
mod indirect;
pub use indirect::*;
