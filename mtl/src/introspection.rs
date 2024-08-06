use super::*;

// [C] MTLComputePipelineReflection
declare!(ComputePipelineReflection);
impl ComputePipelineReflection {
    // [P] bindings
    // iOS only 16.0+
    pub fn bindings(&self) -> Retained<NSArray<Binding>> {
        unsafe { msg_send_id![self, bindings] }
    }
}
// [T] MTLAutoreleasedComputePipelineReflection
pub type AutoreleasedComputePipelineReflection = Retained<ComputePipelineReflection>;
// we'll just want a double pointer when using this as an arg, see about it then
// arguments will be ComputePipelineReflection **

// [E] MTLBindingType
#[repr(isize)]
pub enum BindingType {
    Buffer = 0,
    ThreadgroupMemory = 1,
    Texture = 2,
    VisibleFunctionTable = 24,
}
impl_encode_for_type!(BindingType: isize);

// [Pr] MTLBinding
// iOS only 16.0+
declare!(Binding);
impl Binding {
    // [P] name
    pub fn name(&self) -> Retained<NSString> {
        unsafe { msg_send_id![self, name] }
    }
    // [P] index
    pub fn index(&self) -> usize {
        unsafe { msg_send![self, index] }
    }
    // [P] access
    pub fn access(&self) -> ArgumentAccess {
        unsafe { msg_send![self, access] }
    }
    // [P] isArgument
    pub fn is_argument(&self) -> Bool {
        unsafe { msg_send![self, isArgument] }
    }
    // [P] isUsed
    pub fn is_used(&self) -> Bool {
        unsafe { msg_send![self, isUsed] }
    }
}

// [Pr] MTLBufferBinding
// iOS only 16.0+
declare!(BufferBinding);
impl BufferBinding {
    // [P] bufferAlginment
    // [P] bufferDataSize
    // [P] bufferDataType
    // [P] bufferPointerType
    // [P] bufferStructType
}

// [Pr] MTLTextureBinding
// iOS only 16.0+
declare!(TextureBinding);
impl TextureBinding {
    // [P] arrayLength
    // [P] textureDataType
    // [P] textureType
}

// [Pr] MTLThreadgroupBinding
// iOS only 16.0+
declare!(ThreadgroupBinding);
impl ThreadgroupBinding {
    // [P] threadgroupMemoryAlignment
    // [P] threadgroupMemoryDataSize
}

// but maybe these apis can help us see what we should be encoding functions.
