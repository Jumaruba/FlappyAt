use bracket_lib::prelude::*;
use crate::state::*;
use crate::player::Player;

pub(crate) struct Obstacle {
    pub(crate) x: i32,
    gap_y: i32,
    size: i32,
}

impl Obstacle {
    pub(crate) fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Obstacle {
            x,
            gap_y: random.range(10, 40),
            size: i32::max(2, 20 - score),
        }
    }

    pub(crate) fn is_obstacle_hit(&self, player: &Player) -> bool {
        let same_x = player.x == self.x;
        let bottom_obstacle = self.gap_y - self.size / 2;
        let top_obstacle = self.gap_y + self.size / 2;

        let hit_top_obstacle = player.y >= top_obstacle;
        let hit_bottom_obstacle = player.y <= bottom_obstacle;

        same_x && (hit_top_obstacle || hit_bottom_obstacle)
    }

    pub(crate) fn render(&mut self, ctx: &mut BTerm, player_x: i32 ) {
        let screen_x = self.x - player_x;
        let half_size = self.size / 2;
        for y in 0..self.gap_y - half_size {
            ctx.set(
                screen_x,
                y,
                WHITE,
                BLACK,
                to_cp437('|'),
            );
        }

        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(
                screen_x,
                y,
                WHITE,
                BLACK,
                to_cp437('|'),
            )
        }
    }
}