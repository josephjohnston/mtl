use super::*;

// [C] MTLComputePassDescriptor
declare!(ComputePassDescriptor);
impl ComputePassDescriptor {
    // [M] computePassDescriptor
    pub fn compute_pass_descriptor() -> Retained<ComputePassDescriptor> {
        unsafe { msg_send_id![class!(MTLComputePassDescriptor), computePassDescriptor] }
    }
    // [P] dispatchType and setDispatchType
    pub fn dispatch_type(&self) -> DispatchType {
        unsafe { msg_send![self, dispatchType] }
    }
    pub fn set_dispatch_type(&self, dispatch_type: DispatchType) {
        unsafe { msg_send![self, setDispatchType: dispatch_type] }
    }
    // [P] sampleBufferAttachments
    pub fn sample_buffer_attachments(
        &self,
    ) -> Retained<ComputePassSampleBufferAttachmentDescriptorArray> {
        unsafe { msg_send_id![self, sampleBufferAttachments] }
    }
}

// [Pr] MTLComputeCommandEncoder
declare!(ComputeCommandEncoder);
impl CommandEncoder for ComputeCommandEncoder {}
impl ComputeCommandEncoder {
    // [M] setComputePipelineState:
    pub fn set_compute_pipeline_state(&self, state: &ComputePipelineState) {
        unsafe { msg_send![self, setComputePipelineState: state] }
    }
    // [M] setBuffer:offset:atIndex:
    pub fn set_buffer(&self, buffer: &Buffer, offset: usize, index: usize) {
        unsafe { msg_send![self, setBuffer: buffer, offset: offset, atIndex: index] }
    }
    // [M] setBuffer:offset:withRange:
    pub fn set_buffers(&self, buffers: &[&Buffer], offsets: &[usize], range: NSRange) {
        assert_eq!(buffers.len(), offsets.len());
        unsafe {
            msg_send![
                self,
                setBuffers: buffers.as_ptr(),
                offsets: offsets.as_ptr(),
                withRange: range
            ]
        }
    }
    // [M] setBufferOffset:atIndex
    pub fn set_buffer_offset(&self, offset: usize, index: usize) {
        unsafe { msg_send![self, setBufferOffset: offset, atIndex: index] }
    }
    // [M] setBytes:length:atIndex
    pub fn set_bytes(&self, bytes: *const c_void, length: usize, index: usize) {
        unsafe { msg_send![self, setBytes: bytes, length: length, atIndex: index] }
    }
    // [M] setTexture:atIndex:
    pub fn set_texture(&self, texture: &Texture, index: usize) {
        unsafe { msg_send![self, setTexture: texture, atIndex: index] }
    }
    // [M] setTextures:range:
    pub fn set_textures(&self, textures: &[&Texture], start_index: usize) {
        let range = NSRange::new(start_index, textures.len());
        unsafe { msg_send![self, setTexutres: textures.as_ptr(), range: range] }
    }
    // [M] setThreadgroupMemoryLength:atIndex:
    pub fn set_threadgroup_memory_length(&self, length: usize, index: usize) {
        unsafe { msg_send![self, setThreadgroupMemoryLength: length, atIndex: index] }
    }
    // [M] setVisibleFunctionTable:atBufferIndex:
    pub fn set_visible_function_table(
        &self,
        visible_function_table: &VisibleFunctionTable,
        buffer_index: usize,
    ) {
        unsafe {
            msg_send![
                self,
                setVisibleFunctionTable: visible_function_table,
                atBufferIndex: buffer_index
            ]
        }
    }
    // [M] setVisibleFunctionTables:withBufferRange:
    pub fn set_visible_function_tables(
        &self,
        visible_function_tables: &[&VisibleFunctionTable],
        start_index: usize,
    ) {
        let range = NSRange::new(start_index, visible_function_tables.len());
        unsafe {
            msg_send![
                self,
                setVisibleFunctionTables: visible_function_tables.as_ptr(),
                withBufferRange: range
            ]
        }
    }
    // [M] dispatchThreads:threadsPerThreadgroup:
    pub fn dispatch_threads(&self, threads_per_grid: Size, threads_per_threadgroup: Size) {
        unsafe {
            msg_send![
                self,
                dispatchThreads: threads_per_grid,
                threadsPerThreadgroup: threads_per_threadgroup
            ]
        }
    }
    // [M] dispatchThreadgroups:threadsPerThreadgroup:
    pub fn dispatch_threadgroups(
        &self,
        threadgroups_per_grid: Size,
        threads_per_threadgroup: Size,
    ) {
        unsafe {
            msg_send![
                self,
                dispatchThreadgroups: threadgroups_per_grid,
                threadsPerThreadgroup: threads_per_threadgroup
            ]
        }
    }
    // [M] dispatchThreadgroupsWithIndirectBuffer:indirectBufferOffset:threadsPerThreadgroup:
    pub fn dispatch_threadgroups_with_indirect_buffer(
        &self,
        indirect_buffer: &Buffer,
        indirect_buffer_offset: usize,
        threads_per_threadgroup: Size,
    ) {
        unsafe {
            msg_send![
                self,
                dispatchThreadgroupsWithIndirectBuffer: indirect_buffer,
                indirectBufferOffset: indirect_buffer_offset,
                threadsPerThreadgroup: threads_per_threadgroup
            ]
        }
    }
    // [M] useResource:usage:
    pub fn use_resource(&self, resource: &Resource, usage: ResourceUsage) {
        unsafe { msg_send![self, useResource: resource, usage: usage] }
    }
    // [M] useResources:count:usage:
    pub fn use_resources(&self, resources: &[&Resource], usage: ResourceUsage) {
        unsafe {
            msg_send![self, useResources: resources.as_ptr(), count: resources.len(), usage: usage]
        }
    }
    // [M] useHeap:
    pub fn use_heap(&self, heap: &Heap) {
        unsafe { msg_send![self, useHeap: heap] }
    }
    // [M] useHeaps:count:
    pub fn use_heaps(&self, heaps: &[&Heap]) {
        unsafe { msg_send![self, useHeaps: heaps.as_ptr(), count: heaps.len()] }
    }
    // [M] setStateInRegion:
    pub fn set_stage_in_region(&self, region: Region) {
        unsafe { msg_send![self, setStateInRegion: region] }
    }
    // [P] dispatchType
    pub fn dispatch_type(&self) -> DispatchType {
        unsafe { msg_send![self, dispatchType] }
    }
    // [M] waitForFence:
    pub fn wait_for_fence(&self, fence: &Fence) {
        unsafe { msg_send![self, waitForFence: fence] }
    }
    // [M] updateFence:
    pub fn update_fence(&self, fence: &Fence) {
        unsafe { msg_send![self, updateFence: fence] }
    }
    // [M] memoryBarrierWithScope:
    pub fn memory_barrier_with_scope(&self, scope: BarrierScope) {
        unsafe { msg_send![self, memoryBarrierWithScope: scope] }
    }
    // [M] setImageblockWidth:height:
    pub fn set_imageblock_size(&self, width: usize, height: usize) {
        unsafe { msg_send![self, setImageblockWidth: width, height: height] }
    }
    // [M] sampleCountersInBuffer:atSampleIndex:withBarrier:
    pub fn sample_counters_in_buffer(
        &self,
        sample_buffer: &CounterSampleBuffer,
        sample_index: usize,
        barrier: bool,
    ) {
        unsafe {
            msg_send![
                self,
                sampleCountersInBuffer: sample_buffer,
                atSampleIndex: sample_index,
                withBarrier: Bool::from(barrier)
            ]
        }
    }
}

// [E] MTLBarrierScope
bitflags! {
    pub struct BarrierScope: usize {
        const Buffers = 1 << 0;
        const Textures = 1 << 1;
    }
}
impl_encode_for_type!(BarrierScope: usize);

// [S] MTLDispatchThreadgroupsIndirectArguments
#[repr(C)]
pub struct DispatchThreadgroupsIndirectArguments {
    threadgroups_per_grid: [u32; 3],
}
unsafe impl Encode for DispatchThreadgroupsIndirectArguments {
    const ENCODING: Encoding = Encoding::Struct(
        "DispatchThreadgroupsIndirectArguments",
        &[Encoding::Array(3, &u32::ENCODING)],
    );
}
impl DispatchThreadgroupsIndirectArguments {
    pub fn new(x: u32, y: u32, z: u32) -> Self {
        Self {
            threadgroups_per_grid: [x, y, z],
        }
    }
}
