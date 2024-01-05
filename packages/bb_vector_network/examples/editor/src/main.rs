use comfy::*;

simple_game!("BB Vector Network :: Editor", GameState, config, setup, update);

fn config(config: GameConfig) -> GameConfig {
    GameConfig {
        resolution: ResolutionConfig::Physical(600, 600 * 16 / 9),
        min_resolution: ResolutionConfig::Physical(100, 100 * 16 / 9),
        ..config
    }
}

pub struct GameState {}

impl GameState {
    pub fn new(_c: &EngineState) -> Self {
        Self {}
    }
}

fn setup(_state: &mut GameState, _c: &mut EngineContext) {}

fn update(_state: &mut GameState, _c: &mut EngineContext) {
    draw_text("1/v: Select tool\n2/p: Pen tool", Vec2::ZERO, PINK, TextAlign::Center);
}
