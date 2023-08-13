//! Loads and renders a glTF file as a scene.

use bevy::prelude::*;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use gltf_tool::plugins::{MyFirstUpdatePlugin, MyStartupPlugin, MySimpleLightPlugin/*, LightStuffPlugin*/};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MySimpleLightPlugin::default())
        // .add_plugins(LightStuffPlugin::default())
        .add_plugins(WorldInspectorPlugin::default())
        .add_plugins(MyStartupPlugin::default())
        .add_plugins(MyFirstUpdatePlugin::default())
        .run();
}

