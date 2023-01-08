use super::*;

declare!(Device);

mod device_inspection;
pub use device_inspection::*;

mod work_submission;
pub use work_submission::*;

mod pipeline_state_creation;
pub use pipeline_state_creation::*;

mod resource_creation;
pub use resource_creation::*;

mod library_creation;
pub use library_creation::*;

// // Work Submission
// impl Device {
//     // Creating Command Queues
//     pub fn new_command_queue(&self) -> Id<CommandQueue, Owned> {
//         unsafe {
//             let raw_command_queue: *mut CommandQueue = msg_send![self, newCommandQueue];
//             Id::new(raw_command_queue).unwrap()
//         }
//     }
// }
// Creating Compute Pipelines
// pub fn new_compute_pipeline_state_with_function(
//     &self,
//     compute_function: &Function,
// ) -> Result<Id<ComputePipelineState, Owned>, String> {
//     let mut raw_error: *mut NSError = std::ptr::null_mut();
//     unsafe {
//         let raw_cps: *mut ComputePipelineState = msg_send![
//             self,
//             newComputePipelineStateWithFunction: compute_function,
//             error: &mut raw_error
//         ];
//         if raw_error.is_null() {
//             Ok(Id::new(raw_cps).unwrap())
//         } else {
//             let error: Id<NSError, Owned> = Id::retain_autoreleased(raw_error).unwrap();
//             let desc = error.localized_description();
//             Err(desc.to_str().to_string())
//         }
//     }
// }
