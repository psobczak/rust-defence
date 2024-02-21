mod camera;
mod ground;

use bevy::prelude::*;
use camera::CameraPlugin;
use ground::GroundPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((CameraPlugin, GroundPlugin))
        .run();
}
