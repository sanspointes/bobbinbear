use bevy::{ecs::system::SystemState, prelude::*, sprite::MaterialMesh2dBundle};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use self::sync::{
    execute_in_world, execute_world_tasks_begin, execute_world_tasks_end, ExecutionChannel,
};

mod sync;

pub struct IpcPlugin {}

impl Plugin for IpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(First, execute_world_tasks_begin)
            .add_systems(Last, execute_world_tasks_end);
    }
}

#[wasm_bindgen]
pub struct IpcApi {}

#[wasm_bindgen]
impl IpcApi {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {}
    }

    #[wasm_bindgen]
    pub async fn describe_world(&self) -> js_sys::Promise {
        future_to_promise(execute_in_world(ExecutionChannel::FrameEnd, |w| {
            let e: Vec<_> = w.query::<(Entity, &Transform)>().iter(w).collect();

            Ok(JsValue::from_str(&format!("{e:?}")))
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
