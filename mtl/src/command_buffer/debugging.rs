use super::*;

/// # Identifying the Command Buffer
impl Label for CommandBuffer {}
impl CommandBuffer {
    // [P] device
    pub fn device(&self) -> Id<Device> {
        unsafe { msg_send_id![self, device] }
    }
    // [P] commandQueue
    pub fn command_queue(&self) -> Id<CommandQueue> {
        unsafe { msg_send_id![self, commandQueue] }
    }
}

/// # Grouping GPU Commands within a GPU Frame Capture
impl CommandBuffer {
    // [M] pushDebugGroup:
    pub fn push_debug_group(&self, string: &NSString) {
        unsafe { msg_send![self, pushDebugGroup: string] }
    }
    // [M] popDebugGroup:
    pub fn pop_debug_group(&self) {
        unsafe { msg_send![self, popDebugGroup] }
    }
}

/// # Getting Error Details
impl CommandBuffer {
    // [P] error
    pub fn error(&self) -> Option<Id<NSError>> {
        unsafe { msg_send_id![self, error] }
    }
    // [P] errorOptions
    pub fn error_options(&self) -> CommandBufferErrorOption {
        unsafe { msg_send![self, errorOptions] }
    }
}

/// # Reading the Runtime Message Logs
impl CommandBuffer {
    // [P] logs
    pub fn logs(&self) -> Id<LogContainer> {
        unsafe { msg_send_id![self, logs] }
    }
}

/// # Checking Scheduling Times on the CPU
impl CommandBuffer {
    // [P] kernelStartTime
    pub fn kernel_start_time(&self) -> TimeInterval {
        unsafe { msg_send![self, kernelStartTime] }
    }
    // [P] kernelStartTime
    pub fn kernel_end_time(&self) -> TimeInterval {
        unsafe { msg_send![self, kernelEndTime] }
    }
}

/// # Checking Execution Times on the GPU
impl CommandBuffer {
    // [P] kernelStartTime
    pub fn gpu_start_time(&self) -> TimeInterval {
        unsafe { msg_send![self, kernelStartTime] }
    }
    // [P] kernelStartTime
    pub fn gpu_end_time(&self) -> TimeInterval {
        unsafe { msg_send![self, kernelEndTime] }
    }
}

/// # Determining Whether to Maintain Strong References
impl CommandBuffer {
    // [P] retainedReferences
    pub fn retained_references(&self) -> bool {
        unsafe { msg_send![self, retainedReferences] }
    }
}

// #[repr(C)]
declare!(LogContainer);

pub type TimeInterval = usize;
