pub use mtl::Label;

// use icrate::Foundation::{NSArray, NSError, NSRange, NSString};
use objc::foundation::{NSArray, NSData, NSError, NSRange, NSString};
use objc::msg_send_id;
use objc::rc::autoreleasepool;
use objc::runtime::Bool;
type Id<T> = objc::rc::Id<T, objc::rc::Shared>;

pub use half::f16;
pub use mtl::BarrierScope;
pub use mtl::CommandEncoder;
pub use mtl::Origin;
pub use mtl::Region;
pub use mtl::Size;
use mtl::NSURL;
use std::ops::Range;

mod gpu;
pub use gpu::*;

mod batch;
pub use batch::*;

mod resource;
pub use resource::*;

mod encoder;
pub use encoder::*;

mod pipeline;
pub use pipeline::*;

pub use mtl::Bundle;
