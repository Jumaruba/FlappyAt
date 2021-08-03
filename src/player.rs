use bracket_lib::prelude::*;

pub(crate) struct Player {
    pub x: i32,
    pub y: i32,
    pub vel: f32,
}

impl Player {
    pub(crate) fn new(x: i32, y: i32) -> Player {
        Player {
            x,
            y,
            vel: 0.0,
        }
    }

    pub(crate) fn render(&mut self, ctx: &mut BTerm) {
        // https://docs.rs/bracket-lib/0.8.1/bracket_lib/prelude/struct.BTerm.html#method.set
        ctx.set(
            0,
            self.y,
            YELLOW,
            BLACK,
            to_cp437('@'),
        );
    }

    pub(crate) fn gravity_and_move(&mut self) {
        self.y += self.vel as i32;
        self.x += 1;

        if self.vel < 2.0 {
            self.vel = 1.0;
        }

        // Cannot be higher than the roof.
        if self.y < 0 {
            self.y = 0;
        }
    }

    pub(crate) fn flap(&mut self) {
        self.vel = -2.0;
    }


}
