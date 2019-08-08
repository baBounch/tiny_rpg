//! Provides math based tools mostly related to 2 dimensions.

/// The TwoDimensional trait is for vectors that are in 2d space.
///
/// Maybe be renamed as two dimensional does not say a lot.
///
/// May change type to T in the future to ensure other traits.
pub trait TwoDimensional {
    /// A number type that represents what x and y are.
    type number;

    /// Returns an x axis value.
    fn get_x(&self) -> Self::number;
    /// Returns a y axis value.
    fn get_y(&self) -> Self::number;
    /// Returns the coordinates of x and y as a tuple.
    fn get_coordinates(&self) -> (Self::number, Self::number) {
        (self.get_x(), self.get_y())
    }
}

/// A Point in 2d space.
///
/// May be fused with Dimension to become Point<T> though risks
/// clarity.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    /// Creates a new instance of Point.
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
    /// Sets x to a new value.
    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }
    /// Sets y to a new value.
    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }
}

impl TwoDimensional for Point {
    type number = i32;

    fn get_x(&self) -> i32 {
        self.x
    }
    fn get_y(&self) -> i32 {
        self.y
    }
}

/// Dimension holds the width and height of a 2d object.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Dimension {
    width: u32,
    height: u32,
}

impl Dimension {
    /// Creates a new instance of Dimension.
    pub fn new(width: u32, height: u32) -> Dimension {
        Dimension { width, height }
    }
    /// Gets the height of Dimension.
    pub fn get_height(&self) -> u32 {
        self.height
    }
    /// Gets the width of Dimension.
    pub fn get_width(&self) -> u32 {
        self.width
    }
    /// Sets the width of Dimension.
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }
    /// Sets the height of Dimension.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
    }
}

impl TwoDimensional for Dimension {
    type number = u32;

    /// Gets the width of Dimension.
    fn get_x(&self) -> u32 {
        self.get_width()
    }
    /// Gets the height of Dimension.
    fn get_y(&self) -> u32 {
        self.get_height()
    }
}

/// A Rectangle holds the position and size of an object.
///
/// While, the size can be anything, probably best to not use it as
/// pixels and have it multiplied by pixels for rendering; instead
/// making its value related to any logic or other objects that use
/// or are a Rectangle struct.
///
/// A lot of its methods should be given to Point and Dimension too.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Rectangle {
    point: Point,
    dimension: Dimension,
}

impl Rectangle {
    /// Creates a new instance of Rectangle.
    ///
    /// Likely add a second version with Point and Dimension parameters.
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Rectangle {
        Rectangle {
            point: Point::new(x, y),
            dimension: Dimension::new(width, height),
        }
    }
    /// Returns the position of the rectangle as a Point.
    ///
    /// The position is not the center of the rectangle but one of the corners.
    /// Visually, the top left is the usual corner.
    pub fn get_point(&self) -> Point {
        self.point
    }
    /// Returns the size of the rectangle as a dimension.
    pub fn get_dimension(&self) -> Dimension {
        self.dimension
    }
    /// Changes the values of x by the given amount.
    pub fn move_x(&mut self, amount_x: i32) {
        self.point.set_x(amount_x + self.get_point().get_x());
    }
    /// Changes the value of y by the given amount.
    pub fn move_y(&mut self, amount_y: i32) {
        self.point.set_y(amount_y + self.get_point().get_y());
    }
    /// Changes the values of x and y by the values of a Point.
    pub fn move_xy(&mut self, amount: Point) {
        self.move_x(amount.get_x());
        self.move_y(amount.get_y());
    }
    /// Returns the x position of the Rectangle.
    pub fn x(&self) -> i32 {
        self.get_point().get_x()
    }
    /// Returns the y position of the Rectangle.
    pub fn y(&self) -> i32 {
        self.get_point().get_y()
    }
    /// Returns the width of the rectangle.
    pub fn width(&self) -> u32 {
        self.get_dimension().get_width()
    }
    /// Returns the height of the rectangle.
    pub fn height(&self) -> u32 {
        self.get_dimension().get_height()
    }
    /// Returns true if the Rectangle is colliding with the given other Rectangle.
    /// Else returns false.
    // create collision enum
    pub fn is_colliding(&self, other_rectangle: Rectangle) -> bool {
        if self.y_max() < other_rectangle.y_min() {
            return false;
        }
        if self.y_min() > other_rectangle.y_max() {
            return false;
        }
        if self.x_max() >= other_rectangle.x_min() && self.x_min() <= other_rectangle.x_max() {
            return true;
        }
        false
    }
    /// Returns false if the Rectangle is colliding with the given other Rectangle.
    /// Else returns true.
    pub fn is_not_colliding(&self, other_rectangle: Rectangle) -> bool {
        !self.is_colliding(other_rectangle)
    }
    /// Returns the verticle value of the opposite corner of the rectangle from y.
    ///
    /// Basicly y but accounting for height.
    pub fn y_max(&self) -> i32 {
        self.y() + self.height() as i32 - 1
    }
    /// Returns the horizontal value of the opposite corner of the rectangle from x.
    ///
    /// Basic x but accounting for width.
    pub fn x_max(&self) -> i32 {
        self.x() + self.width() as i32 - 1
    }
    /// Returns y.
    pub fn y_min(&self) -> i32 {
        self.y()
    }
    /// Returns x.
    pub fn x_min(&self) -> i32 {
        self.x()
    }
}
