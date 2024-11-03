use wgpu::Device;

pub trait Object {
    fn Up(&mut self);
    fn Down(&mut self);
    fn Left(&mut self);
    fn Right(&mut self);
    fn placement(&self) -> &Placement;
    fn vertex_buffer(&self) -> Option<&wgpu::Buffer>;
    fn index_buffer(&self) -> Option<&wgpu::Buffer>;
    // fn vertex_buffer(&self) -> &BufferSlice<'_>;
    // fn index_buffer(&self) -> &BufferSlice<'_>;
    fn num_indices(&self) -> u32;
    fn init(&mut self, device: &wgpu::Device);
}

pub struct Placement {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    // eventually rotation things
}

impl Placement {
    pub fn placement_vector(&self) -> [f32; 4]{
        [self.x, self.y, self.z, 0.]
    }
}