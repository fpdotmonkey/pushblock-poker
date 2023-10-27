//! The rules for advancing the game state in Sokoban
//!
//! In Sokoban, you control a character on a grid who pushes numerous
//! crates around in order to get all these crates onto target tiles.
//!
//! It was first created in 1981 by Hiroyuki Imabayashi for the
//! NEC PC-8801 and has seen countless recreations, improvements, and
//! rehashes in the decades since.

// Developer note: a lot of the language around the rules is based on
// that used in the game Baba is You, developed by Arvi Teikari.  You
// should play it https://store.steampowered.com/app/736260/Baba_Is_You/

use crate::coordinate;

/// The primary interface for querying and updating the game state
#[derive(Debug, Clone, PartialEq)]
pub struct Sokoban {
    you: coordinate::U2,
    stops: coordinate::U2Array,
    pushes: coordinate::U2Array,
    targets: coordinate::U2Array,
}

impl Sokoban {
    /// Construct a new sokoban board
    ///
    /// In cases where a `coordinate::U2` is used, the first value is
    /// understood as the horizontal coordinate to the right, and the
    /// second value is the vertical coordinate down.
    ///
    /// # Examples
    ///
    /// ```
    /// // Let's create this board, where @: you, 0: push, -|: stop, and ^: target
    /// //
    /// //   ---
    /// //   |^|
    /// //   | ----
    /// // ---0 0^|
    /// // |^ 0@---
    /// // ----0|
    /// //    |^|
    /// //    ---
    ///
    /// let you: coordinate::U2 = coordinate::U2::new(4, 4);
    /// let stops: coordinate::U2Array = coordinate::U2Array::from(vec![
    ///     [2, 0], [3, 0], [4, 0], [2, 1], [4, 1], [2, 2], [4, 2],
    ///     [5, 2], [6, 2], [7, 2], [0, 3], [1, 3], [2, 3], [7, 3],
    ///     [0, 4], [5, 4], [6, 4], [7, 4], [0, 5], [1, 5], [2, 5],
    ///     [3, 5], [5, 5], [3, 6], [5, 6], [3, 7], [4, 7], [5, 7],
    /// ]);
    /// let pushes: coordinate::U2Array = coordinate::U2Array::from(vec![[3, 3], [5, 3], [3, 4], [4, 5]]);
    /// let targets: coordinate::U2Array = coordinate::U2Array::from(vec![[3, 1], [6, 3], [1, 4], [4, 6]]);
    ///
    /// let board: Sokoban = Sokoban::new(you, stops.clone(), pushes.clone(), targets.clone());
    ///
    /// # assert_eq!(
    /// #     board.you_move(coordinate::Direction::Up),
    /// #     Sokoban::new([4, 3], stops, pushes, targets)
    /// # );
    /// #
    /// # assert_eq!(
    /// #     board
    /// #         .you_move(coordinate::Direction::Down)
    /// #         .you_move(coordinate::Direction::Up)
    /// #         .triggered_targets(),
    /// #     vec![&[4, 6]]
    /// # );
    /// #
    /// # assert!(!board
    /// #     .you_move(coordinate::Direction::Down)
    /// #     .you_move(coordinate::Direction::Up)
    /// #     .all_targets_triggered());
    /// #
    /// # assert!(board
    /// #     .you_move(coordinate::Direction::Down)
    /// #     .you_move(coordinate::Direction::Up)
    /// #     .you_move(coordinate::Direction::Left)
    /// #     .you_move(coordinate::Direction::Left)
    /// #     .you_move(coordinate::Direction::Right)
    /// #     .you_move(coordinate::Direction::Up)
    /// #     .you_move(coordinate::Direction::Up)
    /// #     .you_move(coordinate::Direction::Down)
    /// #     .you_move(coordinate::Direction::Right)
    /// #     .you_move(coordinate::Direction::Right)
    /// #     .all_targets_triggered());
    /// ```
    pub fn new(
        you: coordinate::U2,
        stops: coordinate::U2Array,
        pushes: coordinate::U2Array,
        targets: coordinate::U2Array,
    ) -> Self {
        Sokoban {
            you,
            stops,
            pushes,
            targets,
        }
    }

    /// Move the player one tile over toward direction
    ///
    /// Attempting to move into a tile occupied by a stop will result in
    /// your position not changing.  The same is true of trying to move
    /// such that your position might experience and integer overflow;
    /// it'll simply saturate with a max or min int.
    ///
    /// Moving into a push would result in that push moving in
    /// `direction`.
    ///
    /// # Examples
    ///
    /// ```
    /// # // Let's create this board, where @: you, 0: push, -|: stop, and ^: target
    /// # //
    /// # //   ---
    /// # //   |^|
    /// # //   | ----
    /// # // ---0 0^|
    /// # // |^ 0@---
    /// # // ----0|
    /// # //    |^|
    /// # //    ---
    /// #
    /// let you: coordinate::U2 = coordinate::U2::new(4, 4);
    /// // ...
    /// # let stops: coordinate::U2Array = coordinate::U2Array::from(vec![
    /// #     [2, 0], [3, 0], [4, 0], [2, 1], [4, 1], [2, 2], [4, 2],
    /// #     [5, 2], [6, 2], [7, 2], [0, 3], [1, 3], [2, 3], [7, 3],
    /// #     [0, 4], [5, 4], [6, 4], [7, 4], [0, 5], [1, 5], [2, 5],
    /// #     [3, 5], [5, 5], [3, 6], [5, 6], [3, 7], [4, 7], [5, 7],
    /// # ]);
    /// # let pushes: coordinate::U2Array = coordinate::U2Array::from(vec![[3, 3], [5, 3], [3, 4], [4, 5]]);
    /// # let targets: coordinate::U2Array = coordinate::U2Array::from(vec![[3, 1], [6, 3], [1, 4], [4, 6]]);
    /// #
    /// let board: Sokoban = Sokoban::new(you, stops.clone(), pushes.clone(), targets.clone());
    ///
    /// assert_eq!(
    ///     board.you_move(coordinate::Direction::Up),
    ///     Sokoban::new([4, 3], stops, pushes, targets)
    /// );
    /// #
    /// # assert_eq!(
    /// #     board
    /// #         .you_move(coordinate::Direction::Down)
    /// #         .you_move(coordinate::Direction::Up)
    /// #         .triggered_targets(),
    /// #     vec![&[4, 6]]
    /// # );
    /// #
    /// # assert!(!board
    /// #     .you_move(coordinate::Direction::Down)
    /// #     .you_move(coordinate::Direction::Up)
    /// #     .all_targets_triggered());
    /// #
    /// # assert!(board
    /// #     .you_move(coordinate::Direction::Down)
    /// #     .you_move(coordinate::Direction::Up)
    /// #     .you_move(coordinate::Direction::Left)
    /// #     .you_move(coordinate::Direction::Left)
    /// #     .you_move(coordinate::Direction::Right)
    /// #     .you_move(coordinate::Direction::Up)
    /// #     .you_move(coordinate::Direction::Up)
    /// #     .you_move(coordinate::Direction::Down)
    /// #     .you_move(coordinate::Direction::Right)
    /// #     .you_move(coordinate::Direction::Right)
    /// #     .all_targets_triggered());
    /// ```
    pub fn you_move(&self, direction: coordinate::Direction) -> Sokoban {
        let mut moving_pushes: coordinate::U2Array = coordinate::U2Array::from(vec![]);
        for i in 1.. {
            let test_coordinate: Option<coordinate::U2> = self.you.nudge_by(i, direction);
            if test_coordinate.is_none() || self.stops.contains(&test_coordinate.unwrap()) {
                return Sokoban::new(
                    self.you,
                    self.stops.clone(),
                    self.pushes.clone(),
                    self.targets.clone(),
                );
            }

            let test_coordinate: coordinate::U2 = test_coordinate.unwrap();

            if self.pushes.contains(&test_coordinate) {
                moving_pushes.push(test_coordinate);
            } else {
                break;
            }
        }

        let new_you: coordinate::U2 = self.you.nudge(direction).unwrap();
        let new_pushes: coordinate::U2Array = self
            .pushes
            .iter()
            .map(|push| {
                if moving_pushes.contains(push) {
                    push.nudge(direction).unwrap()
                } else {
                    *push
                }
            })
            .collect();

        Sokoban::new(
            new_you,
            self.stops.clone(),
            new_pushes,
            self.targets.clone(),
        )
    }

    /// The positions of all the targets that have a push on them
    ///
    /// # Examples
    ///
    /// ```
    /// # // Let's create this board, where @: you, 0: push, -|: stop, and ^: target
    /// # //
    /// # //   ---
    /// # //   |^|
    /// # //   | ----
    /// # // ---0 0^|
    /// # // |^ 0@---
    /// # // ----0|
    /// # //    |^|
    /// # //    ---
    /// #
    /// # let you: coordinate::U2 = [4, 4];
    /// # let stops: coordinate::U2Array = coordinate::U2Array::from(vec![
    /// #     [2, 0], [3, 0], [4, 0], [2, 1], [4, 1], [2, 2], [4, 2],
    /// #     [5, 2], [6, 2], [7, 2], [0, 3], [1, 3], [2, 3], [7, 3],
    /// #     [0, 4], [5, 4], [6, 4], [7, 4], [0, 5], [1, 5], [2, 5],
    /// #     [3, 5], [5, 5], [3, 6], [5, 6], [3, 7], [4, 7], [5, 7],
    /// # ]);
    /// # let pushes: coordinate::U2Array = coordinate::U2Array::from(vec![[3, 3], [5, 3], [3, 4], [4, 5]]);
    /// let targets: coordinate::U2Array = coordinate::U2Array::from(vec![[3, 1], [6, 3], [1, 4], [4, 6]]);
    /// // ...
    /// #
    /// let board: Sokoban = Sokoban::new(you, stops.clone(), pushes.clone(), targets.clone());
    ///
    /// # assert_eq!(
    /// #     board.you_move(coordinate::Direction::Up),
    /// #     Sokoban::new([4, 3], stops, pushes, targets)
    /// # );
    /// #
    /// assert_eq!(
    ///     board
    ///         .you_move(coordinate::Direction::Down)
    ///         .you_move(coordinate::Direction::Up)
    ///         .triggered_targets(),
    ///     vec![&[4, 6]]
    /// );
    /// #
    /// # assert!(!board
    /// #     .you_move(coordinate::Direction::Down)
    /// #     .you_move(coordinate::Direction::Up)
    /// #     .all_targets_triggered());
    /// #
    /// # assert!(board
    /// #     .you_move(coordinate::Direction::Down)
    /// #     .you_move(coordinate::Direction::Up)
    /// #     .you_move(coordinate::Direction::Left)
    /// #     .you_move(coordinate::Direction::Left)
    /// #     .you_move(coordinate::Direction::Right)
    /// #     .you_move(coordinate::Direction::Up)
    /// #     .you_move(coordinate::Direction::Up)
    /// #     .you_move(coordinate::Direction::Down)
    /// #     .you_move(coordinate::Direction::Right)
    /// #     .you_move(coordinate::Direction::Right)
    /// #     .all_targets_triggered());
    /// ```
    pub fn triggered_targets(&self) -> Vec<&coordinate::U2> {
        self.targets
            .iter()
            .filter(|target| self.pushes.contains(target))
            .collect::<Vec<&coordinate::U2>>()
    }

    /// Checks if all the targets have been triggered
    ///
    /// # Examples
    ///
    /// ```
    /// # // Let's create this board, where @: you, 0: push, -|: stop, and ^: target
    /// # //
    /// //   ---
    /// //   |^|
    /// //   | ----
    /// // ---0 0^|
    /// // |^ 0@---
    /// // ----0|
    /// //    |^|
    /// //    ---
    ///
    /// # let you: coordinate::U2 = [4, 4];
    /// # let stops: coordinate::U2Array = coordinate::U2Array::from(vec![
    /// #     [2, 0], [3, 0], [4, 0], [2, 1], [4, 1], [2, 2], [4, 2],
    /// #     [5, 2], [6, 2], [7, 2], [0, 3], [1, 3], [2, 3], [7, 3],
    /// #     [0, 4], [5, 4], [6, 4], [7, 4], [0, 5], [1, 5], [2, 5],
    /// #     [3, 5], [5, 5], [3, 6], [5, 6], [3, 7], [4, 7], [5, 7],
    /// # ]);
    /// # let pushes: coordinate::U2Array = coordinate::U2Array::from(vec![[3, 3], [5, 3], [3, 4], [4, 5]]);
    /// # let targets: coordinate::U2Array = coordinate::U2Array::from(vec![[3, 1], [6, 3], [1, 4], [4, 6]]);
    /// #
    /// let board: Sokoban = Sokoban::new(you, stops.clone(), pushes.clone(), targets.clone());
    ///
    /// # assert_eq!(
    /// #     board.you_move(coordinate::Direction::Up),
    /// #     Sokoban::new([4, 3], stops, pushes, targets)
    /// # );
    /// #
    /// # assert_eq!(
    /// #     board
    /// #         .you_move(coordinate::Direction::Down)
    /// #         .you_move(coordinate::Direction::Up)
    /// #         .triggered_targets(),
    /// #     vec![&[4, 6]]
    /// # );
    /// #
    /// assert!(!board
    ///     .you_move(coordinate::Direction::Down)
    ///     .you_move(coordinate::Direction::Up)
    ///     .all_targets_triggered());
    ///
    /// assert!(board
    ///     .you_move(coordinate::Direction::Down)
    ///     .you_move(coordinate::Direction::Up)
    ///     .you_move(coordinate::Direction::Left)
    ///     .you_move(coordinate::Direction::Left)
    ///     .you_move(coordinate::Direction::Right)
    ///     .you_move(coordinate::Direction::Up)
    ///     .you_move(coordinate::Direction::Up)
    ///     .you_move(coordinate::Direction::Down)
    ///     .you_move(coordinate::Direction::Right)
    ///     .you_move(coordinate::Direction::Right)
    ///     .all_targets_triggered());
    /// ```
    pub fn all_targets_triggered(&self) -> bool {
        self.targets
            .iter()
            .all(|target| self.pushes.contains(target))
    }

    /// Gets the position of the player
    pub fn you(&self) -> coordinate::U2 {
        self.you
    }

    /// Gets the positions of all the stopping collision
    pub fn stops(&self) -> coordinate::U2Array {
        self.stops.clone()
    }

    /// Gets the positions of all the pushable objects
    pub fn pushes(&self) -> coordinate::U2Array {
        self.pushes.clone()
    }

    /// Gets the positions of all the targets for the pushable objects
    pub fn targets(&self) -> coordinate::U2Array {
        self.targets.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_move_in_all_cardinal_directions_directions() {
        // .....
        // .@.0^
        // ..+..
        // .0.+.
        // .^...
        let you: coordinate::U2 = coordinate::U2::new(1, 1);
        let stops: coordinate::U2Array = coordinate::U2Array::from(vec![[2, 2], [3, 3]]);
        let pushes: coordinate::U2Array = coordinate::U2Array::from(vec![[3, 1], [1, 3]]);
        let targets: coordinate::U2Array = coordinate::U2Array::from(vec![[4, 1], [1, 4]]);

        let you_up: coordinate::U2 = coordinate::U2::new(1, 0);
        let you_left: coordinate::U2 = coordinate::U2::new(0, 1);
        let you_down: coordinate::U2 = coordinate::U2::new(1, 2);
        let you_right: coordinate::U2 = coordinate::U2::new(2, 1);

        let board: Sokoban = Sokoban::new(you, stops.clone(), pushes.clone(), targets.clone());

        assert_eq!(
            board.you_move(coordinate::Direction::Up),
            Sokoban::new(you_up, stops.clone(), pushes.clone(), targets.clone())
        );
        assert_eq!(
            board.you_move(coordinate::Direction::Left),
            Sokoban::new(you_left, stops.clone(), pushes.clone(), targets.clone())
        );
        assert_eq!(
            board.you_move(coordinate::Direction::Down),
            Sokoban::new(you_down, stops.clone(), pushes.clone(), targets.clone())
        );
        assert_eq!(
            board.you_move(coordinate::Direction::Right),
            Sokoban::new(you_right, stops.clone(), pushes.clone(), targets.clone())
        );
    }

    #[test]
    fn you_dont_move_into_stop() {
        // ...0.^^
        // ...-...
        // .0|@|0.
        // ...-...
        // ...0...
        let you: coordinate::U2 = coordinate::U2::new(3, 3);
        let stops: coordinate::U2Array =
            coordinate::U2Array::from(vec![[3, 2], [2, 3], [3, 4], [4, 3]]);
        let pushes: coordinate::U2Array =
            coordinate::U2Array::from(vec![[3, 1], [1, 3], [3, 5], [5, 3]]);
        let targets: coordinate::U2Array = coordinate::U2Array::from(vec![[6, 1], [7, 1]]);

        let board: Sokoban =
            Sokoban::new(you.clone(), stops.clone(), pushes.clone(), targets.clone());
        assert_eq!(
            board.you_move(coordinate::Direction::Up),
            Sokoban::new(you.clone(), stops.clone(), pushes.clone(), targets.clone())
        );
        assert_eq!(
            board.you_move(coordinate::Direction::Left),
            Sokoban::new(you.clone(), stops.clone(), pushes.clone(), targets.clone())
        );
        assert_eq!(
            board.you_move(coordinate::Direction::Down),
            Sokoban::new(you.clone(), stops.clone(), pushes.clone(), targets.clone())
        );
        assert_eq!(
            board.you_move(coordinate::Direction::Right),
            Sokoban::new(you.clone(), stops.clone(), pushes.clone(), targets.clone())
        );
    }

    #[test]
    fn pushes_move_when_you_walk_into_them() {
        // --...^^
        // ...0...
        // ..0@0..
        // ...0...
        // .......
        let you: coordinate::U2 = coordinate::U2::new(3, 3);
        let you_up: coordinate::U2 = coordinate::U2::new(3, 2);
        let you_left: coordinate::U2 = coordinate::U2::new(2, 3);
        let you_down: coordinate::U2 = coordinate::U2::new(3, 4);
        let you_right: coordinate::U2 = coordinate::U2::new(4, 3);

        let pushes: coordinate::U2Array =
            coordinate::U2Array::from(vec![[3, 2], [2, 3], [3, 4], [4, 3]]);
        let pushes_up: coordinate::U2Array =
            coordinate::U2Array::from(vec![[3, 1], [2, 3], [3, 4], [4, 3]]);
        let pushes_left: coordinate::U2Array =
            coordinate::U2Array::from(vec![[3, 2], [1, 3], [3, 4], [4, 3]]);
        let pushes_down: coordinate::U2Array =
            coordinate::U2Array::from(vec![[3, 2], [2, 3], [3, 5], [4, 3]]);
        let pushes_right: coordinate::U2Array =
            coordinate::U2Array::from(vec![[3, 2], [2, 3], [3, 4], [5, 3]]);

        let stops: coordinate::U2Array = coordinate::U2Array::from(vec![[1, 1], [2, 1]]);
        let targets: coordinate::U2Array = coordinate::U2Array::from(vec![[6, 1], [7, 1]]);

        let board = Sokoban::new(you, stops.clone(), pushes.clone(), targets.clone());

        assert_eq!(
            board.you_move(coordinate::Direction::Up),
            Sokoban::new(you_up, stops.clone(), pushes_up.clone(), targets.clone())
        );
        assert_eq!(
            board.you_move(coordinate::Direction::Left),
            Sokoban::new(
                you_left,
                stops.clone(),
                pushes_left.clone(),
                targets.clone()
            )
        );
        assert_eq!(
            board.you_move(coordinate::Direction::Down),
            Sokoban::new(
                you_down,
                stops.clone(),
                pushes_down.clone(),
                targets.clone()
            )
        );
        assert_eq!(
            board.you_move(coordinate::Direction::Right),
            Sokoban::new(
                you_right,
                stops.clone(),
                pushes_right.clone(),
                targets.clone()
            )
        );
    }

    #[test]
    fn push_moves_when_push_is_pushed_into_it() {
        // .....
        // ..0..
        // ..0..
        // ..0..
        // ..0..
        // ..@..
        let you: coordinate::U2 = coordinate::U2::new(0, 5);
        let you_final: coordinate::U2 = coordinate::U2::new(0, 4);
        let pushes: coordinate::U2Array =
            coordinate::U2Array::from(vec![[0, 1], [0, 2], [0, 3], [0, 4]]);
        let pushes_final: coordinate::U2Array =
            coordinate::U2Array::from(vec![[0, 0], [0, 1], [0, 2], [0, 3]]);
        let stops: coordinate::U2Array = coordinate::U2Array::from(vec![]);
        let targets: coordinate::U2Array = coordinate::U2Array::from(vec![]);

        assert_eq!(
            Sokoban::new(you, stops.clone(), pushes, targets.clone())
                .you_move(coordinate::Direction::Up),
            Sokoban::new(you_final, stops.clone(), pushes_final, targets.clone())
        );
    }

    #[test]
    fn push_doesnt_move_when_push_is_pushed_into_it_but_theres_a_stop() {
        // ..-..
        // ..0..
        // ..0..
        // ..0..
        // ..0..
        // ..@..
        let you: coordinate::U2 = coordinate::U2::new(0, 5);
        let pushes: coordinate::U2Array =
            coordinate::U2Array::from(vec![[0, 1], [0, 2], [0, 3], [0, 4]]);
        let stops: coordinate::U2Array = coordinate::U2Array::from(vec![[0, 0]]);
        let targets: coordinate::U2Array = coordinate::U2Array::from(vec![]);

        assert_eq!(
            Sokoban::new(you, stops.clone(), pushes.clone(), targets.clone())
                .you_move(coordinate::Direction::Up),
            Sokoban::new(you, stops.clone(), pushes.clone(), targets.clone())
        );

        // ..0..
        // ..0..
        // ..0..
        // ..0..
        // ..@..
        let you: coordinate::U2 = coordinate::U2::new(0, 4);
        let pushes: coordinate::U2Array =
            coordinate::U2Array::from(vec![[0, 0], [0, 1], [0, 2], [0, 3]]);
        let targets: coordinate::U2Array = coordinate::U2Array::from(vec![]);

        assert_eq!(
            Sokoban::new(you, stops.clone(), pushes.clone(), targets.clone())
                .you_move(coordinate::Direction::Up),
            Sokoban::new(you, stops.clone(), pushes.clone(), targets.clone())
        );
    }

    #[test]
    fn pushes_dont_move_into_stop() {
        // ..-..
        // ..0..
        // |0@0|
        // ..0..
        // ..-.^
        let you: coordinate::U2 = coordinate::U2::new(2, 2);
        let stops: coordinate::U2Array =
            coordinate::U2Array::from(vec![[2, 0], [0, 2], [2, 4], [4, 2]]);
        let pushes: coordinate::U2Array =
            coordinate::U2Array::from(vec![[2, 1], [1, 2], [2, 3], [3, 2]]);
        let targets: coordinate::U2Array = coordinate::U2Array::from(vec![[4, 4]]);

        let board: Sokoban = Sokoban::new(you, stops.clone(), pushes.clone(), targets.clone());
        assert_eq!(
            board.you_move(coordinate::Direction::Up),
            Sokoban::new(you, stops.clone(), pushes.clone(), targets.clone())
        );
        assert_eq!(
            board.you_move(coordinate::Direction::Left),
            Sokoban::new(you, stops.clone(), pushes.clone(), targets.clone())
        );
        assert_eq!(
            board.you_move(coordinate::Direction::Down),
            Sokoban::new(you, stops.clone(), pushes.clone(), targets.clone())
        );
        assert_eq!(
            board.you_move(coordinate::Direction::Right),
            Sokoban::new(you, stops.clone(), pushes.clone(), targets.clone())
        );
    }

    #[test]
    fn integer_xflow_is_stop() {
        let stops: coordinate::U2Array = coordinate::U2Array::from(vec![]);
        let pushes: coordinate::U2Array = coordinate::U2Array::from(vec![]);
        let targets: coordinate::U2Array = coordinate::U2Array::from(vec![]);

        assert_eq!(
            Sokoban::new(
                coordinate::U2::new(0, u32::MIN),
                stops.clone(),
                pushes.clone(),
                targets.clone()
            )
            .you_move(coordinate::Direction::Up),
            Sokoban::new(
                coordinate::U2::new(0, u32::MIN),
                stops.clone(),
                pushes.clone(),
                targets.clone()
            )
        );
        assert_eq!(
            Sokoban::new(
                coordinate::U2::new(u32::MIN, 0),
                stops.clone(),
                pushes.clone(),
                targets.clone()
            )
            .you_move(coordinate::Direction::Left),
            Sokoban::new(
                coordinate::U2::new(u32::MIN, 0),
                stops.clone(),
                pushes.clone(),
                targets.clone()
            )
        );
        assert_eq!(
            Sokoban::new(
                coordinate::U2::new(0, u32::MAX),
                stops.clone(),
                pushes.clone(),
                targets.clone()
            )
            .you_move(coordinate::Direction::Down),
            Sokoban::new(
                coordinate::U2::new(0, u32::MAX),
                stops.clone(),
                pushes.clone(),
                targets.clone()
            )
        );
        assert_eq!(
            Sokoban::new(
                coordinate::U2::new(u32::MAX, 0),
                stops.clone(),
                pushes.clone(),
                targets.clone()
            )
            .you_move(coordinate::Direction::Right),
            Sokoban::new(
                coordinate::U2::new(u32::MAX, 0),
                stops.clone(),
                pushes.clone(),
                targets.clone()
            )
        );

        assert_eq!(
            Sokoban::new(
                coordinate::U2::new(u32::MAX - 1, 0),
                stops.clone(),
                coordinate::U2Array::from(vec![[u32::MAX, 0]]),
                targets.clone()
            ),
            Sokoban::new(
                coordinate::U2::new(u32::MAX - 1, 0),
                stops.clone(),
                coordinate::U2Array::from(vec![[u32::MAX, 0]]),
                targets.clone()
            )
        );
    }

    #[test]
    fn lonely_target_is_not_triggered() {
        // ..^..
        // ..@..
        let you: coordinate::U2 = coordinate::U2::new(0, 1);
        let pushes: coordinate::U2Array = coordinate::U2Array::from(vec![]);
        let targets: coordinate::U2Array = coordinate::U2Array::from(vec![[0, 0]]);
        let stops: coordinate::U2Array = coordinate::U2Array::from(vec![]);

        let board: Sokoban = Sokoban::new(you, stops, pushes, targets);
        assert_eq!(board.triggered_targets(), Vec::<&coordinate::U2>::new());
        assert!(!board.all_targets_triggered());
    }

    #[test]
    fn target_on_push_is_triggered() {
        // ..^..
        // ..0..
        // ..@..
        let you: coordinate::U2 = coordinate::U2::new(0, 2);
        let pushes: coordinate::U2Array = coordinate::U2Array::from(vec![[0, 1]]);
        let targets: coordinate::U2Array = coordinate::U2Array::from(vec![[0, 0]]);
        let stops: coordinate::U2Array = coordinate::U2Array::from(vec![]);

        let board: Sokoban =
            Sokoban::new(you, stops, pushes, targets.clone()).you_move(coordinate::Direction::Up);
        assert_eq!(
            board.triggered_targets(),
            targets.iter().collect::<Vec<&coordinate::U2>>()
        );
        assert!(board.all_targets_triggered());
    }

    #[test]
    fn target_on_you_is_not_triggered() {
        // ..^..
        // ..@..
        let you: coordinate::U2 = coordinate::U2::new(0, 1);
        let pushes: coordinate::U2Array = coordinate::U2Array::from(vec![]);
        let targets: coordinate::U2Array = coordinate::U2Array::from(vec![[0, 0]]);
        let stops: coordinate::U2Array = coordinate::U2Array::from(vec![]);

        let board: Sokoban =
            Sokoban::new(you, stops, pushes, targets).you_move(coordinate::Direction::Up);
        assert_eq!(board.triggered_targets(), Vec::<&coordinate::U2>::new());
        assert!(!board.all_targets_triggered());
    }

    #[test]
    fn many_target_on_many_push_is_triggered() {
        // ..^..
        // ..0..
        // ^0@0^
        // ..0..
        // ..^..
        let you: coordinate::U2 = coordinate::U2::new(2, 2);
        let pushes: coordinate::U2Array =
            coordinate::U2Array::from(vec![[2, 1], [1, 2], [3, 2], [2, 3]]);
        let targets: coordinate::U2Array =
            coordinate::U2Array::from(vec![[2, 0], [0, 2], [2, 4], [4, 2]]);
        let stops: coordinate::U2Array = coordinate::U2Array::from(vec![]);

        let board: Sokoban = Sokoban::new(you, stops, pushes, targets.clone());
        assert_eq!(
            board.triggered_targets(),
            targets.iter().take(0).collect::<Vec<&coordinate::U2>>()
        );
        assert!(!board.all_targets_triggered());

        let board: Sokoban = board
            .you_move(coordinate::Direction::Up) // top target
            .you_move(coordinate::Direction::Down);
        assert_eq!(
            board.triggered_targets(),
            targets.iter().take(1).collect::<Vec<&coordinate::U2>>()
        );
        assert!(!board.all_targets_triggered());

        let board: Sokoban = board
            .you_move(coordinate::Direction::Left) // left target
            .you_move(coordinate::Direction::Right);
        assert_eq!(
            board.triggered_targets(),
            targets.iter().take(2).collect::<Vec<&coordinate::U2>>()
        );
        assert!(!board.all_targets_triggered());

        let board: Sokoban = board
            .you_move(coordinate::Direction::Down) // bottom target
            .you_move(coordinate::Direction::Up);
        assert_eq!(
            board.triggered_targets(),
            targets.iter().take(3).collect::<Vec<&coordinate::U2>>()
        );
        assert!(!board.all_targets_triggered());

        let board: Sokoban = board
            .you_move(coordinate::Direction::Right) // right target
            .you_move(coordinate::Direction::Left);
        assert_eq!(
            board.triggered_targets(),
            targets.iter().collect::<Vec<&coordinate::U2>>()
        );
        assert!(board.all_targets_triggered());
    }

    #[test]
    fn you_are_where_you_are() {
        let you: coordinate::U2 = coordinate::U2::new(1, 1);
        let stops: coordinate::U2Array = coordinate::U2Array::from(vec![[2, 2], [3, 3]]);
        let pushes: coordinate::U2Array = coordinate::U2Array::from(vec![[3, 1], [1, 3]]);
        let targets: coordinate::U2Array = coordinate::U2Array::from(vec![[4, 1], [1, 4]]);

        let board: Sokoban = Sokoban::new(you, stops, pushes, targets);

        assert_eq!(board.you(), you);
        assert_eq!(
            board.you_move(coordinate::Direction::Right).you(),
            coordinate::U2::new(2, 1)
        );
    }

    #[test]
    fn stops_are_where_they_are() {
        let you: coordinate::U2 = coordinate::U2::new(1, 1);
        let stops: coordinate::U2Array = coordinate::U2Array::from(vec![[2, 2], [3, 3]]);
        let pushes: coordinate::U2Array = coordinate::U2Array::from(vec![[3, 1], [1, 3]]);
        let targets: coordinate::U2Array = coordinate::U2Array::from(vec![[4, 1], [1, 4]]);

        let board: Sokoban = Sokoban::new(you, stops.clone(), pushes, targets);

        assert_eq!(board.stops(), stops);
    }

    #[test]
    fn pushes_are_where_they_are() {
        let you: coordinate::U2 = coordinate::U2::new(1, 1);
        let stops: coordinate::U2Array = coordinate::U2Array::from(vec![[2, 2], [3, 3]]);
        let pushes: coordinate::U2Array = coordinate::U2Array::from(vec![[3, 1], [1, 3]]);
        let targets: coordinate::U2Array = coordinate::U2Array::from(vec![[4, 1], [1, 4]]);

        let board: Sokoban = Sokoban::new(you, stops, pushes.clone(), targets);

        assert_eq!(board.pushes(), pushes);
    }

    #[test]
    fn targets_are_where_they_are() {
        let you: coordinate::U2 = coordinate::U2::new(1, 1);
        let stops: coordinate::U2Array = coordinate::U2Array::from(vec![[2, 2], [3, 3]]);
        let pushes: coordinate::U2Array = coordinate::U2Array::from(vec![[3, 1], [1, 3]]);
        let targets: coordinate::U2Array = coordinate::U2Array::from(vec![[4, 1], [1, 4]]);

        let board: Sokoban = Sokoban::new(you, stops, pushes, targets.clone());

        assert_eq!(board.targets(), targets);
    }

    #[test]
    fn doc_test() {
        // This will be used for doc examples, but doc tests don't run
        // on cdylib crates like this one

        // Let's create this board, where @: you, 0: push, -|: stop, and ^: target
        //
        //   ---
        //   |^|
        //   | ----
        // ---0 0^|
        // |^ 0@---
        // ----0|
        //    |^|
        //    ---

        let you: coordinate::U2 = coordinate::U2::new(4, 4);
        let stops: coordinate::U2Array = coordinate::U2Array::from(vec![
            [2, 0],
            [3, 0],
            [4, 0],
            [2, 1],
            [4, 1],
            [2, 2],
            [4, 2],
            [5, 2],
            [6, 2],
            [7, 2],
            [0, 3],
            [1, 3],
            [2, 3],
            [7, 3],
            [0, 4],
            [5, 4],
            [6, 4],
            [7, 4],
            [0, 5],
            [1, 5],
            [2, 5],
            [3, 5],
            [5, 5],
            [3, 6],
            [5, 6],
            [3, 7],
            [4, 7],
            [5, 7],
        ]);
        let pushes: coordinate::U2Array =
            coordinate::U2Array::from(vec![[3, 3], [5, 3], [3, 4], [4, 5]]);
        let targets: coordinate::U2Array =
            coordinate::U2Array::from(vec![[3, 1], [6, 3], [1, 4], [4, 6]]);

        let board: Sokoban = Sokoban::new(you, stops.clone(), pushes.clone(), targets.clone());

        assert_eq!(
            board.you_move(coordinate::Direction::Up),
            Sokoban::new(coordinate::U2::new(4, 3), stops, pushes, targets)
        );

        assert_eq!(
            board
                .you_move(coordinate::Direction::Down)
                .you_move(coordinate::Direction::Up)
                .triggered_targets(),
            vec![&coordinate::U2::new(4, 6)]
        );

        assert!(!board
            .you_move(coordinate::Direction::Down)
            .you_move(coordinate::Direction::Up)
            .all_targets_triggered());

        assert!(board
            .you_move(coordinate::Direction::Down)
            .you_move(coordinate::Direction::Up)
            .you_move(coordinate::Direction::Left)
            .you_move(coordinate::Direction::Left)
            .you_move(coordinate::Direction::Right)
            .you_move(coordinate::Direction::Up)
            .you_move(coordinate::Direction::Up)
            .you_move(coordinate::Direction::Down)
            .you_move(coordinate::Direction::Right)
            .you_move(coordinate::Direction::Right)
            .all_targets_triggered());
    }
}
