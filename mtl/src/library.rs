use super::*;

// [E] MTLLibraryType
#[repr(isize)]
pub enum LibraryType {
    Executable = 0,
    Dynamic = 1,
}
impl_encode_for_type!(LibraryType: isize);

// [C] MTLCompilerOptions
declare!(CompileOptions);
impl CompileOptions {
    pub fn new() -> Retained<Self> {
        unsafe { msg_send_id![class!(MTLCompileOptions), new] }
    }
    // [P] libraries and compileLibraries
    pub fn libraries(&self) -> Retained<NSArray<DynamicLibrary>> {
        unsafe { msg_send_id![self, libraries] }
    }
    pub fn set_libraries(&self, libraries: &NSArray<DynamicLibrary>) {
        unsafe { msg_send![self, setLibraries: libraries] }
    }
    // [P] libraryType and setLibraryType
    pub fn library_type(&self) -> LibraryType {
        unsafe { msg_send![self, libraryType] }
    }
    pub fn set_library_type(&self, library_type: LibraryType) {
        unsafe { msg_send![self, setLibraryType: library_type] }
    }
    // [P] installName and setInstallName
    pub fn install_name(&self) -> Retained<NSString> {
        unsafe { msg_send_id![self, installName] }
    }
    pub fn set_install_name(&self, install_name: &NSString) {
        unsafe { msg_send![self, setInstallName: install_name] }
    }
}

// [Pr] MTLLibrary
declare!(Library);
impl Label for Library {}
impl Library {
    // [P] device
    pub fn device(&self) -> Retained<Device> {
        unsafe { msg_send_id![self, device] }
    }
    // [P] installName
    pub fn install_name(&self) -> Retained<NSString> {
        unsafe { msg_send_id![self, installName] }
    }
    // [P] type
    pub fn library_type(&self) -> LibraryType {
        unsafe { msg_send![self, type] }
    }
    // [P] functionNames
    pub fn function_names(&self) -> Retained<NSArray<NSString>> {
        unsafe { msg_send_id![self, functionNames] }
    }
    // [M] newFunctionWithName:
    pub fn new_function_with_name(&self, function_name: &NSString) -> Retained<Function> {
        unsafe {
            let raw_function: *mut Function = msg_send![self, newFunctionWithName: function_name];
            Retained::from_raw(raw_function).expect(ID_NEW_FAILURE)
        }
    }
    // [M] newFunctionWithName:constantValues:error:
    pub fn new_function_with_name_error(
        &self,
        name: &NSString,
        constant_values: Option<&FunctionConstantValues>,
    ) -> TryNewCatch<Function> {
        try_new_catch!(raw_error<Function> => msg_send![
            self,
            newFunctionWithName: name,
            constantValues: constant_values,
            error: &mut raw_error
        ])
    }
    // [M] newFunctionWithName:constantValues:completionHandler:
    pub fn new_function_with_name_handler(
        &self,
        name: &NSString,
        constant_values: Option<&FunctionConstantValues>,
        handler: &block::Block<(&Function, *mut NSError), ()>,
    ) {
        unsafe {
            msg_send![
                self,
                newFunctionWithName: name,
                constantValues: constant_values,
                completionHandler: handler
            ]
        }
    }
    // [M] newFunctionWithDescriptor:error:
    pub fn new_function_with_descriptor_error(
        &self,
        descriptor: &FunctionDescriptor,
    ) -> TryNewCatch<Function> {
        try_new_catch!(raw_error<Function> => msg_send![
            self,
            newFunctionWithDescriptor: descriptor,
            error: &mut raw_error
        ])
    }
    // [M] newFunctionWithDescriptor:completionHandler:
    pub fn new_function_with_descriptor_handler(
        &self,
        descriptor: &FunctionDescriptor,
        handler: &block::Block<(&Function, *mut NSError), ()>,
    ) {
        unsafe {
            msg_send![
                self,
                newFunctionWithDescriptor: descriptor,
                completionHandler: handler
            ]
        }
    }
}

// [Pr] MTLDynamicLibrary
declare!(DynamicLibrary);
impl Label for DynamicLibrary {}
impl DynamicLibrary {
    // [P] device
    pub fn device(&self) -> Retained<Device> {
        unsafe { msg_send_id![self, device] }
    }
    // [P] installName
    pub fn install_name(&self) -> Retained<NSString> {
        unsafe { msg_send_id![self, installName] }
    }
    // [M] serializeToURL:error:
    pub fn serialize_to_url(&self, url: &NSURL) -> TryCatch {
        try_catch!(raw_error => msg_send![self, serializeToURL: url, error: &mut raw_error])
    }
}

// [C] MTLBinaryArchiveDescriptor
declare!(BinaryArchiveDescriptor);
impl BinaryArchiveDescriptor {
    pub fn new() -> Retained<Self> {
        unsafe { msg_send_id![class!(MTLBinaryArchiveDescriptor), new] }
    }
    // [P] url and setUrl
    pub fn url(&self) -> Retained<NSURL> {
        unsafe { msg_send_id![self, url] }
    }
    pub fn set_url(&self, url: &NSURL) {
        unsafe { msg_send![self, setUrl: url] }
    }
}

// [Pr] MTLBinaryArchive
declare!(BinaryArchive);
impl Label for BinaryArchive {}
impl BinaryArchive {
    // [P] device
    pub fn device(&self) -> Retained<Device> {
        unsafe { msg_send_id![self, device] }
    }
    // [M] addComputePipelineFunctionsWithDescriptor:error
    pub fn add_compute_pipeline_functions_with_descriptor(
        &self,
        descriptor: &ComputePipelineDescriptor,
    ) -> TryCatch {
        try_catch!(raw_error => msg_send![
            self,
            addComputePipelineFunctionsWithDescriptor: descriptor,
            error: &mut raw_error
        ])
        // unsafe {
        //     let mut raw_error: *mut NSError = std::ptr::null_mut();
        //     let () = msg_send![
        //         self,
        //         addComputePipelineFunctionsWithDescriptor: descriptor,
        //         error: &mut raw_error
        //     ];
        //     if raw_error.is_null() {
        //         println!("got it");
        //         Ok(())
        //     } else {
        //         println!("ppo[s] it");
        //         let error: Retained<NSError> =
        //             Id::retain_autoreleased(raw_error).expect(ID_RETAIN_AUTO_FAILURE);
        //         // Err(error.localizedDescription())
        //         Err(NSString::from_str("oops"))
        //     }
        // }
    }
    // [M] addFunctionWithDescriptor:library:error:
    pub fn add_function_with_descriptor(
        &self,
        library: &Library,
        descriptor: &FunctionDescriptor,
    ) -> TryCatch {
        try_catch!(raw_error => msg_send![self, addFunctionWithDescriptor: descriptor, library: library, error: &mut raw_error])
    }
    // [M] serializeToURL:error:
    pub fn serialize_to_url(&self, url: &NSURL) -> TryCatch {
        try_catch!(raw_error => msg_send![self, serializeToURL: url, error: &mut raw_error])
    }
}
