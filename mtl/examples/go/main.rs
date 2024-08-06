use mtl::*;
// use objc::encode::{Encode, Encoding, RefEncode};
use objc::rc::autoreleasepool;
use objc_foundation::{NSString, NSURL};

const ARRAY_LENGTH: usize = 4;
const BUFFER_SIZE: usize = ARRAY_LENGTH * 4;

fn main() {
    autoreleasepool(|_pool| {
        let device = create_system_default_device();
        let url_str = &NSString::from_str(
            "file:///Users/josephjohnston/saga/mtl/examples/go/shader.metallib",
        );
        let url = unsafe { &NSURL::URLWithString(url_str).unwrap() };
        let lib = device.new_library_with_url(url).unwrap();
        let func = lib.new_function_with_name(&NSString::from_str("go"));

        let pipeline_desc = ComputePipelineDescriptor::new();
        pipeline_desc.set_compute_function(&func);
        let pipeline = device
            .new_compute_pipeline_state_with_descriptor_error(
                &pipeline_desc,
                PipelineOption::None,
                // PipelineOption::FailOnBinaryArchiveMiss,
            )
            .unwrap();

        let options = ResourceOptions::new(
            CPUCacheMode::DefaultCache,
            StorageMode::Shared,
            HazardTrackingMode::Tracked,
        );

        let desc = TextureDescriptor::new();
        desc.set_usage(TextureUsage::SHADER_READ | TextureUsage::SHADER_WRITE);
        desc.set_texture_type(TextureType::D2);
        desc.set_width(1024);
        desc.set_height(16);
        desc.set_resource_options(options);
        let tex = device.new_texture_with_descriptor(&desc);
        let buf = device.new_buffer_with_length(100, options);

        let queue = device.new_command_queue();

        let command_buffer = queue.command_buffer();
        let encoder = command_buffer.compute_command_encoder();
        encoder.set_compute_pipeline_state(&pipeline);

        encoder.set_texture(&tex, 0);
        encoder.set_buffer(&buf, 0, 0);

        encoder.dispatch_threadgroups(Size::new(ARRAY_LENGTH, 1, 1), Size::new(1, 1, 1));
        encoder.end_encoding();
        command_buffer.commit();
        command_buffer.wait_until_completed();

        // let x = unsafe { p.read() };
        let p: *mut i32 = buf.contents().cast();
        unsafe {
            println!("{}, {}", *p.offset(0), p.read());
        }
    });
}
