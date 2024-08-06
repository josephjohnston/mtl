use super::*;

// [C] MTLAttribute
declare!(Attribute);
impl Attribute {
    // [P] name
    // [P] attributeIndex
    // [P] attributeType,
    // [P] active
}

// [E] MTLArgumentAccess

// [E] MTLDataType
#[repr(usize)]
#[derive(Debug)]
pub enum DataType {
    None = 0,

    Struct = 1,
    Array = 2,

    Float = 3,
    Float2 = 4,
    Float3 = 5,
    Float4 = 6,

    Float2x2 = 7,
    Float2x3 = 8,
    Float2x4 = 9,

    Float3x2 = 10,
    Float3x3 = 11,
    Float3x4 = 12,

    Float4x2 = 13,
    Float4x3 = 14,
    Float4x4 = 15,

    Half = 16,
    Half2 = 17,
    Half3 = 18,
    Half4 = 19,

    Half2x2 = 20,
    Half2x3 = 21,
    Half2x4 = 22,

    Half3x2 = 23,
    Half3x3 = 24,
    Half3x4 = 25,

    Half4x2 = 26,
    Half4x3 = 27,
    Half4x4 = 28,

    Int = 29,
    Int2 = 30,
    Int3 = 31,
    Int4 = 32,

    UInt = 33,
    UInt2 = 34,
    UInt3 = 35,
    UInt4 = 36,

    Short = 37,
    Short2 = 38,
    Short3 = 39,
    Short4 = 40,

    UShort = 41,
    UShort2 = 42,
    UShort3 = 43,
    UShort4 = 44,

    Char = 45,
    Char2 = 46,
    Char3 = 47,
    Char4 = 48,

    UChar = 49,
    UChar2 = 50,
    UChar3 = 51,
    UChar4 = 52,

    Bool = 53,
    Bool2 = 54,
    Bool3 = 55,
    Bool4 = 56,

    Texture = 58,
    Sampler = 59,
    Pointer = 60,

    // colors
    RenderPipeline = 78,
    ComputePipeline = 79,
    IndirectCommandBuffer = 80,

    Long = 81,
    Long2 = 82,
    Long3 = 83,
    Long4 = 84,

    ULong = 85,
    ULong2 = 86,
    ULong3 = 87,
    ULong4 = 88,

    VisibleFunctionTable = 115,
    IntersectionFunctionTable = 116,
    PrimitiveAccelerationStructure = 117,
    InstanceAccelerationStructure = 118,
}
impl_encode_for_type!(DataType: usize);

// [E] MTLFunctionType
#[repr(usize)]
#[derive(Debug)]
pub enum FunctionType {
    Vertex = 1,
    Fragment = 2,
    Kernel = 3,
    Visible = 5,
    Intersection = 6,
    Mesh = 7,
    Object = 8,
}
impl_encode_for_type!(FunctionType: usize);

// [E] MTLFunctionOptions
#[repr(usize)]
pub enum FunctionOptions {
    None = 0,
    CompileToBinary = 1 << 0,
}
impl_encode_for_type!(FunctionOptions: usize);

// [C] MTLFunctionConstant
declare!(FunctionConstant);
impl FunctionConstant {
    // [P] name
    pub fn name(&self) -> Retained<NSString> {
        unsafe { msg_send_id![self, name] }
    }
    // [P] type
    pub fn data_type(&self) -> DataType {
        unsafe { msg_send![self, type] }
    }
    // [P] index
    pub fn index(&self) -> usize {
        unsafe { msg_send![self, index] }
    }
    // [P] required
    pub fn required(&self) -> bool {
        unsafe { msg_send![self, required] }
    }
}

// [C] MTLFunctionConstantValues
declare!(FunctionConstantValues);
impl FunctionConstantValues {
    pub fn new() -> Retained<Self> {
        unsafe { msg_send_id![class!(MTLFunctionConstantValues), new] }
    }
    // [M] setConstantValue:type:atIndex:
    pub fn set_constant_value(
        &self,
        constant_value: *const c_void,
        data_type: DataType,
        index: usize,
    ) {
        unsafe {
            msg_send![self, setConstantValue: constant_value, type: data_type, atatIndex: index]
        }
    }
    // [M] setContantValue:type:withName
    pub fn set_constant_value_with_name(
        &self,
        constant_value: *const c_void,
        data_type: DataType,
        name: &NSString,
    ) {
        unsafe {
            msg_send![
                self,
                setContantValue: constant_value,
                type: data_type,
                withName: name
            ]
        }
    }
    // [M] setConstantValues:type:withRange:
    pub fn set_constant_values(
        &self,
        constant_values: *const c_void,
        data_type: DataType,
        range: NSRange,
    ) {
        unsafe {
            msg_send![
                self,
                setConstantValues: constant_values,
                type: data_type,
                withRange: range
            ]
        }
    }
    // [M] reset
    pub fn reset(&self) {
        unsafe { msg_send![self, reset] }
    }
}

// [C] MTLFunctionDescriptor
declare!(FunctionDescriptor);
impl FunctionDescriptor {
    // [M] functionDescriptor
    pub fn function_descriptor() -> Retained<Self> {
        unsafe { msg_send_id![class!(MTLFunctionDescriptor), functionDescriptor] }
    }
    // [P] name and setName
    pub fn name(&self) -> Option<Retained<NSString>> {
        unsafe { msg_send_id![self, name] }
    }
    pub fn set_name(&self, name: &NSString) {
        unsafe { msg_send![self, setName: name] }
    }
    // [P] specializedName and setSpecializedName
    pub fn specialized_name(&self) -> Option<Retained<NSString>> {
        unsafe { msg_send_id![self, specializedName] }
    }
    pub fn set_specialized_name(&self, specialized_name: &NSString) {
        unsafe { msg_send![self, setSpecializedName: specialized_name] }
    }
    // [P] constantValues and setConstantValues
    pub fn constant_values(&self) -> Retained<FunctionConstantValues> {
        unsafe { msg_send_id![self, constantValues] }
    }
    pub fn set_constant_values(&self, constant_values: &FunctionConstantValues) {
        unsafe { msg_send![self, setConstantValues: constant_values] }
    }
    // [P] options and setOptions
    pub fn options(&self) -> FunctionOptions {
        unsafe { msg_send![self, options] }
    }
    pub fn set_options(&self, options: FunctionOptions) {
        unsafe { msg_send![self, setOptions: options] }
    }
    // [P] binaryArchives and setBinaryArchives
    pub fn binary_archives(&self) -> Option<Retained<NSArray<BinaryArchive>>> {
        unsafe { msg_send_id![self, binaryArchives] }
    }
    pub fn set_binary_archives(&self, archives: &NSArray<BinaryArchive>) {
        unsafe { msg_send![self, setBinaryArchives: archives] }
    }
}

// [C] Function
declare!(Function);
impl Label for Function {}
impl Function {
    // [P] device
    pub fn device(&self) -> Retained<Device> {
        unsafe { msg_send_id![self, device] }
    }
    // [P] name
    pub fn name(&self) -> Retained<NSString> {
        unsafe { msg_send_id![self, name] }
    }
    // [P] functionType
    pub fn function_type(&self) -> FunctionType {
        unsafe { msg_send![self, functionType] }
    }
    // [P] options
    pub fn otpions(&self) -> FunctionOptions {
        unsafe { msg_send![self, options] }
    }
    // // [P] shaderInputAttributes
    // pub fn stage_input_attributes(&self) -> Retained<NSArray<*mut Attribute>> {
    //     unsafe { msg_send_id![self, shaderInputAttributes] }
    // }
    // // [P] functionConstantsDictionary
    // pub fn function_constants_dictionary(
    //     &self,
    // ) -> Retained<NSDictionary<*mut NSString, *mut FunctionConstant>> {
    //     unsafe {
    //         let dictionary = msg_send_id![self, functionConstantsDictionary];
    //     }
    // }
}

// [Pr] MTLFunctionHandle
declare!(FunctionHandle);
impl FunctionHandle {
    // [P] name
    pub fn name(&self) -> Retained<NSString> {
        unsafe { msg_send_id![self, name] }
    }
    // [P] device
    pub fn device(&self) -> Retained<Device> {
        unsafe { msg_send_id![self, device] }
    }
    // [P] functionType
    pub fn function_type(&self) -> FunctionType {
        unsafe { msg_send![self, functionType] }
    }
}

// [C] MTLVisibleFunctionTableDescriptor
declare!(VisibleFunctionTableDescriptor);
impl VisibleFunctionTableDescriptor {
    // [M] visibleFunctionTableDescriptor
    pub fn visible_function_table_descriptor() -> Retained<Self> {
        unsafe {
            msg_send_id![
                class!(MTLVisibleFunctionTableDescriptor),
                visibleFunctionTableDescriptor
            ]
        }
    }
    // [P] functionCount and setFunctionCount
    pub fn function_count(&self) -> usize {
        unsafe { msg_send![self, functionCounte] }
    }
    pub fn set_function_count(&self, function_count: usize) {
        unsafe { msg_send![self, setFunctionCount: function_count] }
    }
}

// [C] MTLVisibleFunctionTable
declare!(VisibleFunctionTable);
impl VisibleFunctionTable {
    pub fn gpu_resource_id(&self) -> ResourceID {
        unsafe { msg_send![self, gpuResourceID] }
    }
    // [M] setFunction:atIndex:
    pub fn set_function(&self, function_handle: &FunctionHandle, index: usize) {
        unsafe { msg_send![self, setFunction: function_handle, atIndex: index] }
    }
    // [M] setFunctions:withRange:
    pub fn set_functions(&self, functions: &[&FunctionHandle], start_index: usize) {
        let range = NSRange::new(start_index, functions.len());
        unsafe { msg_send![self, setFunction: functions.as_ptr(), withRange: range] }
    }
}

// [C] MTLLinkedFunctions
declare!(LinkedFunctions);
impl LinkedFunctions {
    // [M] linkedFunctions
    pub fn linked_functions() -> Retained<Self> {
        unsafe { msg_send_id![class!(MTLLinkedFunctions), linkedFunctions] }
    }
    // [P] functions and setFunctions
    pub fn functions(&self) -> Option<Retained<NSArray<Function>>> {
        unsafe { msg_send_id![self, functions] }
    }
    pub fn set_functions(&self, functions: &NSArray<Function>) {
        unsafe { msg_send![self, setFunctions: functions] }
    }
    // [P] binaryFunctions and setBinaryFunctions
    pub fn binary_functions(&self) -> Option<Retained<NSArray<Function>>> {
        unsafe { msg_send_id![self, binaryFunctions] }
    }
    pub fn set_binary_functions(&self, binary_functions: &NSArray<Function>) {
        unsafe { msg_send![self, setBinaryFunctions: binary_functions] }
    }
    // groups
    // private functions
}

// // [C] MTLVisibleFunctionTableDescriptor
// declare!(VisibleFunctionTableDescriptor);
// impl VisibleFunctionTableDescriptor {
//     // [M] visibleFunctionTableDescriptor
//     pub fn visible_function_table_descriptor() -> Retained<Self> {
//         unsafe {
//             msg_send_id![
//                 class!(MTLVisibleFunctionTableDescriptor),
//                 visibleFunctionTableDescriptor
//             ]
//         }
//     }
//     // [P] functionCount and setFunctionCount
//     pub fn function_count(&self) -> usize {
//         unsafe { msg_send![self, functionCount] }
//     }
//     pub fn set_function_count(&self, function_count: usize) {
//         unsafe { msg_send![self, setFunctionCount: function_count] }
//     }
// }
