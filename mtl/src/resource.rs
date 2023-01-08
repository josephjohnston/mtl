use super::*;

// [S] GPUResourceID
#[repr(C)]
pub struct GPUResourceID {
    _impl: u64,
}
unsafe impl Encode for GPUResourceID {
    const ENCODING: Encoding = Encoding::Struct("GPUResourceID", &[u64::ENCODING]);
}

// [Pr] MTLResource
declare!(Resource);

/// # Identifying the Resource
impl Label for Resource {}
impl Resource {
    // [P] device
    pub fn device(&self) -> Id<Device> {
        unsafe { msg_send_id![self, device] }
    }
}

/// # Reading Memory and Storage Properties
impl Resource {
    // [P] cpuCacheMode
    pub fn cpu_cache_mode(&self) -> CPUCacheMode {
        unsafe { msg_send![self, cpuCacheMode] }
    }
    // [P] storageMode
    pub fn storage_mode(&self) -> StorageMode {
        unsafe { msg_send![self, storageMode] }
    }
    // [P] hazardTrackingMode
    pub fn hazard_tracking_mode(&self) -> HazardTrackingMode {
        unsafe { msg_send![self, hazardTracking] }
    }
    // [P] resourceOptions
    pub fn resource_options(&self) -> ResourceOptions {
        unsafe { msg_send![self, resourceOptions] }
    }
}

/// # Setting the Puregeable State of the Resource
impl Resource {
    // [M] setPuregeableState:
    pub fn set_purgeable_state(&self, state: PurgeableState) {
        unsafe { msg_send![self, setPuregeableState: state] }
    }
}

/// # Managing Heap Resources
impl Resource {
    // [P] heap
    pub fn heap(&self) -> Id<Heap> {
        unsafe { msg_send_id![self, heap] }
    }
    // [P] heapOffset
    pub fn heap_offset(&self) -> usize {
        unsafe { msg_send![self, heapOffset] }
    }
    // [M] isAliasable
    pub fn is_aliasable(&self) -> bool {
        unsafe { msg_send![self, isAliasable] }
    }
    // [M] makeAliasable
    pub fn make_aliasable(&self) {
        unsafe { msg_send![self, makeAliasable] }
    }
}

/// # Querying the Allocated Size
impl Resource {
    // [M] allocatedSize
    pub fn allocated_size(&self) -> usize {
        unsafe { msg_send![self, allocatedSize] }
    }
}

// [S] MTLResourceID
// iOS only 16.0+
#[repr(C)]
#[derive(Debug)]
pub struct ResourceID {
    _impl: u64,
}
unsafe impl Encode for ResourceID {
    const ENCODING: Encoding = Encoding::Struct("ResourceID", &[u64::ENCODING]);
}

// [E] MTLResourceUsage
bitflags! {
    pub struct ResourceUsage: usize {
        const READ = 1 << 0;
        const WRITE = 1 << 1;
    }
}

// [E] MTLPuregeableState
#[repr(usize)]
#[derive(Debug)]
pub enum PurgeableState {
    KeepCurrent = 1,
    NonVolatile = 2,
    Volatile = 3,
    Empty = 4,
}
unsafe impl Encode for PurgeableState {
    const ENCODING: Encoding = usize::ENCODING;
}

// [E] MTLResourceOptions
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ResourceOptions(usize);
impl ResourceOptions {
    pub fn new(
        cache_mode: CPUCacheMode,
        storage_mode: StorageMode,
        hazard_tracking_mode: HazardTrackingMode,
    ) -> ResourceOptions {
        let x = (cache_mode as usize) << RESOURCE_CPU_CACHE_MODE_SHIFT;
        let y = (storage_mode as usize) << RESOURCE_STORAGE_MODE_SHIFT;
        let z = (hazard_tracking_mode as usize) << RESOURCE_HAZARD_TRACKING_MODE_SHIFT;
        Self(x + y + z)
    }
    pub fn default(private: bool) -> ResourceOptions {
        let cache_mode = CPUCacheMode::DefaultCache;
        let hazard_tracking_mode = HazardTrackingMode::Default;
        let storage_mode = match private {
            true => StorageMode::Private,
            false => match cfg!(target_os = "macos") {
                true => StorageMode::Managed,
                false => StorageMode::Shared,
            },
        };
        Self::new(cache_mode, storage_mode, hazard_tracking_mode)
    }
}
impl_encode_for_type!(ResourceOptions: usize);

// [E] MTLCPUCacheMode
#[repr(usize)]
#[derive(Debug)]
pub enum CPUCacheMode {
    DefaultCache = 0,
    WriteCombined = 1,
}
impl_encode_for_type!(CPUCacheMode: usize);

// [E] MTLStorageMode
#[repr(usize)]
#[derive(Debug)]
pub enum StorageMode {
    // default for iOS
    Shared = 0,
    // default for OS X
    // unavailable on iOS
    Managed = 1,
    Private = 2,
    Memoryless = 3,
}
impl_encode_for_type!(StorageMode: usize);

// [E] MTLHazardTrackingMode
#[repr(usize)]
#[derive(Debug)]
pub enum HazardTrackingMode {
    Default = 0,
    Untracked = 1,
    Tracked = 2,
}
impl_encode_for_type!(HazardTrackingMode: usize);

// shifts
const RESOURCE_CPU_CACHE_MODE_SHIFT: usize = 0;
const RESOURCE_STORAGE_MODE_SHIFT: usize = 4;
const RESOURCE_HAZARD_TRACKING_MODE_SHIFT: usize = 8;
