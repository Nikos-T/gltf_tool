use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

const RIGGED_FIGURE: &'static str = "models/RiggedFigure/glTF/RiggedFigure.gltf#Scene0";
const RIGGED_FIGURE_NAME: &'static str = "RiggedFigureScene";

#[derive(Default)]
pub struct MyStartupPlugin;

impl Plugin for MyStartupPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(PanOrbitCameraPlugin)
            .add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(1., 1.5, 2.),
            ..default()
        },
        PanOrbitCamera {
            focus: Vec3::new(-1.0, 1., 0.0),
            ..default()
        },

    ));

    let scene: Handle<Scene> = asset_server.load(RIGGED_FIGURE);

    commands.spawn((
        SceneBundle {
            scene,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Name::new(RIGGED_FIGURE_NAME),
    ));
}

