use super::*;

impl Device {
    // [M] newDefaultLibrary
    pub fn new_default_library(&self) -> Retained<Library> {
        unsafe { Retained::from_raw(msg_send![self, newDefaultLibrary]).expect(ID_NEW_FAILURE) }
    }
    // [M] newDefaultLibraryWithBundle:error:
    // [M] newLibraryWithURL:error:
    pub fn new_library_with_url(&self, url: &NSURL) -> TryNewCatch<Library> {
        // try_new_catch!(raw_error<Library> => msg_send![self, newLibraryWithURL: url, error: &mut raw_error])
        unsafe {
            let mut raw_error: *mut NSError = std::ptr::null_mut();
            let raw_t: *mut Library =
                msg_send![self, newLibraryWithURL: url, error: &mut raw_error];
            if raw_error.is_null() {
                Ok(Retained::from_raw(raw_t).expect(ID_NEW_FAILURE))
            } else {
                let error: Retained<NSError> =
                    Retained::retain_autoreleased(raw_error).expect(ID_RETAIN_AUTO_FAILURE);
                Err(error.localizedDescription())
            }
        }
    }
    // [M] newLibraryWithData:error:
    // [M] newLibraryWithSource:options:error:
    // [M] newLibraryWithSource:options:completionHandler:
    pub fn new_library_with_source(
        &self,
        source: &NSString,
        options: &CompileOptions,
    ) -> TryNewCatch<Library> {
        try_new_catch!(raw_error<Library> => msg_send![
            self,
            newLibraryWithSource: source,
            options: options,
            error: &mut raw_error
        ])
    }
    // [M] newDynamicLibrary:error:
    pub fn new_dynamic_library(&self, library: &Library) -> TryNewCatch<DynamicLibrary> {
        try_new_catch!(raw_error<DynamicLibrary> => {
            msg_send![self, newDynamicLibrary: library, error: &mut raw_error]
        })
    }
    // [M] newDynamicLibraryWithURL:error:
    pub fn new_dynamic_library_with_url(&self, url: &NSURL) -> TryNewCatch<DynamicLibrary> {
        try_new_catch!(raw_error<DynamicLibrary> => {
            msg_send![self, newDynamicLibraryWithURL: url, error: &mut raw_error]
        })
    }
    // [M] newBinaryArchiveWithDescriptor:error:
    pub fn new_binary_archive_with_descriptor(
        &self,
        descriptor: &BinaryArchiveDescriptor,
    ) -> TryNewCatch<BinaryArchive> {
        try_new_catch!(raw_error<BinaryArchive> => msg_send![
            self,
            newBinaryArchiveWithDescriptor: descriptor,
            error: &mut raw_error
        ])
    }
}
