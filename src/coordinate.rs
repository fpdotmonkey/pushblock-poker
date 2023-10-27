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
pub struct U2 {
    x: u32,
    y: u32,
}

impl U2 {
    /// Create a coordinate with your horizontal and vertical positions
    ///
    /// The horizontal coordinate, `x`, is understood to be positive to
    /// the right of the origin and the vertical coordinate, `y`, is
    /// _down_ from the origin.  This is upside-down from how you
    /// learned it in grade school, but is common in CG, game dev, and
    /// in some cases in engineering.
    pub fn new(x: u32, y: u32) -> Self {
        U2 { x, y }
    }

    /// Get the horizontal coordinate, which is positive rightward
    pub fn x(&self) -> u32 {
        self.x
    }

    /// Get the vertical coordinate, which is positive downward
    pub fn y(&self) -> u32 {
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
    pub fn nudge_by(&self, n: u32, direction: Direction) -> Option<Self> {
        match direction {
            Direction::Up => self.y.checked_sub(n).map(|y| U2::new(self.x, y)),
            Direction::Left => self.x.checked_sub(n).map(|x| U2::new(x, self.y)),
            Direction::Down => self.y.checked_add(n).map(|y| U2::new(self.x, y)),
            Direction::Right => self.x.checked_add(n).map(|x| U2::new(x, self.y)),
        }
    }
}

impl TryFrom<gdnative::core_types::Vector2> for U2 {
    type Error = &'static str;

    /// Performs the conversion by rounding to nearest [`U2`]
    ///
    /// [`gdnative::core_types::Vector2`]s are pairs of floats, so this
    /// does an element-wise rounding using [`f32::round`].  If the
    /// elements are well beyond the range of a [`u32`] or there's a
    /// `NaN`, then this will `Err`.
    fn try_from(vector2: gdnative::core_types::Vector2) -> Result<Self, Self::Error> {
        if (u32::MIN as f32) - 0.5 < vector2.x
            || vector2.x < (u32::MAX as f32) + 0.5
            || (u32::MIN as f32) - 0.5 < vector2.y
            || vector2.y < (u32::MAX as f32) + 0.5
        {
            return Ok(U2::new(vector2.x.round() as u32, vector2.y.round() as u32));
        }
        Err("Out-of-bounds float present")
    }
}

impl TryFrom<gdnative::prelude::Variant> for U2 {
    type Error = &'static str;

    /// Performs the conversion assuming `variant` is a [`gdnative::core_types::Vector2`]
    ///
    /// Should it not be a `Vector2` or the `Vector2` is invalid as
    /// [`TryFrom<gdnative::core_types::Vector2>`] understands it, then
    /// this will `Err`.
    fn try_from(variant: gdnative::prelude::Variant) -> Result<Self, Self::Error> {
        if variant.get_type() != gdnative::core_types::VariantType::Vector2 {
            return Err("Not a Vector2");
        }
        U2::try_from(variant.coerce_to::<gdnative::core_types::Vector2>())
    }
}

/// An array of [`U2`] coordinates
#[derive(Debug, PartialEq, Clone)]
pub struct U2Array(Vec<U2>);

impl U2Array {
    /// Returns an iterator over the container
    pub fn iter(&self) -> std::slice::Iter<'_, U2> {
        self.0.iter()
    }

    /// Returns `true` if the coordinate is in the array
    pub fn contains(&self, coordinate: &U2) -> bool {
        self.0.contains(coordinate)
    }

    /// Appends a coordinate to the back of the array
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds [`isize::MAX`] bytes.
    pub fn push(&mut self, coordinate: U2) {
        self.0.push(coordinate);
    }
}

impl FromIterator<U2> for U2Array {
    fn from_iter<I: IntoIterator<Item = U2>>(iter: I) -> Self {
        let mut coordinate_vector: Vec<U2> = vec![];

        for coordinate in iter {
            coordinate_vector.push(coordinate);
        }

        U2Array(coordinate_vector)
    }
}

impl From<Vec<[u32; 2]>> for U2Array {
    /// Constructs a new instance from a vector of [`U2`]-shaped arrays
    ///
    /// This is just a developer convenience to save some extra typing.
    ///
    /// # Examples
    ///
    /// ```
    /// let coords: U2Array = U2Array::from(vec![[0, 0], [1, 10], [115, 6]]);
    /// ```
    fn from(coordinates: Vec<[u32; 2]>) -> Self {
        U2Array(
            coordinates
                .iter()
                .map(|coordinate| U2::new(coordinate[0], coordinate[1]))
                .collect(),
        )
    }
}

impl TryFrom<gdnative::prelude::VariantArray> for U2Array {
    type Error = &'static str;

    /// Converts a [`gdnative::prelude::VariantArray`] of [`gdnative::core_types::Vector2`]s
    ///
    /// Should any of the variants in the array not be a `Vector2`, then
    /// this will return an error.
    ///
    /// If any of the given `Vector2`s would cause
    /// [`U2::try_from<gdnative::core_types::Vector2>`] to error, then
    /// they will be silently omitted from the output.
    fn try_from(vector2_array: gdnative::prelude::VariantArray) -> Result<Self, Self::Error> {
        if vector2_array
            .iter()
            .any(|variant| variant.get_type() != gdnative::core_types::VariantType::Vector2)
        {
            return Err("Not an array of Vector2");
        }
        Ok(vector2_array
            .iter()
            .filter_map(|variant| U2::try_from(variant).ok())
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
            let coord: U2 = U2::new(0, 1);
            assert_eq!(coord.x(), 0);
            assert_eq!(coord.y(), 1);
        }

        #[test]
        fn nudge_moves_in_correct_direction() {
            let coord: U2 = U2::new(10, 10);
            assert_eq!(coord.nudge(Direction::Up), Some(U2::new(10, 9)));
            assert_eq!(coord.nudge(Direction::Left), Some(U2::new(9, 10)));
            assert_eq!(coord.nudge(Direction::Down), Some(U2::new(10, 11)));
            assert_eq!(coord.nudge(Direction::Right), Some(U2::new(11, 10)));

            assert_eq!(coord.nudge_by(2, Direction::Up), Some(U2::new(10, 8)));
            assert_eq!(coord.nudge_by(2, Direction::Left), Some(U2::new(8, 10)));
            assert_eq!(coord.nudge_by(2, Direction::Down), Some(U2::new(10, 12)));
            assert_eq!(coord.nudge_by(2, Direction::Right), Some(U2::new(12, 10)));
        }

        #[test]
        fn nudge_is_none_on_integer_xflow() {
            assert_eq!(U2::new(0, 0).nudge(Direction::Up), None);
            assert_eq!(U2::new(0, 0).nudge(Direction::Left), None);
            assert_eq!(U2::new(u32::MAX, u32::MAX).nudge(Direction::Down), None);
            assert_eq!(U2::new(u32::MAX, u32::MAX).nudge(Direction::Right), None);

            assert_eq!(U2::new(1, 1).nudge_by(2, Direction::Up), None);
            assert_eq!(U2::new(1, 1).nudge_by(2, Direction::Left), None);
            assert_eq!(
                U2::new(u32::MAX - 1, u32::MAX - 1).nudge_by(2, Direction::Down),
                None
            );
            assert_eq!(
                U2::new(u32::MAX - 1, u32::MAX - 1).nudge_by(2, Direction::Right),
                None
            );
        }
    }

    mod u2_array {
        use super::*;

        #[test]
        fn can_be_made_from_u2_iter() {
            let coordinate_iter =
                [U2::new(0, 1), U2::new(2, 0), U2::new(3, 3), U2::new(4, 0)].into_iter();

            let coordinate_array = U2Array::from_iter(coordinate_iter);
            assert_eq!(
                coordinate_array,
                U2Array(vec![
                    U2::new(0, 1),
                    U2::new(2, 0),
                    U2::new(3, 3),
                    U2::new(4, 0)
                ])
            );
        }

        #[test]
        fn can_be_iterated_through() {
            let array: U2Array = U2Array(vec![
                U2::new(0, 1),
                U2::new(2, 0),
                U2::new(3, 3),
                U2::new(4, 0),
            ]);

            let mut iter = array.iter();

            assert_eq!(iter.next(), Some(&U2::new(0, 1)));
            assert_eq!(iter.next(), Some(&U2::new(2, 0)));
            assert_eq!(iter.next(), Some(&U2::new(3, 3)));
            assert_eq!(iter.next(), Some(&U2::new(4, 0)));
            assert_eq!(iter.next(), None);
        }

        #[test]
        fn can_be_queried_for_containing() {
            let array: U2Array = U2Array(vec![
                U2::new(0, 1),
                U2::new(2, 0),
                U2::new(3, 3),
                U2::new(4, 0),
            ]);

            assert!(array.contains(&U2::new(0, 1)));
            assert!(!array.contains(&U2::new(6, 6)));
        }

        #[test]
        fn can_be_consructed_from_vec_of_arrays() {
            let coords: Vec<[u32; 2]> = vec![[0, 0], [1, 0], [9, 120]];

            assert_eq!(
                U2Array::from(coords),
                U2Array(vec![U2::new(0, 0), U2::new(1, 0), U2::new(9, 120)])
            );
        }

        #[test]
        fn can_have_coordinates_pushed_to_the_back() {
            let mut coords: U2Array = U2Array::from(vec![[125, 216]]);
            coords.push(U2::new(0, 0));
            coords.push(U2::new(1, 2));

            assert_eq!(coords, U2Array::from(vec![[125, 216], [0, 0], [1, 2]]));
        }
    }
}
