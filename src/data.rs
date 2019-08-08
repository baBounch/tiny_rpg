use sdl2::keyboard::Keycode;

use crate::entity::Entity;
use crate::events;
use crate::helper::{self, Builder};
use crate::maps;
use crate::math;
use crate::quests;
use crate::Screen;

pub struct Data {
    current_screen: Screen,
    player: Entity,
    objects: Vec<Entity>,
    map: maps::Map,
    complete_quests: Vec<quests::Quest>,
    current_quests: Vec<quests::Quest>,
    settings: Settings,
    time: u32,
}

impl Data {
    pub fn new() -> Data {
        Data {
            current_screen: Screen::MainMenu,
            player: Entity::new_player(5, 5),
            objects: Vec::new(),
            map: maps::Map::my_map(),
            complete_quests: Vec::new(),
            current_quests: Vec::new(),
            settings: Settings::new().build().unwrap(),
            time: 0,
        }
    }
    pub fn set_level(&mut self, level: maps::Level) {
        if self.map.change_level(level) {
            self.objects = self.get_level().get_level_entities().clone();
        }
    }
    /// Returns &Entity if at position, else returns none
    pub fn check_position(&self, position: math::Rectangle) -> Option<&Entity> {
        for object in self.objects.iter() {
            if object.get_rectangle().is_colliding(position) {
                return Some(object);
            }
        }
        None
    }
    pub fn check_position_player(&self, position: math::Rectangle) -> Option<&Entity> {
        if self.get_player().get_rectangle().is_not_colliding(position) {
            return None;
        }
        Some(self.get_player())
    }
    pub fn check_position_both(&self, position: math::Rectangle) -> Option<&Entity> {
        match self.check_position_player(position) {
            Some(player) => Some(player),
            None => match self.check_position(position) {
                Some(entity) => Some(entity),
                None => None,
            },
        }
    }
    // create data builder, add this method
    pub fn set_player(&mut self, player: Entity) {
        self.player = player;
    }
    pub fn get_player(&self) -> &Entity {
        &self.player
    }
    pub fn get_mut_player(&mut self) -> &mut Entity {
        &mut self.player
    }
    pub fn get_settings(&self) -> &Settings {
        &self.settings
    }
    // also have preconfig entites, maybe use and enum
    pub fn add_entity(&mut self, entity: Entity) {
        self.objects.push(entity);
    }
    pub fn entities_act(&mut self) {
        for _i in 0..self.objects.len() {
            let mut object = self.objects.remove(0);
            object.perform_turn(self);
            self.add_entity(object);
        }
    }
    pub fn get_entities(&self) -> &Vec<Entity> {
        &self.objects
    }
    pub fn change_entities(&mut self, entities: Vec<Entity>) {
        self.objects = entities;
    }
    pub fn get_mut_entities(&mut self) -> &mut Vec<Entity> {
        &mut self.objects
    }
    pub fn add_one_time(&mut self) {
        self.time += 1;
    }
    pub fn get_time(&self) -> u32 {
        self.time
    }
    pub fn get_level(&self) -> &maps::Level {
        &self.map.get_current_level()
    }
    pub fn get_randomness(&self) -> f32 {
        let mut seed = (self.get_time() as i32 * 10 + self.get_level().get_level_number()) as f32;
        seed *= seed;
        while seed > 1.0 {
            seed /= 10.0;
        }
        seed
    }
    pub fn get_randomness_with(&self, number: i32) -> f32 {
        let mut seed = self.get_randomness() * (number * number) as f32;
        while seed > 1.0 {
            seed /= 10.0;
        }
        seed
    }
}

pub struct SettingsBuilder {
    bindings: Vec<(Keycode, events::PlayerAction)>,
}

impl Builder for SettingsBuilder {
    type product = Settings;

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
    fn build(self) -> Result<Settings, helper::BuilderError> {
        let mut bindings = Vec::new();
        for (keycode, action) in self.bindings.iter() {
            bindings.push((*keycode, *action));
        }
        Ok(Settings { bindings })
    }
}

pub struct Settings {
    bindings: Vec<(Keycode, events::PlayerAction)>,
}

impl Settings {
    pub fn new() -> SettingsBuilder {
        SettingsBuilder::new()
    }
    pub fn get_bindings(&self) -> &Vec<(Keycode, events::PlayerAction)> {
        &self.bindings
    }
}
