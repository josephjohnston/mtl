use super::*;

// [E] MTLDispatchType
#[repr(usize)]
#[derive(Debug)]
pub enum DispatchType {
    Serial = 0,
    Concurrent = 1,
}
impl_encode_for_type!(DispatchType: usize);

impl CommandBuffer {
    // [M] computeCommandEncoder
    pub fn compute_command_encoder(&self) -> Id<ComputeCommandEncoder> {
        unsafe { msg_send_id![self, computeCommandEncoder] }
    }
    // [M] computeCommandEncoderWithDescriptor:
    pub fn compute_command_encoder_with_descriptor(
        &self,
        compute_pass_descriptor: &ComputePassDescriptor,
    ) -> Id<ComputeCommandEncoder> {
        unsafe {
            msg_send_id![
                self,
                computeCommandEncoderWithDescriptor: compute_pass_descriptor
            ]
        }
    }
    // [M] computeCommandEncoderWithDispatchType:
    pub fn compute_command_encoder_with_dispatch_type(
        &self,
        dispatch_type: DispatchType,
    ) -> Id<ComputeCommandEncoder> {
        unsafe { msg_send_id![self, computeCommandEncoderWithDispatchType: dispatch_type] }
    }
    // [M] blitCommandEncoder
    pub fn blit_command_encoder(&self) -> Id<BlitCommandEncoder> {
        unsafe { msg_send_id![self, blitCommandEncoder] }
    }
    // [M] blitCommandEncoderWithDescriptor
    pub fn blit_command_encoder_with_descriptor(
        &self,
        blit_pass_descriptor: &BlitPassDescriptor,
    ) -> Id<BlitCommandEncoder> {
        unsafe { msg_send_id![self, blitCommandEncoderWithDescriptor: blit_pass_descriptor] }
    }
}

// /// # Creating Resource State Encoders
// impl CommandBuffer {
//     // [M] resourceStateCommandEncoder
//     pub fn resource_state_command_encoder(&self) -> Id<ResourceStateCommandEncoder> {
//         unsafe { msg_send_id![self, resourceStateCommandEncoder] }
//     }
//     // [M] resourceStateCommandEncoderWithDescriptor:
//     pub fn resource_state_command_encoder_with_descriptor(
//         &self,
//         resource_state_pass_descriptor: &ResourceStatePassDescriptor,
//     ) -> Id<ResourceStateCommandEncoder> {
//         unsafe {
//             msg_send_id![
//                 self,
//                 resourceStateCommandEncoderWithDescriptor: resource_state_pass_descriptor
//             ]
//         }
//     }
// }

// /// # Creating Render Encoders
// impl CommandBuffer {
//     // [M] renderCommandEncoderWithDescriptor:
//     pub fn render_command_encoder_with_descriptor(
//         &self,
//         render_pass_descriptor: &RenderPassDescriptor,
//     ) -> Id<RenderCommandEncoder, Shared> {
//         unsafe {
//             let raw_encoder: *mut RenderCommandEncoder = msg_send![
//                 self,
//                 renderCommandEncoderWithDescriptor: render_pass_descriptor
//             ];
//             Id::retain(raw_encoder).expect(ID_RETAIN_FAILURE)
//         }
//     }
// }

// /// # Creating Parallel Render Encoders
// impl CommandBuffer {
//     // [M] parallelRenderCommandEncoderWithDescriptor:
//     pub fn parallel_render_command_encoder_with_descriptor(
//         &self,
//         render_pass_descriptor: &RenderPassDescriptor,
//     ) -> Id<ParallelRenderCommandEncoder, Shared> {
//         unsafe {
//             let raw_encoder: *mut ParallelRenderCommandEncoder = msg_send![
//                 self,
//                 parallelRenderCommandEncoderWithDescriptor: render_pass_descriptor
//             ];
//             Id::retain(raw_encoder).expect(ID_RETAIN_FAILURE)
//         }
//     }
// }

// /// # Creating Acceleration Structure Encoders
// impl CommandBuffer {}
