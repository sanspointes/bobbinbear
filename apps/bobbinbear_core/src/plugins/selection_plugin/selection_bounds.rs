use std::ops::Sub;

use bevy::{
    prelude::*,
    window::PrimaryWindow,
};
use bevy_prototype_lyon::{prelude::*, shapes};

use crate::{
    constants::{SELECTION_BOUNDS_STROKE_WIDTH, SELECT_COLOR},
    plugins::{screen_space_root_plugin::ScreenSpaceRoot, bounds_2d_plugin::GlobalBounds2D},
    systems::camera::CameraTag,
    utils::coordinates,
};

use super::Selected;

// Systems responsible for creating and updating the screenspace selection bounds of all selected
// elements.
#[derive(Component)]
pub(super) struct SelectionBoundsTag;

pub(super) fn sys_setup_selection_bounds(
    mut commands: Commands,
    q_ss_root: Query<Entity, With<ScreenSpaceRoot>>,
) {
    #[cfg(feature = "debug_trace")]
    let _span = info_span!("sys_setup_selection_bounds").entered();

    let ss_root = q_ss_root.single();

    let shape = shapes::Rectangle {
        ..Default::default()
    };
    commands
        .spawn((
            Name::from("Selection bounds"),
            SelectionBoundsTag,
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                // visibility: Visibility::Hidden,
                ..default()
            },
            Stroke::new(SELECT_COLOR, SELECTION_BOUNDS_STROKE_WIDTH),
        ))
        .set_parent(ss_root);
}

pub(super) fn sys_selection_bounds_handle_change(
    mut system_set: ParamSet<(
        // Query for selection or bounds changes
        Query<Entity, Or<(Changed<Selected>, Changed<GlobalBounds2D>)>>,
        // Query all to calculate selection box
        Query<(&GlobalBounds2D, &Selected)>,
    )>,

    // To Mutate
    mut q_selection_bounds: Query<
        (&mut Path, &mut Transform, &mut Visibility),
        With<SelectionBoundsTag>,
    >,
    // To Calculate
    q_ss_root: Query<&ScreenSpaceRoot>,
) {
    #[cfg(feature = "debug_trace")]
    let _span = info_span!("sys_selection_bounds_handle_change").entered();

    let needs_update = system_set.p0().iter().next().is_some();
    if !needs_update {
        return;
    }


    #[cfg(feature = "debug_bounds")]
    debug!("Updating selection bounds!");

    let q_all_selecteables = system_set.p1();

    let mut any_selected = false;
    let mut min = Vec2::MAX;
    let mut max = Vec2::MIN;
    for (bounds, selected) in q_all_selecteables.iter() {
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

    let (mut path, mut transform, mut visibility) = q_selection_bounds.single_mut();
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

        let shape = shapes::Rectangle {
            extents,
            origin: RectangleOrigin::BottomLeft,
        };

        *path = GeometryBuilder::build_as(&shape);

        #[cfg(feature = "debug_bounds")]
        debug!("Updated bounds to {transform:?} {extents:?}");
    }
}
