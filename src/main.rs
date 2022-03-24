use bevy::prelude::*;

mod custom_components;
mod custom_entities;

mod custom_systems;
use crate::custom_systems::setup_scene;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        // .add_plugins(DefaultPlugins) is equivalent to the following
        // .add_plugin(CorePlugin::default())
        // .add_plugin(InputPlugin::default())
        // .add_plugin(WindowPlugin::default())
        .add_startup_system(setup_scene::setup_scene)
        .run();
}
