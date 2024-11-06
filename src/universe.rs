use std::iter;

use wgpu::util::DeviceExt;
use winit::event_loop::{EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::{
    event::*,
    window::Window,
};

use crate::controller::InputController;
use crate::object::{Object, Placement};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
}

impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct PlacementUniform {
    location: [f32; 4], // might need to be 4
    color: [f32; 4],
}

impl PlacementUniform {
    fn new() -> Self{
        Self {location: [0., 0., 0., 0.], color: [0., 0.5, 0.5, 1.0]}
    }

    fn update(&mut self, placement: &Placement){
        let placement_vector = placement.placement_vector();
        self.location = placement_vector;
    }
}

pub struct Universe<'a> {
    surface: wgpu::Surface<'a>,
    pub device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    // vertex_buffer: wgpu::Buffer,
    // index_buffer: wgpu::Buffer,
    input_controller: InputController,
    player: Box<dyn Object>,
    placement_uniform: PlacementUniform,
    placement_buffer: wgpu::Buffer,
    placement_bind_group: wgpu::BindGroup,
    window: &'a Window,
}

impl<'a> Universe<'a> {
    pub async fn new(window: &'a Window, mut player: Box<dyn Object>) -> Universe<'a>{
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor{
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });
        let surface = instance.create_surface(window).unwrap();
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions{
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,

        }).await.unwrap();
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor{
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: Default::default(),
            }, None).await.unwrap();
        let surface_cap = surface.get_capabilities(&adapter);
        let surface_format = surface_cap.formats.iter().copied()
            .find(|f| f.is_srgb()).unwrap_or(surface_cap.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_cap.present_modes[0],
            alpha_mode: surface_cap.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        let input_controller = InputController::new();
        let mut placement_uniform = PlacementUniform::new();
        placement_uniform.update(player.placement());
        let placement_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Placement Buffer"),
            contents: bytemuck::cast_slice(&[placement_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let placement_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
            label: Some("placement_bind_group_layout"),
        });
        let placement_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &placement_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: placement_buffer.as_entire_binding(),
            }],
            label: Some("placement_bind_group"),
        });
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&placement_bind_group_layout],
            push_constant_ranges: &[],
        });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState { 
                module: &shader, 
                entry_point: "vs_main", compilation_options: Default::default(),
                buffers: &[Vertex::desc()]
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format:config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState{
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        player.init(&device);

        return Universe{
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            input_controller,
            player,
            placement_uniform,
            placement_buffer,
            placement_bind_group,
            window,
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        self.input_controller.process(event)
    }

    fn update(&mut self) {
        self.input_controller.update(&mut *self.player);
        self.placement_uniform.update(self.player.placement());
        self.queue.write_buffer(
            &self.placement_buffer,
            0,
            bytemuck::cast_slice(&[self.placement_uniform])
        );
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
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
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.placement_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.player.vertex_buffer().unwrap().slice(..));
            render_pass.set_index_buffer(self.player.index_buffer().unwrap().slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.player.num_indices(), 0, 0..1);
        }

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub async fn run(&mut self, event_loop: EventLoop<()>){
        env_logger::init();
        self.surface.configure(&self.device, &self.config);
        let surface_configured = true;
        event_loop
            .run(move |event, control_flow| {
                match event {
                    Event::WindowEvent {
                        ref event,
                        window_id,
                    } if window_id == self.window.id() => {
                        if !self.input(event) {
                            match event {
                                WindowEvent::CloseRequested
                                | WindowEvent::KeyboardInput {
                                    event:
                                        KeyEvent {
                                            state: ElementState::Pressed,
                                            physical_key: PhysicalKey::Code(KeyCode::Escape),
                                            ..
                                        },
                                    ..
                                } => control_flow.exit(),
                                WindowEvent::RedrawRequested => {
                                    self.window.request_redraw();
                                    if !surface_configured {
                                        println!("surface not configured");
                                        return;
                                    }
                                    self.update();
                                    match self.render() {
                                        Ok(_) => {}
                                        Err(
                                            wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated,
                                        ) => (),
                                        Err(wgpu::SurfaceError::OutOfMemory) => {
                                            log::error!("OutOfMemory");
                                            control_flow.exit();
                                        }
                                        Err(wgpu::SurfaceError::Timeout) => {
                                            log::warn!("Surface timeout")
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            })
            .unwrap();
    }

}