use core::option::Option::Some;
use std::collections::HashMap;

use wgpu::{Device, Queue};

use crate::{definitions::{GeometryType, Instance, InstanceRaw, Vertex}, user_interface::{elements::Element, UserInterface}};

pub struct Interface {
    elements: Vec<Box<dyn Element>>,
    instances: HashMap<GeometryType, Vec<InstanceRaw>>,
    max_index: u32,
    vertex_buffers: HashMap<GeometryType, wgpu::Buffer>,
    index_buffers: HashMap<GeometryType, wgpu::Buffer>,
    instance_buffers: HashMap<GeometryType, wgpu::Buffer>,
}

impl Interface {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            instances: HashMap::new(),
            max_index: 0,
            vertex_buffers: HashMap::new(),
            index_buffers: HashMap::new(),
            instance_buffers: HashMap::new(),
        }
    }

    pub fn show<R>(&mut self, elements_builder: impl FnOnce(&mut UserInterface) -> R) -> R {
        let mut user_interface = UserInterface { interface: self };
        elements_builder(&mut user_interface)
    }

    pub fn add_elements(&mut self, element: impl Element + 'static) {
        self.elements.push(Box::new(element));
        self.max_index += 1;
    }

    pub fn geometry_vertices(geometry_type: &GeometryType) -> (Vec<Vertex>, Vec<u16>) {
        match geometry_type {
            GeometryType::Quad => {
            let indices = [0, 1, 2, 2, 3, 0].to_vec();
            let vertices = [
                Vertex {
                    position: [-1.0, -1.0], // Bottom-left corner of a 200x200 area
                    color: [1.0, 0.0, 0.0, 1.0],
                },
                Vertex {
                    position: [1.0, -1.0], // Bottom-right corner
                    color: [1.0, 0.0, 0.0, 1.0],
                },
                Vertex {
                    position: [1.0, 1.0],   // Top-middle corner
                    color: [1.0, 0.0, 0.0, 1.0],
                },
                Vertex {
                    position: [-1.0, 1.0],
                    color: [1.0, 0.0, 0.0, 1.0],
                }
            ].to_vec();
            (vertices, indices)
            },
        }
    }

    pub fn initialize_interface_buffers(&mut self, device: &Device, queue: &Queue) {
        let mut batched_instances: HashMap<GeometryType, Vec<InstanceRaw>> = HashMap::new();

        for element in &self.elements {
            let instance = Instance::new(element.get_geometry_type(), element.get_position(), element.get_color(), element.get_scale());
            let raw_instances = instance.to_raw();
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
        self.update_vertices(queue);
    }

    pub fn update_vertices(&mut self, queue: &Queue) {
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
    }

    pub fn render(&self, render_pass: &mut wgpu::RenderPass) {
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