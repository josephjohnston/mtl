use super::*;

pub struct Texture {
    id: Id<mtl::Texture>,
    pub name: String,
    // heap: &'a mtl::Heap,
    usage: TextureUsage,
    format: PixelFormat,
    size: Size,
    array: bool,
}

impl Texture {
    pub(crate) fn get_mtl(&self) -> &mtl::Texture {
        &*self.id
    }
    pub(crate) fn configure_descriptor(
        desc: &mtl::TextureDescriptor,
        usage: TextureUsage,
        format: PixelFormat,
        size: Size,
        array: bool,
    ) {
        desc.set_usage(get_mtl_usage(usage));
        desc.set_pixel_format(get_mtl_pixel_format(format));
        desc.set_texture_type(get_mtl_texture_type(&size, array));
        desc.set_width(size.width);
        if size.height > 1 {
            desc.set_height(size.height);
        }
        if array {
            desc.set_array_length(get_array_length(&size, get_mtl_texture_type(&size, array)));
        } else if size.depth > 1 {
            desc.set_depth(size.depth);
        }
    }
    pub(crate) fn new(
        // device: &'a Device,
        // heap: &'a mtl::Heap,
        id: Id<mtl::Texture>,
        name: String,
        usage: TextureUsage,
        format: PixelFormat,
        size: Size,
        array: bool,
    ) -> Self {
        id.set_label(&NSString::from_str(name.as_str()));

        Self {
            id,
            // device,
            name,
            usage,
            format,
            size,
            array,
        }
    }
    pub fn discard(&self) {
        // or using drop?
        self.id.make_aliasable();
    }
    pub fn read(&self) {}
    // pub fn derive_new_texture(&self, label: &'a str, new_format: PixelFormat) -> Self {
    //     let new_texture_inner = self
    //         .inner
    //         .new_texture_view_with_pixel_format(get_lib_pixel_format(new_format));
    //     Self::new(
    //         new_texture_inner,
    //         label,
    //         self.device,
    //         self.private,
    //         self.usage,
    //         new_format,
    //         self.size,
    //         self.array,
    //         self.heap,
    //     )
    // }
    pub fn bytes_per_pixel(&self) -> usize {
        match self.format {
            PixelFormat::Bits8(_) => 1,
            PixelFormat::Bits16(_) => 2,
            PixelFormat::Bits32(_) => 4,
            PixelFormat::Bits64(_) => 8,
            PixelFormat::Bits128(_) => 16,
        }
    }
    // pub fn copy_to_region(
    //     &self,
    //     source: *const c_void,
    //     dest_region: Region,
    //     pixels_per_row: usize,
    //     array_index: usize,
    // ) {
    //     let bytes_per_row = self.bytes_per_pixel() * pixels_per_row;
    //     let bytes_per_image = 0;
    //     println!("{}", bytes_per_row);
    //     self.inner.replace_region_long(
    //         dest_region,
    //         array_index,
    //         source,
    //         bytes_per_row,
    //         bytes_per_image,
    //     );
    //     // self.inner.replace_region(dest_region, source, 0);
    // }
    // pub fn copy_from_region(
    //     &self,
    //     dest: *mut c_void,
    //     source_region: Region,
    //     pixels_per_row: usize,
    //     array_index: usize,
    // ) {
    //     let bytes_per_row = self.bytes_per_pixel() * pixels_per_row;
    //     let bytes_per_image = 0;
    //     self.inner.get_bytes_long(
    //         dest,
    //         bytes_per_row,
    //         bytes_per_image,
    //         source_region,
    //         array_index,
    //     )
    // }
}

// impl<'a> Resource for Texture<'a> {
//     fn allocated_size(&self) -> usize {
//         self.inner.allocated_size()
//     }
//     fn heap_offset(&self) -> usize {
//         self.inner.heap_offset()
//     }
//     fn is_aliasable(&self) -> bool {
//         self.inner.is_aliasable()
//     }
//     fn make_aliasable(&self) {
//         self.inner.make_aliasable();
//     }
// }

// Usage
#[repr(usize)]
#[derive(Debug, Clone, Copy)]
pub enum TextureUsage {
    Read = 0x0001,
    Write = 0x0002,
    ReadWrite = 0x0003,
    ReadBase = 0x0011,
    WriteBase = 0x0012,
    ReadWriteBase = 0x0013,
}

// Format
#[derive(Debug, Clone, Copy)]
pub enum Bits8Format {
    U = 13,
    S = 14,
}
#[derive(Debug, Clone, Copy)]
pub enum Bits16Format {
    U = 23,
    S = 24,
    F = 25,
    UU = 33,
    SS = 34,
}
#[derive(Debug, Clone, Copy)]
pub enum Bits32Format {
    U = 53,
    S = 54,
    F = 55,
    UU = 63,
    SS = 64,
    FF = 65,
    UUUU = 73,
    SSSS = 74,
}
#[derive(Debug, Clone, Copy)]
pub enum Bits64Format {
    UU = 103,
    SS = 104,
    FF = 105,
    UUUU = 113,
    SSSS = 114,
    FFFF = 115,
}
#[derive(Debug, Clone, Copy)]
pub enum Bits128Format {
    UUUU = 123,
    SSSS = 124,
    FFFF = 125,
}
#[derive(Debug, Clone, Copy)]
pub enum PixelFormat {
    // Invalid,
    Bits8(Bits8Format),
    Bits16(Bits16Format),
    Bits32(Bits32Format),
    Bits64(Bits64Format),
    Bits128(Bits128Format),
}
pub(super) fn get_mtl_pixel_format(texture_format: PixelFormat) -> mtl::PixelFormat {
    match texture_format {
        // PixelFormat::Invalid => mtl::PixelFormat::Invalid,
        PixelFormat::Bits8(bits_8_format) => match bits_8_format {
            Bits8Format::U => mtl::PixelFormat::U8,
            Bits8Format::S => mtl::PixelFormat::S8,
        },
        PixelFormat::Bits16(bits_16_format) => match bits_16_format {
            Bits16Format::U => mtl::PixelFormat::U16,
            Bits16Format::S => mtl::PixelFormat::S16,
            Bits16Format::F => mtl::PixelFormat::F16,
            Bits16Format::SS => mtl::PixelFormat::SS8,
            Bits16Format::UU => mtl::PixelFormat::UU8,
        },
        PixelFormat::Bits32(bits_32_format) => match bits_32_format {
            Bits32Format::U => mtl::PixelFormat::U32,
            Bits32Format::S => mtl::PixelFormat::S32,
            Bits32Format::F => mtl::PixelFormat::F32,
            Bits32Format::SS => mtl::PixelFormat::SS16,
            Bits32Format::UU => mtl::PixelFormat::UU16,
            Bits32Format::FF => mtl::PixelFormat::FF16,
            Bits32Format::SSSS => mtl::PixelFormat::SSSS8,
            Bits32Format::UUUU => mtl::PixelFormat::UUUU8,
        },
        PixelFormat::Bits64(bits_64_format) => match bits_64_format {
            Bits64Format::UU => mtl::PixelFormat::UU32,
            Bits64Format::SS => mtl::PixelFormat::SS32,
            Bits64Format::FF => mtl::PixelFormat::FF32,
            Bits64Format::UUUU => mtl::PixelFormat::UUUU16,
            Bits64Format::SSSS => mtl::PixelFormat::SSSS16,
            Bits64Format::FFFF => mtl::PixelFormat::FFFF16,
        },
        PixelFormat::Bits128(bits_128_format) => match bits_128_format {
            Bits128Format::UUUU => mtl::PixelFormat::UUUU32,
            Bits128Format::SSSS => mtl::PixelFormat::SSSS32,
            Bits128Format::FFFF => mtl::PixelFormat::FFFF32,
        },
    }
}

// enum TextureType {
//     Buffer,
//     D1,
//     D1Array,
//     D2,
//     D2Array,
//     D3
// }

pub(super) fn get_mtl_usage(usage: TextureUsage) -> mtl::TextureUsage {
    match usage {
        TextureUsage::Read => mtl::TextureUsage::SHADER_READ,
        TextureUsage::ReadBase => {
            mtl::TextureUsage::SHADER_READ | mtl::TextureUsage::PIXEL_FORMAT_VIEW
        }
        TextureUsage::Write => mtl::TextureUsage::SHADER_READ,
        TextureUsage::WriteBase => {
            mtl::TextureUsage::SHADER_WRITE | mtl::TextureUsage::PIXEL_FORMAT_VIEW
        }
        TextureUsage::ReadWrite => mtl::TextureUsage::SHADER_READ | mtl::TextureUsage::SHADER_WRITE,
        TextureUsage::ReadWriteBase => {
            mtl::TextureUsage::SHADER_READ
                | mtl::TextureUsage::SHADER_WRITE
                | mtl::TextureUsage::PIXEL_FORMAT_VIEW
        }
    }
}
// fn get_lib_usage(usage: TextureUsage) -> mtl::TextureUsage {
//     match usage {
//         TextureUsage::Read(false) => mtl::TextureUsage::ShaderRead,
//         TextureUsage::Read(true) => {
//             mtl::TextureUsage::ShaderRead | mtl::TextureUsage::PixelFormatView
//         }
//         TextureUsage::Write(false) => mtl::TextureUsage::ShaderRead,
//         TextureUsage::Write(true) => {
//             mtl::TextureUsage::ShaderWrite | mtl::TextureUsage::PixelFormatView
//         }
//         TextureUsage::ReadWrite(false) => {
//             mtl::TextureUsage::ShaderRead | mtl::TextureUsage::ShaderWrite
//         }
//         TextureUsage::ReadWrite(true) => {
//             mtl::TextureUsage::ShaderRead
//                 | mtl::TextureUsage::ShaderWrite
//                 | mtl::TextureUsage::PixelFormatView
//         }
//     }
// }

pub(super) fn get_mtl_texture_type(size: &Size, array: bool) -> mtl::TextureType {
    assert_ne!(size.width, 0, "texture width at least 1");
    assert_ne!(size.height, 0, "texture width at least 1");
    assert_ne!(size.depth, 0, "texture width at least 1");
    // texture buffer?
    match size.depth {
        1 => match size.height {
            1 => match array {
                false => mtl::TextureType::D1,
                true => mtl::TextureType::D1Array,
            },
            _ => match array {
                false => mtl::TextureType::D2,
                true => mtl::TextureType::D2Array,
            },
        },
        _ => match array {
            false => mtl::TextureType::D3,
            true => mtl::TextureType::D2Array,
        },
    }
}
pub(super) fn get_array_length(size: &Size, texture_type: mtl::TextureType) -> usize {
    match texture_type {
        mtl::TextureType::D2Array => size.depth,
        mtl::TextureType::D1Array => size.height,
        _ => 1,
    }
}
