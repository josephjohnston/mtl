use super::*;

// [C] MTLComputePipelineDescriptor
declare!(ComputePipelineDescriptor);
impl Label for ComputePipelineDescriptor {}
impl ComputePipelineDescriptor {
    pub fn new() -> Retained<ComputePipelineDescriptor> {
        unsafe {
            let class = class!(MTLComputePipelineDescriptor);
            Retained::from_raw(msg_send![class, new]).expect(ID_NEW_FAILURE)
        }
    }
    // [P] computeFunction and setComputeFunction
    pub fn compute_function(&self) -> Retained<Function> {
        unsafe { msg_send_id![self, computeFunction] }
    }
    pub fn set_compute_function(&self, function: &Function) {
        unsafe { msg_send![self, setComputeFunction: function] }
    }
    // [P] threadGroupSizeIsMultipleOfThreadExecutionWidth and setThreadGroupSizeIsMultipleOfThreadExecutionWidth
    pub fn thread_group_size_is_multiple_of_execution_width(&self) -> Bool {
        unsafe {
            Bool::from_raw(msg_send![
                self,
                threadGroupSizeIsMultipleOfThreadExecutionWidth
            ])
        }
    }
    pub fn set_thread_group_size_is_multiple_of_execution_width(&self, value: Bool) {
        unsafe {
            msg_send![
                self,
                setThreadGroupSizeIsMultipleOfThreadExecutionWidth: value
            ]
        }
    }
    // [P] maxTotalThreadsPerThreadgroup and setMaxTotalThreadsPerThreadgroup
    pub fn max_total_threads_per_threadgroup(&self) -> usize {
        unsafe { msg_send![self, threadGroupSizeIsMultipleOfExecutionWidth] }
    }
    pub fn set_max_total_threads_per_threadgroup(&self, value: usize) {
        unsafe { msg_send![self, setMaxTotalThreadsPerThreadgroup: value] }
    }
    // [P] maxCallStackDepth and setMaxCallStackDepth
    pub fn max_call_stack_depth(&self) -> usize {
        unsafe { msg_send![self, maxCallStackDepth] }
    }
    pub fn set_max_call_stack_depth(&self, value: usize) {
        unsafe { msg_send![self, setMaxCallStackDepth: value] }
    }
    // [P] stageInputDescriptor and setStageInputDescriptor
    pub fn stage_input_descriptor(&self) -> Option<Retained<StageInputOutputDescriptor>> {
        unsafe { msg_send_id![self, stageInputDescriptor] }
    }
    pub fn set_stage_input_descriptor(&self, input_descriptor: &StageInputOutputDescriptor) {
        unsafe { msg_send![self, setStageInputDescriptor: input_descriptor] }
    }
    // [P] buffers
    pub fn buffers(&self) -> Retained<PipelineBufferDescriptorArray> {
        unsafe { msg_send_id![self, buffers] }
    }
    // [P] supportIndirectCommandBuffers and setSupportIndirectCommandBuffers:
    pub fn support_indirect_command_buffers(&self) -> Bool {
        unsafe { msg_send![self, supportIndirectCommandBuffers] }
    }
    pub fn set_support_indirect_command_buffers(&self, value: Bool) {
        unsafe { msg_send![self, setSupportIndirectCommandBuffers: value] }
    }
    // [M] reset
    pub fn reset(&self) {
        unsafe { msg_send![self, reset] }
    }
    // [P] preloadedLibraries:
    pub fn preloaded_libraries(&self) -> Retained<NSArray<DynamicLibrary>> {
        unsafe { msg_send_id![self, preloadedLibraries] }
    }
    pub fn set_preloaded_libraries(&self, libraries: &NSArray<DynamicLibrary>) {
        unsafe { msg_send![self, setPreloadedLibraries: libraries] }
    }
    // [P] linkedFunctions and setLinkedFunctions:
    pub fn linked_functions(&self) -> Retained<LinkedFunctions> {
        unsafe { msg_send_id![self, linkedFunctions] }
    }
    pub fn set_linked_functions(&self, linked_functions: &LinkedFunctions) {
        unsafe { msg_send![self, setLinkedFunctions: linked_functions] }
    }
    // [P] supportAddingBinaryFunctions and setSupportAddingBinaryFunctions:
    pub fn support_adding_binary_functions(&self) -> bool {
        unsafe { msg_send![self, supportAddingBinaryFunctions] }
    }
    pub fn set_support_adding_binary_functions(&self, value: Bool) {
        unsafe { msg_send![self, setSupportAddingBinaryFunctions: value] }
    }
    // [P] binaryArchives and setBinaryArchives
    pub fn binary_archives(&self) -> Retained<NSArray<BinaryArchive>> {
        unsafe { msg_send_id![self, binaryArchives] }
    }
    pub fn set_binary_archives(&self, archives: &NSArray<BinaryArchive>) {
        unsafe { msg_send![self, setBinaryArchives: archives] }
    }
}
// others related to libraries

// when making library: source compiled to air.
// when making pipeline state object: intermediate compiled to device code.
// check binary archives when creating a pipline in case it has the function already compiled.

// i guess each archive holds lots of functions.
// here we specify
// we don't serialize the pipeline descriptor, but it's still, along with function descriptors, what goes inside the archives.
// when it searches the archives, what does it look for? it must be somethign we provide in this pipeline descirptor. so what properties do we set? i guess it's the function most of all, that's the required property. so it must search for a function. but what if we set different other properties?

// the archive contains functions and pipline states

// make a library from source. at runtime turns into air. cpu intensive.
// instead can have air at build time.
// once have library, need to create pipeline descriptor with state and functions. lightweight.
// then pipeline state object is cpu intensive.
// it makes gpu binaries and puts them in cache in case they are referenced again.
// binary archives have to be generated at runtime. for us that's not a problem. it's a one-time setup. but now they can be done offline. use a metal pipeline script, like a collection of pipeline descriptors. that's what we want to do, but i don't see how. then we need to load an archive and create PSO's from it.

// pipeline script is json format of pipeline descriptors
// specify libraries and pipelines descriptors.
// maybe we don't do this, and instead have setup, generating archives at setup runtime.

// dynamic libraries
// utility libs. machine code gen at PSO gen

// dynamic libraries are to share logic, so don't have to duplicate the utility lib.
// a collection of functions callable from multiple compute pipelines.
// cannot be used to create Functions. but Functions can import functions in a dynamic library.
// linkage (no compilation) happens at PSO time.

// both of these things -- archives and dynamic libraries, seem for later without dictaitng much design for now.

// lets make pipeline creation simple for now.
// it's complicated enough coordinating with resources in the encoder.
// so we want minimal with Library, Function, and Pipeline stuff.
