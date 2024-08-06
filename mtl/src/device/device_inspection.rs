use super::*;

/// # Checking a GPU Device's Feature Support
impl Device {
    // [M] supportsFamily:
    pub fn supports_family(&self, gpu_family: GPUFamily) -> bool {
        unsafe { msg_send![self, supportsFamily: gpu_family] }
    }
}

// [E] MTLGPUFamily
#[repr(isize)]
#[non_exhaustive]
pub enum GPUFamily {
    Apple1 = 1001,
    Apple2 = 1002,
    Apple3 = 1003,
    Apple4 = 1004,
    Apple5 = 1005,
    Apple6 = 1006,
    Apple7 = 1007,
    Apple8 = 1008,

    // Mac1 = 2001, (depreciated with replacement Mac2)
    Mac2 = 2002,

    Common1 = 3001,
    Common2 = 3002,
    Common3 = 3003,

    // MacCatalyst1 = 4001, (depreciated with replacement Mac2)
    // MacCatalyst2 = 4002, (depreciated with replacement Mac2)
    Metal3 = 5001,
}
unsafe impl Encode for GPUFamily {
    const ENCODING: Encoding = isize::ENCODING;
}

impl GPUFamily {
    pub fn supports_compressed_volume_texture_formats(&self) -> bool {
        true
    }
}

/// # Identifying a GPU Device
impl Device {
    // [P] name
    pub fn name(&self) -> Retained<NSString> {
        unsafe { msg_send_id![self, name] }
    }
    // [P] registryID
    pub fn registry_id(&self) -> u64 {
        unsafe { msg_send![self, registryID] }
    }
    // [P] location
    pub fn location(&self) -> DeviceLocation {
        unsafe { msg_send![self, location] }
    }
    // [P] locationNumber
    pub fn location_number(&self) -> usize {
        unsafe { msg_send![self, locationNumber] }
    }
    // [P] lowPower
    pub fn is_low_power(&self) -> bool {
        unsafe { msg_send![self, isLowPower] }
    }
    // [P] removable
    #[cfg(target_os = "macos")]
    pub fn is_removable(&self) -> bool {
        unsafe { msg_send![self, isRemovable] }
    }
    // [P] headless
    pub fn is_headless(&self) -> bool {
        unsafe { msg_send![self, isHeadless] }
    }
    // [P] peerGroupID
    pub fn peer_group_id(&self) -> u64 {
        unsafe { msg_send![self, peerGroupID] }
    }
    // [P] peerCount
    pub fn peer_count(&self) -> u32 {
        unsafe { msg_send![self, peerCount] }
    }
    // [P] peerIndex
    pub fn peer_index(&self) -> u32 {
        unsafe { msg_send![self, peerIndex] }
    }
}

// [E] MTLDeviceLocation
#[repr(usize)]
#[derive(Debug)]
pub enum DeviceLocation {
    BuiltIn = 0,
    Slot = 1,
    External = 2,
    Unspecified = usize::MAX,
}
unsafe impl Encode for DeviceLocation {
    const ENCODING: Encoding = usize::ENCODING;
}

/// # Checking Compute Support
impl Device {
    // [P] maxThreadgroupMemoryLength
    pub fn max_threadgroup_memory_length(&self) -> usize {
        unsafe { msg_send![self, maxThreadgroupMemoryLength] }
    }
    // [P] maxThreadsPerThreadgroup
    pub fn max_threads_per_threadgroup(&self) -> Size {
        unsafe { msg_send![self, maxThreadsPerThreadgroup] }
    }
}

/// # Checking Render Support
impl Device {
    // [P] readWriteTextureSupport
    pub fn read_write_texture_support(&self) -> ReadWriteTextureTier {
        unsafe { msg_send![self, readWriteTextureSupport] }
    }
}

// [E] MTLReadWriteTextureTier
#[repr(usize)]
#[derive(Debug)]
pub enum ReadWriteTextureTier {
    None = 0,
    Tier1 = 1,
    Tier2 = 2,
}
unsafe impl Encode for ReadWriteTextureTier {
    const ENCODING: Encoding = usize::ENCODING;
}

/// # Checking Function Pointer Support
impl Device {
    // [P] supportsFunctionPointers
    pub fn supports_function_pointers(&self) -> bool {
        unsafe { msg_send![self, supportsFunctionPointers] }
    }
    // [P] supportsFunctionPointersFromRender
    pub fn supports_function_pointers_from_render(&self) -> bool {
        unsafe { msg_send![self, supportsFunctionPointersFromRender] }
    }
}

/// # Checking a GPU Device's Memory
impl Device {
    // [P] currentAllocatedSize
    pub fn current_allocated_size(&self) -> usize {
        unsafe { msg_send![self, currentAllocatedSize] }
    }
    // [P] recommendedMaxWorkingSetSize
    // iOS unavailable
    pub fn recommended_max_working_set_size(&self) -> u64 {
        unsafe { msg_send![self, recommendedMaxWorkingSetSize] }
    }
    // [P] hasUnifiedMemory
    pub fn has_unified_memory(&self) -> bool {
        unsafe { msg_send![self, hasUnifiedMemory] }
    }
    // [P] maxTransferRate
    // iOS unavailable
    pub fn max_transfer_rate(&self) -> u64 {
        unsafe { msg_send![self, maxTransferRate] }
    }
}

/// # Sampling a GPU Device's Counters
impl Device {
    // [P] counterSets
    pub fn counter_sets(&self) -> Retained<NSArray<CounterSet>> {
        // Vec<Retained<CounterSet>> {
        unsafe {
            let array: Retained<NSArray<CounterSet>> = msg_send_id![self, counterSets];
            array
            // array
            //     .to_vec()
            //     .into_iter()
            //     .map(|x| x as *const CounterSet as *mut CounterSet)
            //     .map(|x| Id::retain(x).unwrap())
            //     .collect()
        }
    }
    // [M] supportsCounterSampling:
    pub fn supports_counter_sampling(&self, sampling_point: CounterSamplingPoint) -> bool {
        unsafe { msg_send![self, supportsCounterSampling: sampling_point] }
    }
    // [M] newCounterSampleBufferWithDescriptor:error:
    pub fn new_counter_sample_buffer_with_descriptor(
        &self,
        counter_sample_buffer_descriptor: &CounterSampleBufferDescriptor,
    ) -> Result<Retained<CounterSampleBuffer>, Retained<NSString>> {
        unsafe {
            let mut raw_error: *mut NSError = std::ptr::null_mut();
            let raw_id_t: *mut CounterSampleBuffer = msg_send![
                self,
                newCounterSampleBufferWithDescriptor: counter_sample_buffer_descriptor,
                error: &mut raw_error,
            ];
            if raw_error.is_null() {
                Ok(Retained::from_raw(raw_id_t).expect(ID_NEW_FAILURE))
            } else {
                let error: Retained<NSError> =
                    Retained::retain_autoreleased(raw_error).expect(ID_RETAIN_AUTO_FAILURE);
                Err(error.localizedDescription())
            }
        }
    }
}

/// # Correlating Timestamp Samples
impl Device {
    // sampleTimestamps:gpuTimestamp:
    pub fn sample_timestamps(&self, cpu_timestamp: &Timestamp, gpu_timestamp: &Timestamp) {
        unsafe {
            msg_send![
                self,
                sampleTimestamps: cpu_timestamp,
                gpuTimestamp: gpu_timestamp
            ]
        }
    }
}
