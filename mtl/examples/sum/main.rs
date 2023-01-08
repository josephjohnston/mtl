// use zpu::*;

// const ARRAY_LENGTH: usize = 4;
// const BUFFER_SIZE: usize = ARRAY_LENGTH * 4;

// fn main() {
//     // INIT
//     let device = create_system_default_device();

//     let base_str = "file:///Users/josephjohnston/zpu/src/";
//     let base_url = NSURL::url_with_string(base_str);
//     let main_str = "shader.metallib";
//     let url = NSURL::url_with_string_relative_to_url(main_str, &base_url);

//     let library = device.new_library_with_url(&url).unwrap();
//     // let s = library.function_names();
//     let function = library.new_function_with_name("add_arrays");

//     let cps = device
//         .new_compute_pipeline_state_with_function(&function)
//         .unwrap();

//     let command_queue = device.new_command_queue();

//     // PREPARE DATA
//     let resource_options = ResourceOptions::new(
//         CPUCacheMode::DefaultCache,
//         StorageMode::Shared,
//         HazardTrackingMode::Default,
//     );
//     let buffer_A = device.new_buffer_with_length(BUFFER_SIZE, resource_options.clone());
//     let buffer_B = device.new_buffer_with_length(BUFFER_SIZE, resource_options.clone());
//     let buffer_result = device.new_buffer_with_length(BUFFER_SIZE, resource_options.clone());

//     generate_random_float_data(&buffer_A);
//     generate_random_float_data(&buffer_B);
//     // unsafe {
//     //     use std::ffi::{CStr, CString};
//     //     let ptr = buffer_A.contents();
//     //     let p: *mut i8 = ptr.cast();
//     //     let slice = CStr::from_ptr(p);
//     //     let y = slice.to_str().unwrap();
//     //     println!("{}", y);
//     // }

//     // SEND COMPUTE COMMAND
//     let command_buffer = command_queue.command_buffer();
//     let compute_encoder = command_buffer.compute_command_encoder();

//     // Encode Add Command
//     compute_encoder.set_compute_pipeline_state(&cps);
//     compute_encoder.set_buffer(buffer_A.as_ref(), 0, 0);
//     compute_encoder.set_buffer(&buffer_B, 0, 1);
//     compute_encoder.set_buffer(&buffer_result, 0, 2);

//     let grid_size = MTLSize {
//         width: ARRAY_LENGTH,
//         height: 1,
//         depth: 1,
//     };
//     let mut thread_group_size = cps.max_total_threads_per_threadgroup();
//     if thread_group_size > ARRAY_LENGTH {
//         thread_group_size = ARRAY_LENGTH;
//     }
//     let threadgroup_size = MTLSize {
//         width: thread_group_size,
//         height: 1,
//         depth: 1,
//     };

//     compute_encoder.dispatch_threads(grid_size, threadgroup_size);

//     compute_encoder.end_encoding();
//     command_buffer.commit();
//     command_buffer.wait_until_completed();

//     // VERIFY RESULTS
//     // let p_a: *mut f32 = buffer_A.contents().cast();
//     // let p_b: *mut f32 = buffer_B.contents().cast();
//     let p_result: *mut f32 = buffer_result.contents().cast();
//     unsafe {
//         // assert_eq!(*p_a + *p_b, *p_result);
//         // println!("{}", *p_a.offset(3));
//         // println!("{}", *p_b.offset(3));
//         println!("{}", *p_result.offset(0));
//         println!("{}", *p_result.offset(1));
//         println!("{}", *p_result.offset(2));
//         println!("{}", *p_result.offset(3));
//         // assert_eq!(*p_a.offset(1) + *p_b.offset(1), *p_result.offset(1));
//     }
// }

// fn generate_random_float_data(buffer: &Buffer) {
//     let p: *mut f32 = buffer.contents().cast();
//     unsafe {
//         // for i in 0..ARRAY_LENGTH {
//         *p.offset(0) = 0.9;
//         *p.offset(1) = 0.1;
//         *p.offset(2) = 0.2;
//         *p.offset(3) = 0.3;
//         // }
//     }
// }
fn main() {}
