// use block::ConcreteBlock;
// use objc::rc::autoreleasepool;
// use zpu_lib::*;

fn main() {}
// fn main() {
//     autoreleasepool(|_pool| {
//         let device = create_system_default_device();
//         let command_queue = device.new_command_queue();

//         let data = [
//             1u32, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
//             24, 25, 26, 27, 28, 29, 30,
//         ];

//         let buffer = {
//             let pointer = unsafe { std::mem::transmute(data.as_ptr()) };
//             let length = data.len() * std::mem::size_of::<u32>();
//             device.new_buffer_with_bytes(pointer, length, ResourceOptions::default(false))
//         };
//         let sum = {
//             let data = [0u32];
//             let pointer = unsafe { std::mem::transmute(data.as_ptr()) };
//             let length = data.len() * std::mem::size_of::<u32>();

//             device.new_buffer_with_bytes(pointer, length, ResourceOptions::default(false))
//         };

//         let command_buffer = command_queue.command_buffer();
//         command_buffer.set_label(&zpu_lib::NSString::from_str("label"));

//         let block = ConcreteBlock::new(|cmd_buffer: &CommandBuffer| {
//             println!("{}", cmd_buffer.label().unwrap().as_str(_pool));
//         })
//         .copy();
//         command_buffer.add_completed_handler(&block);

//         let encoder = command_buffer.compute_command_encoder();
//         // let library_path =
//         //     std::path::from(env!("CARGO_MANIFEST_DIR")).join("examples/compute/shader.metallib");

//         let base_str = "file:///Users/josephjohnston/zpu/examples/compute/";
//         let base_url = NSURL::url_with_string(base_str);
//         let main_str = "shader.metallib";
//         let url = NSURL::url_with_string_relative_to_url(main_str, &base_url);

//         let library_result = device.new_library_with_url(&url);
//         let library = match library_result {
//             Err(id) => panic!("here: {};", id.to_str()),
//             Ok(library) => library,
//         };
//         let kernel = library
//             .new_function_with_name_constants(&NSString::from_str("sum"), None)
//             .unwrap();

//         let pipeline_state_descriptor = ComputePipelineDescriptor::new();
//         pipeline_state_descriptor.set_compute_function(&kernel);

//         let func = pipeline_state_descriptor.compute_function();
//         let func = kernel;
//         let pipeline_state = device
//             .new_compute_pipeline_state_with_function(&func)
//             .unwrap();

//         encoder.set_compute_pipeline_state(&pipeline_state);
//         encoder.set_buffer(&buffer, 0, 0);
//         encoder.set_buffer(&sum, 0, 1);

//         let width: usize = 16;
//         let thread_group_count = Size::new(width, 1, 1);
//         let thread_group_size = Size::new((data.len() + width) / width, 1, 1);

//         encoder.dispatch_threadgroups(thread_group_count, thread_group_size);
//         encoder.end_encoding();
//         command_buffer.commit();
//         command_buffer.wait_until_completed();

//         let ptr = sum.contents() as *mut u32;
//         unsafe { assert_eq!(465, *ptr) };
//     });
// }
