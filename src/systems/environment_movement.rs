use amethyst::{
    core::Transform,
    core::timing::Time,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
};
use crate::rgame::{Obstacle, GameOver};

const MOVE_SPEED: f32 = 6.0;

pub struct EnvironmentMovementSystem;

impl<'s> System<'s> for EnvironmentMovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Obstacle>,
        Option<Read<'s, GameOver>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, obstacles, game_over, time): Self::SystemData) {
        if let Some(_) = game_over {
            return;
        }

        for (transform, _obstacle) in (&mut transforms, &obstacles).join() {
            let transform = transform as &mut Transform;
            let translation = MOVE_SPEED * time.delta_seconds();
            transform.prepend_translation_z(translation);
        }
    }
}