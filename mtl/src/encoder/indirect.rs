use super::*;

// [Pr] MTLIndirectComputeCommand
declare!(IndirectComputeCommand);
impl IndirectComputeCommand {
    // [M] setComputePipelineState
    pub fn set_compute_pipeline_state(&self, pipeline_state: &ComputePipelineState) {
        unsafe { msg_send![self, setComputePipelineState: pipeline_state] }
    }
    // [M] setKernelBuffer:offset:atIndex:
    pub fn set_kernel_buffer(&self, buffer: &Buffer, offset: usize, index: usize) {
        unsafe {
            msg_send![
                self,
                setKernelBuffer: buffer,
                offset: offset,
                atIndex: index
            ]
        }
    }
    // [M] setThreadgroupMemoryLength:atIndex:
    pub fn set_threadgroup_memory_length(&self, length: usize, index: usize) {
        unsafe { msg_send![self, setThreadgroupMemoryLength: length, atIndex: index] }
    }
    // [M] setStageInRegion:
    pub fn set_stage_in_region(&self, region: Region) {
        unsafe { msg_send![self, setStageInRegion: region] }
    }
    // [M] setBarrier
    pub fn set_barrier(&self) {
        unsafe { msg_send![self, setBarrier] }
    }
    // [M] clearBarrier
    pub fn clear_barrier(&self) {
        unsafe { msg_send![self, clearBarrier] }
    }
    // [M] concurrentDispatchThreadgroups:threadsPerThreadgroup:
    pub fn concurrent_dispatch_threadgroups(
        &self,
        threadgroups_per_grid: Size,
        threads_per_threadgroup: Size,
    ) {
        unsafe {
            msg_send![
                self,
                concurrentDispatchThreadgroups: threadgroups_per_grid,
                threadsPerThreadgroup: threads_per_threadgroup
            ]
        }
    }
    // [M] concurrentDispatchThreads:threadsPerThreadgroup:
    pub fn concurrent_dispatch_thread(
        &self,
        threadgroups_per_grid: Size,
        threads_per_threadgroup: Size,
    ) {
        unsafe {
            msg_send![
                self,
                concurrentDispatchThreads: threadgroups_per_grid,
                threadsPerThreadgroup: threads_per_threadgroup
            ]
        }
    }
    // [M] reset
    pub fn reset(&self) {
        unsafe { msg_send![self, reset] }
    }
}
