//! Things related to levels and the world map go here.
//!
//! May move fully designed worlds and levels to level.rs
//!
//! # Planned changes:
//!
//! - Make a premade map with multiples levels.
//! - Make a map builder.
//! - Find some way to better refer to levels.

use crate::entity::{self, Entity};
use crate::helper::Builder;
use crate::helper::BuilderError;
use crate::math;

/// A map holds the levels of the game.
///
/// Different maps will be able to be loaded up in the future.
pub struct Map {
    current_level: Level,
    modified_levels: Vec<Level>,
}

/// Implementations of Map that auto create a campain. May get moved to level.rs or
/// cause map.rs to become a directory.
impl Map {
    /// My first map. Creates an instance of Map that is a set of test levels.
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

/// Implementations that revolve around getting, setting and creating levels.
impl Map {
    /// Adds a game level to the map and fills it with the given entities. Tries to find a
    /// level number that isn't taken.
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
        )
        .unwrap();
    }
    /// Gets a reference to the current level.
    pub fn get_current_level(&self) -> &Level {
        &self.current_level
    }
    /// Adds a given level. Use over add_game_level if you want something specific or a
    /// Menu.
    pub fn add_level(&mut self, level: Level) -> Result<(), &'static str> {
        if self.is_modified(&level) {
            return Err("The Level you are trying to add already exists.");
        }
        self.modified_levels.push(level);
        Ok(())
    }
    /// Changes the level with the given one. If the current level is not in the modified or prebuilt list
    /// of levels, it adds it there.
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
}

impl Map {
    /// An associated function that returns 4 wall entities to use as level borders that
    /// can be appended to levels entities.
    pub fn get_boundry_entities(boundry: math::Rectangle) -> Vec<Entity> {
        vec![
            Entity::new_wall(boundry.x(), boundry.y(), boundry.width(), 1),
            Entity::new_wall(boundry.x(), boundry.y() + 1, 1, boundry.height() - 2),
            Entity::new_wall(boundry.x_max(), boundry.y() + 1, 1, boundry.height() - 2),
            Entity::new_wall(boundry.x(), boundry.y_max(), boundry.width(), 1),
        ]
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

/// Builds a instance of Level. Requires a set number and entities list.
///
/// Defaults a game LevelType.
pub struct LevelBuilder {
    level_type: LevelType,
    number: Option<i32>,
    entities: Option<Vec<Entity>>,
}

impl LevelBuilder {
    /// Sets the LevelType of the Level. Default is LevelType::Game.
    pub fn set_level_type(mut self, level_type: LevelType) -> Self {
        self.level_type = level_type;
        self
    }
    /// Short hand for set_level_type. Sets the level type.
    pub fn level_type(self, level_type: LevelType) -> Self {
        self.set_level_type(level_type)
    }
    /// Sets the number of the Level. Required to build.
    ///
    /// Level, along with LevelType, gives the level a unique
    /// identifier and gives an order to the games levels.
    pub fn set_number(mut self, number: i32) -> Self {
        self.number = Some(number);
        self
    }
    /// Short hand for set_number. Sets the number.
    pub fn number(self, number: i32) -> Self {
        self.set_number(number)
    }
    /// Sets the list of entities. Required to build.
    ///
    /// Very few levels will require no entities. Suggest adding get_boundry_entities
    /// associated function from the Map struct. May add it to LevelBuilder in the
    /// future as a requirement.
    pub fn set_entities(mut self, entities: Vec<Entity>) -> Self {
        self.entities = Some(entities);
        self
    }
    /// Short hand for set_entities. Sets the entities vector.
    pub fn entities(self, entities: Vec<Entity>) -> Self {
        self.set_entities(entities)
    }
}

impl Builder for LevelBuilder {
    type product = Level;

    /// Creates an instance of LevelBuilder.
    fn new() -> LevelBuilder {
        LevelBuilder {
            level_type: LevelType::Game,
            number: None,
            entities: None,
        }
    }
    /// Builds an instance of Level from LevelBuilder.
    ///
    /// Returns an error if the level number or entities were not set.
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

/// Represents a level.
///
/// A level is simply a LevelType such as, game or menu; a level number that
/// is unique for the level type; and a list of the entities that fill the levels
/// map.
#[derive(Clone, Debug)]
pub struct Level {
    level_type: LevelType,
    level_number: i32,
    level_entities: Vec<Entity>,
}

impl Level {
    /// Creates a instance of LevelBuilder to help build a instance of Level.
    pub fn new() -> LevelBuilder {
        LevelBuilder::new()
    }
    /// Returns the level number.
    pub fn get_level_number(&self) -> i32 {
        self.level_number
    }
    /// Returns the level type.
    pub fn get_level_type(&self) -> LevelType {
        self.level_type
    }
    /// Returns a reference to the level's vector of entities.
    pub fn get_level_entities(&self) -> &Vec<Entity> {
        &self.level_entities
    }
}

impl PartialEq for Level {
    /// Checks the level type and number for equality. The entities are ignored.
    fn eq(&self, other: &Self) -> bool {
        self.get_level_type() == other.get_level_type()
            && self.get_level_number() == other.get_level_number()
    }
}

/// The LevelType of the game. Used mostly to help keep the game levels ordered
/// without having to dictate menus or other things a level numbers.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum LevelType {
    /// A MainMenu of the game. May be changed to Menu.
    MainMenu,
    /// A Settings menu of the game. May be merged with MainMenu.
    Settings,
    /// A level in the game. Intended to be played on with quests,
    /// enemies and more.
    Game,
}
