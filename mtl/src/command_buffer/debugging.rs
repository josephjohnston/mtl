use super::*;

// [E] CommandBufferError
#[repr(usize)]
pub enum CommandBufferError {
    None = 0,
    Internal = 1,
    Timeout = 2,
    PageFault = 3,
    AccessRevoked = 4,
    NotPermitted = 7,
    OutOfMemory = 8,
    InvalidResource = 9,
    Memoryless = 10,
    // iOS unavailable
    DeviceRemoved = 11,
    // iOS only 15.0+
    StackOverflow = 12,
}

// Identifying the Command Buffer
impl Label for CommandBuffer {}
impl CommandBuffer {
    // [P] device
    pub fn device(&self) -> Retained<Device> {
        unsafe { msg_send_id![self, device] }
    }
    // [P] commandQueue
    pub fn command_queue(&self) -> Retained<CommandQueue> {
        unsafe { msg_send_id![self, commandQueue] }
    }
}

// Grouping GPU Commands within a GPU Frame Capture
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

// [E] CommandBufferErrorOption
// iOS only 14.0+
#[repr(usize)]
pub enum CommandBufferErrorOption {
    None = 0,
    EncoderExecutionStatus = 1 << 0,
}
impl_encode_for_type!(CommandBufferErrorOption: usize);

// Getting Error Details
impl CommandBuffer {
    // [P] error
    pub fn error(&self) -> Option<Retained<NSError>> {
        unsafe { msg_send_id![self, error] }
    }
    // [P] errorOptions
    pub fn error_options(&self) -> CommandBufferErrorOption {
        unsafe { msg_send![self, errorOptions] }
    }
}

declare!(LogContainer);

// Reading the Runtime Message Logs
impl CommandBuffer {
    // [P] logs
    pub fn logs(&self) -> Retained<LogContainer> {
        unsafe { msg_send_id![self, logs] }
    }
}

pub type TimeInterval = usize;

// Checking Scheduling Times on the CPU
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

// Checking Execution Times on the GPU
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

// Determining Whether to Maintain Strong References
impl CommandBuffer {
    // [P] retainedReferences
    pub fn retained_references(&self) -> bool {
        unsafe { msg_send![self, retainedReferences] }
    }
}

// [E] MTLCommandEncoderErrorState
// iOS only 14.0+
#[repr(isize)]
pub enum CommandEncoderErrorState {
    Unknown = 0,
    Completed = 1,
    Affected = 2,
    Pending = 3,
    Faulted = 4,
}
impl_encode_for_type!(CommandEncoderErrorState: usize);

// // [Pr] CommandBufferEncoderInfo
// // iOS only 14.0+
// declare!(CommandBufferEncoderInfo);

// /// # Inspecting Execution Information
// impl CommandBufferEncoderInfo {
//     // [P] label and setLable:
//     pub fn label(&self) -> Retained<NSString, Shared> {
//         unsafe { Id::retain(msg_send![self, label]).expect(ID_RETAIN_FAILURE) }
//     }
//     pub fn set_label(&self, label: &str) {
//         unsafe { msg_send![self, setLabel: NSString::from_str(label).as_ref()] }
//     }
//     // [P] debugSignposts
//     pub fn debug_signposts(&self) -> Retained<NSArray<*mut NSString>, Shared> {
//         unsafe {
//             let raw_array: *mut NSArray<*mut NSString> = msg_send![self, debugSignposts];
//             Id::retain(raw_array).expect(ID_RETAIN_FAILURE)
//         }
//     }
//     // [P] errorState
//     pub fn error_state(&self) -> CommandEncoderErrorState {
//         unsafe { msg_send![self, errorState] }
//     }
// }
