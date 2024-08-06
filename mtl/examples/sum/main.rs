use mtl::*;
use objc::rc::Retained;
use objc_foundation::{NSRange, NSString, NSURL};
use rand::Rng;

const THREADGROUPS_PER_GRID: usize = 4;
const THREADS_PER_THREADGROUP: usize = 32 * 2;
const LENGTH: usize = THREADGROUPS_PER_GRID * THREADS_PER_THREADGROUP;
fn main() {
    let device = create_system_default_device();
    let command_queue = device.new_command_queue();
    command_queue.set_label(&NSString::from_str("my_queue"));
    let command_buffer = command_queue.command_buffer();

    let base_str = "file:///Users/josephjohnston/saga/mtl/examples/sum/";
    let base_url = unsafe { NSURL::URLWithString(&NSString::from_str(base_str)) };
    let main_str = "shader.metallib";
    let url = unsafe {
        NSURL::URLWithString_relativeToURL(&NSString::from_str(main_str), Some(&*base_url.unwrap()))
    };
    let library = device.new_library_with_url(&url.unwrap()).unwrap();
    let function = library.new_function_with_name(&NSString::from_str("test"));
    let cps = device
        .new_compute_pipeline_state_with_function_error(&function)
        .unwrap();

    let threadgroups_per_grid = Size::new(THREADGROUPS_PER_GRID, 1, 1);
    let threads_per_threadgroup = Size::new(THREADS_PER_THREADGROUP, 1, 1);

    // PREPARE DATA
    let resource_options = ResourceOptions::new(
        CPUCacheMode::DefaultCache,
        StorageMode::Shared,
        HazardTrackingMode::Default,
    );
    let input = device.new_buffer_with_length(LENGTH, resource_options.clone());
    let output = device.new_buffer_with_length(LENGTH, resource_options.clone());
    generate_random_data(&input);

    // TIMESTAMPS
    let sample_count = 6;
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
    encoder.set_buffer(&output, 0, 1);
    encoder.dispatch_threadgroups(threadgroups_per_grid, threads_per_threadgroup);
    encoder.end_encoding();
    command_buffer.commit();
    command_buffer.wait_until_completed();

    let timespans = get_timespans(
        &device,
        &sample_buffer,
        sample_count,
        cpu_start_time,
        gpu_start_time,
    );
    println!("timespans: {timespans:?}");

    // VERIFY RESULTS
    let inp: *mut u32 = input.contents().cast();
    let out: *mut u32 = output.contents().cast();
    unsafe {
        for i in 0..LENGTH {
            let j: isize = i.try_into().unwrap();
            println!("{}: {} -> {}", j, *inp.offset(j), *out.offset(j));
        }
    }
}

fn generate_random_data(buffer: &Buffer) {
    let mut rng = rand::thread_rng();

    let p: *mut u32 = buffer.contents().cast();
    unsafe {
        for i in 0..LENGTH {
            let j: isize = i.try_into().unwrap();
            *p.offset(j) = rng.gen::<u32>();
        }
    }
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
            milliseconds
            // nanoseconds
        })
        .collect();
    let timespans: Vec<usize> = timestamps
        .chunks(2)
        .map(|pair| (pair[1] - pair[0]) as usize)
        .collect();
    timespans
}
