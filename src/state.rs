use wgpu::util::DeviceExt;
use winit::window::Window;
use cgmath::{Vector2, Matrix4, SquareMatrix};
use winit::event::*;

use crate::camera::{Camera, CameraUniform};
use crate::renderer::create_render_pipeline;

pub struct State {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub render_pipeline: wgpu::RenderPipeline,
    pub camera: Camera,
    pub camera_uniform: CameraUniform,
    pub camera_buffer: wgpu::Buffer,
    pub camera_bind_group: wgpu::BindGroup,
    pub dot_buffer: wgpu::Buffer,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 2],
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

impl State {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        let camera = Camera::new();
        let mut camera_uniform = CameraUniform::new();
        camera.update_uniform(&mut camera_uniform);

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("camera_bind_group_layout"),
        });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let render_pipeline = create_render_pipeline(&device, &config, &camera_bind_group_layout);

        // Create an initial empty buffer for dots
        let dot_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Dot Vertex Buffer"),
            size: (std::mem::size_of::<Vertex>() * 1000000) as u64, // Allocate space for up to 1 million dots
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            camera,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            dot_buffer,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::MouseWheel { delta, .. } => {
                match delta {
                    MouseScrollDelta::LineDelta(_, y) => {
                        let zoom_factor = 1.1;
                        if *y > 0.0 {
                            self.camera.zoom *= zoom_factor;
                        } else {
                            self.camera.zoom /= zoom_factor;
                        }
                        self.camera.zoom = self.camera.zoom.clamp(0.1, 8.0);
                        true
                    }
                    MouseScrollDelta::PixelDelta(_) => false,
                }
            }
            _ => self.camera.process_input(event),
        }
    }

    pub fn update(&mut self) {
        self.camera.update_uniform(&mut self.camera_uniform);
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );

        let visible_dots = self.compute_visible_dots();
        self.queue.write_buffer(
            &self.dot_buffer,
            0,
            bytemuck::cast_slice(&visible_dots),
        );
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        println!("Rendering frame...");
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 1.0,
                            g: 1.0,
                            b: 1.0,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
    
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.dot_buffer.slice(..));
            let num_vertices = self.compute_visible_dots().len() as u32;
            println!("Drawing {} vertices", num_vertices);
            render_pass.draw(0..num_vertices, 0..1);
        }
    
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    
        Ok(())
    }

    fn compute_visible_dots(&self) -> Vec<Vertex> {
        let dot_spacing = 20.0 / self.camera.zoom;
        let dot_size = 10.0 / self.camera.zoom;
        let viewport_size = Vector2::new(self.size.width as f32, self.size.height as f32);
        let viewport_size_world = viewport_size / self.camera.zoom;
    
        let start_x = (self.camera.position.x / dot_spacing).floor() * dot_spacing;
        let start_y = (self.camera.position.y / dot_spacing).floor() * dot_spacing;
        let end_x = start_x + viewport_size_world.x + dot_spacing;
        let end_y = start_y + viewport_size_world.y + dot_spacing;
    
        let mut vertices = Vec::new();
        let mut x = start_x;
        while x < end_x {
            let mut y = start_y;
            while y < end_y {
                // Generate a square (two triangles) for each dot
                vertices.extend_from_slice(&[
                    Vertex { position: [x - dot_size, y - dot_size] },
                    Vertex { position: [x + dot_size, y - dot_size] },
                    Vertex { position: [x - dot_size, y + dot_size] },
                    Vertex { position: [x + dot_size, y - dot_size] },
                    Vertex { position: [x + dot_size, y + dot_size] },
                    Vertex { position: [x - dot_size, y + dot_size] },
                ]);
                y += dot_spacing;
            }
            x += dot_spacing;
        }
    
        println!("Generated {} vertices for dots", vertices.len());
        vertices
    }
}