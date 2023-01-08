use super::*;

// [E] MTLHeapType
#[repr(usize)]
#[derive(Debug)]
pub enum HeapType {
    Automatic = 0,
    Placement = 1,
    Sparse = 2,
}
unsafe impl Encode for HeapType {
    const ENCODING: Encoding = usize::ENCODING;
}

// [S] MTLSizeAndAlign
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SizeAndAlign {
    size: usize,
    align: usize,
}
unsafe impl Encode for SizeAndAlign {
    const ENCODING: Encoding =
        Encoding::Struct("SizeAndAlign", &[usize::ENCODING, usize::ENCODING]);
}
