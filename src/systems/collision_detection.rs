use amethyst::{
    core::Transform,
    ecs::{Join, ReadStorage, System, Entities}
};
use crate::rgame::{Ship, Obstacle, HitBox};

pub struct CollisionDetectionSystem;

impl<'s> System<'s> for CollisionDetectionSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Ship>,
        ReadStorage<'s, Obstacle>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, HitBox>,
    );

    fn run(&mut self, (entities, ships, obstacles, transforms, hitboxes): Self::SystemData) {
        for (_, transform, hitbox) in (&ships, &transforms, &hitboxes).join() {
            let ship_transform = transform as &Transform;
            let ship_hitbox = hitbox as &HitBox;

            for (obstacle_entity, _, transform, hitbox) in (&*entities, &obstacles, &transforms, &hitboxes).join() {
                let obstacle_transform = transform as &Transform;
                let obstacle_hitbox = hitbox as &HitBox;

                if is_in_range(ship_transform.translation().x - ship_hitbox.size.x, ship_transform.translation().x + ship_hitbox.size.x,
                               obstacle_transform.translation().x - obstacle_hitbox.size.x, obstacle_transform.translation().x + obstacle_hitbox.size.x)
                && is_in_range(ship_transform.translation().y - ship_hitbox.size.y, ship_transform.translation().y + ship_hitbox.size.y,
                                   obstacle_transform.translation().y - obstacle_hitbox.size.y, obstacle_transform.translation().y + obstacle_hitbox.size.y)
                && is_in_range(ship_transform.translation().z - ship_hitbox.size.z, ship_transform.translation().z + ship_hitbox.size.z,
                                       obstacle_transform.translation().z - obstacle_hitbox.size.z, obstacle_transform.translation().z + obstacle_hitbox.size.z) {
                    println!("Collided!");
                    entities.delete(obstacle_entity).unwrap();
                }
            }
        }
    }
}

fn is_in_range(a1: f32, a2: f32, b1: f32, b2: f32) -> bool {
    assert!(a2 >= a1);
    assert!(b2 >= b1);

    (b1 >= a1 && b1 <= a2)
        || (b2 >= a1 && b2 <= a2)
        || (b1 <= a1 && b2 >= a2)
}
