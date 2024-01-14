use bevy::prelude::*;

use super::ScreenSpaceRoot;

#[derive(Component, Clone, Copy, Reflect, Default)]
#[reflect(Component)]
/// If an entity has this component it will modify the Transform position so it's the world to
/// screen of Vec3.  I.e. modify the Vec3 in this struct and it will be positioned at the same
/// point in screenspace.
pub struct WorldToScreen(pub Vec3);

pub fn sys_update_world_to_screen(
    q_ss_root: Query<&ScreenSpaceRoot>,
    mut q_world_to_screen: Query<(&mut Transform, &WorldToScreen), Changed<WorldToScreen>>,
) {
    let ss_root = q_ss_root.single();

    for (mut transform, world_pos) in q_world_to_screen.iter_mut() {
        transform.translation = ss_root.world_to_screen(world_pos.0.xy()).extend(0.);
    }
}
