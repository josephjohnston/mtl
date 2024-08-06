use objc::rc::{Id, Retained};
use objc::runtime::Bool;
use objc_foundation::{NSArray, NSRange, NSString, NSURL};

pub use mtl::BarrierScope;
pub use mtl::CommandEncoder;
pub use mtl::Label;
pub use mtl::Origin;
pub use mtl::Region;
pub use mtl::Size;
use std::ops::Range;

mod gpu;
pub use gpu::*;

mod batch;
pub use batch::*;

pub mod resource;
use resource::*;

mod encoder;
pub use encoder::*;

mod pipeline;
pub use pipeline::*;

// pub use mtl::Bundle;
