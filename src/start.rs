use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::helper::{Builder, BuilderError};
use crate::math;

/// Things done before main_loop
pub fn main_start() -> Result<Init, ()> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let environment = Environment::new().set_title("An RPG").build().unwrap();

    let window = video_subsystem
        .window(
            environment.get_window_title(),
            environment.get_window_width(),
            environment.get_window_height(),
        )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    Ok(Init::new(sdl_context, canvas, environment))
}

pub struct EnvironmentBuilder {
    window_title: &'static str,
    window_size: math::Dimension,
    tile_size: u32,
}

/// The configurations before building
impl EnvironmentBuilder {
    pub fn set_window_height(mut self, width: u32) -> Self {
        self.window_size.set_width(width);
        self
    }
    pub fn set_window_width(mut self, height: u32) -> Self {
        self.window_size.set_height(height);
        self
    }
    pub fn set_window_dimensions(mut self, dimensions: math::Dimension) -> Self {
        self.window_size = dimensions;
        self
    }
    pub fn set_window_title(mut self, title: &'static str) -> Self {
        self.window_title = title;
        self
    }
    pub fn set_title(mut self, title: &'static str) -> Self {
        self.window_title = title;
        self
    }
    pub fn set_tile_size(mut self, pixels: u32) -> Self {
        self.tile_size = pixels;
        self
    }
}

impl Builder for EnvironmentBuilder {
    type product = Environment;

    fn new() -> EnvironmentBuilder {
        let tile_size = 32;
        EnvironmentBuilder {
            window_size: math::Dimension::new(tile_size * 30, tile_size * 20),
            window_title: "Application",
            tile_size,
        }
    }
    fn build(self) -> Result<Environment, BuilderError> {
        Ok(Environment {
            window_size: self.window_size,
            window_title: self.window_title,
            tile_size: self.tile_size,
        })
    }
}

pub struct Environment {
    window_size: math::Dimension,
    window_title: &'static str,
    tile_size: u32,
}

impl Environment {
    pub fn new() -> EnvironmentBuilder {
        EnvironmentBuilder::new()
    }
    pub fn get_window_size(&self) -> math::Dimension {
        self.window_size
    }
    pub fn get_window_width(&self) -> u32 {
        self.window_size.get_width()
    }
    pub fn get_window_height(&self) -> u32 {
        self.window_size.get_height()
    }
    pub fn get_window_title(&self) -> &'static str {
        self.window_title
    }
    pub fn get_tile_size(&self) -> u32 {
        self.tile_size
    }
}

pub struct Init {
    sdl_context: sdl2::Sdl,
    canvas: Canvas<Window>,
    environment: Environment,
}

impl Init {
    pub fn new(sdl_context: sdl2::Sdl, canvas: Canvas<Window>, environment: Environment) -> Self {
        Init {
            sdl_context,
            canvas,
            environment,
        }
    }
    pub fn release(self) -> (sdl2::Sdl, Canvas<Window>, Environment) {
        (self.sdl_context, self.canvas, self.environment)
    }
}
