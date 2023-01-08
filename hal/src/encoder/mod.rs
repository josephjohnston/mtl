use super::*;

mod blit;
pub use blit::*;

mod compute;
pub use compute::*;

pub fn count(device: &mtl::Device) {
    let sets = device.counter_sets();
    for set in sets {
        println!(
            "{} has {} counters",
            set.name().to_string(),
            set.counters().len()
        );
        for counter in set.counters() {
            println!("   {}", counter.name().to_string());
        }
    }
    // let counter_set = &sets[0];
    // // assert!(device.supports_counter_sampling(mtl::CounterSamplingPoint::DispatchBoundary));
    // let desc = mtl::CounterSampleBufferDescriptor::new();
    // desc.set_label(&NSString::from_str("counter sample buffer"));
    // desc.set_counter_set(&counter_set);
    // desc.set_sample_count(1);
    // desc.set_storage_mode(mtl::StorageMode::Shared);
    // let buffer = device.new_counter_sample_buffer_with_descriptor(&desc);
    // // buffer
}
