use wgpu::{Device, Queue};

use crate::{definitions::Vertex, user_interface::{elements::Element, UserInterface}};

pub struct Interface {
    elements: Vec<Box<dyn Element>>,
    max_index: u32,
    vertex_buffer: Option<wgpu::Buffer>,
    index_buffer: Option<wgpu::Buffer>,
}

impl Interface {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            max_index: 0,
            vertex_buffer: None,
            index_buffer: None,
        }
    }

    pub fn show<R>(&mut self, elements_builder: impl FnOnce(&mut UserInterface) -> R) -> R {
        let mut user_interface = UserInterface { interface: self };
        elements_builder(&mut user_interface)
    }

    pub fn add_elements(&mut self, mut element: impl Element + 'static) {
        element.identify_as(self.max_index);
        self.elements.push(Box::new(element));
        self.max_index += 1;
    }

    pub fn initialize_interface_buffers(&mut self, device: &Device, queue: &Queue) {
        let mut total_vertices = 0;
        let mut total_indices = 0;
        for element in &self.elements {
            let (vertices, indices) = element.draw();
            total_vertices += vertices.len();
            total_indices += indices.len();
        }
        let vertex_buffer_size = (total_vertices * std::mem::size_of::<Vertex>()) as wgpu::BufferAddress;
        let index_buffer_size = (total_indices * std::mem::size_of::<u16>()) as wgpu::BufferAddress;
        self.vertex_buffer = Some(device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Vertex Buffer"),
            size: vertex_buffer_size,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false
        }));
        self.index_buffer = Some(device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Index Buffer"),
            size: index_buffer_size,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false
        }));

        self.update_vertices(queue);
    }

    pub fn update_vertices(&mut self, queue: &Queue) {
        let mut all_vertices: Vec<Vertex> = Vec::new();
        let mut all_indices: Vec<u16> = Vec::new();

        for element in &self.elements {
            let (vertices, indices) = element.draw();
            let start_index = all_vertices.len() as u32;

            all_vertices.extend_from_slice(&vertices);
            all_indices.extend(indices.iter().map(|i| *i + start_index as u16));
        }

        queue.write_buffer(self.vertex_buffer.as_ref().unwrap(), 0, bytemuck::cast_slice(&all_vertices));
        queue.write_buffer(self.index_buffer.as_ref().unwrap(), 0, bytemuck::cast_slice(&all_indices));
    }

    pub fn render(&self, render_pass: &mut wgpu::RenderPass) {
        let vertex_buffer = match &self.vertex_buffer {
            Some(buffer) => buffer,
            None => {
                eprintln!("Warning: GUI vertex buffer not initialized. Skipping Render...");
                return;
            }
        };
        let index_buffer = match &self.index_buffer {
            Some(buffer) => buffer,
            None => {
                eprintln!("Warning: GUI index buffer not initialized. Skipping Render...");
                return;
            }
        };

        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        let mut index_offset = 0;
        for element in &self.elements {
            let (_vertices, indices) = element.draw();

            render_pass.draw_indexed(index_offset as u32..(index_offset + indices.len()) as u32, 0, 0..1);
            index_offset += indices.len();
        }

    }
}