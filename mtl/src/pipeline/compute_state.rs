use super::*;

// [Pr] MTLComputePipelineState
declare!(ComputePipelineState);
impl Label for ComputePipelineState {}
impl ComputePipelineState {
    // [P] device
    pub fn device(&self) -> Id<Device> {
        unsafe { msg_send_id![self, device] }
    }
    // [P] GPUResourceId
    pub fn gpu_resource_id(&self) -> GPUResourceID {
        unsafe { msg_send![self, GPUResourceID] }
    }
    // [P] maxTotalThreadsPerThreadgroup
    pub fn max_total_threads_per_threadgroup(&self) -> usize {
        unsafe { msg_send![self, maxTotalThreadsPerThreadgroup] }
    }
    // [P] threadExecutionWidth
    pub fn thread_execution_width(&self) -> usize {
        unsafe { msg_send![self, threadExecutionWidth] }
    }
    // [P] staticThreadgroupMemoryLength
    pub fn static_threadgroup_memory_length(&self) -> usize {
        unsafe { msg_send![self, staticThreadgroupMemoryLength] }
    }
    // [P] supportIndirectCommandBuffers
    pub fn support_indirect_command_buffers(&self) -> bool {
        unsafe { msg_send![self, supportIndirectCommandBuffers] }
    }
    // [M] functionHandleWithFunction:
    pub fn function_handle_with_function(&self, function: &Function) -> Id<FunctionHandle> {
        unsafe { msg_send_id![self, functionHandleWithFunction: function] }
    }
    // [M] newComputePipelineStateWithAdditionalBinaryFunctions:error:
    pub fn new_compute_pipeline_state_with_additional_binary_functions(
        &self,
        functions: &[Id<Function>],
    ) -> TryNewCatch<ComputePipelineState> {
        let ns_array_functions = NSArray::from_slice(functions);
        try_new_catch!(raw_error<ComputePipelineState> =>msg_send![
            self,
            newComputePipelineStateWithAdditionalBinaryFunctions: &*ns_array_functions,
            error: &mut raw_error
        ])
    }
    // [M] newVisibleFunctionTableWithDescriptor:
    pub fn new_visible_function_table_with_descriptor(
        &self,
        descriptor: &VisibleFunctionTableDescriptor,
    ) -> Id<VisibleFunctionTable> {
        unsafe {
            let raw_table: *mut VisibleFunctionTable =
                msg_send![self, newVisibleFunctionTableWithDescriptor: descriptor];
            Id::new(raw_table).expect(ID_NEW_FAILURE)
        }
    }
}