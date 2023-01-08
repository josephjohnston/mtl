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
    id: Id<mtl::Device>,
    pub name: String,
    pub gpu_type: GPUType,
}

#[derive(Debug)]
pub struct GPU {
    id: Id<mtl::Device>,
    heap_desc: Id<mtl::HeapDescriptor>,
    archive_desc: Id<mtl::BinaryArchiveDescriptor>,
    multi_batch_desc: Id<mtl::IndirectCommandBufferDescriptor>,
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
    //     let ids: Vec<Id<mtl::Device>> = (0..array.len())
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
    pub fn new(gpu_info: GPUInfo, name: String) -> Self {
        let id = gpu_info.id;
        let heap_desc = mtl::HeapDescriptor::new();
        heap_desc.set_resource_options(get_gpu_resource_options());
        let archive_desc = mtl::BinaryArchiveDescriptor::new();
        let multi_batch_desc = mtl::IndirectCommandBufferDescriptor::new();
        let x = id.read_write_texture_support();
        println!("{:?}", x);
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
    pub fn new_buffer<T>(&self, name: String, len: usize) -> Buffer<T, Shared> {
        Buffer::<T, Shared>::new(&self.id, name, len)
    }
    pub fn new_archive(
        &self,
        name: String,
        serialize: bool,
        shaders_url_string: String,
    ) -> Archive {
        Archive::new(
            &self.id,
            &self.archive_desc,
            name,
            serialize,
            shaders_url_string,
        )
    }
    pub fn load_archive(&self, name: String) -> Archive {
        Archive::load(&self.id, &self.archive_desc, name)
    }
    pub fn new_multi_batch(&self, max_command_count: usize) -> MultiBatch {
        MultiBatch::new(&self.id, &self.multi_batch_desc, max_command_count)
    }
    // pub fn simplified(&self, name: String) -> Result<Pipeline, String> {
    //     Archive::simplified(&self.id, &self.archive_desc, name)
    // }
    // pub fn new_texture(&self) -> Id<mtl::Texture> {
    // }
}

#[derive(Debug)]
pub struct Queue {
    id: Id<mtl::CommandQueue>,
    pub name: String,
    pub command_buffer_desc: Id<mtl::CommandBufferDescriptor>,
    // device
}
unsafe impl Send for Queue {}
unsafe impl Sync for Queue {}
impl Queue {
    fn new(raw_device: &mtl::Device, name: String, max_command_buffers: usize) -> Self {
        assert!(max_command_buffers <= 64);
        let id = raw_device.new_command_queue_with_max_command_buffer_count(max_command_buffers);
        id.set_label(&NSString::from_str(&*name));
        let command_buffer_desc = mtl::CommandBufferDescriptor::new();
        Self {
            name,
            id,
            command_buffer_desc,
        }
    }
    pub fn new_batch(&self, optimize: bool) -> Batch {
        Batch::new(&self.id, &self.command_buffer_desc, optimize)
    }
    pub fn get_ref(&self) -> &mtl::CommandBufferDescriptor {
        &*self.command_buffer_desc
    }
}
