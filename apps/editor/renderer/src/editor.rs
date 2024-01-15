
use bevy::prelude::*;

use crate::{
    messages::input::InputProcessorPlugin, plugins::raycast::RaycastPlugin,
    systems::camera::sys_setup_camera,
};

// pub enum BBSet {
//     StartupInit,
//     StartupModify,
// }

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PreStartup, sys_setup_camera)
            .add_plugins((RaycastPlugin, InputProcessorPlugin));
    }
}
