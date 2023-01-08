use super::*;

// [C] MTLAttributeDescriptor
declare!(AttributeDescriptor);
impl AttributeDescriptor {
    // [P] format and setFormat
    pub fn format(&self) -> AttributeFormat {
        unsafe { msg_send![self, format] }
    }
    pub fn set_format(&self, format: AttributeFormat) {
        unsafe { msg_send![self, setFormat: format] }
    }
    // [P] bufferIndex and setBufferIndex
    pub fn buffer_index(&self) -> usize {
        unsafe { msg_send![self, bufferIndex] }
    }
    pub fn set_buffer_index(&self, index: usize) {
        unsafe { msg_send![self, setBufferIndex: index] }
    }
    // [P] offset
    pub fn offset(&self) -> usize {
        unsafe { msg_send![self, offset] }
    }
    pub fn set_offset(&self, offset: usize) {
        unsafe { msg_send![self, setOffset: offset] }
    }
}

// [C] MTLAttributeDescriptorArray
declare!(AttributeDescriptorArray);
impl AttributeDescriptorArray {
    // [M] objectAtIndexedSubscript:
    pub fn object_at_index_subscript(&self, index: usize) -> Id<AttributeDescriptor> {
        unsafe { msg_send_id![self, objectAtIndexedSubscript: index] }
    }
    // [M] setObject:atIndexedSubscript:
    pub fn set_object_at_index_subscript(
        &self,
        attribute_desc: &AttributeDescriptor,
        index: usize,
    ) {
        unsafe { msg_send![self, setObject: attribute_desc, atIndexedSubscript: index] }
    }
}

// [C] MTLBufferLayoutDescriptor
declare!(BufferLayoutDescriptor);
impl BufferLayoutDescriptor {
    // [P] stride and setStride
    pub fn stride(&self) -> usize {
        unsafe { msg_send![self, stride] }
    }
    pub fn set_stride(&self, stride: usize) {
        unsafe { msg_send![self, setStride: stride] }
    }
    // [P] stepFunction and setStepFunction
    pub fn step_function(&self) -> StepFunction {
        unsafe { msg_send![self, stepFunction] }
    }
    pub fn set_step_function(&self, step_function: StepFunction) {
        unsafe { msg_send![self, setStepFunction: step_function] }
    }
    // [P] stepRate and setStepRate
    pub fn step_rate(&self) -> usize {
        unsafe { msg_send![self, stepRate] }
    }
    pub fn set_step_rate(&self, step_rate: usize) {
        unsafe { msg_send![self, setStepRate: step_rate] }
    }
}

// [C] MTLBufferLayoutDescriptorArray
declare!(BufferLayoutDescriptorArray);
impl BufferLayoutDescriptorArray {
    // [M] objectAtIndexedSubscript:
    pub fn object_at_indexed_subscript(&self, index: usize) -> Id<BufferLayoutDescriptor> {
        unsafe { msg_send_id![self, objectAtIndexedSubscript: index] }
    }
    // [M] setObject:atIndexedSubscript:
    pub fn set_object_at_indexed_subscript(
        &self,
        buffer_layout_desc: &BufferLayoutDescriptor,
        index: usize,
    ) {
        unsafe {
            msg_send![
                self,
                setObject: buffer_layout_desc,
                atIndexedSubscript: index
            ]
        }
    }
}

// [C] MTLStageInputOuputDescriptor
declare!(StageInputOutputDescriptor);
impl StageInputOutputDescriptor {
    // [M] stageInputOutputDescriptor
    pub fn stage_input_output_descriptor() -> Id<StageInputOutputDescriptor> {
        unsafe {
            msg_send_id![
                class!(MTLStageInputOutputDescriptor),
                stageInputOutputDescriptor
            ]
        }
    }
    // [P] attributes
    pub fn attributes(&self) -> Id<AttributeDescriptorArray> {
        unsafe { msg_send_id![self, attributes] }
    }
    // [P] layouts
    pub fn layouts(&self) -> Id<BufferLayoutDescriptorArray> {
        unsafe { msg_send_id![self, layouts] }
    }
    // [P] indexBufferIndex and setIndexBufferIndex
    pub fn index_buffer_index(&self) -> usize {
        unsafe { msg_send![self, indexBufferIndex] }
    }
    pub fn set_index_buffer_index(&self, index: usize) {
        unsafe { msg_send![self, setIndexBufferIndex: index] }
    }
    // [P] indexType
    pub fn index_type(&self) -> IndexType {
        unsafe { msg_send![self, indexType] }
    }
    pub fn set_index_type(&self, index_type: IndexType) {
        unsafe { msg_send![self, setIndexType: index_type] }
    }
    // [M] reset
    pub fn reset(&self) {
        unsafe { msg_send![self, reset] }
    }
}
