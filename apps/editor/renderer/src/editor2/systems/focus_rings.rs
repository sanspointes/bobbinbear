use std::matches;

use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_prototype_lyon::{
    prelude::*,
    shapes::{self, RectangleOrigin},
};

use crate::{
    debug_log,
    editor2::{
        constants::{FOCUS_RING_STROKE_WIDTH, HOVER_COLOR, SELECT_COLOR, FOCUS_RING_Z_INDEX},
        entities::{
            vector::{VectorObjectSpawner, VectorResource},
            Bounded, HoveredState, SelectableTag, SelectedState,
        },
    },
};

#[derive(Component, Reflect)]
pub struct HasFocusRing(pub Entity);

#[derive(Component, Reflect)]
// Stores entity id and last position
pub struct FocusRingTag(pub Entity, pub Vec2);

fn calculate_focus_ring_stroke(selected: &SelectedState, hovered: &HoveredState) -> Option<Stroke> {
    match (selected, hovered) {
        (SelectedState::Selected, _) => Some(Stroke::new(SELECT_COLOR, FOCUS_RING_STROKE_WIDTH)),
        (_, HoveredState::Hovered) => Some(Stroke::new(HOVER_COLOR, FOCUS_RING_STROKE_WIDTH)),
        _ => None,
    }
}

pub fn update_focus_ring_positions(
    q_changed_position: Query<
        (&Transform, Option<&HasFocusRing>),
        (With<SelectableTag>, Changed<Transform>),
    >,
    mut q_rings: Query<(&mut Transform, &mut FocusRingTag), Without<SelectableTag>>,
) {
    let needs_update = q_changed_position
        .iter()
        .filter_map(|(transform, maybe_has_focus_ring)| {
            maybe_has_focus_ring.map(|has| (transform, has))
        });
    for (new_transform, focus_ring) in needs_update {
        if let Ok((mut fr_transform, mut tag)) = q_rings.get_mut(focus_ring.0) {
            let old_transform = tag.1;
            let td = new_transform.translation.xy() - old_transform;

            fr_transform.translation += Vec3::new(td.x, td.y, 0.);
            tag.1 = new_transform.translation.xy();
        }
    }
}

pub fn update_focus_ring_styles(
    mut commands: Commands,
    q_changed_state: Query<
        (
            Entity,
            &Transform,
            &Bounded,
            &SelectedState,
            &HoveredState,
            Option<&HasFocusRing>,
        ),
        (
            With<SelectableTag>,
            Or<(Changed<HoveredState>, Changed<SelectedState>)>,
        ),
    >,
) {
    let needs_update = q_changed_state
        .iter()
        .filter(|(_, _, bounded, _, _, _)| matches!(bounded, Bounded::Calculated { .. }));

    for (entity, transform, bounds, selected_state, hovered_state, maybe_has_focus_ring) in
        needs_update
    {
        let needs_ring = matches!(selected_state, SelectedState::Selected)
            || matches!(hovered_state, HoveredState::Hovered);

        match (maybe_has_focus_ring, needs_ring) {
            (Some(hover_ring_entity), false) => {
                commands.entity(hover_ring_entity.0).despawn();
                commands.entity(entity).remove::<HasFocusRing>();
            }
            (None, true) => {
                let maybe_focus_ring_stroke =
                    calculate_focus_ring_stroke(selected_state, hovered_state);
                let maybe_origin = bounds.min();
                if let (Some(stroke), Some(origin)) = (maybe_focus_ring_stroke, maybe_origin) {
                    let rect = shapes::Rectangle {
                        extents: bounds.size(),
                        origin: RectangleOrigin::BottomLeft,
                    };
                    let original_position = transform.translation.xy();

                    let focus_ring = commands.spawn((
                        Name::from("FocusRing"),
                        ShapeBundle {
                            path: GeometryBuilder::build_as(&rect),
                            transform: Transform {
                                translation: Vec3::new(origin.x, origin.y, FOCUS_RING_Z_INDEX),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        stroke,
                        FocusRingTag(entity, original_position),
                    )).id();

                    commands.entity(entity).insert(HasFocusRing(focus_ring));
                }
            }
            (_, _) => {}
        }
    }
}
