use crate::data::Data;

use sdl2::event::Event;

pub fn handle_events(event: &sdl2::event::Event, data: &mut Data) {
    handle_key_events(event, data);
}

pub fn handle_key_events(event: &sdl2::event::Event, data: &mut Data) {
    let mut action: Option<PlayerAction> = None;

    for binding in data.get_settings().get_bindings().iter() {
        let keycode = (*binding).0;
        match event {
            Event::KeyDown {
                keycode: Some(v), ..
            } => {
                if v == &keycode {
                    action = Some((*binding).1);
                    break;
                }
            }
            _ => {}
        };
    }

    match action {
        Some(a) => a.perform_action(data),
        None => {}
    }
}

pub fn move_world_forward(data: &mut Data) {
    data.add_one_time();
    data.entities_act();
}

#[derive(Copy, PartialEq, Eq, Debug, Clone)]
pub enum PlayerAction {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    Inventory,
    Settings,
}

impl PlayerAction {
    pub fn perform_action(&self, data: &mut Data) {
        match self {
            PlayerAction::MoveLeft => self.move_left(data),
            PlayerAction::MoveRight => self.move_right(data),
            PlayerAction::MoveUp => self.move_up(data),
            PlayerAction::MoveDown => self.move_down(data),
            PlayerAction::Inventory => self.inventory(data),
            PlayerAction::Settings => self.settings(data),
        }
    }
    // validation will soon be done here for player movement
    // events will also happen when player moves
    fn move_left(&self, data: &mut Data) {
        // soon do collision based on entity type, for now all the same.
        if data
            .check_position(data.get_player().get_move_left())
            .is_none()
        {
            data.get_mut_player().move_left();
        }

        move_world_forward(data);
    }
    fn move_right(&self, data: &mut Data) {
        if data
            .check_position(data.get_player().get_move_right())
            .is_none()
        {
            data.get_mut_player().move_right();
        }

        move_world_forward(data);
    }
    fn move_up(&self, data: &mut Data) {
        if data
            .check_position(data.get_player().get_move_up())
            .is_none()
        {
            data.get_mut_player().move_up();
        }

        move_world_forward(data);
    }
    fn move_down(&self, data: &mut Data) {
        if data
            .check_position(data.get_player().get_move_down())
            .is_none()
        {
            data.get_mut_player().move_down();
        };

        move_world_forward(data);
    }
    fn inventory(&self, data: &mut Data) {}
    fn settings(&self, data: &mut Data) {}
}
