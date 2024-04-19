use bevy::{ecs::system::SystemState, math::prelude::Circle, prelude::*};

use bevy_spts_changeset::commands_ext::WorldChangesetExt;
use bevy_spts_vectorgraphic::prelude::*;
use bevy_wasm_api::bevy_wasm_api;
use wasm_bindgen::prelude::*;

use crate::plugins::{undoredo::{UndoRedoApi, UndoRedoResult}, selected::Selected};

pub struct DebugApi;

#[bevy_wasm_api]
impl DebugApi {
    pub fn spawn_circle(world: &mut World) -> Result<UndoRedoResult, anyhow::Error> {
        let mut sys_state =
            SystemState::<(ResMut<Assets<Mesh>>, ResMut<Assets<ColorMaterial>>)>::new(world);

        let (mut meshes, mut materials) = sys_state.get_mut(world);

        let mesh = meshes.add(Circle::new(25.));
        let material = materials.add(Color::RED);

        let mut builder = world.changeset();
        let mut entity = builder.spawn_empty();
        entity
            .insert(Name::from("Debug Circle"))
            .insert(Transform::default())
            .insert(GlobalTransform::default())
            .insert(Visibility::default())
            .insert(ViewVisibility::default())
            .insert(InheritedVisibility::default())
            .insert(mesh)
            .insert(material)
        ;
        let changeset = builder.build();

        UndoRedoApi::execute(world, changeset)
    }

    pub fn spawn_box(world: &mut World) -> Result<UndoRedoResult, anyhow::Error> {
        let mut sys_state =
            SystemState::<ResMut<Assets<ColorMaterial>>>::new(world);
        let mut materials = sys_state.get_mut(world);
        let material = materials.add(Color::RED);

        let mut builder = world.changeset();
        let vector_graphic = builder
            .spawn_empty()
            .insert(Name::from("Box"))
            .insert(Transform::default())
            .insert(GlobalTransform::default())
            .insert(Visibility::default())
            .insert(ViewVisibility::default())
            .insert(InheritedVisibility::default())

            .insert(VectorGraphic::default())
            .insert(VectorGraphicPathStorage::default())
            .insert(StrokeOptions::default())
            .insert(FillOptions::default())

            .insert(Selected::Deselected)
            .insert(material)
            .uid();

        let e0 = builder
            .spawn_empty()
            .insert(Endpoint::default())
            .insert(Transform::default())
            .set_parent(vector_graphic)
            .uid();
        let e1 = builder
            .spawn_empty()
            .insert(Endpoint::default())
            .insert(Transform::default().with_translation(Vec3::new(100., 0., 0.)))
            .set_parent(vector_graphic)
            .uid();
        let e2 = builder
            .spawn_empty()
            .insert(Endpoint::default())
            .insert(Transform::default().with_translation(Vec3::new(100., 100., 0.)))
            .set_parent(vector_graphic)
            .uid();
        let e3 = builder
            .spawn_empty()
            .insert(Endpoint::default())
            .insert(Transform::default().with_translation(Vec3::new(0., 100., 0.)))
            .set_parent(vector_graphic)
            .uid();

        builder
            .spawn_edge(EdgeVariant::Line, e0, e1)
            .set_parent(vector_graphic);
        builder
            .spawn_edge(EdgeVariant::Line, e1, e2)
            .set_parent(vector_graphic);
        builder
            .spawn_edge(EdgeVariant::Line, e2, e3)
            .set_parent(vector_graphic);
        builder
            .spawn_edge(EdgeVariant::Line, e3, e0)
            .set_parent(vector_graphic);

        let changeset = builder.build();

        let result = UndoRedoApi::execute(world, changeset)?;

        Ok(result)
    }
}
