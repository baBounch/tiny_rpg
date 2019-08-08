//! Primary entry to the use of sdl2 graphics.
//! Objects are rendered to the canvas here based on their positions, color, etc.

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::entity;
use crate::math::{self, TwoDimensional};

/// Renders all of the objects given to it onto the canvas.
pub fn render_objects(
    tile_size: u32,
    screen_size: math::Dimension,
    canvas: &mut Canvas<Window>,
    player: &entity::Entity,
    objects: &Vec<entity::Entity>,
) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    render_player(tile_size, screen_size, canvas, player);
    for object in objects.iter() {
        render_entity(
            tile_size,
            screen_size,
            canvas,
            object,
            player.get_rectangle(),
        );
    }

    canvas.present();
}

/// Create the player and tries to put them in the center of the canvas.
pub fn render_player(
    tile_size: u32,
    screen_size: math::Dimension,
    canvas: &mut Canvas<Window>,
    entity: &entity::Entity,
) {
    let entity_rectangle = Rect::new(
        (screen_size.get_width() / 2 - 1) as i32,
        (screen_size.get_height() / 2 - 1) as i32,
        entity.get_rectangle().width() * tile_size,
        entity.get_rectangle().height() * tile_size,
    );
    let (red, green, blue) = entity.get_skin().get_rgb_colors().unwrap();

    canvas.set_draw_color(Color::RGB(red, green, blue));
    canvas.fill_rect(entity_rectangle).unwrap();
}

/// Creates entities and places them relative to the player.
pub fn render_entity(
    tile_size: u32,
    screen_size: math::Dimension,
    canvas: &mut Canvas<Window>,
    entity: &entity::Entity,
    player_rect: math::Rectangle,
) {
    let (x, y) = entity.get_position().get_coordinates();
    let (width, height) = entity.get_size().get_coordinates();
    let entity_rectangle = Rect::new(
        (x - player_rect.x()) * tile_size as i32 + (screen_size.get_width() / 2 - 1) as i32,
        (y - player_rect.y()) * tile_size as i32 + (screen_size.get_height() / 2 - 1) as i32,
        width * tile_size,
        height * tile_size,
    );
    let (red, green, blue) = entity.get_skin().get_rgb_colors().unwrap();

    canvas.set_draw_color(Color::RGB(red, green, blue));
    canvas.fill_rect(entity_rectangle).unwrap();
}
