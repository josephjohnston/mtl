// use objc::declare::ClassBuilder;
// use objc::encode::{Encode, Encoding, RefEncode};
// use objc::ffi::NSUInteger;
// use objc::foundation::NSArray; //, NSDictionary, NSError, NSString};
// use objc::rc::{autoreleasepool, Id, Owned, Ownership, Shared};
// use objc::runtime::{Bool, Class, Object, Sel};
// use objc::{class, msg_send, msg_send_id, sel, Message};
// use objc::foundation::NSArray;

use objc::encode::{Encode, Encoding, RefEncode};
use objc::runtime::{Bool, Object};
use objc::{class, msg_send, msg_send_id, Message};

use block::Block;
pub use block::ConcreteBlock;
// use icrate::Foundation::{NSArray, NSData, NSError, NSRange, NSString};
use objc::foundation::{NSArray, NSData, NSError, NSRange, NSString};

pub use NSURL;

pub use DataType;

use std::ffi::c_void;

type Id<T> = objc::rc::Id<T, objc::rc::Shared>;
unsafe fn cast<T: Message, U: Message>(id: Id<T>) -> Id<U> {
    objc::rc::Id::cast(id)
}

pub struct Ptr<T>(*mut T)
where
    T: RefEncode;
unsafe impl<T: RefEncode> RefEncode for Ptr<T> {
    const ENCODING_REF: Encoding = Encoding::Pointer(&T::ENCODING_REF);
}
unsafe impl<T: RefEncode> Message for Ptr<T> {}
impl<T: RefEncode> Ptr<T> {
    pub fn new(ptr: *mut T) -> Self {
        Ptr(ptr)
    }
}

// use objc::rc::autoreleasepool;

mod types;
pub use types::*;

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
pub fn create_system_default_device() -> Id<Device> {
    // autoreleasepool(|_pool| unsafe { Id::new(externs::MTLCreateSystemDefaultDevice()).unwrap() })
    unsafe { Id::new(externs::MTLCreateSystemDefaultDevice()).expect(ID_NEW_FAILURE) }
}

// // [f] MTLCopyAllDevices
// // iOS unavailable
// pub fn copy_all_devices() -> Id<NSArray<Device>> {
//     unsafe {
//         let array: Id<NSArray<Device>> =
//             Id::new(externs::MTLCopyAllDevices()).expect(ID_NEW_FAILURE);
//         array
//         // (0..array.len())
//         //     .map(|index| msg_send_id![&array, objectAtIndex: index])
//         //     .collect::<Vec<_>>()
//     }
// }

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
        Encoding::Struct("Size", &[usize::ENCODING, usize::ENCODING, usize::ENCODING]);
}
impl Size {
    // MTLSizeMake replacement
    pub fn new(width: usize, height: usize, depth: usize) -> Size {
        Size {
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
//     fn new() -> Id<Self, Owned> {
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

// // owned vs shared, which is it?

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

// type Id<T> = objc::rc::Id<T,Shared>;

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
    fn label(&self) -> Option<Id<NSString>> {
        unsafe { msg_send_id![self, label] }
    }
    fn set_label(&self, label: &NSString) {
        unsafe { msg_send![self, setLabel: label] }
    }
}
// impl<T: Message> ObjectUtil for T {}

// // // [P] device
// //     fn device(&self) -> Id<Device, Shared> {
// //         unsafe { Id::retain(msg_send![self, device]).expect(ID_RETAIN_FAILURE) }
// //     }

// // Counter Sample Buffers
// mod counters;
// pub use counters::*;

declare!(Bundle);
impl Bundle {
    pub fn get_bundle() -> Option<Id<Self>> {
        unsafe { msg_send_id![class!(NSBundle), mainBundle] }
    }
    pub fn localizations(&self) -> Vec<Id<NSString>> {
        unsafe { NSArray::into_vec(msg_send_id![self, localizations]) }
    }
    pub fn url_for_resource(
        &self,
        name: &NSString,
        ext: &NSString,
        subdirectory: &NSString,
    ) -> Option<Id<NSURL>> {
        unsafe {
            msg_send_id![
                self,
                URLForResource: name,
                withExtension: ext,
                subdirectory: subdirectory
            ]
        }
    }
}
