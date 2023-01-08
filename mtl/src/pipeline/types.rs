use super::*;

// [E] MTLMutability
#[repr(usize)]
#[derive(Debug)]
pub enum Mutability {
    Default = 0,
    Mutable = 1,
    Immutable = 2,
}
impl_encode_for_type!(Mutability: usize);

// [E] MTLPipelineOption
#[repr(usize)]
pub enum PipelineOption {
    None = 0,
    ArgumentInfo = 1 << 0,
    // BufferInfoType = 1 << 1, depreciated
    // iOS only 14.0+
    FailOnBinaryArchiveMiss = 1 << 2,
}
impl_encode_for_type!(PipelineOption: usize);

// [E] MTLArgumentAccess
#[repr(usize)]
pub enum ArgumentAccess {
    ReadOnly = 0,
    ReadWrite = 1,
    WriteOnly = 2,
}
impl_encode_for_type!(ArgumentAccess: usize);

// [E] MTLAttributeFormat
// #[allow(non_camel_case_types)]
#[repr(usize)]
#[derive(Debug)]
pub enum AttributeFormat {
    Invalid = 0,

    UChar2 = 1,
    UChar3 = 2,
    UChar4 = 3,

    Char2 = 4,
    Char3 = 5,
    Char4 = 6,

    UChar2Normalized = 7,
    UChar3Normalized = 8,
    UChar4Normalized = 9,

    Char2Normalized = 10,
    Char3Normalized = 11,
    Char4Normalized = 12,

    UShort2 = 13,
    UShort3 = 14,
    UShort4 = 15,

    Short2 = 16,
    Short3 = 17,
    Short4 = 18,

    UShort2Normalized = 19,
    UShort3Normalized = 20,
    UShort4Normalized = 21,

    Short2Normalized = 22,
    Short3Normalized = 23,
    Short4Normalized = 24,

    Half2 = 25,
    Half3 = 26,
    Half4 = 27,

    Float = 28,
    Float2 = 29,
    Float3 = 30,
    Float4 = 31,

    Int = 32,
    Int2 = 33,
    Int3 = 34,
    Int4 = 35,

    UInt = 36,
    UInt2 = 37,
    UInt3 = 38,
    UInt4 = 39,

    Int1010102Normalized = 40,
    UInt1010102Normalized = 41,

    UChar4NormalizedBGRA = 42,

    UChar = 45,
    Char = 46,
    UCharNormalized = 47,
    CharNormalized = 48,

    UShort = 49,
    Short = 50,
    UShortNormalized = 51,
    ShortNormalized = 52,

    Half = 53,
}
impl_encode_for_type!(AttributeFormat: usize);

// [E] MTLStepFunction
#[repr(usize)]
pub enum StepFunction {
    Constant = 0,
    // Vertex functions only
    PerVertex = 1,
    PerInstance = 2,
    PerPatch = 3,
    PerPathControlPoint = 4,
    // Compute functions only
    ThreadPositionInGridX = 5,
    ThreadPositionInGridY = 6,
    ThreadPositionInGridXIndexed = 7,
    ThreadPositionInGridYIndexed = 8,
}
impl_encode_for_type!(StepFunction: usize);

// [E] MTLIndexType
#[repr(usize)]
#[derive(Debug)]
pub enum IndexType {
    UInt16 = 0,
    UInt32 = 1,
}
impl_encode_for_type!(IndexType: usize);
