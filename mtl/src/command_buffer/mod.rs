use super::*;

// [Pr] MTLIOCommandBuffer
mod io_command_buffer;
pub use io_command_buffer::*;

// type *NSErrorDomain = NSString;
// type CommandBufferErrorDomain = NSErrorDomain;

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

// API_AVAILABLE(macos(11.0), ios(14.0))
// MTL_EXTERN NSErrorUserInfoKey const MTLCommandBufferEncoderInfoErrorKey;

// [E] CommandBufferErrorOption
// iOS only 14.0+
#[repr(usize)]
pub enum CommandBufferErrorOption {
    None = 0,
    EncoderExecutionStatus = 1 << 0,
}
unsafe impl Encode for CommandBufferErrorOption {
    const ENCODING: Encoding = usize::ENCODING;
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
unsafe impl Encode for CommandEncoderErrorState {
    const ENCODING: Encoding = isize::ENCODING;
}

// [Pr] CommandBufferEncoderInfo
// iOS only 14.0+
declare!(CommandBufferEncoderInfo);

/// # Inspecting Execution Information
impl CommandBufferEncoderInfo {
    // // [P] label and setLable:
    // pub fn label(&self) -> Id<NSString, Shared> {
    //     unsafe { Id::retain(msg_send![self, label]).expect(ID_RETAIN_FAILURE) }
    // }
    // pub fn set_label(&self, label: &str) {
    //     unsafe { msg_send![self, setLabel: NSString::from_str(label).as_ref()] }
    // }
    // [P] debugSignposts
    // pub fn debug_signposts(&self) -> Id<NSArray<*mut NSString>, Shared> {
    //     unsafe {
    //         let raw_array: *mut NSArray<*mut NSString> = msg_send![self, debugSignposts];
    //         Id::retain(raw_array).expect(ID_RETAIN_FAILURE)
    //     }
    // }
    // [P] errorState
    pub fn error_state(&self) -> CommandEncoderErrorState {
        unsafe { msg_send![self, errorState] }
    }
}

// If the command buffer has errorOptions CommandBufferErrorOption::EncoderExecutionStatus,
// then on the event of an error, the command buffer's error field is an NSError with userInfo field an NSDictionary with the MTLCommandBufferEncoderInfoErrorKey key yielding a MTLCommandBufferEncoderInfo instance.

mod descriptor;
pub use descriptor::*;

// [Pr] MTLCommandBuffer
declare!(CommandBuffer);

// Creating Command Encoders

mod creating_encoders;
pub use creating_encoders::*;

// Troubleshooting a Command Buffer

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

// Command Buffer Debugging

mod debugging;
// use debugging::*;

// Other

mod other;
pub use other::*;

// https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ErrorHandlingCocoa/ErrorObjectsDomains/ErrorObjectsDomains.html#//apple_ref/doc/uid/TP40001806-CH202-CJBGAIBJ

// Indirect Command Buffers

mod indirect;
pub use indirect::*;
