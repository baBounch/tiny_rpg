use crate::entity::{self, Entity};
use crate::helper::Builder;
use crate::helper::BuilderError;
use crate::math;

pub struct Map {
    current_level: Level,
    modified_levels: Vec<Level>,
}

impl Map {
    pub fn my_map() -> Map {
        let mut first_entities = vec![
            Entity::new()
                .set_xy(7, 7)
                .set_abilities(vec![entity::Ability::Move])
                .build()
                .unwrap(),
            Entity::new()
                .set_xy(9, 9)
                .set_abilities(vec![entity::Ability::Move])
                .build()
                .unwrap(),
        ];
        first_entities.append(&mut Map::get_boundry_entities(math::Rectangle::new(
            0, 0, 50, 50,
        )));
        Map {
            current_level: Level::new()
                .level_type(LevelType::MainMenu)
                .number(1)
                .entities(first_entities)
                .build()
                .unwrap(),
            modified_levels: Vec::new(),
        }
    }
}

impl Map {
    pub fn add_game_level(&mut self, entities: Vec<Entity>) {
        let mut counter = 1;
        let mut is_continued = true;
        while is_continued {
            is_continued = false;
            for level in self.modified_levels.iter() {
                if level.get_level_type() == LevelType::Game {
                    if level.get_level_number() == counter {
                        is_continued = true;
                        counter += 1;
                    }
                }
            }
        }
        self.add_level(
            Level::new()
                .number(0)
                .level_type(LevelType::Game)
                .entities(entities)
                .build()
                .unwrap(),
        );
    }
    pub fn get_current_level(&self) -> &Level {
        &self.current_level
    }
    pub fn get_boundry_entities(boundry: math::Rectangle) -> Vec<Entity> {
        vec![
            Entity::new_wall(boundry.x(), boundry.y(), boundry.width(), 1),
            Entity::new_wall(boundry.x(), boundry.y() + 1, 1, boundry.height() - 2),
            Entity::new_wall(boundry.x_max(), boundry.y() + 1, 1, boundry.height() - 2),
            Entity::new_wall(boundry.x(), boundry.y_max(), boundry.width(), 1),
        ]
    }
    pub fn add_level(&mut self, level: Level) -> Result<(), &'static str> {
        if self.is_modified(&level) {
            return Err("The Level you are trying to add already exists.");
        }
        self.modified_levels.push(level);
        Ok(())
    }
    pub fn change_level(&mut self, level: Level) -> bool {
        if level == self.current_level {
            return false;
        }
        if self.is_not_modified(&self.current_level) {
            self.modified_levels.push(self.current_level.clone());
        } else {
            for modified_level in self.modified_levels.iter_mut() {
                if &mut self.current_level == modified_level {
                    modified_level.level_entities = Vec::new();
                    modified_level
                        .level_entities
                        .append(&mut self.current_level.level_entities);
                    break;
                }
            }
        }
        self.current_level = level;
        true
    }
    fn is_modified(&self, level: &Level) -> bool {
        for modified_level in self.modified_levels.iter() {
            if level == modified_level {
                return true;
            }
        }
        return false;
    }
    fn is_not_modified(&self, level: &Level) -> bool {
        !self.is_modified(level)
    }
    fn move_current_to_modified(&mut self) {
        if self.is_modified(&self.current_level) {
            return ();
        }
        self.modified_levels.push(self.current_level.clone());
    }
}

pub struct LevelBuilder {
    level_type: LevelType,
    number: Option<i32>,
    entities: Option<Vec<Entity>>,
}

impl LevelBuilder {
    pub fn set_level_type(mut self, level_type: LevelType) -> Self {
        self.level_type = level_type;
        self
    }
    pub fn level_type(self, level_type: LevelType) -> Self {
        self.set_level_type(level_type)
    }
    pub fn set_number(mut self, number: i32) -> Self {
        self.number = Some(number);
        self
    }
    pub fn number(self, number: i32) -> Self {
        self.set_number(number)
    }
    pub fn set_entities(mut self, entities: Vec<Entity>) -> Self {
        self.entities = Some(entities);
        self
    }
    pub fn entities(self, entities: Vec<Entity>) -> Self {
        self.set_entities(entities)
    }
}

impl Builder for LevelBuilder {
    type product = Level;

    fn new() -> LevelBuilder {
        LevelBuilder {
            level_type: LevelType::Game,
            number: None,
            entities: None,
        }
    }
    fn build(self) -> Result<Level, BuilderError> {
        if self.number.is_none() {
            return Err(BuilderError::new(
                "Need to include a level number, use set_level_number method.",
            ));
        }
        if self.entities.is_none() {
            return Err(BuilderError::new(
                "Need to add entities, use set_entities method.",
            ));
        }
        Ok(Level {
            level_type: self.level_type,
            level_number: self.number.unwrap(),
            level_entities: self.entities.unwrap(),
        })
    }
}

#[derive(Clone, Debug)]
pub struct Level {
    level_type: LevelType,
    level_number: i32,
    level_entities: Vec<Entity>,
}

impl Level {
    pub fn new() -> LevelBuilder {
        LevelBuilder::new()
    }
    pub fn get_level_number(&self) -> i32 {
        self.level_number
    }
    pub fn get_level_type(&self) -> LevelType {
        self.level_type
    }
    pub fn get_level_entities(&self) -> &Vec<Entity> {
        &self.level_entities
    }
}

impl PartialEq for Level {
    fn eq(&self, other: &Self) -> bool {
        self.get_level_type() == other.get_level_type()
            && self.get_level_number() == other.get_level_number()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum LevelType {
    MainMenu,
    Settings,
    Game,
}
