use super::*;

declare!(HeapDescriptor);

impl HeapDescriptor {
    pub fn new() -> Id<Self> {
        unsafe {
            let raw_desc = msg_send_id![class!(MTLHeapDescriptor), alloc];
            let desc: Id<Self> = msg_send_id![raw_desc, init];
            desc
        }
    }
}

/// # Specifying Heap Attributes
impl HeapDescriptor {
    // [P] size and setSize:
    pub fn size(&self) -> usize {
        unsafe { msg_send![self, size] }
    }
    pub fn set_size(&self, size: usize) {
        unsafe { msg_send![self, setSize: size] }
    }
    // [P] type and setType:
    pub fn heap_type(&self) -> HeapType {
        unsafe { msg_send![self, type] }
    }
    pub fn set_heap_type(&self, heap_type: HeapType) {
        unsafe { msg_send![self, setType: heap_type] }
    }
    // [P] cpuCacheMode and setCpuCacheMode:
    pub fn cpu_cache_mode(&self) -> CPUCacheMode {
        unsafe { msg_send![self, cpuCacheMode] }
    }
    pub fn set_cpu_cache_mode(&self, mode: CPUCacheMode) {
        unsafe { msg_send![self, setCpuCacheMode: mode] }
    }
    // [P] storageMode and setStorageMode:
    pub fn storage_mode(&self) -> StorageMode {
        unsafe { msg_send![self, storageMode] }
    }
    pub fn set_storage_mode(&self, mode: StorageMode) {
        unsafe { msg_send![self, setStorageMode: mode] }
    }
    // [P] hazardTrackingMode and setHazardTrackingMode:
    pub fn hazard_tracking_mode(&self) -> HazardTrackingMode {
        unsafe { msg_send![self, hazardTrackingMode] }
    }
    pub fn set_hazard_tracking_mode(&self, mode: HazardTrackingMode) {
        unsafe { msg_send![self, setHazardTrackingMode: mode] }
    }
    // [P] resourceOptions and setResourceOptions:
    pub fn resource_options(&self) -> ResourceOptions {
        unsafe { msg_send![self, resourceOptions] }
    }
    pub fn set_resource_options(&self, resource_options: ResourceOptions) {
        unsafe { msg_send![self, setResourceOptions: resource_options] }
    }
}
