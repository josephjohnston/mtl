use hal::*;
use rand::Rng;

mod naive;
mod shaders;

#[no_mangle]
#[allow(arithmetic_overflow)]
pub extern "C" fn rust_addition() -> i32 {
    naive::go();

    // let mut infos = GPU::current_gpus();
    // let info = infos.remove(0);
    // let gpu = GPU::new(info);
    // let queue = gpu.new_queue("queue".into(), 4);

    // let shaders_url_str = &"file:///Users/josephjohnston/saga/zpu/shaders/";
    // let archive = gpu.new_archive("risc0".into(), false, shaders_url_str);
    // // let archive = gpu.load_archive("risc0".into(), shaders_url_str);
    // let pipeline = archive.load_pipeline("zkshift".into()).unwrap();
    // drop(archive);

    // // let heap = gpu.new_auto_heap("auto heap".into(), 2024 * 4);
    // // let tex = heap.new_texture(
    // //     "tex".into(),
    // //     TextureUsage::ReadWrite,
    // //     PixelFormat::Bitb64(Bits64Format::SSSS),
    // //     Size::new(16, 8, 1),
    // //     false,
    // // );

    // const N: usize = 1 << 5;
    // const B: usize = 1 << 5;
    // // const EPSILON: u32 = (1 << 24) - (1 << 16) + (1 << 8) - (1 << 0);
    // // const P: u32 = ((1 << 32) - EPSILON as u64) as u32;
    // let block_size = Size::new(B, 1, 1);
    // let grid_size = Size::new(N / B, 1, 1);
    // // let mut g_idata = gpu.new_buffer::<Fp>("g_idata".into(), N, false);
    // // let a = heap.new_buffer::<i32>("my buff".into(), 10);
    // // let mut rng = rand::thread_rng();
    // // let nums: [i32; N];
    // // for i in 0..N {

    // // }
    // // [
    // //     // 4233714684, 1584763778, 1375018704, 2663308306, 4283950214, 566097362, 2564987907,
    // //     // 2599936823, 3454147795, 2865633581, 821761987, 429794313, 1813462976, 2470669340,
    // //     // 1884862287, 930452934, 2505163855, 2013744618, 4168991823, 4142000084, 3003756545,
    // //     // 2744419425, 204446817, 2102182537, 1905073392, 2685750613, 1895557934, 2963501000,
    // //     // 205828317, 3865032365, 2392110732,
    // //     // 3241674815,
    // //     8, 10, -59, 38, 10, 13, 24, -7, -52, 28, 32, 50, -64, 33, 10, -55, 14, 42, -10, 47, -33,
    // //     -56, -42, 22, 54, -12, -19, 11, -28, -25, 16, 36, 8, 10, -59, 38, 10, 13, 24, -7, -52, 28,
    // //     32, 50, -64, 33, 10, -55, 14, 42, -10, 47, -33, -56, -42, 22, 54, -12, -19, 11, -28, -25,
    // //     16, 36,
    // // ];
    // let mut input = gpu.new_buffer::<i32>("input".into(), N, true);
    // for (i, x) in input.as_mut_slice().iter_mut().enumerate() {
    //     // println!("{},", rng.gen_range(-64..65)); //gen::<u32>();
    //     *x = rng.gen_range(-64..65);
    //     // nums[i];
    // }
    // let output = gpu.new_buffer::<i32>("output".into(), 1 << 15, true);

    // let mut timestamp_sampler = gpu.new_timestamp_sampler(5, false);
    // let batch = queue.new_batch(false);

    // let cpass = batch.new_compute_pass(Some(&mut timestamp_sampler));
    // cpass.set_buffer(0, &input, 0);
    // cpass.set_buffer(1, &output, 0);
    // cpass.set_threadgroup_memory_length(1 << 15, 0);

    // // let bytes: *const usize = &N;
    // // cpass.set_bytes(
    // //     2,
    // //     (&N as *const usize).cast(),
    // //     1 * std::mem::size_of::<u32>(),
    // // );
    // // cpass.set_bytes(3, (&P as *const u32).cast(), 1 * std::mem::size_of::<u32>());
    // // cpass.set_texture(0, &tex);
    // // cpass.set_texture(1, &tex);
    // cpass.set_pipeline(&pipeline);
    // cpass.dispatch(block_size, grid_size);
    // cpass.end_encoding();

    // println!("in:");
    // // for (i, x) in input.as_mut_slice().iter_mut().enumerate() {
    // //     if i % (1 << 8) == 0 {
    // //         println!("{}", x);
    // //     }
    // // }
    // // let put = input.as_slice();
    // // for i in N / 2..N {
    // //     let mult = put[i].mul(Fp(1 << 20));
    // //     let lo = put[i - N / 2].add(mult);
    // //     let hi = put[i].sub(mult);
    // //     println!("{:?},{:?}", lo, hi);
    // // }

    // batch.commit();

    // // use std::time::Instant;
    // // let now = Instant::now();
    // batch.wait_until_completed();
    // // let elapsed = now.elapsed();
    // // println!("Elapsed: {:.2?}", elapsed);

    // timestamp_sampler.get_timestamps();
    // // let (x, y) = gpu.read_sample_buffer(&sample_buffer);
    // // let spans = gpu.update_end_time(timestamps.0, timestamps.1, x, y);

    // // println!("\n\nout:");
    // for (i, x) in output.as_slice().iter().enumerate() {
    //     if i % (1 << 5) == 0 {
    //         println!("{}", x);
    //     }
    // }

    // // let now2 = Instant::now();
    // // let mut sum = 0;
    // // for x in input.as_slice() {
    // //     sum += x;
    // // }
    // // println!("Elapsed1: {:.2?}, {}", now2.elapsed(), sum);

    2
}
