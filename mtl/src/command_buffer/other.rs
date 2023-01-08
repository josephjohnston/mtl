use super::*;

// [Pr] MTLEvent
declare!(Event);
impl Label for Event {}
impl Event {
    pub fn device(&self) -> Id<Device> {
        unsafe { msg_send_id![self, device] }
    }
}

/// # Synchronizing Passes With Events
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

/// # Waiting for State Changes
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

/// # Registering State Change Handlers
pub type CommandBufferHandler<'a> = block::Block<(&'a CommandBuffer,), ()>;
impl CommandBuffer {
    // [P] addScheduledHandler: CommandBufferHandler
    pub fn add_scheduled_handler(&self, block: &CommandBufferHandler) {
        unsafe { msg_send![self, addScheduledHandler: block] }
    }
    // [P] addCompletedHandler:
    pub fn add_completed_handler(&self, block: &CommandBufferHandler) {
        unsafe { msg_send![self, addCompletedHandler: block] }
    }
}

/// # Submitting a Command Buffer
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
