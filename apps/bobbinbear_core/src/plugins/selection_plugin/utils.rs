use bevy::{prelude::*, render::mesh::Indices};
use bevy_mod_raycast::prelude::RaycastSource;

use super::Selectable;

const BORDER_WIDTH: f32 = 1.;

pub(super) fn build_selection_bounds_mesh(extends: Vec2) {
    let mut mesh = Mesh::new(bevy::render::render_resource::PrimitiveTopology::TriangleList);

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            // Top left corner, first indice 0
            [0., 0., 0.],
            [BORDER_WIDTH, 0., 0.],
            [BORDER_WIDTH, BORDER_WIDTH, 0.],
            [0., BORDER_WIDTH, 0.],
            // Top connector, first indice 4
            [extends.x - BORDER_WIDTH, 0., 0.],
            [extends.x - BORDER_WIDTH, BORDER_WIDTH, 0.],
        ],
    );

    mesh.set_indices(Some(Indices::U32(vec![
        // Top left corner
        0, 1, 2, 0, 3, 2, // Top connector
        1, 4, 5, 1, 2, 5,
    ])))
}

pub fn get_raycast_hits_selectable(
    world: &mut World,
) -> &[(Entity, bevy_mod_raycast::primitives::IntersectionData)] {
    let rc_source = world.query::<&RaycastSource<Selectable>>().single(world);
    rc_source.intersections()
}

pub fn get_raycast_hits_bb_node(
    world: &mut World,
) -> &[(Entity, bevy_mod_raycast::primitives::IntersectionData)] {
    let rc_source = world.query::<&RaycastSource<Selectable>>().single(world);
    rc_source.intersections()
}
