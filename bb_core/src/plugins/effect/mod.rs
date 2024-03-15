//! Contains logic for effects (side effect events) to keep the JS state synced with the bevy
//! state.

use bevy::prelude::*;

mod js_event_que;

pub use effects::*;
pub use js_event_que::EffectQue;

pub struct EffectPlugin;

impl Plugin for EffectPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(EffectQue::new());
        app.add_systems(Last, sys_emit_effects);
    }
}

pub fn sys_emit_effects(
    mut res: ResMut<EffectQue>,
) {
    res.forward_effects_to_js();
}


#[allow(non_snake_case)]
mod effects {
    use bevy_spts_fragments::prelude::Uid;
    use serde::{Serialize, Deserialize};
    use tsify::Tsify;

    #[derive(Tsify, Serialize, Deserialize, Debug, Clone)]
    #[tsify(into_wasm_abi, from_wasm_abi)]
    #[serde(tag = "tag", content = "value" )]
    pub enum Effect {
        SelectionChanged(Vec<Uid>),
        DocumentChanged,
    }
}
