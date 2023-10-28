use godot::prelude::*;

pub mod coordinate;
pub mod io;
pub mod poker;
pub mod sokoban;

struct PushblockPoker;

#[gdextension]
unsafe impl ExtensionLibrary for PushblockPoker {
    fn min_level() -> InitLevel {
        InitLevel::Editor
    }
}
