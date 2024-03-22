
use std::path::Component;

use iced_wgpu::core::Font;

use iced_wgpu::Renderer;
use iced_widget::{button, column, combo_box, component, container, pick_list, row, slider, text, text_input};
use iced_winit::core::alignment;
use iced_winit::core::{Color, Element, Length};
use iced_winit::runtime::{Command, Program};
use iced_widget::Theme;
use iced_aw::{number_input, style::NumberInputStyles, SelectionList, style::SelectionListStyles};
use iced_wgpu::wgpu::{self, util::DeviceExt, ShaderModuleDescriptor};
use image::codecs::png;
use image::imageops::replace;
// use iced::{window, Element};
use image::Rgba;
use crate::scene::Parameters;
use rfd;
use iced_winit::winit;





use win_screenshot;


// struct ImageLocation {
//     location: String,
//     name: String,
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Shader {
    #[default]
    none,
    invert,
    gaussian_blur,
    quantization,
    sobel_edge_detection,
    difference_of_gaussians_DoG,
    flow_based_XDoG,
    edge_direction,
    bayer_dither,
}

impl Shader {
    const ALL: [Shader; 9] = [
        Shader::none,
        Shader::invert,
        Shader::gaussian_blur,
        Shader::quantization,
        Shader::sobel_edge_detection,
        Shader::difference_of_gaussians_DoG,
        Shader::flow_based_XDoG,
        Shader::edge_direction,
        Shader::bayer_dither,
    ];
}

impl Shader {
    pub fn get_index(&self) -> u32 {
        match self {
            Shader::none => 100,
            Shader::invert => 0,
            Shader::gaussian_blur => 1,
            Shader::quantization => 2,
            Shader::sobel_edge_detection => 3,
            Shader::difference_of_gaussians_DoG => 4,
            Shader::flow_based_XDoG => 5,
            Shader::edge_direction => 6,
            Shader::bayer_dither => 7,
        }
    }

    pub fn get_parameters<'a>(&self, controls: &Controls) -> iced_widget::Container<'a, Message, Theme, Renderer> {
        match self {
            Shader::none => container(column![]),
            Shader::invert => container(column![]),
            Shader::gaussian_blur => container(column![
                row![number_input(controls.sigma1, 10.0, move |v| {Message::Sigma1Changed(v)}).step(0.1),text("sigma"),].width(500).spacing(10),
            ]),
            Shader::quantization => container(column![]),
            Shader::sobel_edge_detection => container(column![]),
            Shader::difference_of_gaussians_DoG => container(column![
                row![number_input(controls.sigma1, 10.0, move |v| {Message::Sigma1Changed(v)}).step(0.1),text("sigma"),].width(500).spacing(10),
                row![number_input(controls.tau, 0.3, move |v| {Message::TauChanged(v)}).step(0.01),text("tau"),].width(500).spacing(10),
            ]),
            Shader::flow_based_XDoG => container(column![
                row![number_input(controls.sigma1, 10.0, move |v| {Message::Sigma1Changed(v)}).step(0.1),text("sigma"),].width(500).spacing(10),
                row![number_input(controls.tau, 0.3, move |v| {Message::TauChanged(v)}).step(0.01),text("tau"),].width(500).spacing(10),
                row![number_input(controls.gfact, 10.0, move |v| {Message::GFactChanged(v)}).step(0.5),text("gamma"),].width(500).spacing(10),
                row![number_input(controls.num_gvf_iterations, 30, move |v| {Message::IsFactChanged(v)}).step(1),text("iterations"),].width(500).spacing(10)
            ]),
            Shader::edge_direction => container(column![]),
            Shader::bayer_dither => container(column![]),
        }
    }
}

impl std::fmt::Display for Shader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Shader::none => "none",
                Shader::invert => "invert",
                Shader::gaussian_blur => "gaussian blur",
                Shader::quantization => "quantization",
                Shader::sobel_edge_detection => "sobel edge detection",
                Shader::difference_of_gaussians_DoG => "difference of gaussians DoG",
                Shader::flow_based_XDoG => "flow based XDoG",
                Shader::edge_direction => "edge direction",
                Shader::bayer_dither => "bayer dither",
            }
        )
    }
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Controls {
    pub background_color: Color,
    pub input: String,
    pub shaders: combo_box::State<Shader>,
    pub selected_shader: Option<Shader>,
    pub selected_image: String,
    pub did_change: bool,
    pub show_ui: bool,
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
    ImageChanger(),
    TakeScreenshot(),
    ToggleUI(),
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
            selected_shader: Some(Shader::none),
            selected_image: String::from("C:/Users/astotts/rust/rts/images/cat.png"),
            did_change: false,
            show_ui: true,
            sigma1: 4.75,
            tau: 0.075,
            gfact: 8.0,
            epsilon: 0.0001,
            num_gvf_iterations: 15,
            enable_xdog: 1,
        }
    }

    pub fn update_did_change(&mut self, v: bool) {
        self.did_change = v; 
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
            shader_index: self.selected_shader.unwrap().get_index(),
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
                self.selected_shader = Some(shader);
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
            Message::ImageChanger() => {
                println!("IMAGE CHANGER");
                let dialog = rfd::FileDialog::new().pick_file().unwrap().into_os_string().into_string().unwrap();
                self.selected_image = str::replace(&dialog, '\\', "/");
                self.did_change = true;
            }
            Message::TakeScreenshot() => {

                // async fn foo() {
                //     println!("The Future will be ready after some time");
                //     set_timeout(Duration::from_secs(5)).await;
                //     println!("Now, it is ready");
                //   }
                  
                //   block_on(foo());
                
                let window_buffer = win_screenshot::capture::capture_window(win_screenshot::utils::find_window("real time shaders").unwrap());
                match window_buffer {
                    Ok(RgbBuf) => {
                        image::save_buffer("./render.png", &RgbBuf.pixels, RgbBuf.width, RgbBuf.height, image::ColorType::Rgba8);
                    } Err(Error) => {
                        println!("could not get window")
                    }
                }
            }
            Message::ToggleUI() => {
                self.show_ui = !self.show_ui;
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
        // let selected_image = self.selected_image;

        let shader_controls = column![
            selected_shader.unwrap().get_parameters(self),
            row![pick_list(&Shader::ALL[..],self.selected_shader,Message::ShaderSelected,)].width(200).spacing(10),
        ]
        .width(500)
        .spacing(2);
        let c = self.selected_image.split('/').last().unwrap();

        let image_loader = row![button("save").on_press(Message::TakeScreenshot()),button(c).on_press(Message::ImageChanger())].width(500).spacing(2);
        // ,button("toggle ui").on_press(Message::ToggleUI())
        if self.show_ui {
            container(
                column![
                container(column![image_loader].spacing(10))
                    .padding(10)
                    .height(Length::Fill)
                    .align_y(alignment::Vertical::Top),
                container(column![shader_controls].spacing(10))
                    .padding(10)
                    .height(Length::Fill)
                    .align_y(alignment::Vertical::Bottom),
            ]).into()
        } else {
            container(row![]).into()
        }

    }
}



// row![number_input(sigma1, 10.0, move |v| {Message::Sigma1Changed(v)}).step(0.1),text("sigma"),].width(500).spacing(10),
// row![number_input(tau, 0.3, move |v| {Message::TauChanged(v)}).step(0.01),text("tau"),].width(500).spacing(10),
// row![number_input(gfact, 10.0, move |v| {Message::GFactChanged(v)}).step(0.5),text("gamma"),].width(500).spacing(10),
// row![number_input(num_gvf_iterations, 30, move |v| {Message::IsFactChanged(v)}).step(1),text("iterations"),].width(500).spacing(10)