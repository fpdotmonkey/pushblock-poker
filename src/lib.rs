use gdnative::api::{InputMap, TileMap, TileSet};
use gdnative::prelude::*;

pub mod coordinate;
pub mod poker;
pub mod sokoban;

fn init(handle: InitHandle) {
    handle.add_class::<Sokoban>();
}

godot_init!(init);

/// A Godot class for managing a game of Sokoban
#[derive(NativeClass)]
#[inherit(TileMap)]
pub struct Sokoban {
    board: sokoban::Sokoban,
    you_tile: i64,
    stop_tile: i64,
    push_tile: i64,
    target_tile: i64,
    triggered_target_tile: i64,
}

impl Sokoban {
    fn new(_owner: &TileMap) -> Self {
        Sokoban {
            board: sokoban::Sokoban::new(
                coordinate::U2::new(0, 0),
                coordinate::U2Array::from(vec![]),
                coordinate::U2Array::from(vec![]),
                coordinate::U2Array::from(vec![]),
            ),
            you_tile: -1,
            stop_tile: -1,
            push_tile: -1,
            target_tile: -1,
            triggered_target_tile: -1,
        }
    }
}

#[methods]
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

    #[export]
    fn _ready(&mut self, owner: &TileMap) {
        let tileset: Ref<TileSet> = match owner.tileset() {
            Some(tileset) => tileset,
            None => panic!("No tileset"),
        };
        let tileset: TRef<TileSet> = unsafe { tileset.assume_safe() };

        self.you_tile = tileset.find_tile_by_name(Sokoban::YOU_TILE_NAME);
        self.stop_tile = tileset.find_tile_by_name(Sokoban::STOP_TILE_NAME);
        self.push_tile = tileset.find_tile_by_name(Sokoban::PUSH_TILE_NAME);
        self.target_tile = tileset.find_tile_by_name(Sokoban::TARGET_TILE_NAME);
        self.triggered_target_tile = tileset.find_tile_by_name(Sokoban::TRIGGERED_TARGET_TILE_NAME);

        self.board = self.initial_board(owner);
    }

    #[export]
    fn _input(&mut self, owner: &TileMap, event: Ref<InputEvent>) {
        let input_map: &InputMap = InputMap::godot_singleton();

        let input_event: TRef<InputEvent> = unsafe { event.assume_safe() };

        if input_event.is_pressed() && !input_event.is_echo() {
            if input_map.action_has_event(Sokoban::MOVE_UP, input_event) {
                self.update_board(owner, self.board.you_move(coordinate::Direction::Up));
            } else if input_map.action_has_event(Sokoban::MOVE_LEFT, input_event) {
                self.update_board(owner, self.board.you_move(coordinate::Direction::Left));
            } else if input_map.action_has_event(Sokoban::MOVE_DOWN, input_event) {
                self.update_board(owner, self.board.you_move(coordinate::Direction::Down));
            } else if input_map.action_has_event(Sokoban::MOVE_RIGHT, input_event) {
                self.update_board(owner, self.board.you_move(coordinate::Direction::Right));
            }
        }
    }

    fn initial_board(&self, owner: &TileMap) -> sokoban::Sokoban {
        sokoban::Sokoban::new(
            coordinate::U2::try_from(owner.get_used_cells_by_id(self.you_tile).get(0))
                .ok()
                .unwrap_or(coordinate::U2::new(0, 0)),
            coordinate::U2Array::try_from(owner.get_used_cells_by_id(self.stop_tile))
                .ok()
                .unwrap_or(coordinate::U2Array::from(vec![])),
            coordinate::U2Array::try_from(owner.get_used_cells_by_id(self.push_tile))
                .ok()
                .unwrap_or(coordinate::U2Array::from(vec![])),
            coordinate::U2Array::try_from(owner.get_used_cells_by_id(self.target_tile))
                .ok()
                .unwrap_or(coordinate::U2Array::from(vec![])),
        )
    }

    fn update_board(&mut self, owner: &TileMap, board: sokoban::Sokoban) {
        self.board = board;

        owner.clear();

        for stop in self.board.stops().iter() {
            owner.set_cell(
                stop.x().into(),
                stop.y().into(),
                self.stop_tile,
                false,
                false,
                false,
                Vector2::new(0.0, 0.0),
            );
        }
        for push in self.board.pushes().iter() {
            owner.set_cell(
                push.x().into(),
                push.y().into(),
                self.push_tile,
                false,
                false,
                false,
                Vector2::new(0.0, 0.0),
            );
        }
        for target in self.board.targets().iter() {
            owner.set_cell(
                target.x().into(),
                target.y().into(),
                self.target_tile,
                false,
                false,
                false,
                Vector2::new(0.0, 0.0),
            );
        }
        for triggered_target in self.board.triggered_targets().iter() {
            owner.set_cell(
                triggered_target.x().into(),
                triggered_target.y().into(),
                self.triggered_target_tile,
                false,
                false,
                false,
                Vector2::new(0.0, 0.0),
            );
        }
        owner.set_cell(
            self.board.you().x().into(),
            self.board.you().y().into(),
            self.you_tile,
            false,
            false,
            false,
            Vector2::new(0.0, 0.0),
        );

        if self.board.all_targets_triggered() {
            godot_print!("Win!");
        }
    }
}
