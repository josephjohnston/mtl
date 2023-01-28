use rand::Rng;

mod numbers;
use numbers::*;

#[allow(arithmetic_overflow)]
fn main() {
    let pG: u128 = (1 << 64) - (1 << 32) + 1;
    // let pH: u32 = ((1 << 32) as u64 - (1 << 24) + (1 << 16) - (1 << 8) + 1) as u32;
    // ((2 as u64).pow(32) - (2 as u64).pow(24) + (2 as u64).pow(16) - (2 as u64).pow(8) + 1)
    //     as u32;
    // for a in (2 as u32).pow(16)..(2 as u32).pow(31) {
    //     for b in (2 as u32).pow(16)..(2 as u32).pow(31) {
    let mut rng = rand::thread_rng();

    for i in (pG - (1 << 14))..pG {
        for j in (pG - (1 << 14))..pG {
            let a = rng.gen::<u64>();
            let b = rng.gen::<u64>();
            // let a = 628008511146584023;
            // let b = 14368702434318396250;
            let result = G::mult(a, b);
            let correct = (a as u128) * (b as u128) % pG;
            // println!("{}", correct);
            if result != (correct as u64) && result - (pG as u64) != (correct as u64) {
                println!("{a},{b},{result},{correct}");
                panic!();
            }
        }
        // let correct = (a as u64) * (b as u64) % (p as u64);
        // if result != correct as u32 && result - p != correct as u32 {
        //     println!("{a},{b},{result},{correct}");
        //     panic!();
    }
}

// fn main() {
//     let p = (2 as u64).pow(42)
//         + (2 as u64).pow(35)
//         + (2 as u64).pow(28)
//         + (2 as u64).pow(21)
//         + (2 as u64).pow(14)
//         + (2 as u64).pow(7)
//         + 1;
//     let alow = 1 as u16;
//     let ahigh = 1 as u32;
// }

// use primal::*;
// fn main() {
//     let mut p = 0;
//     // let lower_bound = 1 << 31;
//     // let upper_bound = 1 << 32;
//     // let mut b3: Vec<u32> = vec![0; 10];
//     // let mut b5: Vec<u32> = vec![0; 10];
//     // let mut b5: Vec<u32> = vec![0; 10];

//     // let mut b5:

//     // let top: u64 = 0xffffffffffffffff;
//     for k in ((1 << 56) - (1 << 8))..(1 << 56) {
//         if k % 2 == 0 {
//             continue;
//         }
//         p = k * (1 << 8) + 1;
//         // let diff = 64 - 10;
//         if is_prime(p) {

//     let mut d3 = 0;
//     let mut d5 = 0;
//     let mut d7 = 0;
//             // let r = (p << diffß
//             for i in 0..10 {
//                 if k % (3 as u64).pow((i as u32) + 1) == 0 {
//                     d3 += 1;
//                 }
//                 if k % (5 as u64).pow((i as u32) + 1) == 0 {
//                     d5 += 1;
//                 }
//                 if k % (7 as u64).pow((i as u32) + 1) == 0 {
//                     d7 += 1;
//                 }
//             }
//             let dt = 1.5849 * (d3 as f32) + 2.3219 * (d5 as f32) + 2.8073 * (d7 as f32);
//             // if dt > 8.0 {
//             println!("p: {}, k: {}, dt: {}", p, k, dt);
//             // }
//             // println!("p: {}, k: {}", p, k);
//         }
//     }
// }

// fn pow(x: u32) -> u32 {
//     (2 as u32).pow(x)
// }

// const P4: usize = 2 * 2 * 2 * 2;
// const P8: usize = P4 * P4;
// const P16: usize = P8 * P8;
// fn main() {
//     let mut results: [u32; P16] = [0; P16];
//     for i in 0..pow(16) {
//         let Y = pow(3) - 1;
//         let Z = (Y << 12) as usize;
//         if (i as usize) & Z == Z {
//             continue;
//         }
//         let x = f16::from_bits(i as u16).to_f32();
//         for j in 0..pow(16) {
//             let Y = pow(3) - 1;
//             let Z = (Y << 12) as usize;
//             if (j as usize) & Z == Z {
//                 continue;
//             }
//             let y = f16::from_bits(j as u16).to_f32();
//             let r = f16::from_f32(x * y);
//             let address = usize::from(r.to_bits());
//             results[address] += 1;
//             let Y = pow(5) - 1;
//             let Z = (Y << 10) as usize;
//             if (r.to_bits() as usize) == Z {
//                 println!("x: {:b}, {}", x.to_bits(), x);
//                 println!("y: {:b}, {}", y.to_bits(), y);
//             }
//         }
//     }

//     // let max = results.iter().max().unwrap();
//     // let index = results.iter().position(|element| element == max).unwrap();
//     // for k in (0..results.len()).rev() {
//     //     if k % (pow(4) as usize) == 0 {
//     //         println!(
//     //             "{binary:016b}, {hits}, {float}",
//     //             hits = results[k],
//     //             binary = k,
//     //             float = f16::from_bits(k as u16),
//     //         );
//     //     }
//     // }
// }

// use primal::*;

// fn main() {
//     let p = Primes::all().nth(10001 - 1).unwrap();
//     println!("{}", p);
// }

// fn pow(x: u32) -> u64 {
//     u64::pow(2, x)
// }
// fn comp(x0: u64, x1: u64, x2: u64, x3: u64) -> u64 {
//     pow(0) * x0 + pow(8) * x1 + pow(16) * x2 + pow(24) * x3
// }

// fn main() {
//     let N = pow(32) - pow(24) + pow(16) - pow(8) + 1;

//     let a = (1, 1, 1, 1);
//     let b = (1, 1, 1, 1);

//     let c0 = a.0 * b.0;

//     let c1 = a.0 * b.1 + a.1 * b.0;

//     let c2 = a.0 * b.2 + a.1 * b.1 + a.2 * b.0;

//     let c3 = a.0 * b.3 + a.1 * b.2 + a.2 * b.1 + a.3 * b.0;

//     let c4 = a.1 * b.3 + a.2 * b.2 + a.3 * b.1;

//     let c5 = a.2 * b.3 + a.3 * b.2;

//     let c6 = a.3 * b.3;

//     // let c = (N + c0 - c5 + c4, N + c1 - c6 - c4, c2 + c4, N + c3 - c4);
//     let c = (
//         N + c0 - c4 - c5,
//         c1 + c4 - c6,
//         N + c2 - c4,
//         c3 + c4,
//         0,
//         0,
//         0,
//     );

//     // let C = pow(0) * c.0
//     //     + pow(8) * c.1
//     //     + pow(16) * c.2
//     //     + pow(24) * c.3
//     //     + pow(32) * c.4
//     //     + pow(40) * c.5
//     //     + pow(48) * c.6;

//     println!("{}", N);
//     // println!("{:?}", (c0, c1, c2, c3));
//     println!("a: {}", comp(a.0, a.1, a.2, a.3));
//     println!("b: {}", comp(a.0, a.1, a.2, a.3));
//     println!("c: {}", comp(c.0, c.1, c.2, c.3));
//     // println!("C: {}", C);
// }

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

// // modify resources and also read them, and synchronize.
// // heaps
// // argument buffers
// // i wonder if we should just be working with the base api, rather than building a second on top. we'd have to make helper functions. the issue is, where would we store the extra data? we'd probably want to query them, but we don't want to send messages.
// // it may be too difficult to try to create an intermediate api, exposing only what we need. especially if we try to abstract away lots. but that's what we want, except that its for us, we're not gonna try to translate errors

// // https://www.crowdcast.io/c/manta-openzl
// // a cool idea is making the computation with the cpu, with tiny memory/io footprint cuz we just save some intermediate states. if just registers this is fine, but actually we may need to copy a lot of the stack/heap. then on gpu each compute unit computes starting at one intermediate state to the next. hope is to do as much of the computation as possible on the gpu.
// // note that we can have full control flow by haveing control over the compute unit.
// // maybe we could even have corresponding algebraic operations for the subgroup operations.
// // a compute unit could potentially be really fast. we could design the model of computation, in fact, around exactly what these can do. this is where subgroup operations can shine! they make a simd-group like a single thread, and they execute one after the other (well actually we have have threads-per-threadgroup = execution-width).
// // but i think gpus needs all cores to be doing the same thing at once. cuz commands are given to all at once.
// // why don't we just have a single kernel? one reason to have multiple is to intertwine with blit commands. is that all? another reason is to change the thread dispatch or other params. another reason is composing different functions you only know at runtime.
// // but a single kernel can still do a lot, as there's communication across the whole device.
// // in this case i indeed wonder if compute kernels can all be doing different things, syncing with each other as they go. the benefit is that we want to compute and prove on the gpu, but we do compute all then prove all. we need to go block by block, something like proving a block in a compute unit, then
// // suppose all compute units must do the same thing. we could combine the computation with the proving, using the JIT sumcheck version. so a compute unit can run through the comp, saving intermediate results periodically, needing to recalculate all those in between when summing.
// // note latency is only a problem in memory accesses, but unfortunately that includes threadgroup memory access.

// // dot product as a primitive op
