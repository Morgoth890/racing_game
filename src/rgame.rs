use amethyst::{
    assets::{RonFormat, PrefabData, ProgressCounter, PrefabLoader, Handle, Prefab},
    core::transform::{Transform},
    core::math::Vector3,
    derive::PrefabData,
    ecs::{Entity, Component, DenseVecStorage, NullStorage},
    error::Error,
    input::{is_close_requested, is_key_down},
    prelude::*,
    ui::UiCreator,
    renderer::{
        Camera,
        camera,
        light::{Light, PointLight, DirectionalLight},
        palette::Srgb,
        formats::GraphicsPrefab,
        rendy::mesh::{Normal, Position, TexCoord},
    },
    winit::VirtualKeyCode,
};
use serde::{Deserialize, Serialize};
use log::info;

use std::f32::consts::FRAC_PI_3;

//pub const GAME_WIDTH: f32 = 500.0;
//pub const GAME_HEIGHT: f32 = 500.0;

#[derive(Debug, Deserialize, Serialize, PrefabData)]
#[serde(deny_unknown_fields)]
pub struct ShipPrefab {
    graphics: GraphicsPrefab<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>,
    transform: Transform,
//    hitbox: HitBox,
}

#[derive(Component, Debug, Deserialize, Serialize, PrefabData, Default)]
#[storage(NullStorage)]
pub struct Ship;

#[derive(Debug, Deserialize, Serialize, PrefabData)]
#[serde(deny_unknown_fields)]
pub struct FloorPrefab {
    graphics: GraphicsPrefab<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>,
    transform: Transform
}

#[derive(Debug, Deserialize, Serialize, PrefabData)]
#[serde(deny_unknown_fields)]
pub struct ObstaclePrefab {
    graphics: GraphicsPrefab<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>,
//    hitbox: HitBox,
}

#[derive(Component, Debug, Deserialize, Serialize, PrefabData, Default)]
#[storage(NullStorage)]
pub struct Obstacle;

#[derive(Component, Debug, Deserialize, Serialize, Clone)]
#[storage(DenseVecStorage)]
pub struct HitBox {
    pub size: Vector3<f32>
}

impl Default for HitBox {
    fn default() -> Self {
        HitBox {
            size: Vector3::new(0.0, 0.0, 0.0)
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CameraConfig {
    translation_x: f32,
    translation_y: f32,
    translation_z: f32,
    rotation_x: f32,
    rotation_y: f32,
    rotation_z: f32,
}

impl Default for CameraConfig {
    fn default() -> Self {
        CameraConfig {
            translation_x: 0.0,
            translation_y: 0.0,
            translation_z: 0.0,
            rotation_x: 0.0,
            rotation_y: 0.0,
            rotation_z: 0.0,
        }
    }
}

#[derive(Default)]
pub struct ObstacleSpawnData {
    pub next_spawn_time: f64
}

pub struct PrefabResource {
    pub obstacle: Handle<Prefab<ObstaclePrefab>>,
}

pub struct MyState;

impl SimpleState for MyState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
//        let StateData { mut world, .. } = data;
//        Use world directly

        data.world.register::<HitBox>();

        let handle = data.world.exec(|loader: PrefabLoader<'_, ShipPrefab>| {
            loader.load("prefab/ship.ron", RonFormat, ())
        });
        data.world.create_entity()
            .with(handle)
            .with(Ship)
            .with(HitBox { size: Vector3::new(0.3, 0.3, 0.3) })
            .build();

        let handle = data.world.exec(|loader: PrefabLoader<'_, FloorPrefab>| {
            loader.load("prefab/floor.ron", RonFormat, ())
        });
        data.world.create_entity()
            .with(handle)
            .build();

        let handle = data.world.exec(|loader: PrefabLoader<'_, FloorPrefab>| {
            loader.load("prefab/road.ron", RonFormat, ())
        });
        data.world.create_entity()
            .with(handle)
            .build();

        let handle = data.world.exec(|loader: PrefabLoader<'_, ObstaclePrefab>| {
            loader.load("prefab/obstacle.ron", RonFormat, ())
        });

        let prefab_resource = PrefabResource {
            obstacle: handle
        };
        data.world.insert(prefab_resource);
        data.world.insert(ObstacleSpawnData::default());

        data.world.exec(|mut creator: UiCreator<'_>| {
            creator.create("prefab/ui.ron", ());
        });

        initialize_camera(data.world);
        initialize_lights(data.world);
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent
    ) -> SimpleTrans {
        match &event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(ui_event) => {
//                info!(
//                    "[HANDLE_EVENT] Interacted with a ui element: {:?}",
//                    ui_event
//                );
                Trans::None
            }
            StateEvent::Input(input) => {
//                info!("Input Event detected: {:?}.", input);
                Trans::None
            }
        }
    }
}

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();

    {
        let config = world.read_resource::<CameraConfig>();
        transform.set_translation_xyz(config.translation_x, config.translation_y, config.translation_z);
        transform.set_rotation_euler(config.rotation_x, config.rotation_y, config.rotation_z);
    }

    world
        .create_entity()
        .with(Camera::from(camera::Projection::perspective(
            1.3,
            FRAC_PI_3,
            0.1,
            20000.0
        )))
        .with(transform)
        .build();
}

fn initialize_lights(world: &mut World) {
    let point_light: Light = PointLight {
        intensity: 10.0,
        color: Srgb::new(1.0, 1.0, 1.0),
        ..PointLight::default()
    }.into();

    let mut transform = Transform::default();
    transform.set_translation_xyz(-1.5, 2.5, 3.0);

    world
        .create_entity()
        .with(point_light)
        .with(transform)
        .build();

    let directional_light: Light = DirectionalLight {
        color: Srgb::new(1.0, 1.0, 1.0),
        direction: Vector3::new(-1.0, -1.0, -1.0),
        intensity: 0.15,
    }
    .into();

    world
        .create_entity()
        .with(directional_light)
        .build();
}
