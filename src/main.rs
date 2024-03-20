mod controls;
mod scene;

use controls::Controls;
use iced_wgpu::wgpu::core::id::DeviceId;
// use iced_winit::winit::event::KeyEvent;

use iced_winit::winit::event;
use iced_winit::winit::platform::modifier_supplement::KeyEventExtModifierSupplement;
// use iced_winit::winit::platform::modifier_supplement::KeyEventExtModifierSupplement;
use scene::Scene;
use scene::Parameters;

use iced_wgpu::graphics::Viewport;
use iced_wgpu::{wgpu, Backend, Renderer, Settings};
use iced_winit::conversion;
use iced_winit::core::mouse;
use iced_winit::core::renderer;
use iced_winit::core::window;
use iced_winit::core::{Color, Font, Pixels, Size};
use iced_winit::futures;
use iced_winit::runtime::program;
use iced_winit::runtime::Debug;
use iced_winit::winit;
use iced_winit::Clipboard;
use iced_widget::Theme;
use winit::dpi::{LogicalSize, PhysicalPosition};
// use winit::event::WindowEvent::KeyboardInput;
use winit::{
    event::{Event, ElementState, WindowEvent, MouseScrollDelta, MouseButton },
    event_loop::{ControlFlow, EventLoop},
    keyboard::ModifiersState,
};
use std::sync::Arc;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32; 2],
    texcoord: [f32; 2], 
}



struct PanState {
    is_panning: bool,
    prev_mouse_pos: PhysicalPosition<f64>,
}












pub fn main() -> Result<(), Box<dyn std::error::Error>> {


    #[cfg(not(target_arch = "wasm32"))]
    tracing_subscriber::fmt::init();

    // Initialize winit
    let event_loop = EventLoop::new()?;

    #[cfg(target_arch = "wasm32")]
    let window = winit::window::WindowBuilder::new()
        .with_canvas(Some(canvas_element))
        .build(&event_loop)?;

    #[cfg(not(target_arch = "wasm32"))]
    let window = winit::window::Window::new(&event_loop)?;

    let window = Arc::new(window);

    let physical_size = window.inner_size();
    let mut viewport = Viewport::with_physical_size(
        Size::new(physical_size.width, physical_size.height),
        window.scale_factor(),
    );
    let mut cursor_position = None;
    let mut modifiers = ModifiersState::default();
    let mut clipboard = Clipboard::connect(&window);


    let default_backend = wgpu::Backends::PRIMARY;
    let backend =
        wgpu::util::backend_bits_from_env().unwrap_or(default_backend);

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: backend,
        ..Default::default()
    });
    let surface = instance.create_surface(window.clone())?;

    let (format, adapter, device, queue) =
        futures::futures::executor::block_on(async {
            let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("Create adapter");

            let adapter_features = adapter.features();
            let needed_limits = wgpu::Limits::default();

            let capabilities = surface.get_capabilities(&adapter);

            let (device, queue) = adapter
                .request_device(
                    &wgpu::DeviceDescriptor {
                        label: None,
                        required_features: adapter_features
                            & wgpu::Features::default(),
                        required_limits: needed_limits,
                    },
                    None,
                )
                .await
                .expect("Request device");

            (
                capabilities
                    .formats
                    .iter()
                    .copied()
                    .find(wgpu::TextureFormat::is_srgb)
                    .or_else(|| capabilities.formats.first().copied())
                    .expect("Get preferred format"),
                adapter,
                device,
                queue,
            )
        });

        println!("gpu: {}", adapter.get_info().name);

    surface.configure(
        &device,
        &wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: physical_size.width,
            height: physical_size.height,
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        },
    );

    let mut resized = false;

    // Initialize scene and GUI controls
    let scene = Scene::new(&device, format, &queue);
    let controls = Controls::new();

    // Initialize iced
    let mut debug = Debug::new();
    let mut renderer = Renderer::new(
        Backend::new(&device, &queue, Settings::default(), format),
        Font::default(),
        Pixels(16.0),
    );

    let mut state = program::State::new(
        controls,
        viewport.logical_size(),
        &mut renderer,
        &mut debug,
    );

    let mut zoom_level: f32 = 1.0;
    let mut pan_offset = [0.0, 0.0];
    let mut pan_state = PanState {
        is_panning: false,
        prev_mouse_pos: PhysicalPosition::new(0.0, 0.0),
    };

    let mut current_mouse_position = PhysicalPosition::new(0.0, 0.0);

    // Run event loop
    event_loop.run(move |event, window_target| {
        // You should change this if you want to render continuously
        window_target.set_control_flow(ControlFlow::Wait);


        match event {
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                if resized {
                    let size = window.inner_size();

                    viewport = Viewport::with_physical_size(
                        Size::new(size.width, size.height),
                        window.scale_factor(),
                    );

                    surface.configure(
                        &device,
                        &wgpu::SurfaceConfiguration {
                            format,
                            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                            width: size.width,
                            height: size.height,
                            present_mode: wgpu::PresentMode::AutoVsync,
                            alpha_mode: wgpu::CompositeAlphaMode::Auto,
                            view_formats: vec![],
                            desired_maximum_frame_latency: 2,
                        },
                    );

                    resized = false;
                }

                match surface.get_current_texture() {
                    Ok(frame) => {
                        let mut encoder = device.create_command_encoder(
                            &wgpu::CommandEncoderDescriptor { label: None },
                        );

                        let program = state.program();

                        let view = frame.texture.create_view(
                            &wgpu::TextureViewDescriptor::default(),
                        );


                        {
                            // We clear the frame
                            let mut render_pass = Scene::clear(
                                &view,
                                &mut encoder,
                                program.background_color(),
                            );




                            let window_size = window.inner_size();
                            let window_aspect_ratio = window_size.width as f32 / window_size.height as f32;
                            // let params: Parameters = controls.params();

                            
                            // Draw the scene
                            scene.draw(&mut render_pass, &queue, window_aspect_ratio, bytemuck::cast_slice(&[program.params()]));
                        }

                        // And then iced on top
                        renderer.with_primitives(|backend, primitive| {
                            backend.present(
                                &device,
                                &queue,
                                &mut encoder,
                                None,
                                frame.texture.format(),
                                &view,
                                primitive,
                                &viewport,
                                &debug.overlay(),
                            );
                        });

                        // Then we submit the work
                        queue.submit(Some(encoder.finish()));
                        frame.present();

                        // Update the mouse cursor
                        window.set_cursor_icon(
                            iced_winit::conversion::mouse_interaction(
                                state.mouse_interaction(),
                            ),
                        );
                    }
                    Err(error) => match error {
                        wgpu::SurfaceError::OutOfMemory => {
                            panic!(
                                "Swapchain error: {error}. \
                                Rendering cannot continue."
                            )
                        }
                        _ => {
                            // Try rendering again next frame.
                            window.request_redraw();
                        }
                    },
                }
            }
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CursorMoved { position, .. } => {
                        cursor_position = Some(position);
                    }
                    WindowEvent::ModifiersChanged(new_modifiers) => {
                        modifiers = new_modifiers.state();
                    }
                    WindowEvent::Resized(_) => {
                        resized = true;
                    }
                    WindowEvent::CloseRequested => {
                        window_target.exit();
                    }
                    WindowEvent::KeyboardInput { device_id, event, is_synthetic } => {
                        println!("a key was pressed");
                    }
                    _ => {}
                }
                // Map window event to iced event
                if let Some(event) = iced_winit::conversion::window_event(
                    window::Id::MAIN,
                    event,
                    window.scale_factor(),
                    modifiers,
                ) {
                    state.queue_event(event);
                }

            }
            _ => {}
        }

        // If there are events pending
        if !state.is_queue_empty() {
            // We update iced
            let _ = state.update(
                viewport.logical_size(),
                cursor_position
                    .map(|p| {
                        conversion::cursor_position(p, viewport.scale_factor())
                    })
                    .map(mouse::Cursor::Available)
                    .unwrap_or(mouse::Cursor::Unavailable),
                &mut renderer,
                &Theme::Dark,
                &renderer::Style {
                    text_color: Color::WHITE,
                },
                &mut clipboard,
                &mut debug,
            );

            // and request a redraw
            window.request_redraw();
        }
    })?;

    Ok(())
}