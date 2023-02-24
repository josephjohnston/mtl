use super::*;

// [Pr] MTLCounterSet
declare!(CounterSet);
impl CounterSet {
    // [P] name
    pub fn name(&self) -> Id<NSString> {
        unsafe { msg_send_id![self, name] }
    }
    // [P] counters
    pub fn counters(&self) -> Vec<Id<Counter>> {
        unsafe { NSArray::into_vec(msg_send_id![self, counters]) }
    }
}

// MTLCommonCounterSet
pub const COMMON_COUNTER_SET_TIMESTAMP: &str = "timestamp";
pub const COMMON_COUNTER_SET_STATISTIC: &str = "statistic";

// [Pr] MTLCounter
declare!(Counter);
impl Counter {
    // [P] name
    pub fn name(&self) -> Id<NSString> {
        unsafe { msg_send_id![self, name] }
    }
}

// MTLCommonCounter
pub const COMMON_COUNTER_TIMESTEP: &str = "GPUTimestamp";
pub const COMMON_COUNTER_COMPUTE_KERNEL_INVOCATIONS: &str = "KernelInfocations";

// [C] MTLCounterSampleBufferDescriptor
declare!(CounterSampleBufferDescriptor);
impl Label for CounterSampleBufferDescriptor {}
impl CounterSampleBufferDescriptor {
    pub fn new() -> Id<CounterSampleBufferDescriptor> {
        unsafe { msg_send_id![class!(MTLCounterSampleBufferDescriptor), new] }
    }
    // [P] counterSet and setCounterSet
    pub fn counter_set(&self) -> Option<Id<CounterSet>> {
        unsafe { msg_send_id![self, counterSet] }
    }
    pub fn set_counter_set(&self, counter_set: &CounterSet) {
        unsafe { msg_send![self, setCounterSet: counter_set] }
    }
    // [P] sampleCount and setSampleCount
    pub fn sample_count(&self) -> usize {
        unsafe { msg_send![self, sampleCount] }
    }
    pub fn set_sample_count(&self, sample_count: usize) {
        unsafe { msg_send![self, setSampleCount: sample_count] }
    }
    // [P] storageMode and setStorageMode
    pub fn storage_mode(&self) -> StorageMode {
        unsafe { msg_send![self, storageMode] }
    }
    pub fn set_storage_mode(&self, storage_mode: StorageMode) {
        unsafe { msg_send![self, setStorageMode: storage_mode] }
    }
}

// [Pr] MTLCounterSampleBuffer
declare!(CounterSampleBuffer);
impl CounterSampleBuffer {
    // [P] device
    pub fn device(&self) -> Id<Device> {
        unsafe { msg_send_id![self, device] }
    }
    // [P] sampleCount
    pub fn sample_count(&self) -> usize {
        unsafe { msg_send![self, sampleCounte] }
    }
    // [M] resolveCounterRange:
    pub fn resolve_counter_range(&self, range: std::ops::Range<usize>) -> Id<NSData> {
        unsafe { msg_send_id![self, resolveCounterRange: NSRange::from(range)] }
    }
}

// [E] MTLCounterSamplingPoint
#[derive(Debug, Clone, Copy)]
#[repr(usize)]
pub enum CounterSamplingPoint {
    StageBoundary = 0,
    DrawBoundary = 1,
    DispatchBoundary = 2,
    TileDispatchBoundary = 3,
    BlitBoundary = 4,
}
impl_encode_for_type!(CounterSamplingPoint: usize);

// [C] MTLBlitPassSampleBufferAttachmentDescriptor
declare!(BlitPassSampleBufferAttachmentDescriptor);
impl BlitPassSampleBufferAttachmentDescriptor {
    // [P] sampleBuffer and setSampleBuffer
    pub fn sample_buffer(&self) -> Option<Id<CounterSampleBuffer>> {
        unsafe { msg_send_id![self, sampleBuffer] }
    }
    pub fn set_sample_buffer(&self, sample_buffer: &CounterSampleBuffer) {
        unsafe { msg_send![self, setSampleBuffer: sample_buffer] }
    }
    // [P] startOfEncoderSampleIndex and setStartOfEncoderSampleIndex
    pub fn start_of_encoder_sample_index(&self) -> usize {
        unsafe { msg_send![self, startOfEncoderSampleIndex] }
    }
    pub fn set_start_of_encoder_sample_index(&self, index: usize) {
        unsafe { msg_send![self, setStartOfEncoderSampleIndex: index] }
    }
    // [P] endOfEncoderSampleIndex and setEndOfEncoderSampleIndex
    pub fn end_of_encoder_sample_index(&self) -> usize {
        unsafe { msg_send![self, endOfEncoderSampleIndex] }
    }
    pub fn set_end_of_encoder_sample_index(&self, index: usize) {
        unsafe { msg_send![self, setEndOfEncoderSampleIndex: index] }
    }
}

// [C] MTLBlitPassSampleBufferAttachmentDescriptorArray
declare!(BlitPassSampleBufferAttachmentDescriptorArray);
impl BlitPassSampleBufferAttachmentDescriptorArray {
    // [M] objectAtIndexedSubscript:
    pub fn object_at_indexed_subscript(
        &self,
        index: usize,
    ) -> Id<BlitPassSampleBufferAttachmentDescriptor> {
        unsafe { msg_send_id![self, objectAtIndexedSubscript: index] }
    }
    // [M] setObject:atIndexedSubscript:
    pub fn set_object_at_indexed_subscript(
        &self,
        attachment: &BlitPassSampleBufferAttachmentDescriptor,
        index: usize,
    ) {
        unsafe { msg_send![self, setObject: attachment, atIndexedSubscript: index] }
    }
}

// [C] MTLComputePassSampleBufferAttachmentDescriptor
declare!(ComputePassSampleBufferAttachmentDescriptor);
impl ComputePassSampleBufferAttachmentDescriptor {
    // [P] sampleBuffer and setSampleBuffer
    pub fn sample_buffer(&self) -> Id<CounterSampleBuffer> {
        unsafe { Id::retain(msg_send![self, sampleBuffer]).expect(ID_RETAIN_FAILURE) }
    }
    pub fn set_sample_buffer(&self, sample_buffer: &CounterSampleBuffer) {
        unsafe { msg_send![self, setSampleBuffer: sample_buffer] }
    }
    // [P] startOfEncoderSampleIndex and setStartOfEncoderSampleIndex
    pub fn start_of_encoder_sample_index(&self) -> usize {
        unsafe { msg_send![self, startOfEncoderSampleIndex] }
    }
    pub fn set_start_of_encoder_sample_index(&self, index: usize) {
        unsafe { msg_send![self, setStartOfEncoderSampleIndex: index] }
    }
    // [P] endOfEncoderSampleIndex and setEndOfEncoderSampleIndex
    pub fn end_of_encoder_sample_index(&self) -> usize {
        unsafe { msg_send![self, endOfEncoderSampleIndex] }
    }
    pub fn set_end_of_encoder_sample_index(&self, index: usize) {
        unsafe { msg_send![self, setEndOfEncoderSampleIndex: index] }
    }
}

// [C] MTLComputePassSampleBufferAttachmentDescriptorArray
declare!(ComputePassSampleBufferAttachmentDescriptorArray);
impl ComputePassSampleBufferAttachmentDescriptorArray {
    // [M] objectAtIndexedSubscript
    pub fn object_at_indexed_subscript(
        &self,
        attachment_index: usize,
    ) -> Id<ComputePassSampleBufferAttachmentDescriptor> {
        unsafe {
            Id::retain(msg_send![self, objectAtIndexedSubscript: attachment_index])
                .expect(ID_RETAIN_FAILURE)
        }
    }
    // [M] setObject:atIndexedSubscript:
    pub fn set_object_at_indexed_subscript(
        &self,
        attachment: &ComputePassSampleBufferAttachmentDescriptor,
        index: usize,
    ) {
        unsafe { msg_send![self, setObject: attachment, atIndexedSubscript: index] }
    }
}

// [T] MTLTimestamp
// pub type Timestamp = usize;
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Timestamp {
    pub value: usize,
}
unsafe impl Encode for Timestamp {
    const ENCODING: Encoding = Encoding::Struct("Timestamp", &[usize::ENCODING]);
}
unsafe impl RefEncode for Timestamp {
    // const ENCODING_REF: Encoding = Object::ENCODING_REF;
    const ENCODING_REF: Encoding = Encoding::Pointer(&usize::ENCODING);
}
impl Timestamp {
    pub fn new(value: usize) -> Self {
        Self { value }
    }
}

// create a BlitPassSampleBufferAttachmentDescriptor
// by accessing an element in the sampleBufferAttachments property of the blitPassDescriptor
// set the sample buffer on this descriptor as well as the indices to write to in this buffer at the start and end. can also do counter-don't-sample.

// so we have a pass descriptor, it has a property sampleBufferAttachments, implying multiple sample buffers may be attached. this property yields
// XPassSampleBufferAttachmentDescriptorArray, and indeed it is an array of
// XPassSampleBufferAttachmentDescriptor's
// each such descriptor specifies a sampleBuffer and indices for start and stop
// we separately create sample buffers and we can assign them here. we create these CounterSampleBuffer's by passing the device a CounterSampleBufferDescriptor. Such a descriptor has a label, a counter set, a sample count, and a storage mode. the actual buffer has a label, a device, and a sample count, all readonly. resolveCounterRange: also.
// one point of confusion, is how the sampleBufferAttachments is readonly, yet we need to modify the entries, but how many entries are there. it looks like it may be prefilled and we just access and modify, so we don't create the attachment descriptors. so it looks like it has as many attachments as there are counter sets. this means one buffer for each counter set. does this make sense? should all data from a counter set go in the same buffer? actually that's not true, there's 4 buffers but only 3 counter sets. maybe its constant.
