use super::*;

pub struct BlitPass {
    id: Id<mtl::BlitCommandEncoder>,
}

impl BlitPass {
    pub fn new(cmd_buffer: &mtl::CommandBuffer, desc: &mtl::BlitPassDescriptor) -> Self {
        // let attachments = blit_pass_desc.sample_buffer_attachments();
        let id = cmd_buffer.blit_command_encoder_with_descriptor(&desc);
        Self { id }
    }
    pub fn fill_buffer<T>(&self, buffer: &Buffer<T, Private>, range: Range<usize>, value: u8) {
        self.id
            .fill_buffer(buffer.get_mtl(), NSRange::from(range), value);
    }
    pub fn copy_between_buffers<T, E1: Env, E2: Env>(
        &self,
        source: &Buffer<T, E1>,
        source_offset: usize,
        dest: &Buffer<T, E2>,
        dest_offset: usize,
        len: usize,
    ) {
        let element_size = std::mem::size_of::<T>();
        let source_offset_in_bytes = source_offset * element_size;
        let dest_offset_in_bytes = dest_offset * element_size;
        let length_in_bytes = len * element_size;
        self.id.copy_between_buffers(
            source.get_mtl(),
            source_offset_in_bytes,
            dest.get_mtl(),
            dest_offset_in_bytes,
            length_in_bytes,
        );
    }
    pub fn copy_buffer_to_texture<T, E: Env>(
        &self,
        buffer: &Buffer<T, E>,
        offset: usize,
        elements_per_row: usize,
        elements_per_image: usize,
        size: Size,
        texture: &Texture,
        slice: usize,
        origin: Origin,
    ) {
        let bytes_per_element = std::mem::size_of::<T>();
        self.id.copy_from_buffer_to_texture(
            buffer.get_mtl(),
            offset * bytes_per_element,
            elements_per_row * bytes_per_element,
            elements_per_image * bytes_per_element,
            size,
            texture.get_mtl(),
            slice,
            0,
            origin,
        );
    }
    // pub fn copy_between_textures() {}
    pub fn end_encoding(&self) {
        self.id.end_encoding();
    }
    // fence
    // optimize
    // counters
}
