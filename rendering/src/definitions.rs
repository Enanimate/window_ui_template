#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
}

impl Vertex {
    pub(crate) fn description() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4,
                }
            ]
        }
    }
}

#[derive(Debug, Clone)]
pub struct UiAtlas {
    pub entries: Vec<UiAtlasTexture>,
    width: u32,
    height: u32,
}

impl UiAtlas {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            entries: Vec::new(),
            width,
            height
        }
    }

    pub fn add_entry(&mut self, entry: UiAtlasTexture) {
        self.entries.push(entry.generate_tex_coords(self.width, self.height));
    }
}

#[derive(Debug, Clone)]
pub struct UiAtlasTexture {
    pub name: String,
    x_start: u32,
    y_start: u32,
    image_width: u32,
    image_height: u32,
    pub start_coord: Option<(f32, f32)>,
    pub end_coord: Option<(f32, f32)>
}

impl UiAtlasTexture {
    pub fn new(name: String, x_0: u32, y_0: u32, image_width: u32, image_height: u32) -> Self {
        Self {
            name,
            x_start: x_0,
            y_start: y_0,
            image_width,
            image_height,
            start_coord: None,
            end_coord: None,
        }
    }

    fn generate_tex_coords(mut self, width: u32, height: u32) -> Self {
        let x0 = self.x_start as f32 / width as f32;
        let y0 = self.y_start as f32 / height as f32;
        let x1 = (self.x_start + self.image_width) as f32 / width as f32;
        let y1 = (self.y_start + self.image_height) as f32 / height as f32;

        self.start_coord = Some((x0, y0));
        self.end_coord = Some((x1, y1));
        self
    }
}