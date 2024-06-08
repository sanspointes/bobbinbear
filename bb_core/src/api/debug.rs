use bevy::{ecs::system::SystemState, prelude::*};

use bevy_spts_changeset::commands_ext::WorldChangesetExt;
use bevy_spts_vectorgraphic::prelude::*;
use bevy_wasm_api::bevy_wasm_api;
use wasm_bindgen::prelude::*;

use crate::{
    ecs::object::{InternalObject, ObjectBundle, ObjectType},
    plugins::undoredo::{UndoRedoApi, UndoRedoResult},
    utils::schedule_graph,
};

pub struct DebugApi;

#[bevy_wasm_api]
impl DebugApi {
    pub fn spawn_line(world: &mut World) -> Result<UndoRedoResult, anyhow::Error> {
        let mut sys_state = SystemState::<ResMut<Assets<VectorGraphicMaterial>>>::new(world);
        let mut materials = sys_state.get_mut(world);
        let material = materials.add(VectorGraphicMaterial::default());

        let mut builder = world.changeset();
        let vector_graphic = builder
            .spawn((
                Name::from("Shape"),
                ObjectBundle::new(ObjectType::Vector),
                VectorGraphic::default(),
                VectorGraphicPathStorage::default(),
                StrokeOptions::default().with_line_width(3.),
                StrokeColor(Color::BLACK),
                FillOptions::default(),
                FillColor(Color::GRAY),
                material,
            ))
            .uid();

        let e0 = builder
            .spawn((
                Name::from("Endpoint"),
                ObjectBundle::new(ObjectType::VectorEndpoint),
                Endpoint::default(),
                InternalObject,
            ))
            .set_parent(vector_graphic)
            .uid();
        let e1 = builder
            .spawn((
                Name::from("Endpoint"),
                ObjectBundle::new(ObjectType::VectorEndpoint).with_position((100., 0.)),
                Endpoint::default(),
                InternalObject,
            ))
            .set_parent(vector_graphic)
            .uid();

        builder
            .spawn_edge(EdgeVariant::Line, e0, e1)
            .insert((
                Name::from("Edge"),
                ObjectBundle::new(ObjectType::VectorEdge),
                InternalObject,
            ))
            // .insert(ObjectBundle::new(ObjectType::VectorSegment))
            .set_parent(vector_graphic);

        let changeset = builder.build();

        let result = UndoRedoApi::execute(world, changeset)?;

        Ok(result)
    }
    pub fn spawn_box(world: &mut World) -> Result<UndoRedoResult, anyhow::Error> {
        let mut sys_state = SystemState::<ResMut<Assets<VectorGraphicMaterial>>>::new(world);
        let mut materials = sys_state.get_mut(world);
        let material = materials.add(VectorGraphicMaterial::default());

        let mut builder = world.changeset();
        let vector_graphic = builder
            .spawn((
                Name::from("Shape"),
                ObjectBundle::new(ObjectType::Vector),
                VectorGraphic::default(),
                VectorGraphicPathStorage::default(),
                StrokeOptions::default().with_line_width(3.),
                StrokeColor(Color::BLACK),
                FillOptions::default(),
                FillColor(Color::GRAY),
                material,
            ))
            .uid();

        let e0 = builder
            .spawn((
                Name::from("Endpoint"),
                ObjectBundle::new(ObjectType::VectorEndpoint),
                Endpoint::default(),
                InternalObject,
            ))
            .set_parent(vector_graphic)
            .uid();
        let e1 = builder
            .spawn((
                Name::from("Endpoint"),
                ObjectBundle::new(ObjectType::VectorEndpoint).with_position((100., 0.)),
                Endpoint::default(),
                InternalObject,
            ))
            .set_parent(vector_graphic)
            .uid();
        let e2 = builder
            .spawn((
                Name::from("Endpoint"),
                ObjectBundle::new(ObjectType::VectorEndpoint).with_position((100., 100.)),
                Endpoint::default(),
                InternalObject,
            ))
            .set_parent(vector_graphic)
            .uid();
        let e3 = builder
            .spawn((
                Name::from("Endpoint"),
                ObjectBundle::new(ObjectType::VectorEndpoint).with_position((0., 100.)),
                Endpoint::default(),
                InternalObject,
            ))
            .set_parent(vector_graphic)
            .uid();

        builder
            .spawn_edge(EdgeVariant::Line, e0, e1)
            .insert((
                Name::from("Edge"),
                ObjectBundle::new(ObjectType::VectorEdge),
                InternalObject,
            ))
            // .insert(ObjectBundle::new(ObjectType::VectorSegment))
            .set_parent(vector_graphic);
        builder
            .spawn_edge(EdgeVariant::Line, e1, e2)
            .insert((
                Name::from("Edge"),
                ObjectBundle::new(ObjectType::VectorEdge),
                InternalObject,
            ))
            // .insert(ObjectBundle::new(ObjectType::VectorSegment))
            .set_parent(vector_graphic);
        builder
            .spawn_edge(EdgeVariant::Line, e2, e3)
            .insert((
                Name::from("Edge"),
                ObjectBundle::new(ObjectType::VectorEdge),
                InternalObject,
            ))
            // .insert(ObjectBundle::new(ObjectType::VectorSegment))
            .set_parent(vector_graphic);
        builder
            .spawn_edge(EdgeVariant::Line, e3, e0)
            .insert((
                Name::from("Edge"),
                ObjectBundle::new(ObjectType::VectorEdge),
                InternalObject,
            ))
            // .insert(ObjectBundle::new(ObjectType::VectorSegment))
            .set_parent(vector_graphic);

        let changeset = builder.build();

        let result = UndoRedoApi::execute(world, changeset)?;

        Ok(result)
    }

    pub fn graph_schedule(
        world: &mut World,
        schedule_label: schedule_graph::definitions::ScheduleLabel,
        settings: schedule_graph::definitions::ScheduleGraphSettings,
    ) -> Result<String, anyhow::Error> {
        world.resource_scope::<Schedules, _>(|world, mut schedules| {
            use crate::utils::schedule_graph::*;
            let schedule = match schedule_label {
                definitions::ScheduleLabel::First => schedules.get_mut(First),
                definitions::ScheduleLabel::PreUpdate => schedules.get_mut(PreUpdate),
                definitions::ScheduleLabel::Update => schedules.get_mut(Update),
                definitions::ScheduleLabel::PostUpdate => schedules.get_mut(PostUpdate),
                definitions::ScheduleLabel::Last => schedules.get_mut(Last),
            };

            let schedule = schedule.ok_or(anyhow::anyhow!("Schedule doesn't exist"))?;
            let grapher = ScheduleGrapher::new(schedule, world, &settings);
            Ok(format!("{}", grapher))
        })
    }
}
