pub trait TwoDimensional {
    type number;

    fn get_x(&self) -> Self::number;
    fn get_y(&self) -> Self::number;
    fn get_coordinates(&self) -> (Self::number, Self::number) {
        (self.get_x(), self.get_y())
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Dimension {
    width: u32,
    height: u32,
}

impl Dimension {
    pub fn new(width: u32, height: u32) -> Dimension {
        Dimension { width, height }
    }
    pub fn get_height(&self) -> u32 {
        self.height
    }
    pub fn get_width(&self) -> u32 {
        self.width
    }
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
    }
}

impl TwoDimensional for Dimension {
    type number = u32;

    fn get_x(&self) -> u32 {
        self.get_width()
    }
    fn get_y(&self) -> u32 {
        self.get_height()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Rectangle {
    point: Point,
    dimension: Dimension,
}

impl Rectangle {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Rectangle {
        Rectangle {
            point: Point::new(x, y),
            dimension: Dimension::new(width, height),
        }
    }
    pub fn get_point(&self) -> Point {
        self.point
    }
    pub fn get_dimension(&self) -> Dimension {
        self.dimension
    }
    pub fn move_x(&mut self, amount_x: i32) {
        self.point.set_x(amount_x + self.get_point().get_x());
    }
    pub fn move_y(&mut self, amount_y: i32) {
        self.point.set_y(amount_y + self.get_point().get_y());
    }
    pub fn move_xy(&mut self, amount: Point) {
        self.move_x(amount.get_x());
        self.move_y(amount.get_y());
    }
    pub fn x(&self) -> i32 {
        self.get_point().get_x()
    }
    pub fn y(&self) -> i32 {
        self.get_point().get_y()
    }
    pub fn width(&self) -> u32 {
        self.get_dimension().get_width()
    }
    pub fn height(&self) -> u32 {
        self.get_dimension().get_height()
    }
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
    pub fn is_not_colliding(&self, other_rectangle: Rectangle) -> bool {
        !self.is_colliding(other_rectangle)
    }
    pub fn y_max(&self) -> i32 {
        self.y() + self.height() as i32 - 1
    }
    pub fn x_max(&self) -> i32 {
        self.x() + self.width() as i32 - 1
    }
    pub fn y_min(&self) -> i32 {
        self.y()
    }
    pub fn x_min(&self) -> i32 {
        self.x()
    }
}
