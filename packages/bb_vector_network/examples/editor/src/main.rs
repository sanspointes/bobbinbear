mod tesselation;

use comfy::*;
use bb_vector_network::prelude::*;
use lyon_tessellation::{FillTessellator, VertexBuffers, geometry_builder::simple_builder};
use tesselation::{tessellate_fill, tessellate_stroke};

simple_game!("BB Vector Network :: Editor", GameState, config, setup, update);

fn config(config: GameConfig) -> GameConfig {
    GameConfig {
        resolution: ResolutionConfig::Physical(1200, 800),
        min_resolution: ResolutionConfig::Physical(120 * 5, 80 * 5),
        ..config
    }
}

pub struct GameState {
    graph: BBGraph,
}

impl GameState {
    pub fn new(_c: &EngineState) -> Self {
        let mut g = BBGraph::new();
        let (_, first_edge) = g.line(Vec2::new(0., 0.), Vec2::new(5., 0.));
        let (_, edge) = g.line_from(first_edge.end_idx(), Vec2::new(5., 5.));
        let (_, edge) = g.line_from(edge.end_idx(), Vec2::new(0., 5.));
        let (_, edge) = g.line_from_to(edge.end_idx(), first_edge.start_idx());
        let (_, _) = g.line_from(edge.start_idx(), Vec2::new(-5., 5.));

        Self {
            graph: g,
        }
    }
}

fn setup(_state: &mut GameState, _c: &mut EngineContext) {}

fn update(state: &mut GameState, _c: &mut EngineContext) {
    // draw_text("1/v: Select tool\n2/p: Pen tool", Vec2::ZERO, PINK, TextAlign::Center);
    for x in -5..5 {
        draw_circle(vec2(x as f32 / 2., 0.), 0.2, GREEN, 1);
    }
    for y in -5..5 {
        draw_circle(vec2(0., y as f32 / 2.), 0.2, GREEN, 1);
    }

    for node in state.graph.nodes.values() {
        draw_circle(node.position(), 0.2, RED, 1);
    }

    let _ = state.graph.update_regions();

    match tessellate_fill(&state.graph) {
        Ok(mesh) => {
            println!("Drawing mesh {mesh:?}");
            draw_mesh(mesh);
        }
        Err(reason) => {
            draw_text(&format!("{reason}"), Vec2::ZERO, PINK, TextAlign::Center);
        }
    }
    match tessellate_stroke(&state.graph) {
        Ok(mesh) => {
            println!("Drawing mesh {mesh:?}");
            draw_mesh(mesh);
        }
        Err(reason) => {
            draw_text(&format!("{reason}"), Vec2::ZERO, PINK, TextAlign::Center);
        }
    }
}
