mod draw;
mod scenarios;

use comfy::*;
use scenarios::{Scenario, build_scenerios};

simple_game!("bb_vector_network :: Scenarios", GameState, config, setup, update);

pub struct GameState {
    scenarios: Vec<Scenario>,
    current_scenario: i32,
}

impl GameState {
    pub fn new(_c: &EngineState) -> Self {
        Self {
            scenarios: build_scenerios(),

            current_scenario: 0,
        }
    }
}

fn config(config: GameConfig) -> GameConfig {
    GameConfig {
        resolution: ResolutionConfig::Physical(1600, 1200),
        min_resolution: ResolutionConfig::Physical(120 * 5, 80 * 5),
        ..config
    }
}

fn setup(_state: &mut GameState, c: &mut EngineContext) {
    c.load_fonts_from_bytes(&[(
        "comfy-font",
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/Roboto-Regular.ttf"
        )),
    )]);
}

fn update(state: &mut GameState, _c: &mut EngineContext) {
    if is_key_pressed(KeyCode::Left) {
        state.current_scenario -= 1;
        if state.current_scenario < 0 {
            state.current_scenario = (state.scenarios.len() - 1) as i32;
        }
    }
    if is_key_pressed(KeyCode::Right) {
        state.current_scenario = (state.current_scenario + 1) % state.scenarios.len() as i32;
    }

    let Some(scenario) = state.scenarios.get(state.current_scenario as usize) else {
        draw_text("Invalid Test", Vec2::new(0., 0.), WHITE, TextAlign::Center);
        return;
    };

    draw_text(
        &format!("<< {} >>", scenario.name),
        Vec2::new(0., -8.),
        WHITE,
        TextAlign::Center,
    );

    (scenario.executor)();
}
