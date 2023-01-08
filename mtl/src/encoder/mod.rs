use super::*;

mod counter;
pub use counter::*;

// [Pr] MTLFence
declare!(Fence);
impl Label for Fence {}
impl Fence {
    pub fn device(&self) -> Id<Device> {
        unsafe { msg_send_id![self, device] }
    }
}

// [E] MTLResourceUsage
#[repr(usize)]
pub enum ResourceUsage {
    Read = 1 << 0,
    Write = 1 << 1,
}
unsafe impl Encode for ResourceUsage {
    const ENCODING: Encoding = usize::ENCODING;
}

// [Pr] MTLCommandEncoder
pub trait CommandEncoder
where
    Self: Message,
{
    // Identifying the Command Encoder: inherited from ObjectUtil
    // End Command Encoding
    // [M] endEncoding
    fn end_encoding(&self) {
        unsafe { msg_send![self, endEncoding] }
    }
    // Annotating the Command Buffer with Debug Information
    // [M] insertDebugSignpost:
    fn insert_debug_signpost(&self, string: &str) {
        unsafe { msg_send![self, insertDebugSignpost: NSString::from_str(string).as_ref()] }
    }
    // [M] pushDebugGroup:
    fn push_debug_group(&self, string: &str) {
        unsafe { msg_send![self, pushDebugGroup: NSString::from_str(string).as_ref()] }
    }
    // [M] popDebugGroup:
    fn pop_debug_group(&self) {
        unsafe { msg_send![self, popDebugGroup] }
    }
}

// Compute Passes
mod compute;
pub use compute::*;

mod indirect;
pub use indirect::*;

// mod render;
// pub use render::*;

mod blit;
pub use blit::*;
