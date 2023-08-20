//! Hubo is back!

/// Types that represent a direction.
pub trait Direction {
    /// Get the direction in the form of a 2-dimensional vector.
    /// The resulting value doesn't have to be normalized.
    fn get_vector(&self) -> (f32, f32);
}

/// 4-way enum to indicate directions.
#[derive(Debug)]
pub enum Dir4 {
    /// +x direction
    Right,
    /// -x direction
    Left,
    /// +y direction
    Up,
    /// -y direction
    Down,
}

impl Direction for Dir4 {
    fn get_vector(&self) -> (f32, f32) {
        todo!()
    }
}

impl Direction for (f32, f32) {
    fn get_vector(&self) -> (f32, f32) {
        todo!()
    }
}

/// Hubo.
/// It's direction can be represented by an arbitrary type.
///
/// It can be controlled by [HuboController] only if the direction type implements the [Direction] trait.
#[derive(Debug)]
pub struct Hubo<TDir> {
    direction: TDir,
    x: f32,
    y: f32,
}

/// Controller of the Hubo
#[derive(Debug)]
pub struct HuboController<'s, TDir> {
    hubo: &'s mut Hubo<TDir>,
}

impl<TDir> Hubo<TDir> {
    /// Create a Hubo.
    pub fn new(direction: TDir, x: f32, y: f32) -> Self {
        Self { direction, x, y }
    }

    /// Return the current position of Hubo.
    pub fn get_position(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}

impl<'s, TDir: Direction> HuboController<'s, TDir> {
    /// Return the controller of the given Hubo.
    /// Note that the lifetime of hubo's mutable reference \['s\] is repeated in the return type.
    ///
    /// This represents that the controller cannot live longer than the mutable reference,
    /// since the controller takes and stores the reference.
    pub fn new(hubo: &'s mut Hubo<TDir>) -> HuboController<'s, TDir> {
        todo!()
    }

    /// Make Hubo move forward by the given distance. You might need to normalize the vector
    /// acquired from `Direction::get_move_vector`.
    pub fn move_hubo_forward(&mut self, distance: f32) {
        todo!()
    }

    /// Make Hubo turn to the given direction.
    pub fn set_hubo_direction(&mut self, dir: TDir) {
        todo!()
    }
}
