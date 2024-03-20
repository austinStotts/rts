use iced_wgpu::core::Font;
use iced_wgpu::Renderer;
use iced_widget::{column, container, row, slider, text, text_input, combo_box};
use iced_winit::core::alignment;
use iced_winit::core::{Color, Element, Length};
use iced_winit::runtime::{Command, Program};
use iced_widget::Theme;
use iced_aw::{number_input, style::NumberInputStyles, SelectionList, style::SelectionListStyles};
use iced_wgpu::wgpu::{self, util::DeviceExt, ShaderModuleDescriptor};

use crate::scene::Parameters;





#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Shader {
    #[default]
    invert,
    gaussian_blur,
    quantization,
    sobel_edge_detection,
    difference_of_gaussians_DoG,
    flow_based_XDoG
}

impl Shader {
    const ALL: [Shader; 6] = [
        Shader::invert,
        Shader::gaussian_blur,
        Shader::quantization,
        Shader::sobel_edge_detection,
        Shader::difference_of_gaussians_DoG,
        Shader::flow_based_XDoG,
    ];
}

impl Shader {
    pub fn getVertex(&self) -> ShaderModuleDescriptor<'_> {
        match self {
            Shader::invert => wgpu::include_wgsl!("../shaders/invert/vertex.wgsl"),
            Shader::gaussian_blur => wgpu::include_wgsl!("../shaders/gaussian-blur/vertex.wgsl"),
            Shader::quantization => wgpu::include_wgsl!("../shaders/quantization/vertex.wgsl"),
            Shader::sobel_edge_detection => wgpu::include_wgsl!("../shaders/sobel-edge-detection/vertex.wgsl"),
            Shader::difference_of_gaussians_DoG => wgpu::include_wgsl!("../shaders/difference-of-gaussians/vertex.wgsl"),
            Shader::flow_based_XDoG => wgpu::include_wgsl!("../shaders/flow-based-xdog/vertex.wgsl"),
        }
    }
    pub fn getFragment(&self) -> ShaderModuleDescriptor<'_> {
        match self {
            Shader::invert => wgpu::include_wgsl!("../shaders/invert/fragment.wgsl"),
            Shader::gaussian_blur => wgpu::include_wgsl!("../shaders/gaussian-blur/fragment.wgsl"),
            Shader::quantization => wgpu::include_wgsl!("../shaders/quantization/fragment.wgsl"),
            Shader::sobel_edge_detection => wgpu::include_wgsl!("../shaders/sobel-edge-detection/fragment.wgsl"),
            Shader::difference_of_gaussians_DoG => wgpu::include_wgsl!("../shaders/difference-of-gaussians/fragment.wgsl"),
            Shader::flow_based_XDoG => wgpu::include_wgsl!("../shaders/flow-based-xdog/fragment.wgsl"),
        }
    }
}

impl std::fmt::Display for Shader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Shader::invert => "invert",
                Shader::gaussian_blur => "gaussian blur",
                Shader::quantization => "quantization",
                Shader::sobel_edge_detection => "sobel edge detection",
                Shader::difference_of_gaussians_DoG => "difference of gaussians DoG",
                Shader::flow_based_XDoG => "flow based XDoG",
            }
        )
    }
}

pub struct Controls {
    pub background_color: Color,
    pub input: String,
    pub shaders: combo_box::State<Shader>,
    pub selected_shader: Option<Shader>,
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
    ShaderSelected(Shader),
}

impl Controls {
    pub fn new() -> Controls {
        // let mut shaders = Vec::with_capacity(6);

        // for i in Shader::ALL.iter() {
        //     shaders.push(format!("{i}"))
        // }

        return Controls {
            background_color: Color::BLACK,
            input: String::default(),
            shaders: combo_box::State::new(Shader::ALL.to_vec()),
            selected_shader: Some(Shader::invert),
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
            Message::ShaderSelected(shader) => {
                self.selected_shader = Some(shader)
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
        let selected_shader = self.selected_shader;

        let sliders = column![
            row![
                combo_box(
                    &self.shaders,
                    "pick a shader",
                    self.selected_shader.as_ref(),
                    Message::ShaderSelected)
            ]
                .width(200).spacing(10),
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