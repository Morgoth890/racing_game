mod environment_movement;
mod ship_movement;
mod obstacle_spawn;
mod collision_detection;
mod score_update;

pub use self::{
    environment_movement::EnvironmentMovementSystem,
    ship_movement::ShipMovementSystem,
    obstacle_spawn::ObstacleSpawnSystem,
    collision_detection::CollisionDetectionSystem,
    score_update::ScoreUpdateSystem,
};
