use godot::engine::InputEvent;
use godot::engine::TileMap;
use godot::engine::TileMapVirtual;
use godot::prelude::*;

use crate::coordinate::{Direction, I2Array, I2};
use crate::sokoban;

/// A Godot class for managing a game of Sokoban
#[derive(GodotClass)]
#[class(base=TileMap)]
pub struct Sokoban {
    initial_board: sokoban::Sokoban,
    board: sokoban::Sokoban,
    you_tile: i32,
    stop_tile: i32,
    push_tile: i32,
    target_tile: i32,
    triggered_target_tile: i32,

    #[base]
    base: Base<TileMap>,
}

#[godot_api]
impl TileMapVirtual for Sokoban {
    fn init(base: Base<TileMap>) -> Self {
        Sokoban {
            initial_board: sokoban::Sokoban::new(
                I2::new(0, 0),
                I2Array::from(vec![]),
                I2Array::from(vec![]),
                I2Array::from(vec![]),
            ),
            board: sokoban::Sokoban::new(
                I2::new(0, 0),
                I2Array::from(vec![]),
                I2Array::from(vec![]),
                I2Array::from(vec![]),
            ),
            you_tile: 4,
            stop_tile: 1,
            push_tile: 0,
            target_tile: 2,
            triggered_target_tile: 3,
            base,
        }
    }

    fn ready(&mut self) {
        self.initial_board = self.get_initial_board();
        self.update_board(self.initial_board.clone());
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if event.is_pressed() && !event.is_echo() {
            if event.is_action_pressed(Sokoban::MOVE_UP.into()) {
                self.update_board(self.board.you_move(Direction::Up));
            } else if event.is_action_pressed(Sokoban::MOVE_LEFT.into()) {
                self.update_board(self.board.you_move(Direction::Left));
            } else if event.is_action_pressed(Sokoban::MOVE_DOWN.into()) {
                self.update_board(self.board.you_move(Direction::Down));
            } else if event.is_action_pressed(Sokoban::MOVE_RIGHT.into()) {
                self.update_board(self.board.you_move(Direction::Right));
            } else if event.is_action_pressed(Sokoban::RESET.into()) {
                self.update_board(self.initial_board.clone());
            }
        }
    }
}

impl Sokoban {
    /// `you`, the name of the tile in the tileset used for you
    pub const YOU_TILE_NAME: &'static str = "you";
    /// `stop`, the name of the tile in the tileset used for stops    
    pub const STOP_TILE_NAME: &'static str = "stop";
    /// `push`, the name of the tile in the tileset used for pushs    
    pub const PUSH_TILE_NAME: &'static str = "push";
    /// `target`, the name of the tile in the tileset used for targets    
    pub const TARGET_TILE_NAME: &'static str = "target";
    /// `triggered_target`, the name of the tile in the tileset used for
    /// triggered targets    
    pub const TRIGGERED_TARGET_TILE_NAME: &'static str = "triggered_target";

    /// The [`InputMap`] key for the up input, `move_up`
    pub const MOVE_UP: &'static str = "move_up";
    /// The [`InputMap`] key for the left input, `move_left`
    pub const MOVE_LEFT: &'static str = "move_left";
    /// The [`InputMap`] key for the down input, `move_down`
    pub const MOVE_DOWN: &'static str = "move_down";
    /// The [`InputMap`] key for the right input, `move_right`
    pub const MOVE_RIGHT: &'static str = "move_right";
    pub const RESET: &'static str = "reset";

    fn get_initial_board(&self) -> sokoban::Sokoban {
        let mut pushes = self
            .base
            .get_used_cells_by_id_ex(0)
            .source_id(self.push_tile)
            .done();
        let mut targets = self
            .base
            .get_used_cells_by_id_ex(0)
            .source_id(self.target_tile)
            .done();
        let triggered_targets = self
            .base
            .get_used_cells_by_id_ex(0)
            .source_id(self.triggered_target_tile)
            .done();
        pushes.extend_array(triggered_targets.clone());
        targets.extend_array(triggered_targets.clone());
        sokoban::Sokoban::new(
            I2::try_from(
                self.base
                    .get_used_cells_by_id_ex(0)
                    .source_id(self.you_tile)
                    .done()
                    .get(0),
            )
            .unwrap_or(I2::new(0, 0)),
            I2Array::try_from(
                self.base
                    .get_used_cells_by_id_ex(0)
                    .source_id(self.stop_tile)
                    .done(),
            )
            .unwrap_or(I2Array::from(vec![])),
            I2Array::try_from(pushes).unwrap_or(I2Array::from(vec![])),
            I2Array::try_from(targets).unwrap_or(I2Array::from(vec![])),
        )
    }

    fn update_board(&mut self, board: sokoban::Sokoban) {
        self.board = board;
        self.base.clear_layer(0);
        for stop in self.board.stops().iter() {
            self.base
                .set_cell_ex(0, (*stop).into())
                .source_id(self.stop_tile)
                .atlas_coords(Vector2i::new(0, 0))
                .done();
        }
        for push in self.board.pushes().iter() {
            self.base
                .set_cell_ex(0, (*push).into())
                .source_id(self.push_tile)
                .atlas_coords(Vector2i::new(0, 0))
                .done();
        }
        for target in self.board.targets().iter() {
            self.base
                .set_cell_ex(0, (*target).into())
                .source_id(self.target_tile)
                .atlas_coords(Vector2i::new(0, 0))
                .done();
        }
        for triggered_target in self.board.triggered_targets().iter() {
            self.base
                .set_cell_ex(0, (**triggered_target).into())
                .source_id(self.triggered_target_tile)
                .atlas_coords(Vector2i::new(0, 0))
                .done();
        }
        self.base
            .set_cell_ex(0, dbg!(self.board.you().into()))
            .source_id(self.you_tile)
            .atlas_coords(Vector2i::new(0, 0))
            .done();

        if self.board.all_targets_triggered() {
            godot_print!("Win!");
        }
    }
}
