use bracket_lib::prelude::*;
use crate::player::Player;
use crate::game_mode::GameMode;
use crate::obstacle::Obstacle;

const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;
const BIRD_START_Y: i32 = 35;
const BIRD_START_X: i32 = 5;

pub(crate) struct State {
    player: Player,
    pub(crate) mode: GameMode,
    frame_time: f32,
    score: i32,
    obstacle: Obstacle,
}

impl State {
    pub(crate) fn new() -> Self {
        State {
            player: Player::new(BIRD_START_X, BIRD_START_Y),
            mode: GameMode::Menu,
            frame_time: 0.0,
            score: 0,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
        }
    }

    fn restart(&mut self) {
        self.player = Player::new(BIRD_START_X, BIRD_START_Y);
        self.mode = GameMode::Playing;
        self.frame_time = 0.0;
    }

    fn handle_game_over(&mut self){
        if self.player.y > SCREEN_HEIGHT {
            self.mode = GameMode::End;
        }
    }

    pub(crate) fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }

        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }

        self.player.render(ctx);
        ctx.print_centered(0, "Press space to fly");
        self.handle_game_over();
    }

    pub(crate) fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(15, "Play (P)");
        ctx.print_centered(17, "Exit (E)");

        // Handle the options.
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::E => ctx.quitting = true,
                _ => {}
            }
        }
    }

    pub(crate) fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Game Over!");
        ctx.print_centered(15, "Play (P)");
        ctx.print_centered(17, "Exit (E)");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::E => ctx.quitting = true,
                _ => {}
            }
        }
    }
}
