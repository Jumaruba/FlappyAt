mod player;
mod state;
mod game_mode;
mod obstacle;

use crate::player::Player;
use crate::state::State;
use bracket_lib::prelude::*;


fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(context, State::new())
}
