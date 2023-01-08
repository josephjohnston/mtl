use super::*;

// [C] MTLTextureDescriptor
declare!(TextureDescriptor);

/// # Creating Texture Descriptors
impl TextureDescriptor {
    pub fn new() -> Id<Self> {
        unsafe {
            let raw_desc = msg_send_id![class!(MTLTextureDescriptor), alloc];
            let desc: Id<Self> = msg_send_id![raw_desc, init];
            desc
        }
    }
    // [P] textureType and setTextureType
    // pub fn texture_type(&self) -> TextureType {
    //     unsafe {
    //         msg_send![self, textureType]
    //     }
    // }
    pub fn set_texture_type(&self, texture_type: TextureType) {
        unsafe { msg_send![self, setTextureType: texture_type] }
    }
    // [P] pixelFormat and setPixelFormat
    pub fn set_pixel_format(&self, pixel_format: PixelFormat) {
        unsafe { msg_send![self, setPixelFormat: pixel_format] }
    }
    // [P] usage and setUsage
    pub fn set_usage(&self, usage: TextureUsage) {
        unsafe { msg_send![self, setUsage: usage] }
    }
    // [P] width and setWidth
    pub fn set_width(&self, width: usize) {
        unsafe { msg_send![self, setWidth: width] }
    }
    // [P] height and setHeight
    pub fn set_height(&self, height: usize) {
        unsafe { msg_send![self, setHeight: height] }
    }
    // [P] depth and setDepth
    pub fn set_depth(&self, depth: usize) {
        unsafe { msg_send![self, setDepth: depth] }
    }
    // [P] resourceOptions and setResourceOptions
    pub fn set_resource_options(&self, resource_options: ResourceOptions) {
        unsafe { msg_send![self, setResourceOptions: resource_options] }
    }
    // // [P] mipmapLevelCount and setMipmapLevelCount
    // pub fn set_mipmap_level_count(&self, count: usize) {
    //     unsafe { msg_send![self, setMipmapLevelCount: count] }
    // }
    // // [P] sampleCount and setSampleCount
    // pub fn set_sample_count(&self, count: usize) {
    //     unsafe { msg_send![self, setSampleCount: count] }
    // }
    // [P] arrayLength and setArrayLength
    pub fn set_array_length(&self, length: usize) {
        unsafe { msg_send![self, setArrayLength: length] }
    }
    // [P] compressionType and setCompressiontype
    pub fn set_compression_type(&self, compression_type: CompressionType) {
        unsafe { msg_send![self, setCompressionType: compression_type] }
    }
}

declare!(Texture: Resource);

/// # Copying Data into a Texture Image
impl Texture {
    // [M] replaceRegion:mipmapLevel:slice:withBytes:bytesPerRow:bytesPerImage:
    pub fn replace_region_long(
        &self,
        region: Region,
        slice: usize,
        pixel_bytes: *const c_void,
        bytes_per_row: usize,
        bytes_per_image: usize,
    ) {
        unsafe {
            msg_send![self, replaceRegion: region, mipmapLevel: 0, slice: slice, withBytes: pixel_bytes, bytesPerRow: bytes_per_row, bytesPerImage: bytes_per_image]
        }
    }
    // [M] replaceRegion:mipmapLevel:slice:withBytes:bytesPerRow:
    pub fn replace_region(&self, region: Region, pixel_bytes: *const c_void, bytes_per_row: usize) {
        unsafe {
            msg_send![self, replaceRegion: region, mipmapLevel: 0, withBytes: pixel_bytes, bytesPerRow: bytes_per_row]
        }
    }
}

/// # Copying Data from a Texture Image
impl Texture {
    // [M] getBytes:bytesPerRow:bytesPerImage:fromRegion:mipmapLevel:slice:
    pub fn get_bytes_long(
        &self,
        pixel_bytes: *mut c_void,
        bytes_per_row: usize,
        bytes_per_image: usize,
        from_region: Region,
        slice: usize,
    ) {
        unsafe {
            msg_send![self, getBytes: pixel_bytes, bytesPerRow: bytes_per_row, bytesPerImage: bytes_per_image, fromRegion: from_region, mipmapLevel: 0, slice: slice]
        }
    }
    // [M] getBytes:bytesPerRow:fromRegion:mipmapLevel:
    pub fn get_bytes(
        &self,
        pixel_bytes: *mut c_void,
        bytes_per_row: usize,
        // bytes_per_image: usize,
        from_region: Region,
    ) {
        unsafe {
            msg_send![
                self,
                getBytes: pixel_bytes,
                bytesPerRow: bytes_per_row,
                fromRegion: from_region,
                mipmapLevel: 0
            ]
        }
    }
}

/// # Creating Textures by Reinterpreting Existing Texture Data
impl Texture {
    // [M] newTextureViewWithPixelFormat
    pub fn new_texture_view_with_pixel_format(&self, pixel_format: PixelFormat) -> Id<Texture> {
        unsafe {
            let raw_texture: *mut Texture =
                msg_send![self, newTextureViewWithPixelFormat: pixel_format];
            Id::new(raw_texture).expect(ID_NEW_FAILURE)
        }
    }
    // [M] newTextureViewWithPixelFormat:textureType:levels:slices:
    pub fn new_texture_view_with_pixel_format_texture_type(
        &self,
        pixel_format: PixelFormat,
        texture_type: TextureType,
        levels: NSRange,
        slices: NSRange,
    ) -> Id<Texture> {
        unsafe {
            let raw_texture: *mut Texture = msg_send![
                self,
                newTextureViewWithPixelFormat: pixel_format,
                textureType: texture_type,
                levels: levels,
                slices: slices
            ];
            Id::new(raw_texture).expect(ID_NEW_FAILURE)
        }
    }
}

/// # Querying Texture Attributes
impl Texture {
    // [P] textureType
    pub fn texture_type(&self) -> TextureType {
        unsafe { msg_send![self, textureType] }
    }
}

// [E] MTLTextureUsage
bitflags! {
    pub struct TextureUsage: usize {
        const UNKNOWN = 0x0000;
        const SHADER_READ = 0x0001;
        const SHADER_WRITE = 0x0002;
        const RENDER_TARGET = 0x0004;
        const PIXEL_FORMAT_VIEW = 0x0010;
        // const Unknown = 0x0000;
        // const ShaderRead = 0x0001;
        // const ShaderWrite = 0x0002;
        // const RenderTarget = 0x0004;
        // const PixelFormatView = 0x0010;
    }
}
impl_encode_for_type!(TextureUsage: usize);

// #[repr(usize)]
// #[derive(Debug)]
// pub enum TextureUsage {
//     Unknown = 0x0000,
//     ShaderRead = 0x0001,
//     ShaderWrite = 0x0002,
//     RenderTarget = 0x0004,
//     PixelFormatView = 0x0010,
// }
// impl_encode_for_type!(TextureUsage: usize);

// [E] MTLTextureType
#[repr(usize)]
#[derive(Debug, Clone, Copy)]
pub enum TextureType {
    D1 = 0,
    D1Array = 1,
    D2 = 2,
    D2Array = 3,
    D2Multisample = 4,
    Cube = 5,
    CubeArray = 6,
    D3 = 7,
    D2MultisampleArray = 8,
    TextureBuffer = 9,
}
impl_encode_for_type!(TextureType: usize);

// [E] MTLPixelFormat
#[repr(usize)]
#[derive(Debug, Clone)]
pub enum PixelFormat {
    Invalid = 0,
    // 8 bit
    U8 = 13,
    S8 = 14,
    // 16 bit
    U16 = 23,
    S16 = 24,
    F16 = 25,
    UU8 = 33,
    SS8 = 34,
    // 32 bit
    U32 = 53,
    S32 = 54,
    F32 = 55,
    UU16 = 63,
    SS16 = 64,
    FF16 = 65,
    UUUU8 = 73,
    SSSS8 = 74,
    // 64 bit
    UU32 = 103,
    SS32 = 104,
    FF32 = 105,
    UUUU16 = 113,
    SSSS16 = 114,
    FFFF16 = 115,
    // 128 bit
    UUUU32 = 123,
    SSSS32 = 124,
    FFFF32 = 125,
}
impl_encode_for_type!(PixelFormat: usize);

// [E] MTLCompressionType
#[repr(isize)]
pub enum CompressionType {
    Lossless = 0,
    Lossy = 1,
}
impl_encode_for_type!(CompressionType: isize);

// [S] MTLRegion
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Region {
    origin: Origin,
    size: Size,
}
unsafe impl Encode for Region {
    const ENCODING: Encoding = Encoding::Struct("Region", &[Origin::ENCODING, Size::ENCODING]);
}
impl Region {
    pub fn new(origin: Origin, size: Size) -> Self {
        Region { origin, size }
    }
}

// [S] MTLOrigin
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Origin {
    x: usize,
    y: usize,
    z: usize,
}
unsafe impl Encode for Origin {
    const ENCODING: Encoding = Encoding::Struct(
        "Origin",
        &[usize::ENCODING, usize::ENCODING, usize::ENCODING],
    );
}

impl Origin {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Origin { x, y, z }
    }
}
