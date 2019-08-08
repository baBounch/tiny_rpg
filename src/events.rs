//! Handles events in the game. Events will occur from the player moving.
//!
//! First the player's action is read. Based on the action the other
//! entities will change as well. Hoping to make it so Entities are
//! the only thing that makes changes to the game.

use crate::data::Data;

use sdl2::event::Event;

/// Handles events. Likely handle_key_events will be moved here as it
/// is unlikey any event will occur without it.
///
/// Takes a reference to the event and mutable data. This along with
/// handle_key_events may be moved to render.rs to make the program
/// less dependent on sdl2.
pub fn handle_events(event: &sdl2::event::Event, data: &mut Data) {
    handle_key_events(event, data);
}

/// Handles keypress events. Reads the keycode from the given event,
/// then performs a player action based on the code.
///
/// The binding attribute of the Settings enum allows the keycodes to be
/// independent of the action and can be changed.
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

/// Moves the game time one tick and gives all non-player entities their
/// turns.
pub fn move_world_forward(data: &mut Data) {
    data.add_one_time();
    data.entities_act();
}

/// An enum of the players possible actions. These should be binded to
/// keys for use by the player.
#[derive(Copy, PartialEq, Eq, Debug, Clone)]
pub enum PlayerAction {
    /// Moves the player left and updates the game world.
    MoveLeft,
    /// Moves the player right and updates the game world.
    MoveRight,
    /// Moves the player up and updates the game world.
    MoveUp,
    /// Moves the player down and updates the game world.
    MoveDown,
    /// Allows the player to check their inventory.
    Inventory,
    /// Allows the player to check their settings.
    Settings,
}

impl PlayerAction {
    /// The action is read and a method is performed based on it.
    ///
    /// data is required because most actions will need to know about
    /// the world.
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
