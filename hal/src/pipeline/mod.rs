use super::*;

mod archive;
pub use archive::*;

pub struct Pipeline {
    id: Id<mtl::ComputePipelineState>,
    execution_width: usize,
    threadgroup_memory_length: usize,
    max_threads_per_threadgroup: usize,
}
impl Pipeline {
    pub fn new(
        device: &mtl::Device,
        desc: &mtl::ComputePipelineDescriptor,
    ) -> Result<Self, String> {
        let options = mtl::PipelineOption::FailOnBinaryArchiveMiss;
        let id_result = device.new_compute_pipeline_state_with_descriptor_error(&desc, options);
        match id_result {
            Ok(id) => {
                let execution_width = id.thread_execution_width();
                let threadgroup_memory_length = id.static_threadgroup_memory_length();
                let max_threads_per_threadgroup = id.max_total_threads_per_threadgroup();
                // println!("{}", id.thread_execution_width());
                Ok(Self {
                    id,
                    execution_width,
                    threadgroup_memory_length,
                    max_threads_per_threadgroup,
                })
            }
            Err(msg_id) => Err(msg_id.to_string()),
        }
    }
    pub(crate) fn get_mtl(&self) -> &mtl::ComputePipelineState {
        &*self.id
    }
    pub fn go(&self, width: usize, height: usize, depth: usize) -> usize {
        let size = mtl::Size::new(width, height, depth);
        self.id.imageblock_memory_length_for_dimensions(size)
    }
}

// we also have to describe the mutability of each resource.

#[repr(usize)]
pub enum Step {
    Constant = 0,
    PerInstance = 2,
    ThreadPositionX = 5,
    ThreadPositionY = 6,
    ThreadPositionXIndexed = 7,
    ThreadPositionYIndexed = 8,
}
fn get_mtl_step_function(step: Step) -> mtl::StepFunction {
    match step {
        Step::Constant => mtl::StepFunction::Constant,
        Step::PerInstance => mtl::StepFunction::PerInstance,
        Step::ThreadPositionX => mtl::StepFunction::ThreadPositionInGridX,
        Step::ThreadPositionY => mtl::StepFunction::ThreadPositionInGridY,
        Step::ThreadPositionXIndexed => mtl::StepFunction::ThreadPositionInGridXIndexed,
        Step::ThreadPositionYIndexed => mtl::StepFunction::ThreadPositionInGridYIndexed,
    }
}

#[repr(usize)]
#[derive(Debug, Clone, Copy)]
pub enum AttributeFormat {
    U8 = 45,
    UU8 = 1,
    UUU8 = 2,
    UUUU8 = 3,

    S8 = 46,
    SS8 = 4,
    SSS8 = 5,
    SSSS8 = 6,

    U16 = 49,
    UU16 = 13,
    UUU16 = 14,
    UUUU16 = 15,

    S16 = 50,
    SS16 = 16,
    SSS16 = 17,
    SSSS16 = 18,

    F16 = 53,
    FF16 = 25,
    FFF16 = 26,
    FFFF16 = 27,

    U32 = 36,
    UU32 = 37,
    UUU32 = 38,
    UUUU32 = 39,

    S32 = 32,
    SS32 = 33,
    SSS32 = 34,
    SSSS32 = 35,

    F32 = 28,
    FF32 = 29,
    FFF32 = 30,
    FFFF32 = 31,
}
fn get_mtl_attribute_format(format: AttributeFormat) -> mtl::AttributeFormat {
    match format {
        AttributeFormat::U8 => mtl::AttributeFormat::UChar,
        AttributeFormat::UU8 => mtl::AttributeFormat::UChar2,
        AttributeFormat::UUU8 => mtl::AttributeFormat::UChar3,
        AttributeFormat::UUUU8 => mtl::AttributeFormat::UChar4,

        AttributeFormat::S8 => mtl::AttributeFormat::Char,
        AttributeFormat::SS8 => mtl::AttributeFormat::Char2,
        AttributeFormat::SSS8 => mtl::AttributeFormat::Char3,
        AttributeFormat::SSSS8 => mtl::AttributeFormat::Char4,

        AttributeFormat::U16 => mtl::AttributeFormat::UShort,
        AttributeFormat::UU16 => mtl::AttributeFormat::UShort2,
        AttributeFormat::UUU16 => mtl::AttributeFormat::UShort3,
        AttributeFormat::UUUU16 => mtl::AttributeFormat::UShort4,

        AttributeFormat::S16 => mtl::AttributeFormat::Short,
        AttributeFormat::SS16 => mtl::AttributeFormat::Short2,
        AttributeFormat::SSS16 => mtl::AttributeFormat::Short3,
        AttributeFormat::SSSS16 => mtl::AttributeFormat::Short4,

        AttributeFormat::F16 => mtl::AttributeFormat::Half,
        AttributeFormat::FF16 => mtl::AttributeFormat::Half2,
        AttributeFormat::FFF16 => mtl::AttributeFormat::Half3,
        AttributeFormat::FFFF16 => mtl::AttributeFormat::Half4,

        AttributeFormat::U32 => mtl::AttributeFormat::UInt,
        AttributeFormat::UU32 => mtl::AttributeFormat::UInt2,
        AttributeFormat::UUU32 => mtl::AttributeFormat::UInt3,
        AttributeFormat::UUUU32 => mtl::AttributeFormat::UInt4,

        AttributeFormat::S32 => mtl::AttributeFormat::Int,
        AttributeFormat::SS32 => mtl::AttributeFormat::Int2,
        AttributeFormat::SSS32 => mtl::AttributeFormat::Int3,
        AttributeFormat::SSSS32 => mtl::AttributeFormat::Int4,

        AttributeFormat::F32 => mtl::AttributeFormat::Float,
        AttributeFormat::FF32 => mtl::AttributeFormat::Float2,
        AttributeFormat::FFF32 => mtl::AttributeFormat::Float3,
        AttributeFormat::FFFF32 => mtl::AttributeFormat::Float4,
    }
}

pub struct Attribute {
    index: usize,
    offset: usize,
    format: AttributeFormat,
    stride: usize,
    step: Step,
    rate: usize,
}

pub struct StageInInfo {
    attributes: Vec<Attribute>,
    index_buffer_index: usize,
    index_buffer_half_precision: bool,
}
