// #[link(name = "zpu")]
// extern "C" {
//     fn rust_addition(a: i32, b: i32) -> i32;
// }

// fn main() {
//     unsafe {
//         rust_addition(1, 2);
//     }
// }

// use hal::*;

// fn main() {
//     let mut infos = GPU::current_gpus();
//     let info = infos.remove(0);
//     let gpu = GPU::new(info, "gpu".into());
//     let queue = gpu.new_queue("queue".into(), 4);

//     let archive = gpu.new_archive(
//         "shader".into(),
//         false,
//         "file:///Users/josephjohnston/saga/main/shaders/".into(),
//     );
//     // let archive = gpu.load_archive("shader".into());
//     let pipeline = archive.load_pipeline("reduce_neighborhood".into()).unwrap();
//     drop(archive);

//     let heap = gpu.new_auto_heap("auto heap".into(), 2024 * 4);
//     let tex = heap.new_texture(
//         "tex".into(),
//         TextureUsage::ReadWrite,
//         PixelFormat::Bits64(Bits64Format::SS),
//         Size::new(16, 8, 1),
//         false,
//     );

//     let N = 1 << 5;
//     let block_length = 32;
//     let grid_length = N / block_length;
//     let mut g_idata = gpu.new_buffer::<i32>("g_idata".into(), N);
//     for (i, x) in g_idata.as_mut_slice().iter_mut().enumerate() {
//         *x = i as i32;
//     }
//     let mut g_odata = gpu.new_buffer::<i32>("g_odata".into(), grid_length);

//     let batch = queue.new_batch(false);

//     let cpass = batch.new_compute_pass();
//     cpass.set_buffer(0, &g_idata, 0);
//     cpass.set_buffer(1, &g_odata, 0);
//     // cpass.set_texture(0, &tex);
//     let bytes: *const usize = &N;
//     cpass.set_bytes(2, bytes.cast(), 1 * std::mem::size_of::<i32>());
//     cpass.set_pipeline(&pipeline);
//     let block_size = Size::new(block_length, 1, 1);
//     let grid_size = Size::new(grid_length, 1, 1);
//     cpass.dispatch(block_size, grid_size);
//     cpass.end_encoding();

//     batch.commit();
//     batch.wait_until_completed();

//     for x in g_idata.as_slice() {
//         println!("{:?}", x);
//     }
// }

//     // let bpass = batch.new_blit_pass();
//     // pass.fill_buffer(&b, 0..3, 1);
//     // bpass.copy_buffer_to_texture(
//     //     &bb,
//     //     0,
//     //     32,
//     //     32 * 32,
//     //     Size::new(32, 32, 1),
//     //     &tex,
//     //     0,
//     //     Origin::new(0, 0, 0),
//     // );
//     // bpass.end_encoding();

//     // we want blit and compute passes to run in parallel.

//     // events work across command buffers, while fences don't

//     // now we need to set this buffer in encoding. lets work with that before creating other resource types.

//     // let t = buf.mut_slice;
//     // let r = buf.mut_slice;

//     // thread::scope(|s| {
//     //     s.spawn(move || {
//     //         // let c = &gpu;
//     //         // let q2 = gpu.new_queue("q2".into(), 6);
//     //         let t = buf.as_mut_slice();
//     //     });
//     // });

//     // sl[0] += 2;

//     // let x = 2;
//     // let y = &x;

//     // let handle = thread::spawn(|| {
//     //     // let z = &x;
//     //     println!("{:?}", y);
//     //     return 3;
//     // });

//     // handle.join();

//     // x.val += 3;`
// }

// // const tex1_label: &str = "tex1";

// // fn main() {
// //     autoreleasepool(|_| {
// //         let buf1 = CPUBuffer::new("buf1".into());
// //         println!("{}", buf1.os);
// //         // let device = Device::new();
// //         // let queue = device.new_queue("queue", 1);
// //         // let command_buffer = queue.new_command_buffer(false);

// //         // #[repr(C)]
// //         // #[derive(Debug)]
// //         // struct Mine {
// //         //     a: u32,
// //         //     b: u32,
// //         // }
// //         // let buf1: Buffer<Mine> = device.new_buffer_empty("buf1", false, 16);

// //         // let x = buf1.as_mut_slice();
// //         // for (i, j) in x.iter_mut().enumerate() {
// //         //     *j = Mine {
// //         //         a: i as u32,
// //         //         b: (i * 2) as u32,
// //         //     }
// //         // }
// //         // let z = buf1.as_slice();

// //         // // let buf2 = device.new_buffer_with_ptr("buf2", false, 8, unsafe { buf1.as_ptr().offset(4) });
// //         // let y = buf1.as_slice();
// //         // for i in y {
// //         //     println!("{:?}", i);
// //         // }

// //         // let encoder = command_buffer.new_compute_encoder(DispatchType::Serial);
// //         // why can't we use heaps for shared resources.

// //         // let copy: Buffer<Mine> = device.new_buffer_empty("copy", false, 4);

// //         // let encoder = command_buffer.new_blit_command_encoder();

// //         // encoder.copy_between_buffers(&buf1, 1, &copy, 0, 3);

// //         // command_buffer.commit();
// //         // command_buffer.wait_until_completed();

// //         // let y = copy.as_slice();
// //         // for i in y {
// //         //     println!("{:?}", i);
// //         // }

// //         // let ptr = unsafe { buf1.as_ptr().offset(3) };
// //         // let deallocator = |a: *const c_void, b: usize| {
// //         //     println!("{}", b);
// //         // };
// //         // let buf2 = device.new_buffer_with_ptr_copy("buf2", false, 3, ptr);

// //         // let x = unsafe { buf1.as_mut_ptr().unwrap() };

// //         // let y = buf1.cast::<u8>();

// //         // let tex1 = device.new_texture(
// //         //     tex1_label,
// //         //     false,
// //         //     TextureUsage::Read,
// //         //     PixelFormat::Bits32(Bits32Format::F),
// //         //     Size {
// //         //         width: 2,
// //         //         height: 2,
// //         //         depth: 1,
// //         //     },
// //         //     false,
// //         // );
// //         // let mut x: [f32; 4] = [0.1, 0.2, 0.3, 0.4];
// //         // let source = x.as_ptr().cast();
// //         // let region = Region::new(Origin::new(0, 0, 0), Size::new(2, 2, 1));
// //         // tex1.copy_to_region(source, region, 2, 0);

// //         // println!("{:?}", x);
// //         // for (_, elem) in x.iter_mut().enumerate() {
// //         //     *elem = 0.0;
// //         // }
// //         // println!("{:?}", x);

// //         // let dest = x.as_mut_ptr().cast();
// //         // let region = Region::new(Origin::new(1, 0, 0), Size::new(2, 2, 1));
// //         // tex1.copy_from_region(dest, region, 2, 0);
// //         // println!("{:?}", x);

// //         // let tex2 = tex1.derive_new_texture("tex2", TextureFormat::Bits32(Bits32Format::SSSS));

// //         // let heap = device.new_heap("heap", (2 as usize).pow(14));
// //         // // let buf2 = heap.new_buffer("buf2", (2 as usize).pow(13));
// //         // let size = Size {
// //         //     width: 40,
// //         //     height: 10,
// //         //     depth: 2,
// //         // };
// //         // let s = device.heap_texture_size_and_align(
// //         //     TextureUsage::Read,
// //         //     TextureFormat::Bits32(Bits32Format::F),
// //         //     size,
// //         //     false,
// //         // );
// //         // println!("{:?}", s);
// //         // let tex2 = heap
// //         //     .new_texture(
// //         //         "tex2",
// //         //         TextureUsage::Read,
// //         //         TextureFormat::Bits32(Bits32Format::F),
// //         //         size,
// //         //         false,
// //         //     )
// //         //     .unwrap();
// //         // println!("{},{}", heap.used_size(), heap.allocated_size());
// //     });
// // }
