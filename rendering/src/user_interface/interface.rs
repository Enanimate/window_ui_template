use core::option::Option::Some;
use std::collections::HashMap;

use wgpu::{Device, Queue};
use wgpu_text::{glyph_brush::{ab_glyph::FontRef, Section, Text}, BrushBuilder, TextBrush};

use crate::{definitions::{GeometryType, Instance, InstanceRaw, UiAtlas, Vertex}, user_interface::{elements::Element, UserInterface}};

pub struct Interface {
    pub elements: Vec<Box<dyn Element>>,
    instances: HashMap<GeometryType, Vec<InstanceRaw>>,
    id_iterator: u32,
    illegal_ids: Vec<u32>,
    vertex_buffers: HashMap<GeometryType, wgpu::Buffer>,
    index_buffers: HashMap<GeometryType, wgpu::Buffer>,
    instance_buffers: HashMap<GeometryType, wgpu::Buffer>,
    brush: Option<TextBrush<FontRef<'static>>>,
    atlas: UiAtlas,
}

impl Interface {
    pub fn new(atlas: UiAtlas) -> Self {
        Self {
            elements: Vec::new(),
            instances: HashMap::new(),
            id_iterator: 0,
            illegal_ids: Vec::new(),
            vertex_buffers: HashMap::new(),
            index_buffers: HashMap::new(),
            instance_buffers: HashMap::new(),
            brush: None,
            atlas,
        }
    }

    pub fn show<R>(&mut self, elements_builder: impl FnOnce(&mut UserInterface) -> R) -> R {
        let mut user_interface = UserInterface { interface: self };
        elements_builder(&mut user_interface)
    }

    pub fn add_elements(&mut self, mut element: impl Element + 'static, id: Option<u32>) {
        if let Some(id_number) = id {
            element.set_id(id_number);
            self.illegal_ids.push(id_number);
        } else {
            while self.illegal_ids.iter().any(|&i| i == self.id_iterator) {
                self.id_iterator += 1;
            }

            element.set_id(self.id_iterator);
        }
        self.elements.push(Box::new(element));
        self.id_iterator += 1;
    }

    pub fn geometry_vertices(geometry_type: &GeometryType) -> (Vec<Vertex>, Vec<u16>) {
        match geometry_type {
            GeometryType::Quad => {
                let indices = [0, 1, 2, 2, 3, 0].to_vec();
                let vertices = [
                    Vertex {
                        position: [-0.5, -0.5], // Bottom-left
                        quad_uv: [0.0, 0.0],
                    },
                    Vertex {
                        position: [0.5, -0.5], // Bottom-right
                        quad_uv: [1.0, 0.0],
                    },
                    Vertex {
                        position: [0.5, 0.5],  // Top-right
                        quad_uv: [1.0, 1.0],
                    },
                    Vertex {
                        position: [-0.5, 0.5], // Top-left
                        quad_uv: [0.0, 1.0],
                    }
                ].to_vec();
                (vertices, indices)
            },
            GeometryType::Label => {
                let indices = [0, 1, 2, 2, 3, 0].to_vec();
                let vertices = [
                    Vertex {
                        position: [0.0, 0.0],
                        quad_uv: [0.0, 1.0],
                    },
                    Vertex {
                        position: [0.0, 0.0],
                        quad_uv: [1.0, 1.0],
                    },
                    Vertex {
                        position: [0.0, 0.0],
                        quad_uv: [1.0, 0.0],
                    },
                    Vertex {
                        position: [0.0, 0.0],
                        quad_uv: [0.0, 0.0],
                    }
                ].to_vec();
                (vertices, indices)
            },
        }
    }

    pub fn initialize_interface_buffers(&mut self, device: &Device, queue: &Queue, window_size: [u32; 2], config: &wgpu::SurfaceConfiguration) {
        let mut batched_instances: HashMap<GeometryType, Vec<InstanceRaw>> = HashMap::new();
        let font_bytes = include_bytes!("../../../ComicMono.ttf");
        let atlas = &self.atlas;

        self.brush = Some(BrushBuilder::using_font_bytes(font_bytes)
            .unwrap()
            .build(device, config.width, config.height, config.format));
        
        for element in &self.elements {
            let atlas_entry = atlas.clone().get_entry_by_name(element.get_texture_name().unwrap_or("solid".to_string())).unwrap();
            let tex_coords = [
                atlas_entry.start_coord.unwrap().0,
                atlas_entry.start_coord.unwrap().1,
                atlas_entry.end_coord.unwrap().0,
                atlas_entry.end_coord.unwrap().1,
            ];

            let instance = Instance::new(element.get_id(), element.get_geometry_type(), element.get_position(window_size), element.get_color(), element.get_scale(window_size));
            let mut raw_instances = instance.to_raw();
            raw_instances.tex_coords = tex_coords;
            batched_instances
                .entry(element.get_geometry_type())
                .or_insert(Vec::new())
                .push(raw_instances);
        }

        for (geometry_type, instances) in batched_instances.iter() {
            let (vertices, indices) = Self::geometry_vertices(geometry_type);

            let vertex_buffer_size = (vertices.len() * std::mem::size_of::<Vertex>()) as wgpu::BufferAddress;
            let index_buffer_size = (indices.len() * std::mem::size_of::<u16>()) as wgpu::BufferAddress;
            let instance_buffer_size = (instances.len() * std::mem::size_of::<InstanceRaw>()) as wgpu::BufferAddress;

            self.vertex_buffers.insert(*geometry_type, device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Vertex Buffer"),
                size: vertex_buffer_size,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false
            }));

            self.index_buffers.insert(*geometry_type, device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Index Buffer"),
                size: index_buffer_size,
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false
            }));

            self.instance_buffers.insert(*geometry_type, device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Instance Buffer"),
                size: instance_buffer_size,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false
            }));
        }

        self.instances = batched_instances;
        self.update_vertices(queue, device, window_size);
    }

    pub fn update_vertices(&mut self, queue: &Queue, device: &Device, window_size: [u32; 2]) {
        self.brush.as_ref().unwrap().resize_view(window_size[0] as f32, window_size[1] as f32, queue);

        for (geometry_type, instances) in self.instances.iter() {
            let (vertices, indices) = Self::geometry_vertices(geometry_type);
            if let Some(vertex_buffer) = self.vertex_buffers.get(geometry_type) {
                queue.write_buffer(vertex_buffer, 0, bytemuck::cast_slice(&vertices));
            }
            if let Some(index_buffer) = self.index_buffers.get(geometry_type) {
                queue.write_buffer(index_buffer, 0, bytemuck::cast_slice(&indices));
            }
            if let Some(instance_buffer) = self.instance_buffers.get(geometry_type) {
                queue.write_buffer(instance_buffer, 0, bytemuck::cast_slice(instances));
            }
        }

        let mut label_data: Vec<(String, [f32; 4], Option<[f32; 2]>, [f32; 2])> = Vec::new();
        for element in self.elements.iter_mut() {
            if element.get_geometry_type() == GeometryType::Label {
                let text_ref = element.get_text().expect("Label element contained no text...");
                label_data.push((
                    text_ref.to_string(),
                    element.get_color(),
                    element.get_bounds(),
                    element.get_position(window_size)
                ));
            }
        }

        let mut sections: Vec<Section> = Vec::new();
        for data in &label_data {
            let mut section_builder = Section::builder()
                .with_screen_position(data.3)
                .with_text(vec![
                    Text::new(&data.0) 
                        .with_scale(30.0)
                        .with_color(data.1)
                ]);

            if data.2.is_some() {
                section_builder = section_builder.with_bounds(data.2.unwrap());
            }
            sections.push(section_builder);
        }
        
        if !sections.is_empty() {
            self.brush.as_mut().unwrap().queue(device, queue, sections).unwrap();
        }
    }

    pub(crate)  fn draw_text_brush<'a>( &'a self, renderpass: &mut wgpu::RenderPass<'a>) {
        if let Some(brush) = self.brush.as_ref() {
            brush.draw(renderpass);
        } else {
            eprintln!("Warning: Brush not initialized for drawing.");
        }
    }

    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        for (geometry_type, instances) in self.instances.iter() {
            let vertex_buffer = self.vertex_buffers.get(geometry_type).unwrap();
            let index_buffer = self.index_buffers.get(geometry_type).unwrap();
            let instance_buffer = self.instance_buffers.get(geometry_type).unwrap();
            let (_vertices, indices) = Self::geometry_vertices(geometry_type);

            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, instance_buffer.slice(..));
            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            render_pass.draw_indexed(0..indices.len() as u32, 0, 0..instances.len() as u32);
        }
    }
}