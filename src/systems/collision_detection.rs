use amethyst::{
    core::Transform,
    core::math::Vector3,
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

                if is_colliding(ship_transform.translation(), ship_hitbox, obstacle_transform.translation(), obstacle_hitbox) {
                    println!("Collided!");
                    entities.delete(obstacle_entity).unwrap();
                }
            }
        }
    }
}

fn is_colliding(a1_pos: &Vector3<f32>, a1_hitbox: &HitBox, a2_pos: &Vector3<f32>, a2_hitbox: &HitBox) -> bool {
    is_in_range(a1_pos.x - a1_hitbox.size.x, a1_pos.x + a1_hitbox.size.x,
                   a2_pos.x - a2_hitbox.size.x, a2_pos.x + a2_hitbox.size.x)
        && is_in_range(a1_pos.y - a1_hitbox.size.y, a1_pos.y + a1_hitbox.size.y,
                       a2_pos.y - a2_hitbox.size.y, a2_pos.y + a2_hitbox.size.y)
        && is_in_range(a1_pos.z - a1_hitbox.size.z, a1_pos.z + a1_hitbox.size.z,
                       a2_pos.z - a2_hitbox.size.z, a2_pos.z + a2_hitbox.size.z)
}

fn is_in_range(a1: f32, a2: f32, b1: f32, b2: f32) -> bool {
    assert!(a2 >= a1);
    assert!(b2 >= b1);

    (b1 >= a1 && b1 <= a2)
        || (b2 >= a1 && b2 <= a2)
        || (b1 <= a1 && b2 >= a2)
}
