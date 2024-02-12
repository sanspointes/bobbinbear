mod sync;
mod undoredo;
mod scene;

use bevy::{ecs::system::SystemState, prelude::*, sprite::MaterialMesh2dBundle, app::AppExit};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use crate::index::Idx;

use self::{sync::{
    execute_in_world, execute_world_tasks_begin, execute_world_tasks_end, ExecutionChannel,
}, undoredo::UndoRedoApi, scene::SceneApi};

pub struct IpcPlugin;

impl Plugin for IpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(First, execute_world_tasks_begin)
            .add_systems(Last, execute_world_tasks_end);
    }
}

pub fn anyhow_result_to_js_result(result: Result<(), anyhow::Error>) -> Result<JsValue, JsValue> {
    match result {
        Ok(_) => Ok(JsValue::UNDEFINED),
        Err(reason) => Err(JsValue::from(JsError::new(&format!("{reason}")))),
    }
}

#[wasm_bindgen]
pub struct Api {
    pub undoredo: UndoRedoApi,
    pub scene: SceneApi,
}

#[wasm_bindgen]
impl Api {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            undoredo: UndoRedoApi,
            scene: SceneApi,
        }
    }

    #[wasm_bindgen]
    pub async fn exit(&self) -> js_sys::Promise {
        future_to_promise(execute_in_world(ExecutionChannel::FrameEnd, |w| {
            let mut exit_events = w.resource_mut::<Events<AppExit>>();
            exit_events.send(AppExit);
            Ok(JsValue::UNDEFINED)
        }))
    }

    #[wasm_bindgen]
    pub async fn describe_world(&self) -> js_sys::Promise {
        future_to_promise(execute_in_world(ExecutionChannel::FrameEnd, |w| {
            let e: Vec<_> = w.query::<Entity>().iter(w).collect();

            let info: Vec<_> = e.iter().map(|e| {
                let v = w.inspect_entity(*e);
                let info: Vec<_> = v.iter().map(|info| info.name()).collect();
                (e, info)
            }).collect();

            Ok(JsValue::from_str(&format!("{info:#?}")))
        }))
    }

    #[wasm_bindgen]
    pub async fn spawn_circle(&self, x: f32, y: f32) -> js_sys::Promise {
        let result = future_to_promise(execute_in_world(ExecutionChannel::FrameStart, move |world| {
            let mut sys_state = SystemState::<(
                Commands,
                ResMut<Assets<Mesh>>,
                ResMut<Assets<ColorMaterial>>,
            )>::new(world);

            let (mut commands, mut meshes, mut materials) = sys_state.get_mut(world);
            // Circle
            let id = commands.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(50.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                transform: Transform::from_translation(Vec3::new(x, y, 0.)),
                ..default()
            }).id();

            println!("RS: Spawning circle at {x:?} {y:?}");

            sys_state.apply(world);
            Ok(JsValue::from_str(&format!("Spawned circle with id : {id:?}")))
        }));

        result
    }
}
