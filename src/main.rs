use amethyst::{
    assets::{PrefabLoaderSystemDesc},
    core::transform::{TransformBundle},
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderShaded3D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};
use crate::rgame::{CameraConfig, ShipPrefab, FloorPrefab, ObstaclePrefab, MyState};

mod rgame;
mod systems;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");
    let camera_config_path = config_dir.join("camera.ron");
    let bindings_path = config_dir.join("bindings.ron");

    let arena_config = CameraConfig::load(&camera_config_path);

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(bindings_path)?;

    let game_data = GameDataBuilder::default()
        .with_system_desc(PrefabLoaderSystemDesc::<ShipPrefab>::default(), "", &[])
        .with_system_desc(PrefabLoaderSystemDesc::<FloorPrefab>::default(), "", &[])
        .with_system_desc(PrefabLoaderSystemDesc::<ObstaclePrefab>::default(), "", &[])
        .with(systems::EnvironmentMovementSystem, "environment_movement_system", &[])
        .with(systems::ShipMovementSystem, "ship_system", &[])
        .with(systems::ObstacleSpawnSystem, "obstacle_spawn_system", &[])
        .with(systems::CollisionDetectionSystem, "collision_detection_system", &[])
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderShaded3D::default()),
        )?;

    let mut game = Application::build(assets_dir, MyState)?
        .with_resource(arena_config)
        .build(game_data)?;
    game.run();

    Ok(())
}

