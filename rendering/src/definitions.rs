#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 2],
    pub quad_uv: [f32; 2],
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
                    format: wgpu::VertexFormat::Float32x2,
                },
            ]
        }
    }
}

pub struct Instance {
    id: u32,
    pub geometry_type: GeometryType,
    position: [f32; 2],
    color: [f32; 4],
    scale: [f32; 2],
}

impl Instance {
    pub fn new(id: u32, geometry_type: GeometryType, position: [f32; 2], color: [f32; 4], scale: [f32; 2]) -> Self {
        Self { 
            id,
            geometry_type, 
            position,
            color,
            scale,
        }
    }

    pub fn to_raw(&self) -> InstanceRaw {
        InstanceRaw {
            id: self.id,
            position: self.position,
            color: self.color,
            scale: self.scale,
            tex_coords: [0.0, 0.0, 0.0, 0.0]
        }
    }
}

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy, Debug)]
pub struct InstanceRaw {
    pub id: u32,
    position: [f32; 2],
    color: [f32; 4],
    scale: [f32; 2],
    pub tex_coords: [f32; 4],
}

impl InstanceRaw {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    shader_location: 2,
                    offset: 0,
                    format: wgpu::VertexFormat::Uint32,
                },
                wgpu::VertexAttribute {
                    shader_location: 3,
                    offset: std::mem::size_of::<u32>() as wgpu::BufferAddress,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    shader_location: 4,
                    offset: std::mem::size_of::<u32>() as wgpu::BufferAddress + std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<u32>() as wgpu::BufferAddress + std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress + std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<u32>() as wgpu::BufferAddress + std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress + std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress + std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub enum GeometryType {
    Quad,
    Label
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

    pub fn get_entry_by_name(&self, name: String) -> Option<UiAtlasTexture> {
        self.entries.iter().find(|entry| entry.name == name).cloned()
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
        // Calculate a half-pixel offset based on the atlas dimensions
        let half_pixel_x = 0.5 / width as f32;
        let half_pixel_y = 0.5 / height as f32;

        let x0 = self.x_start as f32 / width as f32;
        let y0 = self.y_start as f32 / height as f32;
        let x1 = (self.x_start + self.image_width) as f32 / width as f32;
        let y1 = (self.y_start + self.image_height) as f32 / height as f32;

        // Inset the coordinates by a half-pixel to avoid the edges
        self.start_coord = Some((x0 + half_pixel_x, y0 + half_pixel_y));
        self.end_coord = Some((x1 - half_pixel_x, y1 - half_pixel_y));
        
        self
    }
}

#[derive(Clone)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
    a: f64
}

impl Color {
    pub fn into_vec4(&self) -> [f32; 4] {
        [self.r as f32, self.g as f32, self.b as f32, self.a as f32]
    }
}

impl ColorExt for Color {
    fn from_hex(hex_color: &str) -> Self {
        if let Some(hex) = hex_color.strip_prefix("#") {
            let mut chars = hex.chars();
            let red: String = [chars.next().unwrap(), chars.next().unwrap()].iter().collect() ;
            let green: String = [chars.next().unwrap(), chars.next().unwrap()].iter().collect();
            let blue: String = [chars.next().unwrap(), chars.next().unwrap()].iter().collect();
            let alpha: String = [chars.next().unwrap(), chars.next().unwrap()].iter().collect();

            let red_value = u32::from_str_radix(&red, 16).unwrap() as f64 / 255.0;
            let green_value = u32::from_str_radix(&green, 16).unwrap() as f64 / 255.0;
            let blue_value = u32::from_str_radix(&blue, 16).unwrap() as f64 / 255.0;
            let alpha_value = u32::from_str_radix(&alpha, 16).unwrap() as f64 / 255.0;

            let (corrected_r, corrected_g, corrected_b) = Self::srgb_correction(red_value, green_value, blue_value);
            
            Self {
                r: corrected_r,
                g: corrected_g,
                b: corrected_b,
                a: alpha_value,
            }
        } else {
            panic!("Color was not hex")
        }
    }

        fn srgb_correction(x: f64, y: f64, z: f64) -> (f64, f64, f64) {
        let mut linear_color = (0.0, 0.0, 0.0);

        if x <= 0.04045 {
            linear_color.0 = x / 12.92;
        } else {
            linear_color.0 = ((x + 0.055) / 1.055).powf(2.4);
        }

        if y <= 0.04045 {
            linear_color.1 = y / 12.92;
        } else {
            linear_color.1 = ((y + 0.055) / 1.055).powf(2.4);
        }

        if z <= 0.04045 {
            linear_color.2 = z / 12.92;
        } else {
            linear_color.2 = ((z + 0.055) / 1.055).powf(2.4);
        }

        linear_color
    }
}

pub trait ColorExt {
    fn from_hex(hex: &str) -> Self;
    fn srgb_correction(x: f64, y: f64, z: f64) -> (f64, f64, f64);
}

impl ColorExt for wgpu::Color {
    fn from_hex(hex_color: &str) -> Self {
        if let Some(hex) = hex_color.strip_prefix("#") {
            let mut chars = hex.chars();
            let red: String = [chars.next().unwrap(), chars.next().unwrap()].iter().collect() ;
            let green: String = [chars.next().unwrap(), chars.next().unwrap()].iter().collect();
            let blue: String = [chars.next().unwrap(), chars.next().unwrap()].iter().collect();

            let red_value = u32::from_str_radix(&red, 16).unwrap() as f64 / 255.0;
            let green_value = u32::from_str_radix(&green, 16).unwrap() as f64 / 255.0;
            let blue_value = u32::from_str_radix(&blue, 16).unwrap() as f64 / 255.0;

            let (corrected_r, corrected_g, corrected_b) = Self::srgb_correction(red_value, green_value, blue_value);
            
            Self {
                r: corrected_r,
                g: corrected_g,
                b: corrected_b,
                a: 1.0,
            }
        } else {
            panic!("Color was not hex")
        }
    }

        fn srgb_correction(x: f64, y: f64, z: f64) -> (f64, f64, f64) {
        let mut linear_color = (0.0, 0.0, 0.0);

        if x <= 0.04045 {
            linear_color.0 = x / 12.92;
        } else {
            linear_color.0 = ((x + 0.055) / 1.055).powf(2.4);
        }

        if y <= 0.04045 {
            linear_color.1 = y / 12.92;
        } else {
            linear_color.1 = ((y + 0.055) / 1.055).powf(2.4);
        }

        if z <= 0.04045 {
            linear_color.2 = z / 12.92;
        } else {
            linear_color.2 = ((z + 0.055) / 1.055).powf(2.4);
        }

        linear_color
    }
}