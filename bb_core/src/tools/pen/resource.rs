use bevy::{
    core::Name,
    ecs::prelude::*,
    math::{Vec2, Vec3Swizzles},
    render::{color::Color, view::Visibility},
    transform::components::GlobalTransform,
};
use bevy_mod_raycast::deferred::RaycastMesh;
use bevy_spts_changeset::{commands_ext::WorldChangesetExt, resource::ChangesetResource};
use bevy_spts_uid::Uid;
use bevy_spts_vectorgraphic::{
    components::{EdgeVariant, Endpoint, VectorGraphic, VectorGraphicPathStorage},
    lyon_components::StrokeOptions,
    material::StrokeColor,
    prelude::VectorGraphicChangesetExt,
};

use crate::{
    ecs::{InternalObject, ObjectBundle, ObjectType, Position},
    plugins::{selected::Selectable, undoredo::UndoRedoTag},
    tools::InputState,
    views::{vector_edge::VectorEdgeVM, vector_endpoint::VectorEndpointVM},
};

use super::{PenTool, PenToolBuildingFromEndpointTag};

#[derive(Resource, Debug, Clone)]
pub struct PenToolResource {
    pub preview: PenToolPreview,
}

impl PenToolResource {
    pub fn resource_scope<T>(
        world: &mut World,
        f: impl FnOnce(&mut World, Mut<PenToolResource>) -> T,
    ) -> T {
        world.resource_scope::<PenToolResource, T>(f)
    }
}

impl FromWorld for PenToolResource {
    fn from_world(world: &mut World) -> Self {
        Self {
            preview: PenToolPreview::from_world(world),
        }
    }
}

#[derive(Resource, Debug, Clone)]
/// PenToolPreview dummy endpoints and line for visualising the next endpoint/line before it's
/// committed to the VectorObject.
pub struct PenToolPreview {
    vector_object: Uid,
    endpoint_0: Uid,
    edge: Uid,
    endpoint_1: Uid,
}

impl PenToolPreview {
    /// Uses the current state of the world to update the pen tool preview
    pub fn refresh(world: &mut World) {
        PenToolResource::resource_scope(world, |world, res| {
            let state = world.resource::<PenTool>();
            let input_state = *world.resource::<InputState>();

            match state {
                PenTool::Deactive => {
                    res.preview.hide_all(world);
                }
                PenTool::Default => {
                    res.preview.show_only_endpoint_0(world);
                    res.preview
                        .set_endpoint_0_world_pos(world, input_state.world_pos());
                }
                PenTool::BuildingEdge => {
                    let pos = world
                        .query_filtered::<&Position, With<PenToolBuildingFromEndpointTag>>()
                        .get_single(world).copied();
                    let Ok(pos) = pos else {
                        return;
                    };

                    res.preview.show_all(world);
                    res.preview.update_to_line(world);
                    res.preview.set_endpoint_0_world_pos(world, *pos);
                    res.preview
                        .set_endpoint_1_world_pos(world, input_state.world_pos());
                }
                _ => {
                    res.preview.hide_all(world);
                }
            };
        });
    }

    pub fn set_endpoint_0_world_pos(&self, world: &mut World, world_position: Vec2) {
        let entity = self.endpoint_0.entity(world).unwrap();
        let mut position = world.get_mut::<Position>(entity).unwrap();
        position.0 = world_position;
    }
    pub fn set_endpoint_1_world_pos(&self, world: &mut World, world_position: Vec2) {
        let entity = self.endpoint_1.entity(world).unwrap();
        let mut position = world.get_mut::<Position>(entity).unwrap();
        position.0 = world_position;
    }

    pub fn hide_all(&self, world: &mut World) {
        let entities = [
            self.vector_object.entity(world).unwrap(),
            self.endpoint_0.entity(world).unwrap(),
            self.endpoint_1.entity(world).unwrap(),
            self.edge.entity(world).unwrap(),
        ];

        for entity in entities {
            let mut visibility = world.get_mut::<Visibility>(entity).unwrap();
            *visibility = Visibility::Hidden;
        }
    }

    pub fn show_all(&self, world: &mut World) {
        let entities = [
            self.vector_object.entity(world).unwrap(),
            self.endpoint_0.entity(world).unwrap(),
            self.endpoint_1.entity(world).unwrap(),
            self.edge.entity(world).unwrap(),
        ];

        for entity in entities {
            let mut visibility = world.get_mut::<Visibility>(entity).unwrap();
            *visibility = Visibility::Visible;
        }
    }

    pub fn show_only_endpoint_0(&self, world: &mut World) {
        self.hide_all(world);

        let entity = self.endpoint_0.entity(world).unwrap();
        let mut visiblity = world.get_mut::<Visibility>(entity).unwrap();
        *visiblity = Visibility::Visible;
    }

    pub fn update_to_line(&self, world: &mut World) {
        let entity = self.edge.entity(world).unwrap();
        let mut edge_variant = world.get_mut::<EdgeVariant>(entity).unwrap();
        *edge_variant = EdgeVariant::Line;
    }

    pub fn update_to_quadratic(&self, world: &mut World, ctrl1: Vec2) {
        let entity = self.edge.entity(world).unwrap();
        let mut edge_variant = world.get_mut::<EdgeVariant>(entity).unwrap();
        *edge_variant = EdgeVariant::Quadratic { ctrl1 };
    }
}

impl FromWorld for PenToolPreview {
    fn from_world(world: &mut World) -> Self {
        let mut builder = world.changeset();

        let vector_object = builder
            .spawn((
                Name::from("PenToolPreview_VectorObject"),
                ObjectBundle::new(ObjectType::Vector),
                VectorGraphic::default(),
                VectorGraphicPathStorage::default(),
                StrokeOptions::default(),
                StrokeColor(Color::WHITE),
                InternalObject,
            ))
            .remove::<RaycastMesh<Selectable>>()
            .apply(Selectable::Locked)
            .apply(Visibility::Hidden)
            .uid();

        let endpoint_0 = builder
            .spawn((
                Name::from("Endpoint0"),
                ObjectBundle::new(ObjectType::VectorEndpoint),
                Endpoint::default(),
                InternalObject,
            ))
            .remove::<RaycastMesh<Selectable>>()
            .apply(Selectable::Locked)
            .apply(Visibility::Hidden)
            .set_parent(vector_object)
            .uid();

        let endpoint_1 = builder
            .spawn((
                Name::from("Endpoint1"),
                ObjectBundle::new(ObjectType::VectorEndpoint),
                Endpoint::default(),
                InternalObject,
            ))
            .remove::<RaycastMesh<Selectable>>()
            .apply(Selectable::Locked)
            .apply(Visibility::Hidden)
            .set_parent(vector_object)
            .uid();

        let edge = builder
            .spawn_edge(EdgeVariant::Line, endpoint_0, endpoint_1)
            .insert((
                Name::from("Edge"),
                ObjectBundle::new(ObjectType::VectorEdge),
                InternalObject,
            ))
            .remove::<RaycastMesh<Selectable>>()
            .apply(Selectable::Locked)
            .apply(Visibility::Hidden)
            // .insert(ObjectBundle::new(ObjectType::VectorSegment))
            .set_parent(vector_object)
            .uid();

        let changeset = builder.build();
        ChangesetResource::<UndoRedoTag>::context_scope(world, |world, cx| {
            changeset
                .apply(world, cx)
                .expect("Error creating PenToolPreview.");
        });

        world
            .entity_mut(endpoint_0.entity(world).unwrap())
            .insert(VectorEndpointVM);
        world
            .entity_mut(endpoint_1.entity(world).unwrap())
            .insert(VectorEndpointVM);
        world
            .entity_mut(edge.entity(world).unwrap())
            .insert(VectorEdgeVM);

        Self {
            vector_object,
            endpoint_0,
            endpoint_1,
            edge,
        }
    }
}
