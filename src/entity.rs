//! A group of structures for creating a very general entity.
//! The entitiy should be able to be anything, walls, zones,
//! enemies, etc.
//!
//! While it has a builder, the entity itself has a few premade
//! entities for things such as walls.
//!
//! # Planned changes:
//!
//! - Implement warp
//! - Give entities unique identifiers and type names.
//! - Turn into a directory
//! - Fill in empty structs
//! - Find system for abilities with counters
//! - Create LevelPoint of some kind or Rc and / or RefCell

use crate::data::Data;
use crate::helper::{Builder, BuilderError};
use crate::maps::Level;
use crate::math::{Dimension, Point, Rectangle, TwoDimensional};

/// Holds any currencies in the game. For your own games, it
/// wouldn't be a bad idea to create your own struct if there
/// is any currency you would like to add.
#[derive(Debug, Clone, PartialEq)]
struct Currencies {}

/// Inventory holds currency, along with a bunch of items. The
/// Items will probably be entities with an item ability.
#[derive(Debug, Clone, PartialEq)]
struct Inventory {}

/// The stats on an entity. Will have a bunch of stuff including,
/// strength, etc. along with currencies, may become a trait,
/// That other things take T: impl Stats to allow users to
/// Create their own.
#[derive(Debug, Clone, PartialEq)]
struct Stats {}

/// A structure that holds a bunch of abilities that entities can
/// perform. These abilities need to be checked for somewhere. Again
/// like stats, may become a trait.
#[derive(Debug, PartialEq, Clone)]
pub struct Abilities(Vec<Ability>);

impl Abilities {
    /// Create a new instance of Abilities with no abilities.
    pub fn new() -> Abilities {
        Abilities(Vec::new())
    }
    /// Checks through Abilities list of abilities and returns
    /// true if Abilities has the ability and false if not.
    fn check_for_ability(&self, ability: &Ability) -> bool {
        for owned_ability in self.0.iter() {
            if ability == owned_ability {
                return true;
            }
        }
        false
    }
    /// Checks if the Ability Warp exists and if so returns
    /// The data of where something is supposed to teleport.
    /// Else returns None.
    // might convert is warp to i32 instead of level
    pub fn is_warp(&self) -> Option<(Level, Point)> {
        for owned_ability in self.0.iter() {
            if let Ability::Warp(l, p) = owned_ability.clone() {
                return Some((l, p));
            }
        }
        None
    }
    /// Checks if Ability list includes speaking.
    pub fn can_speak(&self) -> bool {
        if !self.check_for_ability(&Ability::Speak) {
            return false;
        }
        true
    }
    /// Checks if Ability list does not include speaking.
    pub fn can_not_speak(&self) -> bool {
        !self.can_speak()
    }
    /// Checks if Ability list includes moving.
    pub fn can_move(&self) -> bool {
        if !self.check_for_ability(&Ability::Move) {
            return false;
        }
        true
    }
    /// Checks if Ability list does not include moving.
    pub fn can_not_move(&self) -> bool {
        !self.can_move()
    }
    /// Add an ability to the list of Abilities.
    pub fn add_ability(&mut self, ability: Ability) {
        if !self.check_for_ability(&ability) {
            self.0.push(ability);
        }
    }
}

/// A list of abilities, may become a trait though
/// Not completely likely.
///
/// Entities should be explicit of
/// what they can exactly do so game logic can react
/// appropriately, this can be as simple as moving or
/// as complex as summoning other entities.
/// May get reworked to allow copy.
///
/// May also be split into a struct and enum to allow more
/// specific abilities. such as adding AbilityProperties
/// and AbilityType.
///
/// Note to self: Could have some preloaded entities not
/// in the game directly for summoning.
#[derive(Debug, Clone, PartialEq)]
pub enum Ability {
    /// Can the Entity move? Walls can't, the player and entities can.
    Move,
    /// Can the Entity speak? Should be for things such as communicating
    /// with the player.
    Speak,
    /// Interacting with this entity warps targets to another place.
    Warp(Level, Point),
    /// You do not go through the object when interacting.
    Physical,
    /// Can clone itself
    Clone,
    /// Lasts u32 turns.
    Temporary(u32),
}

/// Sets alignment, used with Option for neutral. Can
/// probably be a trait, either way helps people know
/// who the allies and villans are. May add other factions
/// down the road.
#[derive(Debug, Clone, PartialEq)]
enum Alignment {
    /// The player should probably be set to good. Good isn't
    /// good per-se just good for the player.
    Good,
    /// Evil should have logic to attack the player or bother
    /// them. Almost feel this could be turned into an ability.
    Evil,
}

/// The possible skins in the game, again could be a trait.
/// For now only has RGB with plans for RGBA very soon.
// future includes rgba and images
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Skin {
    /// RGB rectangle. uses a tuple of (r, g, b) to create
    /// rectangles with colour.
    RGB(u8, u8, u8),
}

/// Implementations of Skin related to rgb and rgba.
impl Skin {
    /// Gets the rgb value if it is rgb, otherwise returns an error.
    pub fn get_rgb(&self) -> Result<Skin, &'static str> {
        match self {
            Skin::RGB(r, g, b) => Ok(Skin::RGB(*r, *g, *b)),
            // _ => Err("Skin enum value not an RGB value."),
        }
    }
    /// Just gets the pure values from the rgb as a tuple instead of surrounding
    /// it in a skin like get_rgb. Again, results in an error if not rgb.
    pub fn get_rgb_colors(&self) -> Result<(u8, u8, u8), &'static str> {
        match self {
            Skin::RGB(r, g, b) => Ok((*r, *g, *b)),
            // _ => Err("Skin enum value not an RGB value."),
        }
    }
}

/// Builds an instance of Entity. Requires a position to be set, in order
/// to place the entity. A lot of premade Entities exist in Entity, they
/// don't use the builder for now, but soon should.
pub struct EntityBuilder {
    position: Option<Point>,
    size: Dimension,
    skin: Skin,
    stats: Stats,
    abilities: Abilities,
    inventory: Option<Inventory>,
    alignment: Option<Alignment>,
}

impl EntityBuilder {
    /// Set the position of an Entity using a Point. Position is the top left
    /// of an object. lower values are upward, higher values are downward for y.
    /// Lower values are left, higher values are right for y.
    pub fn set_position(mut self, point: Point) -> Self {
        self.position = Some(point);
        self
    }
    /// Same as set_position except instead of using a Point, you can simply
    /// use x and y.
    pub fn set_xy(mut self, x: i32, y: i32) -> Self {
        self.position = Some(Point::new(x, y));
        self
    }
    /// Set the width and height of an entity by a Dimension.
    pub fn set_dimension(mut self, dimension: Dimension) -> Self {
        self.size = dimension;
        self
    }
    /// Set the width and height of an entity using u32 values.
    pub fn set_size(mut self, width: u32, height: u32) -> Self {
        self.size = Dimension::new(width, height);
        self
    }
    /// Give the entity builder a vector of abilities to have.
    pub fn set_abilities(mut self, abilities: Vec<Ability>) -> Self {
        for ability in abilities.into_iter() {
            self.abilities.add_ability(ability);
        }
        self
    }
}

impl Builder for EntityBuilder {
    type product = Entity;

    /// Creates a new instance of EntityBuilder with some defaults. Still need to
    /// specify a position.
    fn new() -> EntityBuilder {
        EntityBuilder {
            position: None,
            size: Dimension::new(1, 1),
            skin: Skin::RGB(255, 255, 0),
            inventory: None,
            stats: Stats {},
            abilities: Abilities(Vec::new()),
            alignment: None,
        }
    }
    /// Creates an instance of Entity from EntityBuilder.
    fn build(self) -> Result<Entity, BuilderError> {
        if self.position.is_none() {
            return Err(BuilderError::new(
                "Could not build Entity, no position set. use set_position or set_xy methods to set position.",
            ));
        }
        let placement = Rectangle::new(
            self.position.unwrap().get_x(),
            self.position.unwrap().get_y(),
            self.size.get_x(),
            self.size.get_y(),
        );
        Ok(Entity {
            placement,
            skin: self.skin,
            inventory: self.inventory,
            stats: self.stats,
            abilities: self.abilities,
            alignment: self.alignment,
        })
    }
}

/// An Entity can be nearly anything. A field, a wall, a creature. It has
/// values to allow interactions to intermingle. Maybe walls can be teleported,
/// or broken down. Maybe someone can capture a creature and use it later on.
#[derive(Clone, PartialEq, Debug)]
pub struct Entity {
    placement: Rectangle,
    skin: Skin,
    stats: Stats,
    abilities: Abilities,
    inventory: Option<Inventory>,
    alignment: Option<Alignment>,
}

/// Implementations for Entity that create standard default entities, such as,
/// - Player
/// - Specific  Enemies
/// - Walls
impl Entity {
    /// Creates a good default player. Asks for a position.
    pub fn new_player(x: i32, y: i32) -> Entity {
        Entity {
            placement: Rectangle::new(x, y, 1, 1),
            skin: Skin::RGB(255, 0, 0),
            inventory: None,
            stats: Stats {},
            abilities: Abilities(Vec::new()),
            alignment: Some(Alignment::Good),
        }
    }
    /// Creates an evil slimer entity.
    pub fn new_slimer_entity(x: i32, y: i32) -> Entity {
        Entity {
            placement: Rectangle::new(x, y, 1, 1),
            skin: Skin::RGB(30, 215, 30),
            inventory: None,
            stats: Stats {},
            abilities: Abilities(Vec::new()),
            alignment: Some(Alignment::Evil),
        }
    }
    /// Creates a wall where you specify height, width and position.
    pub fn new_wall(x: i32, y: i32, height: u32, width: u32) -> Entity {
        Entity {
            placement: Rectangle::new(x, y, height, width),
            skin: Skin::RGB(90, 90, 90),
            inventory: None,
            stats: Stats {},
            abilities: Abilities(Vec::new()),
            alignment: None,
        }
    }
}

/// Implementations of movement for the entity.
impl Entity {
    /// Returns a rectangle of the entity if it moved by movement amount.
    pub fn get_move(&self, movement: Point) -> Rectangle {
        let mut new_rect = self.placement;
        new_rect.move_xy(movement);
        new_rect
    }
    /// Returns movement if the entity moved 1 left.
    pub fn get_move_left(&self) -> Rectangle {
        self.get_move(Point::new(-1, 0))
    }
    /// Returns movement if the entity moved 1 right.
    pub fn get_move_right(&self) -> Rectangle {
        self.get_move(Point::new(1, 0))
    }
    /// Returns movement if the entity moved 1 up.
    pub fn get_move_up(&self) -> Rectangle {
        self.get_move(Point::new(0, -1))
    }
    /// Returns movement if the entity moved 1 down.
    pub fn get_move_down(&self) -> Rectangle {
        self.get_move(Point::new(0, 1))
    }
    /// Moves the entity one left. Should usually be used with get_move_left first.
    /// May be removed in future versions.
    // move will soon be based on stats and validated with amount
    pub fn move_left(&mut self) {
        self.placement.move_x(-1);
    }
    /// Moves the entity one right. Should usually be used with get_move_right first.
    /// May be removed in future versions.
    pub fn move_right(&mut self) {
        self.placement.move_x(1);
    }
    /// Moves the entity one up. Should usually be used with get_move_up first.
    /// May be removed in future versions.
    pub fn move_up(&mut self) {
        self.placement.move_y(-1);
    }
    /// Moves the entity one down. Should usually be used with get_move_down first.
    /// May be removed in future versions.
    pub fn move_down(&mut self) {
        self.placement.move_y(1);
    }
}

impl Entity {
    /// Creates an instance of EntityBuilder to start building and Entity.
    pub fn new() -> EntityBuilder {
        EntityBuilder::new()
    }
    /// Returns the current position of the entity.
    pub fn get_position(&self) -> Point {
        self.placement.get_point()
    }
    /// Returns the size of the entity.
    pub fn get_size(&self) -> Dimension {
        self.placement.get_dimension()
    }
    /// Returns both the size and position of an entity using a Rectangle.
    pub fn get_rectangle(&self) -> Rectangle {
        self.placement
    }
    /// Returns the skin of the entity.
    pub fn get_skin(&self) -> Skin {
        self.skin
    }
    /// Returns a reference to the abilities of an entity.
    pub fn get_abilities(&self) -> &Abilities {
        &self.abilities
    }
}

/// AI logic here. Should only need to call perform_turn for the object to do what it thinks is best. Almost
/// certainly will become a trait.
impl Entity {
    /// The entity performs an action based on its environment.
    pub fn perform_turn(&mut self, data: &mut Data) {
        if self.abilities.can_move() {
            let new_position = match data
                .get_randomness_with(self.get_rectangle().x() * self.get_rectangle().y())
            {
                v if v > 0.75 => self.get_move_right(),
                v if v > 0.5 => self.get_move_left(),
                v if v > 0.25 => self.get_move_up(),
                _v => self.get_move_down(),
            };
            self.move_direction(data, new_position);
        }
    }
    fn move_direction(&mut self, data: &mut Data, new_rectangle: Rectangle) {
        match data.check_position_both(new_rectangle) {
            Some(_e) => {}
            None => self.placement = new_rectangle,
        };
    }
}

/// Another builder for an entity that builds an Evil entity.
pub struct EnemyBuilder {}
