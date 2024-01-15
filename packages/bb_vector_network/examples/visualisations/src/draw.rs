use bb_vector_network::{
    debug_draw::{draw_graph_fill, draw_graph_stroke},
    prelude::*,
};
use comfy::*;

use crate::scenarios::ScenarioOutcome;

pub static ERR_TEXT_PARAMS: Lazy<TextParams> = Lazy::new(|| TextParams {
    color: ORANGE_RED,
    font: egui::FontId::new(12.0, egui::FontFamily::Name("comfy-font".into())),
    ..Default::default()
});
pub static SCC_TEXT_PARAMS: Lazy<TextParams> = Lazy::new(|| TextParams {
    color: LIME_GREEN,
    font: egui::FontId::new(12.0, egui::FontFamily::Name("comfy-font".into())),
    ..Default::default()
});

pub fn draw_error(error: &str) {
    draw_text_ex(
        &format!("Error: {error}"),
        screen_to_world(vec2(0.5, 2.)),
        TextAlign::TopLeft,
        ERR_TEXT_PARAMS.clone(),
    )
}
pub fn draw_success(message: &str) {
    draw_text_ex(
        &format!("Success: {message}"),
        screen_to_world(vec2(0.5, 2.)),
        TextAlign::TopLeft,
        SCC_TEXT_PARAMS.clone(),
    )
}

pub fn assert_edge(received: BBEdgeIndex, expected: BBEdgeIndex) -> ScenarioOutcome {
    if received == expected {
        draw_success(&format!("Successfully traverses to expected edge {expected}"));
        ScenarioOutcome::Pass
    } else {
        draw_error(&format!("Incorrect next edge.\nEXPECTED: {expected}\nRECEIVED {received}"));
        ScenarioOutcome::Fail
    }

}

pub fn draw_node(graph: &BBGraph, node_idx: BBNodeIndex) -> BBResult<()> {
    let node = graph.node(node_idx)?;

    let p = node.position();
    draw_circle(p, 0.1, BLUE, 60);
    draw_circle(p, 0.08, WHITE, 60);

    draw_text_ex(
        &format!("{node_idx} ({:.1},{:.1})", p.x, p.y),
        node.position() + vec2(0.2, 0.2),
        TextAlign::BottomLeft,
        ERR_TEXT_PARAMS.clone(),
    );
    Ok(())
}

pub fn draw_graph(graph: &BBGraph) -> BBResult<()> {
    draw_graph_fill(graph)?;
    draw_graph_stroke(graph)?;
    for (idx, edge) in graph.edges.iter() {
        draw_text_ex(
            &format!("{idx}"),
            edge.t_point(graph, 0.5),
            TextAlign::BottomLeft,
            ERR_TEXT_PARAMS.clone(),
        );
    }
    for node in graph.nodes.keys() {
        draw_node(graph, *node)?;
    }
    Ok(())
}
