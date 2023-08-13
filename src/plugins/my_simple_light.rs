use bevy::prelude::*;

#[derive(Default)]
pub struct MySimpleLightPlugin;

impl Plugin for MySimpleLightPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(AmbientLight {
                color: Color::WHITE,
                brightness: 1.0 / 5.0f32,
            })
            .add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },

        ..default()
    });
}

