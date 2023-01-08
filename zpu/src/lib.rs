use hal::*;
use std::os::raw::c_int;

#[no_mangle]
pub extern "C" fn rust_addition(a: c_int, b: c_int) -> c_int {
    // let mut file = File::open("/Users/josephjohnston/saga/saga/Bridge.h").unwrap();
    // let mut contents = String::new();
    // let x = file.read_to_string(&mut contents).unwrap();
    // println!("{}", contents);

    let mut infos = GPU::current_gpus();
    let info = infos.remove(0);
    let gpu = GPU::new(info, "gpu".into());
    let queue = gpu.new_queue("queue".into(), 4);

    let archive = gpu.new_archive(
        "shader".into(),
        false,
        "file:///Users/josephjohnston/saga/zpu/shaders/".into(),
    );
    // let archive = gpu.load_archive("shader".into());
    let pipeline = archive.load_pipeline("reduce_neighborhood".into()).unwrap();

    // let pipeline = gpu.simplified("shader".into()).unwrap();

    drop(archive);

    let heap = gpu.new_auto_heap("auto heap".into(), 2024 * 4);
    let tex = heap.new_texture(
        "tex".into(),
        TextureUsage::ReadWrite,
        PixelFormat::Bits64(Bits64Format::SSSS),
        Size::new(16, 8, 1),
        false,
    );

    let N = 1 << 5;
    let block_length = 32;
    let grid_length = N / block_length;
    let mut g_idata = gpu.new_buffer::<i32>("g_idata".into(), N);
    for (i, x) in g_idata.as_mut_slice().iter_mut().enumerate() {
        *x = i as i32;
    }
    let mut g_odata = gpu.new_buffer::<i32>("g_odata".into(), grid_length);

    let batch = queue.new_batch(false);

    let cpass = batch.new_compute_pass();
    cpass.set_buffer(0, &g_idata, 0);
    cpass.set_buffer(1, &g_odata, 0);
    cpass.set_texture(0, &tex);
    cpass.set_texture(1, &tex);
    let bytes: *const usize = &N;
    cpass.set_bytes(2, bytes.cast(), 1 * std::mem::size_of::<i32>());
    cpass.set_pipeline(&pipeline);
    let block_size = Size::new(block_length, 1, 1);
    let grid_size = Size::new(grid_length, 1, 1);
    cpass.dispatch(block_size, grid_size);

    cpass.end_encoding();

    batch.commit();
    batch.wait_until_completed();

    for x in g_idata.as_slice() {
        println!("{:?}", x);
    }

    println!("out");
    for x in g_odata.as_slice() {
        println!("{:?}", x);
    }

    a + b - 7
}
