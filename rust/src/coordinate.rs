//! Spacial coordinates to do computations on

/// The directions which things can move in
///
/// This should be understood in the context of a coordinate system
/// where the y-axis points down and the x-axis points right.
#[derive(Clone, Copy)]
pub enum Direction {
    /// Toward the side of the screen in which blocks of text begin
    Up,
    /// Toward the side of the screen where lines of text end (or begin
    /// for Hebrew and Arabic readers)
    Left,
    /// Toward the side of the screen in which blocks of text end
    Down,
    /// Toward the side of the screen where lines of text begin (or end
    /// for Hebrew and Arabic readers)
    Right,
}

/// A 2D unsigned integer coordinate
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct I2 {
    x: i32,
    y: i32,
}

impl I2 {
    /// Create a coordinate with your horizontal and vertical positions
    ///
    /// The horizontal coordinate, `x`, is understood to be positive to
    /// the right of the origin and the vertical coordinate, `y`, is
    /// _down_ from the origin.  This is upside-down from how you
    /// learned it in grade school, but is common in CG, game dev, and
    /// in some cases in engineering.
    pub fn new(x: i32, y: i32) -> Self {
        I2 { x, y }
    }

    /// Get the horizontal coordinate, which is positive rightward
    pub fn x(&self) -> i32 {
        self.x
    }

    /// Get the vertical coordinate, which is positive downward
    pub fn y(&self) -> i32 {
        self.y
    }

    /// Calculate the coordinate that is one unit away in `direction`
    ///
    /// If the nudge would cause an integer over- or under-flow, then
    /// this will return `None`.
    ///
    /// Recall that the vertical coordinate is positive toward
    /// [`Direction::Down`].
    pub fn nudge(&self, direction: Direction) -> Option<Self> {
        self.nudge_by(1, direction)
    }

    /// Calculate the coordinate that is `n` units away in `direction`
    ///
    /// If the nudge would cause an integer over- or under-flow, then
    /// this will return `None`.
    ///
    /// Recall that the vertical coordinate is positive toward
    /// [`Direction::Down`].
    pub fn nudge_by(&self, n: i32, direction: Direction) -> Option<Self> {
        match direction {
            Direction::Up => self.y.checked_sub(n).map(|y| I2::new(self.x, y)),
            Direction::Left => self.x.checked_sub(n).map(|x| I2::new(x, self.y)),
            Direction::Down => self.y.checked_add(n).map(|y| I2::new(self.x, y)),
            Direction::Right => self.x.checked_add(n).map(|x| I2::new(x, self.y)),
        }
    }
}

impl From<godot::builtin::Vector2i> for I2 {
    fn from(vector2: godot::builtin::Vector2i) -> Self {
        I2::new(vector2.x, vector2.y)
    }
}

impl Into<godot::builtin::Vector2i> for I2 {
    fn into(self) -> godot::builtin::Vector2i {
        godot::builtin::Vector2i::new(self.x.into(), self.y.into())
    }
}

impl TryFrom<godot::prelude::Variant> for I2 {
    type Error = &'static str;

    /// Performs the conversion assuming `variant` is a [`godot::core_types::Vector2i`]
    ///
    /// Should it not be a `Vector2i` or the `Vector2i` is invalid as
    /// [`TryFrom<godot::core_types::Vector2i>`] understands it, then
    /// this will `Err`.
    fn try_from(variant: godot::prelude::Variant) -> Result<Self, Self::Error> {
        if variant.get_type() != godot::builtin::VariantType::Vector2i {
            return Err("Not a Vector2i");
        }
        Ok(I2::from(variant.to::<godot::builtin::Vector2i>()))
    }
}

/// An array of [`I2`] coordinates
#[derive(Debug, PartialEq, Clone)]
pub struct I2Array(Vec<I2>);

impl I2Array {
    /// Returns an iterator over the container
    pub fn iter(&self) -> std::slice::Iter<'_, I2> {
        self.0.iter()
    }

    /// Returns `true` if the coordinate is in the array
    pub fn contains(&self, coordinate: &I2) -> bool {
        self.0.contains(coordinate)
    }

    /// Appends a coordinate to the back of the array
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds [`isize::MAX`] bytes.
    pub fn push(&mut self, coordinate: I2) {
        self.0.push(coordinate);
    }
}

impl FromIterator<I2> for I2Array {
    fn from_iter<I: IntoIterator<Item = I2>>(iter: I) -> Self {
        let mut coordinate_vector: Vec<I2> = vec![];

        for coordinate in iter {
            coordinate_vector.push(coordinate);
        }

        I2Array(coordinate_vector)
    }
}

impl From<Vec<[i32; 2]>> for I2Array {
    /// Constructs a new instance from a vector of [`I2`]-shaped arrays
    ///
    /// This is just a developer convenience to save some extra typing.
    ///
    /// # Examples
    ///
    /// ```
    /// let coords: I2Array = I2Array::from(vec![[0, 0], [1, 10], [115, 6]]);
    /// ```
    fn from(coordinates: Vec<[i32; 2]>) -> Self {
        I2Array(
            coordinates
                .iter()
                .map(|coordinate| I2::new(coordinate[0], coordinate[1]))
                .collect(),
        )
    }
}

impl TryFrom<godot::prelude::Array<godot::builtin::Vector2i>> for I2Array {
    type Error = &'static str;

    /// Converts a [`godot::prelude::Array<Vector2i>`] of [`godot::builtin::Vector2i`]s
    ///
    /// Should any of the variants in the array not be a `Vector2i`, then
    /// this will return an error.
    ///
    /// If any of the given `Vector2i`s would cause
    /// [`I2::try_from<godot::builtin::Vector2i>`] to error, then
    /// they will be silently omitted from the output.
    fn try_from(
        vector2_array: godot::prelude::Array<godot::builtin::Vector2i>,
    ) -> Result<Self, Self::Error> {
        Ok(vector2_array
            .iter_shared()
            .filter_map(|variant| I2::try_from(variant).ok())
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod u2 {
        use super::*;

        #[test]
        fn can_get_coordinates() {
            let coord: I2 = I2::new(0, 1);
            assert_eq!(coord.x(), 0);
            assert_eq!(coord.y(), 1);
        }

        #[test]
        fn nudge_moves_in_correct_direction() {
            let coord: I2 = I2::new(10, 10);
            assert_eq!(coord.nudge(Direction::Up), Some(I2::new(10, 9)));
            assert_eq!(coord.nudge(Direction::Left), Some(I2::new(9, 10)));
            assert_eq!(coord.nudge(Direction::Down), Some(I2::new(10, 11)));
            assert_eq!(coord.nudge(Direction::Right), Some(I2::new(11, 10)));

            assert_eq!(coord.nudge_by(2, Direction::Up), Some(I2::new(10, 8)));
            assert_eq!(coord.nudge_by(2, Direction::Left), Some(I2::new(8, 10)));
            assert_eq!(coord.nudge_by(2, Direction::Down), Some(I2::new(10, 12)));
            assert_eq!(coord.nudge_by(2, Direction::Right), Some(I2::new(12, 10)));
        }

        #[test]
        fn nudge_is_none_on_integer_xflow() {
            assert_eq!(I2::new(i32::MIN, i32::MIN).nudge(Direction::Up), None);
            assert_eq!(I2::new(i32::MIN, i32::MIN).nudge(Direction::Left), None);
            assert_eq!(I2::new(i32::MAX, i32::MAX).nudge(Direction::Down), None);
            assert_eq!(I2::new(i32::MAX, i32::MAX).nudge(Direction::Right), None);

            assert_eq!(
                I2::new(i32::MIN + 1, i32::MIN + 1).nudge_by(2, Direction::Up),
                None
            );
            assert_eq!(
                I2::new(i32::MIN + 1, i32::MIN + 1).nudge_by(2, Direction::Left),
                None
            );
            assert_eq!(
                I2::new(i32::MAX - 1, i32::MAX - 1).nudge_by(2, Direction::Down),
                None
            );
            assert_eq!(
                I2::new(i32::MAX - 1, i32::MAX - 1).nudge_by(2, Direction::Right),
                None
            );
        }
    }

    mod u2_array {
        use super::*;

        #[test]
        fn can_be_made_from_u2_iter() {
            let coordinate_iter =
                [I2::new(0, 1), I2::new(2, 0), I2::new(3, 3), I2::new(4, 0)].into_iter();

            let coordinate_array = I2Array::from_iter(coordinate_iter);
            assert_eq!(
                coordinate_array,
                I2Array(vec![
                    I2::new(0, 1),
                    I2::new(2, 0),
                    I2::new(3, 3),
                    I2::new(4, 0)
                ])
            );
        }

        #[test]
        fn can_be_iterated_through() {
            let array: I2Array = I2Array(vec![
                I2::new(0, 1),
                I2::new(2, 0),
                I2::new(3, 3),
                I2::new(4, 0),
            ]);

            let mut iter = array.iter();

            assert_eq!(iter.next(), Some(&I2::new(0, 1)));
            assert_eq!(iter.next(), Some(&I2::new(2, 0)));
            assert_eq!(iter.next(), Some(&I2::new(3, 3)));
            assert_eq!(iter.next(), Some(&I2::new(4, 0)));
            assert_eq!(iter.next(), None);
        }

        #[test]
        fn can_be_queried_for_containing() {
            let array: I2Array = I2Array(vec![
                I2::new(0, 1),
                I2::new(2, 0),
                I2::new(3, 3),
                I2::new(4, 0),
            ]);

            assert!(array.contains(&I2::new(0, 1)));
            assert!(!array.contains(&I2::new(6, 6)));
        }

        #[test]
        fn can_be_consructed_from_vec_of_arrays() {
            let coords: Vec<[i32; 2]> = vec![[0, 0], [1, 0], [9, 120]];

            assert_eq!(
                I2Array::from(coords),
                I2Array(vec![I2::new(0, 0), I2::new(1, 0), I2::new(9, 120)])
            );
        }

        #[test]
        fn can_have_coordinates_pushed_to_the_back() {
            let mut coords: I2Array = I2Array::from(vec![[125, 216]]);
            coords.push(I2::new(0, 0));
            coords.push(I2::new(1, 2));

            assert_eq!(coords, I2Array::from(vec![[125, 216], [0, 0], [1, 2]]));
        }
    }
}
