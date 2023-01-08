use super::*;

// we pair archives with libraries.
// a single function descriptor, (compute) pipeline descriptor, and stage input/output descriptor will be associated and reused when necessary.

const BASE_URL_STR: &str = "file:///Users/josephjohnston/saga/zpu/shaders/";

// contains the mtl-archive
pub struct Archive<'a> {
    id: Id<mtl::BinaryArchive>,
    device: &'a mtl::Device,
    library: Id<mtl::Library>,
    func_desc: Id<mtl::FunctionDescriptor>,
    io_desc: Id<mtl::StageInputOutputDescriptor>,
    pipeline_desc: Id<mtl::ComputePipelineDescriptor>,
}
impl<'a> Archive<'a> {
    // we'll have all shader related stuff in a folder with url get_base_url()
    fn get_base_url() -> Id<NSURL> {
        let base_url = NSURL::url_with_string(&NSString::from_str(BASE_URL_STR));
        base_url
    }
    // inside the shader folder, we'll have libraries as files with <lib_name>.metallib.
    fn get_library_url(name_str: &str) -> Id<NSURL> {
        let library_url = NSURL::url_with_string_relative_to_url(
            &NSString::from_str(&*format!("{}{}", name_str, ".metallib")),
            &Self::get_base_url(),
        );
        library_url
    }
    // inside the shader folder, we have an archive folder, with an archive as <lib_name>.metallib.
    fn get_archive_url(name_str: &str) -> Id<NSURL> {
        let archive_url = NSURL::url_with_string_relative_to_url(
            &NSString::from_str(&*format!("archives/{}{}", name_str, ".metallib")),
            &Self::get_base_url(),
        );
        archive_url
    }
    pub(crate) fn new(
        device: &'a mtl::Device,
        desc: &mtl::BinaryArchiveDescriptor,
        name: String,
        serialize: bool,
        shaders_url_string: String,
    ) -> Self {
        let name_str = &*name;
        let instance = Self::inner_new(device, desc, name_str, false);
        instance.populate_with_pipelines(name_str);
        if serialize {
            instance
                .id
                .serialize_to_url(&Self::get_archive_url(name_str))
                .unwrap();
        }
        instance
    }
    pub(crate) fn load(
        device: &'a mtl::Device,
        desc: &mtl::BinaryArchiveDescriptor,
        name: String,
    ) -> Self {
        Self::inner_new(device, desc, &*name, true)
    }
    // creates library and descriptors, and set universal properties.
    fn inner_new<'b>(
        device: &'a mtl::Device,
        desc: &mtl::BinaryArchiveDescriptor,
        name_str: &'b str,
        loading: bool,
    ) -> Self {
        if loading {
            desc.set_url(&Self::get_archive_url(name_str));
        }
        let id = device.new_binary_archive_with_descriptor(&desc).unwrap();

        let label = NSString::from_str(name_str);

        let bundle = Bundle::get_bundle().unwrap();
        // let library_url = bundle
        //     .url_for_resource(
        //         &NSString::from_str("shader_ios"),
        //         &NSString::from_str("metallib"),
        //         &NSString::from_str("shaders"),
        //     )
        //     .unwrap();
        let library_url = Self::get_library_url("shader_macos");
        let library = device.new_library_with_url(&library_url).unwrap();
        library.set_label(&label);
        let func_desc = mtl::FunctionDescriptor::function_descriptor();
        let io_desc = mtl::StageInputOutputDescriptor::stage_input_output_descriptor();
        let pipeline_desc = mtl::ComputePipelineDescriptor::new();
        pipeline_desc.set_label(&label);
        // pipeline_desc.set_thread_group_size_is_multiple_of_execution_width(Bool::from(true));
        // pipeline_desc.set_support_indirect_command_buffers(Bool::from(true));
        pipeline_desc.set_max_call_stack_depth(1);
        // if execution_width {
        pipeline_desc.set_thread_group_size_is_multiple_of_execution_width(Bool::from(true));
        // }
        Self {
            id,
            device,
            library,
            func_desc,
            io_desc,
            pipeline_desc,
        }
    }
    // convert every function in library to pipeline and add to archive
    fn populate_with_pipelines(&self, name_str: &'a str) {
        let func_names = self.library.function_names();
        for i in 0..func_names.len() {
            let func_name = func_names.get(i).unwrap();
            self.func_desc.set_name(func_name);
            // self.func_desc.set_binary_archives(&NSArray::from_vec(vec![self.id.clone()]));
            let func = self
                .library
                .new_function_with_descriptor_error(&self.func_desc)
                .unwrap();
            Self::add_pipeline_desc(self, func_name, &func);
        }
    }
    fn add_pipeline_desc(&self, func_name: &NSString, func: &mtl::Function) {
        let desc = &self.pipeline_desc;
        desc.set_label(func_name);
        desc.set_compute_function(func);

        // self.io_desc.reset();
        // let attribute_array = self.io_desc.attributes();
        // let x = attribute_array.object_at_index_subscript(1);
        // // oh, this is for re-ordering the args I guess.
        // // index
        // // offset
        // // x.set_format(mtl::AttributeFormat::UInt2);
        // let layout_array = self.io_desc.layouts();
        // let y = layout_array.object_at_indexed_subscript(0);
        // y.set_stride(1);
        // y.set_step_function(mtl::StepFunction::PerInstance);
        // y.set_step_rate(2);
        // self.io_desc.set_index_buffer_index(2);
        // desc.set_stage_input_descriptor(&self.io_desc);

        // let buffer_mutability_array = desc.buffers();
        // let z = buffer_mutability_array.object_at_indexed_subscript(0);
        // // let desc_0 = mtl::PipelineBufferDescriptor();
        // // desc_0.set_mutability();
        // // buffer_mutability_array.set_object_at_indexed_subscript(&buff_0, 0);

        // println!("buffers: {:?}", obj.mutability());
        let y = self
            .id
            .add_compute_pipeline_functions_with_descriptor(&desc)
            .unwrap();
    }
    pub fn load_pipeline(&self, func_name: String) -> Result<Pipeline, String> {
        self.func_desc.set_name(&NSString::from_str(&*func_name));
        let func = self
            .library
            .new_function_with_descriptor_error(&self.func_desc)
            .unwrap();
        self.pipeline_desc.set_compute_function(&func);
        let archives = NSArray::from_vec(vec![self.id.clone()]);
        self.pipeline_desc.set_binary_archives(&archives);
        Pipeline::new(self.device, &self.pipeline_desc)
    }
    // pub fn simplified(
    //     device: &mtl::Device,
    //     desc: &mtl::BinaryArchiveDescriptor,
    //     name: String,
    // ) -> Result<Pipeline, String> {
    //     let name_str = &*name;
    //     let archive = device.new_binary_archive_with_descriptor(&desc).unwrap();
    //     let label = NSString::from_str(name_str);

    //     let bundle = Bundle::get_bundle().unwrap();
    //     let library_url = bundle
    //         .url_for_resource(
    //             &NSString::from_str("shader_macos"),
    //             &NSString::from_str("metallib"),
    //             &NSString::from_str("shaders"),
    //         )
    //         .unwrap();
    //     // let library_url = &Self::get_library_url("shader_macos");
    //     let library = device.new_library_with_url(&library_url).unwrap();
    //     library.set_label(&label);
    //     let func_desc = mtl::FunctionDescriptor::function_descriptor();
    //     let func_name = &NSString::from_str("reduce_neighborhood");
    //     func_desc.set_name(func_name);
    //     let func = library
    //         .new_function_with_descriptor_error(&func_desc)
    //         .unwrap();

    //     let pipeline_desc = mtl::ComputePipelineDescriptor::new();
    //     pipeline_desc.set_label(&label);
    //     pipeline_desc.set_compute_function(&func);
    //     let y = archive
    //         .add_compute_pipeline_functions_with_descriptor(&pipeline_desc)
    //         .unwrap();

    //     let archives = NSArray::from_vec(vec![archive.clone()]);
    //     println!("resetting");
    //     pipeline_desc.reset();
    //     pipeline_desc.set_binary_archives(&archives);
    //     Pipeline::new(device, &pipeline_desc)
    // }
}

// usually we'd want to create an archive, store it, and load it later.
// we'd run new, serialize, then load on another occasion, and loadPipeline.
// but we'd like to just prototype, not serialize cuz we might change it every time.
// so in order to load a pipeline, we

// let func = lib_id.new_func
// let max_call_stack_depth = 2;
// configure_pipeline_descriptor(
//     &desc,
//     &label_id,
//     &func,
//     max_call_stack_depth,
// );
// let options = mtl::PipelineOption::None;
// let handler = mtl::ConcreteBlock::new( |ipeline: &mtl::ComputePipelineState, reflection: &mtl::ComputePipelineReflection, error: &NSError| {}).copy();
// raw_device.new_compute_pipeline_state_with_descriptor_handler(&desc, options, &handler);

// let name_str = &*name;
// let desc = mtl::BinaryArchiveDescriptor::new();
// desc.set_url(&Self::get_archive_url(name_str));
// let id = device.new_binary_archive_with_descriptor(&desc).unwrap();

// let library = device
//     .new_library_with_url(&Self::get_library_url(name_str))
//     .unwrap();

// library.set_label(&NSString::from_str(name_str));
// let func_desc = mtl::FunctionDescriptor::function_descriptor();
// func_desc.set_name(&NSString::from_str("go"));

// let pipeline_desc = mtl::ComputePipelineDescriptor::new();
// let func = library
//     .new_function_with_descriptor_error(&func_desc)
//     .unwrap();
// pipeline_desc.set_compute_function(&func);
// let archives = NSArray::from_vec(vec![self.id.clone()]);
// pipeline_desc.set_binary_archives(&archives);

// let options = mtl::PipelineOption::FailOnBinaryArchiveMiss;
// let id_result = device
//     .new_compute_pipeline_state_with_descriptor_error(&pipeline_desc, options)
//     .unwrap();
// }
