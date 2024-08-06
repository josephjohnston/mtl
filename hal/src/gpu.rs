use super::*;

#[derive(Debug)]
pub enum GPUType {
    // Integrated,
    // Discrete,
    // External,
    Regular,
}

#[derive(Debug)]
pub struct GPUInfo {
    id: Retained<mtl::Device>,
    pub name: String,
    pub gpu_type: GPUType,
}

fn get_gpu_resource_options() -> mtl::ResourceOptions {
    mtl::ResourceOptions::new(
        mtl::CPUCacheMode::WriteCombined,
        mtl::StorageMode::Private,
        mtl::HazardTrackingMode::Untracked,
    )
}

#[derive(Debug)]
pub struct GPU {
    id: Retained<mtl::Device>,
    heap_desc: Retained<mtl::HeapDescriptor>,
    archive_desc: Retained<mtl::BinaryArchiveDescriptor>,
    multi_batch_desc: Retained<mtl::IndirectCommandBufferDescriptor>,
}
unsafe impl Send for GPU {}
unsafe impl Sync for GPU {}
impl GPU {
    fn get_gpu_type(id: &mtl::Device) -> GPUType {
        // #[cfg(target_os = "macos")]
        // if id.is_removable() {
        //     GPUType::External
        // } else if id.is_low_power() {
        //     GPUType::Integrated
        // } else {
        //     GPUType::Discrete
        // }
        // #[cfg(target_os = "ios")]
        // if id.is_low_power() {
        //     GPUType::Integrated
        // } else {
        //     GPUType::Discrete
        // }
        GPUType::Regular
    }
    // #[cfg(target_os = "ios")]
    pub fn current_gpus() -> Vec<GPUInfo> {
        let id = mtl::create_system_default_device();
        vec![GPUInfo {
            name: id.name().to_string(),
            gpu_type: Self::get_gpu_type(&id),
            id,
        }]
    }
    // #[cfg(target_os = "macos")]
    // pub fn current_gpus() -> Vec<GPUInfo> {
    //     let array = mtl::copy_all_devices();
    //     let ids: Vec<Retained<mtl::Device>> = (0..array.len())
    //         .map(|index| unsafe { msg_send_id![&array, objectAtIndex: index] })
    //         .collect::<Vec<_>>();
    //     ids.into_iter()
    //         .map(move |id| GPUInfo {
    //             name: id.name().to_string(),
    //             gpu_type: Self::get_gpu_type(&id),
    //             id,
    //         })
    //         .collect()
    // }
    // pub fn new_timestamp_sampler(&self, sample_count: usize, private: bool) -> TimestampSampler {
    //     TimestampSampler::new(&self.id, sample_count, private)
    // }
    pub fn resource_usage(&self) {
        println!(
            "current_allocated_size: {}, recommended_max_working_set_size: {}, has_unified_memory: {}",
            self.id.current_allocated_size(),
            self.id.recommended_max_working_set_size(),
            self.id.has_unified_memory()
        );
    }
    pub fn new(gpu_info: GPUInfo) -> Self {
        let id = gpu_info.id;
        // heap descriptor
        let heap_desc = mtl::HeapDescriptor::new();
        heap_desc.set_resource_options(get_gpu_resource_options());
        // archive descriptor
        let archive_desc = mtl::BinaryArchiveDescriptor::new();
        // multi batch descriptor
        let multi_batch_desc = mtl::IndirectCommandBufferDescriptor::new();

        Self {
            id,
            heap_desc,
            archive_desc,
            multi_batch_desc,
        }
    }
    pub fn new_queue(&self, name: String, max_command_buffers: usize) -> Queue {
        Queue::new(&self.id, name, max_command_buffers)
    }
    pub fn new_auto_heap(&self, name: String, length: usize) -> AutoHeap {
        AutoHeap::new(&self.id, &self.heap_desc, name, length)
    }
    pub fn new_self_heap(&self, name: String, length: usize) -> SelfHeap {
        SelfHeap::new(&self.id, &self.heap_desc, name, length)
    }
    pub fn new_buffer<T>(&self, name: String, len: usize, readable: bool) -> Buffer<T, Shared> {
        Buffer::<T, Shared>::new(&self.id, name, len, readable)
    }
    pub fn new_archive(&self, name: String, serialize: bool, shaders_url_str: &str) -> Archive {
        Archive::new(
            &self.id,
            &self.archive_desc,
            name,
            shaders_url_str,
            serialize,
        )
    }
    pub fn load_archive(&self, name: String, shaders_url_str: &str) -> Archive {
        Archive::load(&self.id, &self.archive_desc, name, shaders_url_str)
    }
    pub fn new_multi_batch(&self, max_command_count: usize) -> MultiBatch {
        MultiBatch::new(&self.id, &self.multi_batch_desc, max_command_count)
    }
    // pub fn simplified(&self, name: String) -> Result<Pipeline, String> {
    //     Archive::simplified(&self.id, &self.archive_desc, name)
    // }
}

#[derive(Debug)]
pub struct Queue {
    id: Retained<mtl::CommandQueue>,
    pub name: String,
    pub command_buffer_desc: Retained<mtl::CommandBufferDescriptor>,
}
unsafe impl Send for Queue {}
unsafe impl Sync for Queue {}
impl Queue {
    fn new(device: &mtl::Device, name: String, command_buffer_count: usize) -> Self {
        let id = device.new_command_queue_with_max_command_buffer_count(command_buffer_count);
        id.set_label(&NSString::from_str(&*name));
        let command_buffer_desc = mtl::CommandBufferDescriptor::new();
        Self {
            name,
            id,
            command_buffer_desc,
        }
    }
    // pub fn new_batch(&self, optimize: bool) -> Batch {
    //     Batch::new(&self.id, &self.command_buffer_desc, optimize)
    // }
    pub(crate) fn get_ref(&self) -> &mtl::CommandBufferDescriptor {
        &*self.command_buffer_desc
    }
}

pub struct TimestampSampler<'a> {
    device: &'a mtl::Device,
    sample_buffer: Retained<mtl::CounterSampleBuffer>,
    sample_count: usize,
    next_start_index: usize,
    cpu_start_time: mtl::Timestamp,
    // cpu_end_time: mtl::Timestamp,
    gpu_start_time: mtl::Timestamp,
    // gpu_end_time: mtl::Timestamp,
}
impl<'a> TimestampSampler<'a> {
    // pub(crate) fn new(device: &'a mtl::Device, sample_count: usize, private: bool) -> Self {
    //     assert!(device.supports_counter_sampling(mtl::CounterSamplingPoint::StageBoundary));
    //     let counter_set = device
    //         .counter_sets()
    //         .into_iter()
    //         .find(|counter_set| counter_set.name().to_string() == "timestamp")
    //         .unwrap();
    //     // fn get_counter_set() -> Option<Retained<mtl::CounterSet>> {
    //     //     for counter_set in device.counter_sets() {
    //     //         if counter_set.name().to_string() == "timestamp" {
    //     //             return Some(counter_set);
    //     //         }
    //     //     }
    //     //     None
    //     // }
    //     // let counter_set = get_counter_set(&*self.id).unwrap();
    //     let desc = mtl::CounterSampleBufferDescriptor::new();
    //     desc.set_label(&NSString::from_str("timestamp counter"));
    //     desc.set_counter_set(&*counter_set);
    //     desc.set_sample_count(2 * sample_count);
    //     desc.set_storage_mode(match private {
    //         false => mtl::StorageMode::Shared,
    //         true => mtl::StorageMode::Private,
    //     });
    //     let sample_buffer = device
    //         .new_counter_sample_buffer_with_descriptor(&desc)
    //         .unwrap();
    //     let cpu_start_time = mtl::Timestamp::new(0);
    //     let gpu_start_time = mtl::Timestamp::new(0);
    //     device.sample_timestamps(&cpu_start_time, &gpu_start_time);
    //     Self {
    //         device,
    //         sample_buffer,
    //         sample_count,
    //         next_start_index: 0,
    //         cpu_start_time,
    //         gpu_start_time,
    //     }
    // }
    // pub(crate) fn get_ref(&self) -> &mtl::CounterSampleBuffer {
    //     &*self.sample_buffer
    // }
    // pub(crate) fn get_next_start_index(&mut self) -> usize {
    //     assert!(self.next_start_index + 1 < 2 * self.sample_count);
    //     let next_start_index = self.next_start_index;
    //     self.next_start_index += 2;
    //     next_start_index
    // }
    // pub fn get_timestamps(&self) {
    //     let cpu_end_time = mtl::Timestamp::new(0);
    //     let gpu_end_time = mtl::Timestamp::new(0);
    //     self.device.sample_timestamps(&cpu_end_time, &gpu_end_time);
    //     let cpu_time_span = cpu_end_time.value - self.cpu_start_time.value;
    //     let gpu_time_span = gpu_end_time.value - self.gpu_start_time.value;
    //     let gpu_to_cpu_scaling = cpu_time_span / gpu_time_span;

    //     let data = self
    //         .sample_buffer
    //         .resolve_counter_range(NSRange::from(0..2 * self.sample_count));
    //     let timestamps: Vec<usize> = data
    //         .bytes()
    //         .chunks(8)
    //         .map(|limbs| {
    //             let mut gpu_time = 0;
    //             for i in 0..limbs.len() {
    //                 gpu_time += limbs[i] as usize * ((1 << 8) as usize).pow(i as u32);
    //             }
    //             let nanoseconds = self.cpu_start_time.value + gpu_time * gpu_to_cpu_scaling;
    //             let microseconds = nanoseconds / (10 as usize).pow(3);
    //             let milliseconds = microseconds / (10 as usize).pow(3);
    //             // nanoseconds
    //             // microseconds
    //             milliseconds
    //         })
    //         .collect();
    //     let timespans: Vec<usize> = timestamps.chunks(2).map(|pair| pair[1] - pair[0]).collect();
    //     println!("{timespans:?}");
    // }
}
