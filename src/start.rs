//! The start sdl and argument handler for tiny rpg.
//!
//! Checks given, arguments, environment variables etc. and parses
//! them to something readable for the main program.
//!
//! Initializes the sdl2 graphic. May be removed from here to make
//! the code less sdl2 dependent.
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::helper::{Builder, BuilderError};
use crate::math;

/// Things done before main_loop. adds created variable to an Init enum to send
/// to main loop.
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

/// Builds the environment.
///
/// In the future will handle parsing arguments, etc.
pub struct EnvironmentBuilder {
    window_title: &'static str,
    window_size: math::Dimension,
    tile_size: u32,
}

impl EnvironmentBuilder {
    /// Sets the window height.
    pub fn set_window_height(mut self, width: u32) -> Self {
        self.window_size.set_width(width);
        self
    }
    /// Sets the window width.
    pub fn set_window_width(mut self, height: u32) -> Self {
        self.window_size.set_height(height);
        self
    }
    /// Sets the window height and width using a Dimension.
    pub fn set_window_dimensions(mut self, dimensions: math::Dimension) -> Self {
        self.window_size = dimensions;
        self
    }
    /// Sets the title of the window.
    pub fn set_window_title(mut self, title: &'static str) -> Self {
        self.window_title = title;
        self
    }
    /// Short hand for set_window_title.
    ///
    /// Probably will be removed or changed.
    pub fn set_title(mut self, title: &'static str) -> Self {
        self.window_title = title;
        self
    }
    /// Set the size of tile, smallest entities, etc. in pixels.
    pub fn set_tile_size(mut self, pixels: u32) -> Self {
        self.tile_size = pixels;
        self
    }
}

impl Builder for EnvironmentBuilder {
    type product = Environment;

    /// Creates a new instance of EnvironmentBuilder.
    /// May later handle parsing of arguments, etc.
    fn new() -> EnvironmentBuilder {
        let tile_size = 32;
        EnvironmentBuilder {
            window_size: math::Dimension::new(tile_size * 30, tile_size * 20),
            window_title: "Application",
            tile_size,
        }
    }
    /// Build Environment from EnvironmentBuilder.
    fn build(self) -> Result<Environment, BuilderError> {
        Ok(Environment {
            window_size: self.window_size,
            window_title: self.window_title,
            tile_size: self.tile_size,
        })
    }
}

/// The Environment is used to get information from the environment surrounding the game.
///
/// Some of this may get merged with Settings from data.rs.
pub struct Environment {
    window_size: math::Dimension,
    window_title: &'static str,
    tile_size: u32,
}

impl Environment {
    /// Creates an instance of EnvironmentBuilder to build Environment
    pub fn new() -> EnvironmentBuilder {
        EnvironmentBuilder::new()
    }
    /// Returns the window size.
    pub fn get_window_size(&self) -> math::Dimension {
        self.window_size
    }
    /// Returns the window width.
    pub fn get_window_width(&self) -> u32 {
        self.window_size.get_width()
    }
    /// Returns the window height.
    pub fn get_window_height(&self) -> u32 {
        self.window_size.get_height()
    }
    /// Returns the window title.
    pub fn get_window_title(&self) -> &'static str {
        self.window_title
    }
    /// Returns the tile size.
    pub fn get_tile_size(&self) -> u32 {
        self.tile_size
    }
}

/// Holds information that is sent to the main game loop.
pub struct Init {
    sdl_context: sdl2::Sdl,
    canvas: Canvas<Window>,
    environment: Environment,
}

impl Init {
    /// Creates a new instance of Init will all information ready.
    pub fn new(sdl_context: sdl2::Sdl, canvas: Canvas<Window>, environment: Environment) -> Self {
        Init {
            sdl_context,
            canvas,
            environment,
        }
    }
    /// Sends all information in a tuple from the Init.
    ///
    /// Init will likely only need to be used once, so no need of reference just change ownership.
    pub fn release(self) -> (sdl2::Sdl, Canvas<Window>, Environment) {
        (self.sdl_context, self.canvas, self.environment)
    }
}
