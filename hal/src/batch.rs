use super::*;

// what's the design?
// we have 'batches' (command buffers)
// batches create 'passes'
// a pass either is a compute or blit pass

// pub enum BatchStatus {
//     NotEnqueued = 0,
//     Enqueued = 1,
//     Committed = 2,
//     Scheduled = 3,
//     Completed = 4,
//     StatusError = 5,
// }
type BatchStatus = mtl::CommandBufferStatus;
type Event = mtl::Event;

pub struct Batch {
    id: Id<mtl::CommandBuffer>,
    blit_pass_desc: Id<mtl::BlitPassDescriptor>,
    compute_pass_desc: Id<mtl::ComputePassDescriptor>,
    // attachments: Id<
}

impl Batch {
    pub(crate) fn new(
        queue: &mtl::CommandQueue,
        desc: &mtl::CommandBufferDescriptor,
        optimize: bool,
    ) -> Self {
        if optimize {
            desc.set_retained_references(false);
        }
        desc.set_error_options(match optimize {
            false => mtl::CommandBufferErrorOption::EncoderExecutionStatus,
            true => mtl::CommandBufferErrorOption::None,
        });
        let id = queue.command_buffer_with_descriptor(&desc);
        // let block = block::ConcreteBlock::new(move |_: &mtl::CommandBuffer| {}).copy();
        // id.add_scheduled_handler(&block);
        // id.add_completed_handler();
        let blit_pass_desc = mtl::BlitPassDescriptor::blit_pass_descriptor();
        let compute_pass_desc = mtl::ComputePassDescriptor::compute_pass_descriptor();
        // compute_pass_desc.set_dispatch_type(mtl::DispatchType::Concurrent);
        let attachments = compute_pass_desc.sample_buffer_attachments();
        let attachment_desc = attachments.object_at_indexed_subscript(0);

        Self {
            id,
            blit_pass_desc,
            compute_pass_desc,
            // attachments,
        }
    }
    pub fn new_blit_pass(&self) -> BlitPass {
        BlitPass::new(&self.id, &self.blit_pass_desc)
    }
    pub fn new_compute_pass(&self) -> ComputeEncoder {
        ComputeEncoder::new(&self.id, &self.compute_pass_desc)
    }
    pub fn status(&self) -> BatchStatus {
        // match self.id.status() {
        //     mtl::CommandBufferStatus::StatusError =>
        // }
        self.id.status()
    }
    pub fn enqueue(&self) {
        self.id.enqueue();
    }
    pub fn commit(&self) {
        self.id.commit();
    }
    pub fn wait_until_scheduled(&self) {
        self.id.wait_until_scheduled()
    }
    pub fn wait_until_completed(&self) {
        self.id.wait_until_completed()
    }
    // handlers
    pub fn encode_wait_for_event(&self, event: &Event, value: u64) {
        self.id.encode_wait_for_event(event, value);
    }
    pub fn encode_signal_event(&self, event: &Event, value: u64) {
        self.id.encode_signal_event(event, value);
    }
}

// indirect
// can

pub struct MultiBatch {
    id: Id<mtl::IndirectCommandBuffer>,
    inherit_buffers: bool,
    inherit_pipeline: bool,
    max_command_count: usize,
}
impl MultiBatch {
    pub(crate) fn new(
        device: &mtl::Device,
        desc: &mtl::IndirectCommandBufferDescriptor,
        max_command_count: usize,
    ) -> Self {
        let command_types = mtl::IndirectCommandType::ConcurrentDispatch; // i don't think we want DispatchThreads, for now
        desc.set_command_types(command_types);
        let inherit_pipeline = false;
        desc.set_inherit_pipeline_state(Bool::from(inherit_pipeline));
        let inherit_buffers = false;
        desc.set_inherit_buffers(Bool::from(inherit_buffers));

        let id = device.new_indirect_command_buffer(
            &desc,
            max_command_count,
            mtl::ResourceOptions::new(
                mtl::CPUCacheMode::DefaultCache,
                mtl::StorageMode::Shared,
                mtl::HazardTrackingMode::Untracked,
            ),
        );
        Self {
            id,
            inherit_buffers,
            inherit_pipeline,
            max_command_count,
        }
    }
    pub fn configure_command(&self, index: usize) {
        assert!(index < self.max_command_count);
        let command = self.id.indirect_compute_command_at_index(index);
        // command.set_compute_pipeline_state(pipeline_state);
        // command.
    }
}

pub struct IndirectComputeCommand {
    id: Id<mtl::IndirectComputeCommand>,
}
impl IndirectComputeCommand {
    pub fn set_pipeline(&self, pipeline: Pipeline) {
        self.id.set_compute_pipeline_state(pipeline.get_mtl());
    }
    pub fn set_buffer<T, E: Env>(&self, index: usize, buffer: &Buffer<T, E>, offset: usize) {
        self.id.set_kernel_buffer(buffer.get_mtl(), offset, index);
    }
    pub fn set_threadgroup_memory_length(&self, index: usize, length: usize) {
        self.id.set_threadgroup_memory_length(length, index);
    }
    pub fn set_stage_in_region(&self, region: Region) {
        self.id.set_stage_in_region(region);
    }
}
