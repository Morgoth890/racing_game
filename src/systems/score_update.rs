use amethyst::{
    core::timing::Time,
    ecs::{Read, Write, System},
};
use crate::rgame::{GameOver, GameState};

pub struct ScoreUpdateSystem;

impl<'s> System<'s> for ScoreUpdateSystem {
    type SystemData = (
        Option<Write<'s, GameState>>,
        Option<Read<'s, GameOver>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (game_state, game_over, time): Self::SystemData) {
        if let Some(_) = game_over {
            return;
        }

        if let Some(mut game_state) = game_state {
            game_state.time += time.delta_seconds();
        }
    }
}