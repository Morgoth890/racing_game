use amethyst::{
    core::Transform,
    ecs::{Join, ReadStorage, System}
};
use crate::rgame::{Ship, Obstacle, HitBox};

pub struct CollisionDetectionSystem;

impl<'s> System<'s> for CollisionDetectionSystem {
    type SystemData = (
        ReadStorage<'s, Ship>,
        ReadStorage<'s, Obstacle>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, HitBox>
    );

    fn run(&mut self, (ships, obstacles, transforms, hitboxes): Self::SystemData) {
        for (ship, transform, hitbox) in (&ships, &transforms, &hitboxes).join() {
            let ship_transform = transform as &Transform;
            let ship_hitbox = hitbox as &HitBox;

            for (obstacle, transform, hitbox) in (&obstacles, &transforms, &hitboxes).join() {
                let obstacle_transform = transform as &Transform;
                let obstacle_hitbox = hitbox as &HitBox;

                if ship_transform.translation().x > 2.0 {
                }
            }
        }
    }
}