extern crate sdl2;

pub mod data;
pub mod entity;
pub mod events;
pub mod helper;
pub mod maps;
pub mod math;
pub mod quests;
pub mod render;
pub mod start;

use render::*;

/// The main loop. Events and such happen here
// fix result here, an error enum would be cool to get past
// dyn errors maybe dont bother with normal errors
pub fn main_loop(init: start::Init) -> Result<(), ()> {
    let (sdl_context, mut canvas, environment) = init.release();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut data = data::Data::new();
    data.change_entities(data.get_level().get_level_entities().to_vec());

    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. }
                | sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => break 'running,
                _ => events::handle_events(&event, &mut data),
            };
        }

        // change to one that handles menus
        render_objects(
            environment.get_tile_size(),
            environment.get_window_size(),
            &mut canvas,
            &data.get_player(),
            &data.get_entities(),
        );

        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}

pub enum Screen {
    MainMenu,
    Settings,
    Inventory,
    Combat,
    Game,
    Dialog,
}

/// What occurs after main loop
pub fn main_end() {}
