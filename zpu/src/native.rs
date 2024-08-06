use mtl::*;
use objc::rc::Retained;
use objc_foundation::{NSRange, NSString, NSURL};
use std::env;
use std::path::Path;

use crate::params::*;

const LENGTH: usize = D * E * F * G;

pub fn go(input_vals: &[Input], to_hide: usize) -> Vec<Output> {
    let device = create_system_default_device();
    let command_queue = device.new_command_queue();
    command_queue.set_label(&NSString::from_str("my_queue"));
    let command_buffer = command_queue.command_buffer();

    let current_dir = env::current_dir().expect("Failed to get current directory");
    let base_path = current_dir.join("zpu/src/shaders/");
    let base_string = format!("file://{}", base_path.to_str().unwrap());
    let base_url = unsafe { NSURL::URLWithString(&NSString::from_str(base_string.as_str())) };
    let main_str = format!("{}_macos.metallib", super::SHADER);
    let url = unsafe {
        NSURL::URLWithString_relativeToURL(
            &NSString::from_str(&main_str),
            Some(&*base_url.unwrap()),
        )
    };
    let library = device.new_library_with_url(&url.unwrap()).unwrap();
    let function = library.new_function_with_name(&NSString::from_str("go"));
    let cps = device
        .new_compute_pipeline_state_with_function_error(&function)
        .unwrap();

    // PREPARE DATA
    let resource_options = ResourceOptions::new(
        CPUCacheMode::DefaultCache,
        StorageMode::Shared,
        HazardTrackingMode::Default,
    );

    // Input
    let input_byte_length = LENGTH * std::mem::size_of::<Input>();
    let input = device.new_buffer_with_length(input_byte_length, resource_options.clone());
    let input_slice =
        unsafe { std::slice::from_raw_parts_mut::<Input>(input.contents().cast(), LENGTH) };
    for (i, x) in input_slice.iter_mut().enumerate() {
        *x = input_vals[i];
    }
    // Output
    let output_byte_length = LENGTH * std::mem::size_of::<Output>();
    let output = device.new_buffer_with_length(output_byte_length, resource_options.clone());
    let output_slice =
        unsafe { std::slice::from_raw_parts::<Output>(output.contents().cast(), LENGTH) };

    // // TIMESTAMPS
    let sample_count = 2;
    let (sample_buffer, cpu_start_time, gpu_start_time) =
        setup_timestamps(&device, sample_count, false);
    let start_index = 0;
    let encoder = make_encoder(
        DispatchType::Serial,
        &command_buffer,
        &sample_buffer,
        Some(start_index),
        Some(start_index + 1),
    );
    encoder.set_compute_pipeline_state(&cps);
    encoder.set_buffer(&input, 0, 0);
    encoder.set_threadgroup_memory_length(G * D * 4, 0);
    encoder.set_buffer(&output, 0, 1);
    // encoder.set_buffer(&constants, 0, 2);
    let block_size = Size::new(T * G, 1, 1);
    let grid_size = Size::new(E, 1, 1);
    encoder.dispatch_threadgroups(grid_size, block_size);
    encoder.end_encoding();
    command_buffer.commit();
    println!("ENCODING DONE");
    command_buffer.wait_until_completed();

    println!("\nGPU COEFS:");
    for (i, x) in output_slice.iter().enumerate() {
        if i % (1 << to_hide) == 0 {
            println!("{i}: {x}");
        }
    }

    let timespans = get_timespans(
        &device,
        &sample_buffer,
        sample_count,
        cpu_start_time,
        gpu_start_time,
    );
    println!("timespan: {timespans:?}");

    output_slice.to_vec()
}

fn make_encoder(
    dispatch_type: DispatchType,
    cmd_buffer: &CommandBuffer,
    sample_buffer: &CounterSampleBuffer,
    start_index: Option<usize>,
    end_index: Option<usize>,
) -> Retained<ComputeCommandEncoder> {
    let desc = ComputePassDescriptor::compute_pass_descriptor();
    desc.set_dispatch_type(dispatch_type);
    // let compute_pass_attachments = compute_pass_desc.sample_buffer_attachments();
    // seems there's room for 4, ie only accessible at indices {0,1,2,3}. but since we're only ever using one counter (for timestamps) I think we only ever need one counter sampler buffer and thus one sample buffer attachment.
    let attachment_desc = desc
        .sample_buffer_attachments()
        .object_at_indexed_subscript(0);
    attachment_desc.set_sample_buffer(sample_buffer);
    attachment_desc.set_start_of_encoder_sample_index(match start_index {
        Some(index) => index,
        None => usize::MAX,
    });
    attachment_desc.set_end_of_encoder_sample_index(match end_index {
        Some(index) => index,
        None => usize::MAX,
    });
    let encoder = cmd_buffer.compute_command_encoder_with_descriptor(&desc);
    encoder
}

fn setup_timestamps(
    device: &Device,
    sample_count: usize,
    private: bool,
) -> (Retained<CounterSampleBuffer>, Timestamp, Timestamp) {
    assert!(device.supports_counter_sampling(CounterSamplingPoint::StageBoundary));
    let counter_set_ptr: *const CounterSet = device
        .counter_sets()
        .iter()
        .find(|counter_set| counter_set.name().to_string() == "timestamp")
        .unwrap();
    let counter_set = unsafe { Retained::retain(counter_set_ptr as *mut CounterSet).unwrap() };
    assert!(counter_set
        .counters()
        .iter()
        .any(|counter| counter.name().to_string() == "GPUTimestamp"));

    let desc = CounterSampleBufferDescriptor::new();
    desc.set_label(&NSString::from_str("timestamp counter sample buffer"));
    desc.set_counter_set(&counter_set);
    desc.set_sample_count(sample_count);
    desc.set_storage_mode(match private {
        false => StorageMode::Shared,
        true => StorageMode::Private,
    });
    let sample_buffer = device
        .new_counter_sample_buffer_with_descriptor(&desc)
        .unwrap();
    let mut cpu_start_time = Timestamp::new(0);
    let mut gpu_start_time = Timestamp::new(0);
    device.sample_timestamps(&mut cpu_start_time, &mut gpu_start_time);
    (sample_buffer, cpu_start_time, gpu_start_time)
}

fn get_timespans(
    device: &Device,
    sample_buffer: &CounterSampleBuffer,
    sample_count: usize,
    cpu_start_time: Timestamp,
    gpu_start_time: Timestamp,
) -> Vec<usize> {
    let mut cpu_end_time = Timestamp::new(0);
    let mut gpu_end_time = Timestamp::new(0);
    device.sample_timestamps(&mut cpu_end_time, &mut gpu_end_time);
    // somehow printing these prevents them from going to zero in release mode.
    // the API of MTLTimestamp says u64 but somehow them may be getting rounded in release mode
    println!(
        "\nend times: {}, {}",
        cpu_end_time.value, gpu_end_time.value
    );
    let cpu_time_span = cpu_end_time.value - cpu_start_time.value;
    let gpu_time_span = gpu_end_time.value - gpu_start_time.value;
    let gpu_to_cpu_scaling = cpu_time_span / gpu_time_span;
    let data = sample_buffer.resolve_counter_range(NSRange::from(0..sample_count));
    let timestamps: Vec<u64> = data
        .bytes()
        // data holds MTLCounterResultTimestamp values, which are structs with single fields of type uint64_t, so we decode the NSData to u64 values
        .chunks(8)
        .map(|limbs| {
            let mut gpu_time: u64 = 0;
            for i in 0..limbs.len() {
                gpu_time += limbs[i] as u64 * ((1 << 8) as u64).pow(i as u32);
            }
            gpu_time
        })
        .map(|gpu_time| {
            let nanoseconds = cpu_start_time.value + gpu_time * gpu_to_cpu_scaling;
            let microseconds = nanoseconds / (10 as u64).pow(3);
            let milliseconds = microseconds / (10 as u64).pow(3);
            // nanoseconds
            // microseconds
            milliseconds
        })
        .collect();
    let timespans: Vec<usize> = timestamps
        .chunks(2)
        .map(|pair| (pair[1] - pair[0]) as usize)
        .collect();
    timespans
}
