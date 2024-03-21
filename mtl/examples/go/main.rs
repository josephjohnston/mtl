// use objc::encode::{Encode, Encoding, RefEncode};
// use icrate::Foundation::{NSArray, NSError, NSString};
use mtl::*;
use objc::foundation::{NSArray, NSError, NSString};
use objc::rc::autoreleasepool;

const ARRAY_LENGTH: usize = 4;
const BUFFER_SIZE: usize = ARRAY_LENGTH * 4;

fn main() {
    autoreleasepool(|_pool| {
        let device = create_system_default_device();
        let lib = device
            .new_library_with_url(&NSURL::url_with_string(&NSString::from_str(
                "file:///Users/josephjohnston/saga/mtl/examples/go/shader.metallib",
            )))
            .unwrap();
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

        // let archive_desc = BinaryArchiveDescriptor::new();
        // let archive = device
        //     .new_binary_archive_with_descriptor(&archive_desc)
        //     .unwrap();
        // let _ = archive.add_compute_pipeline_functions_with_descriptor(&pipeline_desc);
        // let url = NSURL::url_with_string(
        //     "file:///Users/josephjohnston/zpu/mtl/examples/go/archive.metallib",
        // );
        // let _ = archive.serialize_to_url(&url).unwrap();

        // // looks like we can't get binary precompilation to work. others having trouble too. not enough documentation on the command line tools. need to wait. maybe later. for now we build in a runtime setup in which we generate the archives. to do so we make a library, extract functions, make pipeline ddescriptors, and archive.
        // // we'll be writing metal files by hand, they might share code, but we'll create a file out of each one
        // // to expose the minimum, we could have a lib, then pass the lib and function name to create a pipeline. well maybe we

        // // LOAD
        // let new_archive_desc = BinaryArchiveDescriptor::new();
        // new_archive_desc.set_url(&url);
        // let new_archive = device
        //     .new_binary_archive_with_descriptor(&new_archive_desc)
        //     .unwrap();

        // let archives = NSArray::from_vec(vec![new_archive]);

        // let pipeline = device
        //     .new_compute_pipeline_state_with_function_error(&func)
        //     .unwrap();

        // let table_desc = VisibleFunctionTableDescriptor::visible_function_table_descriptor();
        // let table = pipeline.new_visible_function_table_with_descriptor(&table_desc);
        // let handle = pipeline.function_handle_with_function(&new_func);
        // table.set_function(&handle, 0);

        // let blck = block::ConcreteBlock::new(
        //     |a: &ComputePipelineState, b: &ComputePipelineReflection, c: &NSError| {
        //         println!("hello {}", 2);
        //     },
        // )
        // .copy();
        // let pipeline = device.new_compute_pipeline_state_with_descriptor_handler(
        //     &pipe_desc,
        //     PipelineOption::None,
        //     &blck,
        // );
        // std::thread::sleep(std::time::Duration::from_millis(5000));

        // let function_table_descriptor =
        //     VisibleFunctionTableDescriptor::visible_function_table_descriptor();
        // let table = pipeline.new_visible_function_table_with_descriptor(&function_table_descriptor);
        // this is failing, probably cuz the functions we pass in are not 'visible'. so visible functions are still function objects, they are what go in the table, and they are what hsould be exported by linked functions.
        // let handle = pipeline.function_handle_with_function(&func2);
        // table.set_function(&handle, 0);

        // when searching for a function, it searches the archives in the function descriptor.

        // so we still need to reference the compute funciton, but i guess it skips the step of compiling the binary.

        // now we have an archive from disk. we want to search it. so first

        // let pipeline_desc = ComputePipelineDescriptor::new();
        // pipeline_desc.set_compute_function(&func);
        // let pipeline = device
        //     .new_compute_pipeline_state_with_descriptor_error(
        //         &pipeline_desc,
        //         PipelineOption::FailOnBinaryArchiveMiss,
        //     )
        //     .unwrap();

        // xcrun metal-source -flatbuffers=json archive.metallib -o descriptors.mtlp-json
        // xcrun shader.metal -N descriptors.mtlp-json -o archive2.metallib

        // let queue = device.new_command_queue();
        // let command_buffer = queue.command_buffer();
        // // pipeline

        // let library = device.new_library_with_url(&url).unwrap();
        // let function = library.new_function_with_name(&NSString::from_str("go"));
        // let state = device
        //     .new_compute_pipeline_state_with_function_error(&function)
        //     .unwrap();
        // // encoder
        // let compute_encoder = command_buffer.compute_command_encoder();
        // compute_encoder.set_compute_pipeline_state(&state);

        // let data = device.new_buffer_with_length(64, ResourceOptions::default(false));
        // let sum = device.new_buffer_with_length(64, ResourceOptions::default(false));

        // compute_encoder.set_buffer(&data, 0, 0);
        // compute_encoder.set_buffer(&sum, 0, 1);
        // compute_encoder.dispatch_threadgroups(Size::new(8, 1, 1), Size::new(2, 1, 1));

        // compute_encoder.end_encoding();
        // command_buffer.commit();
    });
}

// DYNAMIC LIBRARY
// let source = NSString::from_str(
//     "
// #include <metal_stdlib>
// using namespace metal;
// uint add(uint a,uint b) {
//     return a + b + 3;
// }
// ",
// );
// let options = CompileOptions::new();
// options.set_library_type(LibraryType::Dynamic);
// options.set_install_name(&NSString::from_str("install_name"));
// let reglib = device.new_library_with_source(&source, &options).unwrap();
// let dynlib = device.new_dynamic_library(&reglib).unwrap();

// // REGULAR LIBRARY
// let base_str = "file:///Users/josephjohnston/zpu/mtl/examples/go/";
// let base_url = NSURL::url_with_string(base_str);
// let main_str = "shader.metallib";
// let url = NSURL::url_with_string_relative_to_url(main_str, &base_url);
// let lib = device.new_library_with_url(&url).unwrap();
// // let source = NSString::from_str(
// //     "
// // #include <metal_stdlib>
// // using namespace metal;
// // uint add(uint a,uint b);
// // kernel void kernel_func(
// //     device uint * buff,
// //     uint gid [[ thread_position_in_grid ]]
// // ) {
// //     uint x = add(gid, 0);
// //     if (gid == 0) {
// //         buff[0] = x;
// //     }
// // }
// // ",
// // );
// // let options = CompileOptions::new();
// // options.set_libraries(&NSArray::from_vec(vec![dynlib1.clone()]));
// // let lib = device.new_library_with_source(&source, &options).unwrap();
