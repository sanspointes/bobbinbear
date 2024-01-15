pub mod inspect_vector_plugin;

use bevy::{prelude::*, ecs::system::SystemState};

use crate::components::bbid::BBId;

pub struct InspectPlugin;

impl Plugin for InspectPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<InspectState>()
            .add_plugins(inspect_vector_plugin::InspectVectorPlugin)
        ;

    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
/// Component that declares something as an artifact of another object,
/// only visible when inspecting.  Usually this is used to cleanup after a BBObject
/// is no longer inspected.
pub struct InspectArtifact(pub BBId);

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum InspectState {
    #[default]
    None,
    InspectVector,
}


pub fn update_inspect_state(world: &mut World, next_value: InspectState) {
    let mut sys_state: SystemState<(Res<State<InspectState>>, ResMut<NextState<InspectState>>)> = SystemState::new(world);
    let (inspect_state, mut next) = sys_state.get_mut(world);
    if inspect_state.get() != &next_value {
        next.set(next_value);
    }
}
