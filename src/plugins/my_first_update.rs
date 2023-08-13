use bevy::prelude::*;

#[derive(Resource)]
struct HasRun(bool);

impl Default for HasRun {
    fn default() -> Self {
        Self(false)
    }
}

#[derive(Default)]
pub struct MyFirstUpdatePlugin;

impl Plugin for MyFirstUpdatePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(HasRun::default())
            .add_systems(Update, first_update);
    }
}

// Test place
fn first_update(
    meshes: Res<Assets<Mesh>>,
    mut has_run: ResMut<HasRun>
) {
    if meshes.is_empty() {
        return;
    }
    if !has_run.0 {
        println!();
        for (_handle, mesh) in meshes.iter() {
            println!("Primitive Topology: {:#?}", mesh.primitive_topology());
            println!();
            // println!("Attributes:");
            // let mut iter = mesh.attributes();
            // println!("{:?}", iter.next());
            // println!("{:?}", iter.next());
            // break;
            println!("Has morph targets = {}", mesh.has_morph_targets());

            if let Some(target_names) = mesh.morph_target_names() {
                println!();
                println!("Morph Target Names:");
                for target_name in target_names {
                    println!("{:#?}", target_name);
                }
            }
        }
        has_run.0 = true;
    }
}

