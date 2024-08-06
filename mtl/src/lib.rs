use objc::encode::{Encode, Encoding, RefEncode};
use objc::runtime::{AnyObject, Bool};
use objc::{class, msg_send, msg_send_id, Message};
use objc_foundation::{NSArray, NSData, NSError, NSRange, NSString, NSURL};

pub use DataType;

use std::ffi::c_void;

type Retained<T> = objc::rc::Retained<T>;
// unsafe fn cast<T: Message, U: Message>(id: Retained<T>) -> Retained<U> {
//     objc::rc::Id::cast(id)
// }

mod macros;
use macros::*;

use bitflags::bitflags;

// GPU DEVICES

// Locating and Inspecting a GPU Device

mod externs {
    #[link(name = "CoreGraphics", kind = "framework")]
    #[link(name = "Metal", kind = "framework")]
    extern "C" {
        pub fn MTLCreateSystemDefaultDevice() -> *mut crate::device::Device;
        pub fn MTLCopyAllDevices() -> *mut crate::NSArray<crate::device::Device>;
    }
}

// [f] MTLCreateSystemDefaultDevice
pub fn create_system_default_device() -> Retained<Device> {
    // autoreleasepool(|_pool| unsafe { Retained::from_raw(externs::MTLCreateSystemDefaultDevice()).unwrap() })
    unsafe { Retained::from_raw(externs::MTLCreateSystemDefaultDevice()).expect(ID_NEW_FAILURE) }
}

// [f] MTLCopyAllDevices
// iOS unavailable
pub fn copy_all_devices() -> Retained<NSArray<Device>> {
    unsafe {
        let array: Retained<NSArray<Device>> =
            Retained::from_raw(externs::MTLCopyAllDevices()).expect(ID_NEW_FAILURE);
        array
        // (0..array.len())
        //     .map(|index| msg_send_id![&array, objectAtIndex: index])
        //     .collect::<Vec<_>>()
    }
}

// [Pr] MTLDevice
mod device;
pub use device::*;

// Submitting Work to a GPU

// [Pr] MTLCommandQueue
mod command_queue;
pub use command_queue::*;

// [C] MTLCommandBufferDescriptor, [Pr] MTLCommandBuffer, etc.
mod command_buffer;
pub use command_buffer::*;

// COMMAND ENCODERS

mod encoder;
pub use encoder::*;

mod pipeline;
pub use pipeline::*;

mod introspection;
pub use introspection::*;

// [C] MTLComputePassDescriptor
// [S] MTLDispatchThreadgroupsIndirectArguments
// Topic: Compute Pipeline States
// [Pr] MTLComputePipelineState
// [C] MTLComputePipelineDescriptor

// API Collection: Indirect Command Encoding
// Topic: Indirect Command Buffers
// Topic: Indirect Compute Commands
// [S] MTLSize
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub width: usize,
    pub height: usize,
    pub depth: usize,
}
unsafe impl Encode for Size {
    const ENCODING: Encoding =
        Encoding::Struct("?", &[usize::ENCODING, usize::ENCODING, usize::ENCODING]);
}
impl Size {
    // MTLSizeMake replacement
    pub fn new(width: usize, height: usize, depth: usize) -> Self {
        Self {
            width,
            height,
            depth,
        }
    }
}

// declare!(CompileOptions);
// impl CompileOptions {
//     fn class() -> &'static Class {
//         class!(MTLCompileOptions)
//     }
//     fn new() -> Retained<Self, Owned> {
//         unsafe {
//             let x = msg_send_id![Self::class(), new];
//             x.unwrap()
//         }
//     }
//     fn preprocessor_macros(&self) -> *mut Object {
//         unsafe {
//             let x: *mut Object = msg_send![&self.0, preprocessorMacros];
//             x
//         }
//     }
// }

// RESOURCES

// [Pr] MTLResource
mod resource;
pub use resource::*;

// [Pr] MTLBuffer
mod buffer;
pub use buffer::*;

// [Pr] MTLTexture
mod texture;
pub use texture::*;

// [Pr] MTLHeap
mod heap;
pub use heap::*;

// owned vs shared, which is it?

// SHADERS

mod library;
pub use library::*;

mod function;
pub use function::*;

// // Stitched Function Libraries

// // Compile-Time Variant Functions

// // [C] FunctionConstantValues
// declare!(FunctionConstantValues);
// impl FunctionConstantValues {
//     pub fn class() -> &'static Class {
//         class!(MTLFunctionConstantValues)
//     }
// }

const ID_NEW_FAILURE: &str = "ID_NEW_FAILURE";
const ID_RETAIN_AUTO_FAILURE: &str = "ID_RETAIN_AUTO_FAILURE";
const ID_RETAIN_FAILURE: &str = "ID_RETAIN_FAILURE";

// type Retained<T> = objc::rc::Retained<T,Shared>;

// SHADER LOGS

// // [Pr] LogContainer
// declare!(LogContainer);
// impl LogContainer {}

// type TimeInterval = c_double;

// // RESOURCES

// // Resource Synchronization

// mod synchronization;
// pub use synchronization::*;

pub trait Label
where
    Self: Message,
{
    // [P] label and setLable
    fn label(&self) -> Option<Retained<NSString>> {
        unsafe { msg_send_id![self, label] }
    }
    fn set_label(&self, label: &NSString) {
        unsafe { msg_send![self, setLabel: label] }
    }
}
