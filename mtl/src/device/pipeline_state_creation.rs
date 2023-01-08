use super::*;

// [T] MTLNewComputePipelineStateCompletionHandler
pub type NewComputePipelineStateCompletionHandler<'a> =
    block::Block<(&'a ComputePipelineState, &'a NSError), ()>;
// [T] MTLNewComputePipelineStateWithReflectionCompletionHandler
pub type NewComputePipelineStateWithReflectionCompletionHandler<'a> = block::Block<
    (
        &'a ComputePipelineState,
        &'a ComputePipelineReflection,
        &'a NSError,
    ),
    (),
>;

impl Device {
    // By Function Without Options or Reflection
    // [M] newComputePipelineStateWithFunction:error:
    pub fn new_compute_pipeline_state_with_function_error(
        &self,
        compute_function: &Function,
    ) -> TryNewCatch<ComputePipelineState> {
        try_new_catch!(raw_error<ComputePipelineState> => msg_send![
            self,
            newComputePipelineStateWithFunction: compute_function,
            error: &mut raw_error
        ])
    }
    // [M] newComputePipelineStateWithFunction:completionHandler:
    pub fn new_compute_pipeline_state_with_function_handler(
        &self,
        compute_function: &Function,
        completion_handler: &NewComputePipelineStateCompletionHandler,
    ) {
        unsafe {
            msg_send![
                self,
                newComputePipelineStateWithFunction: compute_function,
                completionHandler: completion_handler
            ]
        }
    }
    // By Function With Options and Reflection
    // // [M] newComputePipelineStateWithFunction:options:reflection:error:
    // pub fn new_compute_pipeline_state_with_function_reflection_error(
    //     &self,
    //     compute_function: &Function,
    //     options: PipelineOption,
    //     reflection: &AutoreleasedComputePipelineReflection,
    // ) -> TryNewCatch<ComputePipelineState> {
    //     unsafe {
    //         try_new_catch!(
    //             raw_error<ComputePipelineState> => msg_send![
    //                 self,
    //                 newComputePipelineStateWithFunction: compute_function,
    //                 options: options,
    //                 reflection: reflection,
    //                 error: &mut raw_error
    //             ]
    //         )
    //     }
    // }
    // [M] newComputePipelineStateWithFunction:options:completionHandler:
    pub fn new_compute_pipeline_state_with_function_reflection_handler(
        &self,
        compute_function: &Function,
        options: PipelineOption,
        completion_handler: &NewComputePipelineStateWithReflectionCompletionHandler,
    ) {
        unsafe {
            msg_send![
                self,
                newComputePipelineStateWithFunction: compute_function,
                options: options,
                completionHandler: completion_handler
            ]
        }
    }
    // By Descriptor
    // [M] newComputePipelineStateWithDescriptor:options:reflection:error:
    pub fn new_compute_pipeline_state_with_descriptor_error(
        &self,
        descriptor: &ComputePipelineDescriptor,
        options: PipelineOption,
        // reflection: &AutoreleasedComputePipelineReflection,
    ) -> TryNewCatch<ComputePipelineState> {
        let mut ptr: *mut Object = std::ptr::null_mut();
        try_new_catch!(
            raw_error<ComputePipelineState> => msg_send![
                self,
                newComputePipelineStateWithDescriptor: descriptor,
                options: options,
                reflection: &mut ptr,
                error: &mut raw_error
            ]
        )
    }
    // [M] newComputePipelineStateWithDescriptor:options:completionHandler:
    pub fn new_compute_pipeline_state_with_descriptor_handler(
        &self,
        descriptor: &ComputePipelineDescriptor,
        options: PipelineOption,
        completion_handler: &NewComputePipelineStateWithReflectionCompletionHandler,
    ) {
        unsafe {
            msg_send![
                self,
                newComputePipelineStateWithDescriptor: descriptor,
                options: options,
                completionHandler: completion_handler
            ]
        }
    }
}
