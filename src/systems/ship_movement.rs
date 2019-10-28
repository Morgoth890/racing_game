use amethyst::{
    core::Transform,
    core::timing::Time,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
};
use crate::rgame::Ship;

const MOVE_SPEED: f32 = 3.0;

pub struct ShipMovementSystem;

impl<'s> System<'s> for ShipMovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Ship>,
        Read<'s, Time>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, ships, time, input): Self::SystemData) {
        for (transform, _ship) in (&mut transforms, &ships).join() {
            let transform = transform as &mut Transform;

            if let Some(mv_amount) = input.axis_value("move") {
                let translation = MOVE_SPEED * mv_amount * time.delta_seconds();
                transform.prepend_translation_x(translation);
//                println!("Ship translation: {:?}", transform.translation());
            }
        }
    }
}