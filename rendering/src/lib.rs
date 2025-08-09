use std::{error::Error, sync::{Arc, Mutex}};

use wgpu::util::DeviceExt;
use winit::{dpi::PhysicalSize, window::Window};

use crate::{camera::{Camera2D, Camera2DUniform}, definitions::{InstanceRaw, Vertex}, pipeline::PipeLineBuilder, user_interface::interface::Interface};

mod camera;
mod pipeline;
pub mod definitions;
pub mod user_interface;

pub struct RenderState {
    interface_arc: Arc<Mutex<Interface>>,

    window_size: PhysicalSize<u32>,
    surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,

    camera: Camera2D,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,

    ui_pipeline: wgpu::RenderPipeline,

    surface_configured: bool,
}

impl RenderState {
    pub async fn new(window: Arc<Window>, interface_arc: Arc<Mutex<Interface>>) -> Result<RenderState, Box<dyn Error>> {
        let window_size = window.inner_size();

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let surface = instance.create_surface(window.clone())?;
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptionsBase { 
            power_preference: wgpu::PowerPreference::HighPerformance, 
            force_fallback_adapter: false, 
            compatible_surface: Some(&surface) 
        }).await?;

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::defaults(),
            memory_hints: Default::default(),
            trace: wgpu::Trace::Off,
        }).await?;

        let surface_capabilities = surface.get_capabilities(&adapter);

        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_capabilities.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window_size.width,
            height: window_size.height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            desired_maximum_frame_latency: 2,
            view_formats: vec![],
        };

        let camera = Camera2D::new(window_size.width, window_size.height);
        let camera_uniform = Camera2DUniform {
            view_proj: camera.build_view_projection_matrix().to_cols_array_2d(),
        };
        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Uniform Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
        });
        let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None
                    },
                    count: None
                }
            ],
            label: Some("Camera Bind Group Layout"),
        });
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor { 
            label: Some("Camera 2D Bind Group"), 
            layout: &camera_bind_group_layout, 
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                }
            ] 
        });

        let ui_pipeline = PipeLineBuilder::new(&device)
            .set_pixel_format(wgpu::TextureFormat::Bgra8UnormSrgb)
            .set_shader_module("rendering//shaders/ui_shader.wgsl", "vs_main", "fs_main")
            .add_vertex_buffer_layout(Vertex::description())
            .add_vertex_buffer_layout(InstanceRaw::desc())
            .add_bind_group_layout(&camera_bind_group_layout)
            .build("Render Pipeline");


        Ok(Self {
            interface_arc,

            window_size,
            surface,
            device,
            queue,
            config,

            camera,
            camera_buffer,
            camera_bind_group,

            ui_pipeline,

            surface_configured: false
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.window_size = PhysicalSize::new(width, height);
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            self.surface_configured = true;

            self.camera.update_screen_size(PhysicalSize::new(width, height));
            self.queue.write_buffer(
                &self.camera_buffer, 
                0, 
            bytemuck::cast_slice(&[Camera2DUniform {
                view_proj: self.camera.build_view_projection_matrix().to_cols_array_2d(),
            }]));
        }
    }

    pub fn render(&self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render encoder")
        });

        let interface_guard = self.interface_arc.lock().unwrap();

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor { 
                label: Some("Render pass"), 
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 }),
                        store: wgpu::StoreOp::Store
                    },
                    depth_slice: None
                })], 
                depth_stencil_attachment: None, 
                timestamp_writes: None, 
                occlusion_query_set: None 
            });

            render_pass.set_pipeline(&self.ui_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);

            interface_guard.render(&mut render_pass);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}