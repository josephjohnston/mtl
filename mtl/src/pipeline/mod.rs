use super::*;

mod types;
pub use types::*;

// [C] MTLComputePipelineDescriptor
mod compute_descriptor;
pub use compute_descriptor::*;

mod compute_state;
pub use compute_state::*;

mod stage_input_output;
pub use stage_input_output::*;

// [C] MTLPipelineBufferDescriptor
declare!(PipelineBufferDescriptor);
impl PipelineBufferDescriptor {
    // [P] mutability and setMutability
    pub fn mutability(&self) -> Mutability {
        unsafe { msg_send![self, mutability] }
    }
    pub fn set_mutability(&self, mutability: Mutability) {
        unsafe { msg_send![self, setMutability: mutability] }
    }
}

// [C] MTLPipelineBufferDescriptorArray
declare!(PipelineBufferDescriptorArray);
impl PipelineBufferDescriptorArray {
    // [M] objectAtIndexSubscript:
    pub fn object_at_indexed_subscript(&self, index: usize) -> Retained<PipelineBufferDescriptor> {
        unsafe { msg_send_id![self, objectAtIndexedSubscript: index] }
    }
    // [M] setObject:atIndexedSubscript:
    pub fn set_object_at_indexed_subscript(&self, desc: &PipelineBufferDescriptor, index: usize) {
        unsafe { msg_send![self, setObject: desc, atIndexedSubscript: index] }
    }
}
