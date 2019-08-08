//! Stores data that is manipulated or read in other
//! parts of the program. Soon will have save functionality.
//!
//! # Planned changes:
//!
//! - Add save struct
//! - Probably add DataBuilder
//! - Remove objects attribute from Data struct.

use sdl2::keyboard::Keycode;

use crate::entity::Entity;
use crate::events;
use crate::helper::{self, Builder};
use crate::maps;
use crate::math;
use crate::quests;
// use crate::Screen;

/// A structure that stores data for tiny RPG.
///
/// Soon will have the ability to save.
pub struct Data {
    // current_screen: Screen,
    player: Entity,
    objects: Vec<Entity>,
    map: maps::Map,
    complete_quests: Vec<quests::Quest>,
    current_quests: Vec<quests::Quest>,
    settings: Settings,
    time: u32,
}

impl Data {
    /// Creates a new instance of Data.
    ///
    /// Does not use a build as the data is always
    /// changing anyways.
    pub fn new() -> Data {
        Data {
            // current_screen: Screen::MainMenu,
            player: Entity::new_player(5, 5),
            objects: Vec::new(),
            map: maps::Map::my_map(),
            complete_quests: Vec::new(),
            current_quests: Vec::new(),
            settings: Settings::new().build().unwrap(),
            time: 0,
        }
    }
    /// Takes a level and gets the entities from it.
    pub fn set_level(&mut self, level: maps::Level) {
        if self.map.change_level(level) {
            self.objects = self.get_level().get_level_entities().clone();
        }
    }
    /// Returns &Entity if it ia at position, else returns none.
    ///
    /// Function excludes player.
    pub fn check_position(&self, position: math::Rectangle) -> Option<&Entity> {
        for object in self.objects.iter() {
            if object.get_rectangle().is_colliding(position) {
                return Some(object);
            }
        }
        None
    }
    /// Returns a reference to the player if they are at position.
    pub fn check_position_player(&self, position: math::Rectangle) -> Option<&Entity> {
        if self.get_player().get_rectangle().is_not_colliding(position) {
            return None;
        }
        Some(self.get_player())
    }
    /// Returns &Entity if it ia at position, else returns none.
    ///
    /// This includes the player.
    pub fn check_position_both(&self, position: math::Rectangle) -> Option<&Entity> {
        match self.check_position_player(position) {
            Some(player) => Some(player),
            None => match self.check_position(position) {
                Some(entity) => Some(entity),
                None => None,
            },
        }
    }
    /// Sets the player to a given Entity.
    ///
    /// May want to make Data a builder just for this method.
    pub fn set_player(&mut self, player: Entity) {
        self.player = player;
    }
    /// Returns a reference to the player.
    pub fn get_player(&self) -> &Entity {
        &self.player
    }
    /// Returns a mutable reference to the player.
    pub fn get_mut_player(&mut self) -> &mut Entity {
        &mut self.player
    }
    /// Returns a reference to settings.
    pub fn get_settings(&self) -> &Settings {
        &self.settings
    }
    /// Adds a given Entity to the list of entities.
    // also have preconfig entites, maybe use and enum
    pub fn add_entity(&mut self, entity: Entity) {
        self.objects.push(entity);
    }
    /// Goes through the list of entities and have them perform
    /// their actions.
    pub fn entities_act(&mut self) {
        for _i in 0..self.objects.len() {
            let mut object = self.objects.remove(0);
            object.perform_turn(self);
            self.add_entity(object);
        }
    }
    /// Gets a reference to the entities in the level.
    pub fn get_entities(&self) -> &Vec<Entity> {
        &self.objects
    }
    /// Gets a mutable reference to the entities in the level.
    pub fn get_mut_entities(&mut self) -> &mut Vec<Entity> {
        &mut self.objects
    }
    /// Changes out entities in the level.
    ///
    /// Useful for room changes.
    pub fn change_entities(&mut self, entities: Vec<Entity>) {
        self.objects = entities;
    }
    /// Changes the time by one. Time is the turn counter.
    ///
    /// May rename increment.
    pub fn add_one_time(&mut self) {
        self.time += 1;
    }
    /// Gets the game time. Time is the turn counter.
    pub fn get_time(&self) -> u32 {
        self.time
    }
    /// Returns a reference to the level.
    pub fn get_level(&self) -> &maps::Level {
        &self.map.get_current_level()
    }
    /// Tries to give the program randomness based on game events.
    ///
    /// May replace with rand library.
    pub fn get_randomness(&self) -> f32 {
        let mut seed = (self.get_time() as i32 * 10 + self.get_level().get_level_number()) as f32;
        seed *= seed + self.get_time() as f32;
        while seed > 1.0 {
            seed /= 10.0;
        }
        seed
    }
    /// Same as randomness but with an added number to make things
    /// different.
    pub fn get_randomness_with(&self, number: i32) -> f32 {
        let mut seed = self.get_randomness() * (number * number) as f32;
        while seed > 1.0 {
            seed /= 10.0;
        }
        seed
    }
}

/// Builds the Settings enum.
///
/// Settings are things that can be changed in game or on the main menu.
pub struct SettingsBuilder {
    bindings: Vec<(Keycode, events::PlayerAction)>,
}

impl Builder for SettingsBuilder {
    type product = Settings;

    /// Creates a new instance of SettingsBuilder.
    fn new() -> SettingsBuilder {
        use events::PlayerAction::*;

        let bindings = vec![
            (Keycode::A, MoveLeft),
            (Keycode::D, MoveRight),
            (Keycode::S, MoveDown),
            (Keycode::W, MoveUp),
        ];

        SettingsBuilder { bindings }
    }
    /// Creates a instance of Settings from the SettingsBuilder.
    fn build(self) -> Result<Settings, helper::BuilderError> {
        let mut bindings = Vec::new();
        for (keycode, action) in self.bindings.iter() {
            bindings.push((*keycode, *action));
        }
        Ok(Settings { bindings })
    }
}

/// Settings is an enum that can be changed in game or on the main menu.
///
/// May make a binding struct or type alias.
pub struct Settings {
    bindings: Vec<(Keycode, events::PlayerAction)>,
}

impl Settings {
    /// Creates a instance of SettingsBuilder to help build an instance
    /// of Settings.
    pub fn new() -> SettingsBuilder {
        SettingsBuilder::new()
    }
    /// Returns a reference of the bindings attribute.
    ///
    /// The binding attribute is a tuple of Keycode followed by action.
    /// May be moved away from data to make the program less dependent
    /// on sdl2.
    pub fn get_bindings(&self) -> &Vec<(Keycode, events::PlayerAction)> {
        &self.bindings
    }
}
