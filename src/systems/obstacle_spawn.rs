use amethyst::{
    core::Transform,
    core::timing::Time,
    core::math::Vector3,
    ecs::{Read, Write, ReadExpect, System, Entities, LazyUpdate},
    ecs::prelude::Builder,
};
use rand::{thread_rng, Rng};
use crate::rgame::{Obstacle, ObstacleSpawnData, PrefabResource, HitBox};

const SPAWN_INTERVAL: f32 = 1.0;

pub struct ObstacleSpawnSystem;

impl<'s> System<'s> for ObstacleSpawnSystem {
    type SystemData = (
        Entities<'s>,
        ReadExpect<'s, PrefabResource>,
        Write<'s, ObstacleSpawnData>,
        Read<'s, Time>,
        ReadExpect<'s, LazyUpdate>,
    );

    fn run(&mut self, (entities, prefab_resource, mut obstacle_spawn_data, time, lazy_update): Self::SystemData) {
        let absolute_time = time.absolute_time_seconds();

        if absolute_time >= obstacle_spawn_data.next_spawn_time {
            let rand = thread_rng().gen::<f32>();
            let spawn_range = 5.0;
            let translation_x = rand * spawn_range - spawn_range / 2.0;

            let mut transform = Transform::default();
            transform.set_translation(Vector3::new(translation_x, 1.0, -20.0));
            transform.set_scale(Vector3::new(0.3, 0.3, 0.3));
            transform.set_rotation_euler(-1.5, 0.0, 0.0);

            lazy_update.create_entity(&entities)
                .with(prefab_resource.obstacle.clone())
                .with(Obstacle)
                .with(transform)
                .with(HitBox { size: Vector3::new(0.3, 0.3, 0.3) })
                .build();

            obstacle_spawn_data.next_spawn_time = absolute_time + SPAWN_INTERVAL as f64;
        }
    }
}
