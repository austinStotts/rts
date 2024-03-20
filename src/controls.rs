use iced_wgpu::Renderer;
use iced_widget::{column, container, row, slider, text, text_input};
use iced_winit::core::alignment;
use iced_winit::core::{Color, Element, Length};
use iced_winit::runtime::{Command, Program};
use iced_widget::Theme;
use iced_aw::{number_input, style::NumberInputStyles};

use crate::scene::Parameters;


const ZOOM_SPEED: f32 = 0.1;
// const PAN_SCALAR: f32 = 10000.0;




// fn handle_zoom(delta: &MouseScrollDelta, zoom_level: &mut f32, pan_offset: &[f32; 2], window_aspect_ratio: f32, image_aspect_ratio: f32) -> Vec<Vertex> {
//     match delta {
//         MouseScrollDelta::LineDelta(_, y) => {
//             *zoom_level += y * ZOOM_SPEED;
//             *zoom_level = zoom_level.clamp(0.1, 10.0); // Clamp the zoom level between 0.1 and 10.0
//         }
//         MouseScrollDelta::PixelDelta(PhysicalPosition { y, .. }) => {
//             *zoom_level -= (*y as f32) * ZOOM_SPEED;
//             *zoom_level = zoom_level.clamp(0.1, 10.0); // Clamp the zoom level between 0.1 and 10.0
//         }
//     }

//     update_vertex_data(zoom_level, pan_offset, window_aspect_ratio, image_aspect_ratio)
// }



// fn handle_pan(
//     curr_mouse_pos: &PhysicalPosition<f64>,
//     prev_mouse_pos: &mut PhysicalPosition<f64>,
//     zoom_level: &f32,
//     pan_offset: &mut [f32; 2],
// ) {
//     let panning_speed_factor = 0.005 / *zoom_level;

//     let delta_x = (curr_mouse_pos.x - prev_mouse_pos.x) as f32;
//     let delta_y = (curr_mouse_pos.y - prev_mouse_pos.y) as f32;

//     pan_offset[0] += delta_x * panning_speed_factor;
//     pan_offset[1] += -delta_y * panning_speed_factor;

//     *prev_mouse_pos = *curr_mouse_pos;
// }


pub struct Controls {
    pub background_color: Color,
    pub input: String,
    pub sigma1: f32,
    pub tau: f32,
    pub gfact: f32,
    pub epsilon: f32,
    pub num_gvf_iterations: i32,
    pub enable_xdog: u32,
}

#[derive(Debug, Clone)]
pub enum Message {
    BackgroundColorChanged(Color),
    InputChanged(String),
    Sigma1Changed(f32),
    TauChanged(f32),
    GFactChanged(f32),
    IsFactChanged(i32),
}

impl Controls {
    pub fn new() -> Controls {
        Controls {
            background_color: Color::BLACK,
            input: String::default(),
            sigma1: 4.75,
            tau: 0.075,
            gfact: 8.0,
            epsilon: 0.0001,
            num_gvf_iterations: 30,
            enable_xdog: 1,
        }
    }

    pub fn background_color(&self) -> Color {
        self.background_color
    }

    pub fn params(&self) -> Parameters {
        return Parameters {
            sigma1: self.sigma1,
            tau: self.tau,
            gfact: self.gfact,
            epsilon: self.epsilon,
            num_gvf_iterations: self.num_gvf_iterations,
            enable_xdog: self.enable_xdog,
        }
    }
}

impl Program for Controls {
    type Theme = Theme;
    type Message = Message;
    type Renderer = Renderer;

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::BackgroundColorChanged(color) => {
                self.background_color = color;
            }
            Message::InputChanged(input) => {
                self.input = input;
            }
            Message::Sigma1Changed(v) => {
                self.sigma1 = v;
            }
            Message::TauChanged(v) => {
                self.tau = v;
            }
            Message::GFactChanged(v) => {
                self.gfact = v;
            }
            Message::IsFactChanged(v) => {
                self.num_gvf_iterations = v;
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message, Theme, Renderer> {
        let background_color = self.background_color;
        let sigma1 = self.sigma1;
        let tau = self.tau;
        let gfact = self.gfact;
        let epsilon = self.epsilon;
        let num_gvf_iterations = self.num_gvf_iterations;
        let enable_xdog = self.enable_xdog;

        let sliders = column![
            row![
                number_input(sigma1, 10.0, move |v| {
                    Message::Sigma1Changed(v)
                }).step(0.1),
                text("sigma"),
            ].width(500).spacing(10),
            row![
                number_input(tau, 0.3, move |v| {
                    Message::TauChanged(v)
                }).step(0.01),
                text("tau"),
            ].width(500).spacing(10),
            row![
                number_input(gfact, 10.0, move |v| {
                    Message::GFactChanged(v)
                }).step(0.5),
                text("gamma"),
            ].width(500).spacing(10),
            row![
                number_input(num_gvf_iterations, 30, move |v| {
                    Message::IsFactChanged(v)
                }).step(1),
                text("iterations"),
            ].width(500).spacing(10)
        ]
        .width(500)
        .spacing(2);

        container(
            column![
                sliders,
            ]
            .spacing(10),
        )
        .padding(10)
        .height(Length::Fill)
        .align_y(alignment::Vertical::Bottom)
        .into()
    }
}