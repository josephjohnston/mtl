use super::*;

// pub enum ArgType {
//     Bytes,
//     Buffer,
//     Texture,
//     ThreadgroupMemory,
// }
// pub struct Arg {
//     arg_type: ArgType,
//     resource: Resource,
//     offset: usize,
// }
// impl Arg {
//     pub fn bytes() {

//     }
//     pub fn buffer(buffer: &Buffer, ) -> Self {
//         Self {
//             arg_type: ArgType::Buffer,
//             resource: buffer,
//             offset
//             stage_index:
//         }
//     }
// }

pub struct ComputeEncoder {
    id: Id<mtl::ComputeCommandEncoder>,
}
impl ComputeEncoder {
    pub fn new(cmd_buffer: &mtl::CommandBuffer, desc: &mtl::ComputePassDescriptor) -> Self {
        // desc sample buffer attachments
        let id = cmd_buffer.compute_command_encoder_with_descriptor(&desc);
        Self { id }
    }
    pub fn set_pipeline(&self, pipeline: &Pipeline) {
        self.id.set_compute_pipeline_state(pipeline.get_mtl())
    }
    pub fn set_imageblock_size(&self, width: usize, height: usize) {
        self.id.set_imageblock_size(width, height);
    }
    pub fn set_threadgroup_memory_length(&self, length: usize, index: usize) {
        self.id.set_threadgroup_memory_length(length, index);
    }
    pub fn set_bytes(&self, index: usize, bytes: *const std::ffi::c_void, length: usize) {
        self.id.set_bytes(bytes, length, index);
    }
    pub fn set_buffer<T, E: Env>(&self, index: usize, buffer: &Buffer<T, E>, offset: usize) {
        self.id.set_buffer(buffer.get_mtl(), offset, index);
    }
    pub fn set_texture(&self, index: usize, texture: &Texture) {
        self.id.set_texture(texture.get_mtl(), index)
    }
    pub fn dispatch(&self, threadgroup_size: Size, grid_size: Size) {
        self.id.dispatch_threadgroups(grid_size, threadgroup_size);
    }
    pub fn barrier(&self, scope: BarrierScope) {
        self.id.memory_barrier_with_scope(scope);
    }
    pub fn set_counter(
        &self,
        sample_buffer: &mtl::CounterSampleBuffer,
        index: usize,
        barrier: bool,
    ) {
        self.id
            .sample_counters_in_buffer(sample_buffer, index, barrier);
    }
    pub fn end_encoding(&self) {
        self.id.end_encoding();
    }
}

// indirect
