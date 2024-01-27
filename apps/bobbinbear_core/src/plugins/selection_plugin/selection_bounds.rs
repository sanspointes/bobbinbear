use std::ops::Sub;

use bevy::{
    math::{vec2, vec3},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

use crate::{
    constants::{SELECTION_BOUNDS_STROKE_WIDTH, SELECT_COLOR},
    msgs::effect::EffectMsg,
    plugins::{bounds_2d_plugin::GlobalBounds2D, screen_space_root_plugin::ScreenSpaceRoot},
    utils::mesh::translate_mesh,
};

use super::{selection_bounds_material::SelectionBoundsMaterial, Selected};

// Systems responsible for creating and updating the screenspace selection bounds of all selected
// elements.
#[derive(Component)]
pub(super) struct SelectionBoundsTag;

pub(super) fn sys_setup_selection_bounds(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<SelectionBoundsMaterial>>,
    q_ss_root: Query<Entity, With<ScreenSpaceRoot>>,
) {
    #[cfg(feature = "debug_trace")]
    let _span = info_span!("sys_setup_selection_bounds").entered();

    let ss_root = q_ss_root.single();

    let mesh = Mesh::from(shape::Quad::new(vec2(1., 1.)));
    let handle = meshes.add(translate_mesh(&mesh, vec3(0.5, 0.5, 0.)).unwrap());
    commands
        .spawn((
            Name::from("Selection bounds"),
            SelectionBoundsTag,
            MaterialMesh2dBundle {
                mesh: handle.into(),
                material: materials.add(SelectionBoundsMaterial {
                    color: SELECT_COLOR,
                    border_color: SELECT_COLOR.with_a(0.0),
                    border_width: SELECTION_BOUNDS_STROKE_WIDTH,
                    dimensions: Vec2::default(),
                }),
                // visibility: Visibility::Hidden,
                ..default()
            },
        ))
        .set_parent(ss_root);
}

pub(super) fn sys_selection_bounds_handle_change(
    mut selection_bounds_material: ResMut<Assets<SelectionBoundsMaterial>>,

    mut ev_effect_reader: EventReader<EffectMsg>,

    q_all_selectables: Query<(&GlobalBounds2D, &Selected)>,

    // To Mutate
    mut q_selection_bounds: Query<
        (
            &Handle<SelectionBoundsMaterial>,
            &mut Transform,
            &mut Visibility,
        ),
        With<SelectionBoundsTag>,
    >,
    // To Calculate
    q_ss_root: Query<&ScreenSpaceRoot>,
) {
    #[cfg(feature = "debug_trace")]
    let _span = info_span!("sys_selection_bounds_handle_change").entered();

    let has_emitted_dirty_event = ev_effect_reader.read().any(|ev| {
        matches!(
            ev,
            EffectMsg::ObjectSelectionChanged { .. }
                | EffectMsg::ObjectMoved(_)
                | EffectMsg::ObjectRemoved { .. }
                | EffectMsg::CameraMoved { .. }
                | EffectMsg::CameraZoomed { .. }
        )
    });

    if !has_emitted_dirty_event {
        return;
    }

    #[cfg(feature = "debug_bounds")]
    debug!("Updating selection bounds!");

    let mut any_selected = false;
    let mut min = Vec2::MAX;
    let mut max = Vec2::MIN;
    for (bounds, selected) in q_all_selectables.iter() {
        if let (Selected::Yes, GlobalBounds2D::Calculated(bounds)) = (selected, bounds) {
            #[cfg(feature = "debug_bounds")]
            debug!("\tHandling {bounds:?}!");
            min = bounds.min.min(min);
            max = bounds.max.max(max);
            any_selected = true;
        }
    }

    #[cfg(feature = "debug_bounds")]
    debug!("Recalculated selection bounds (global min max: {min:?} {max:?})");

    let (mat_handle, mut transform, mut visibility) = q_selection_bounds.single_mut();
    *visibility = match any_selected {
        true => Visibility::Visible,
        false => Visibility::Hidden,
    };

    if matches!(*visibility, Visibility::Visible) {
        let ss_root = q_ss_root.single();
        let screen_min = ss_root.world_to_screen(min);
        let screen_max = ss_root.world_to_screen(max);
        let extents = screen_max.sub(screen_min);
        let extents_size = extents.length_squared();

        #[cfg(feature = "debug_bounds")]
        debug!("Transformed global bounds to screen space {screen_min:?} {screen_max:?}");

        if extents_size == 0. || !extents_size.is_finite() {
            // println!("Extents not finite {extents:?}");
            return;
        }

        transform.translation.x = screen_min.x;
        transform.translation.y = screen_min.y;

        if let Some(mat) = selection_bounds_material.get_mut(mat_handle) {
            mat.dimensions = extents;
        }

        // let shape = shapes::Rectangle {
        //     extents,
        //     origin: RectangleOrigin::BottomLeft,
        // };
        //
        // *handle = GeometryBuilder::build_as(&shape);

        #[cfg(feature = "debug_bounds")]
        debug!("Updated bounds to {transform:?} {extents:?}");
    }
}
