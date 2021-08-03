use bracket_lib::prelude::*;
use crate::state::State;

pub(crate) enum GameMode {
    Menu,
    Playing,
    End,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.game_over(ctx),
        }
    }
}
