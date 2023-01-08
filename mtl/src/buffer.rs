use super::*;

declare!(Buffer: Resource);

// impl Label for Buffer {}
// impl Resource for Buffer {}

/// # Instance Properties
impl Buffer {
    // [P] gpuAddress
    pub fn gpu_address(&self) -> u64 {
        unsafe { msg_send![self, gpuAddress] }
    }
}

// /// # Creating a Texture that Shares Buffer Data
// impl Buffer {
//     // [M] newTextureWithDescriptor:offset:bytesPerRow
//     pub fn new_texture_with_descriptor(
//         &self,
//         descriptor: &TextureDescriptor,
//         offset: usize,
//         bytes_per_row: usize,
//     ) -> Id<Texture> {
//         unsafe {
//             msg_send![
//                 self,
//                 newTextureWithDescriptor: descriptor,
//                 offset: offset,
//                 bytesPerRow: bytes_per_row
//             ]
//         }
//     }
// }

impl Buffer {
    // [M] contents
    pub fn contents(&self) -> *mut std::ffi::c_void {
        unsafe { msg_send![self, contents] }
    }
    // [M] didModifyRange:
    pub fn did_modify_range(&self, range: NSRange) {
        unsafe { msg_send![self, didModifyRange: range] }
    }
    // [M] addDebugMarker:range:
    pub fn add_debug_marker(&self, marker: &NSString, range: NSRange) {
        unsafe { msg_send![self, addDebutgMarker: marker, range: range] }
    }
    // [M] removeAllDebugMarkers
    pub fn remove_all_debug_markers(&self) {
        unsafe { msg_send![self, removeAllDebugMarkers] }
    }
    // [P] length
    pub fn length(&self) -> usize {
        unsafe { msg_send![self, length] }
    }
    // // [M] remoteStorageBuffer
    // pub fn remote_storage_buffer(&self) -> Id<Buffer> {
    //     unsafe { msg_send_id![self, remoteStorageBuffer] }
    // }
    // // [M] newRemoteBufferViewForDevice:
    // pub fn new_remote_buffer_view_for_device(&self, device: &Device) {
    //     unsafe { msg_send![self, newRemoteBufferViewForDevice: device] }
    // }
}

// // [C] MTLArgumentDescriptor
// declare!(ArgumentDescriptor);
// impl ArgumentDescriptor {
//     // [M] argumentDescriptor
//     pub fn argument_descriptor() -> Id<ArgumentDescriptor> {
//         unsafe { msg_send_id![class!(MTLArgumentDescriptor), argumentDescriptor] }
//     }
//     // [P] index and setIndex
//     pub fn index(&self) -> usize {
//         unsafe { msg_send![self, index] }
//     }
//     pub fn setIndex(&self, index: usize) {
//         unsafe { msg_send![self, setIndex: index] }
//     }
//     // [P] dataType and setDataType
//     pub fn data_type(&self) -> DataType {
//         unsafe { msg_send![self, dataType] }
//     }
//     pub fn set_data_type(&self, data_type: DataType) {
//         unsafe { msg_send![self, setDataType: data_type] }
//     }
// }
