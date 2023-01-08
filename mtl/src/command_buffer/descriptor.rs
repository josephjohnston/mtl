use super::*;

// [C] MTLCommandBufferDescriptor
declare!(CommandBufferDescriptor);
impl CommandBufferDescriptor {
    pub fn new() -> Id<CommandBufferDescriptor> {
        unsafe { msg_send_id![class!(MTLCommandBufferDescriptor), new] }
    }
    // [P] retainedReferences and setRetainedReferences
    pub fn retained_references(&self) -> bool {
        unsafe { msg_send![self, retainedReferences] }
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
