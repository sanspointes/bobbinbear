use bb_vector_network::impl2::prelude::*;
use comfy::*;

simple_game!("Simple Testbed", GameState, config, setup, update);

fn config(config: GameConfig) -> GameConfig {
    GameConfig { ..config }
}

struct Test {
    name: String,
    executor: Box<dyn Fn()>,
}

pub struct GameState {
    tests: Vec<Test>,
    current_test: i32,
}

impl GameState {
    pub fn new(_c: &EngineContext) -> Self {
        Self {
            tests: vec![
                Test {
                    name: "CCW Test".to_string(),
                    executor: Box::new(|| {
                        let p0 = Vec2::new(0., -5.);
                        let p1 = Vec2::new(0., 0.);
                        let p2 = Vec2::new(-3., 4.);
                        let p3 = mouse_world();
                        
                        comfy::draw_line(p0, p1, 0.1, comfy::DARKGREEN, 1);
                        comfy::draw_line(p1, p2, 0.1, comfy::DARKGREEN, 1);
                        comfy::draw_line(p1, p3, 0.1, comfy::DARKGREEN, 1);

                        comfy::draw_circle(p0, 0.2, comfy::WHITE, 2);
                        comfy::draw_text("p0", p0 + 0.5, comfy::WHITE, comfy::TextAlign::Center);
                        comfy::draw_circle(p1, 0.2, comfy::WHITE, 2);
                        comfy::draw_text("p1", p1 + 0.5, comfy::WHITE, comfy::TextAlign::Center);
                        comfy::draw_circle(p2, 0.2, comfy::WHITE, 2);
                        comfy::draw_text("p2", p2 + 0.5, comfy::WHITE, comfy::TextAlign::Center);
                        comfy::draw_circle(p3, 0.2, comfy::WHITE, 2);
                        comfy::draw_text("p3", p3 + 0.5, comfy::WHITE, comfy::TextAlign::Center);

                        let dcurr = p1 - p0;
                        comfy::draw_text(&format!("dcurr: {dcurr}"), p0 + dcurr / 2., comfy::WHITE, comfy::TextAlign::Center);
                        let da = p2 - p1;
                        comfy::draw_text(&format!("da: {da}\ndet: {}", dcurr.determinate(da)), p1 + da / 2., comfy::WHITE, comfy::TextAlign::Center);
                        let db = p3 - p1;
                        comfy::draw_arrow(p1 + 0.5, p3 + 0.5, 0.05, comfy::PINK, 2);
                        comfy::draw_text(&format!("da: {db}\ndet: {}", dcurr.determinate(db)), p1 + db / 2., comfy::WHITE, comfy::TextAlign::Center);

                        let is_convex = dcurr.determinate(db);


                    }),
                },
                Test {
                    name: "Shape 0".to_string(),
                    executor: Box::new(|| {
                        let mut g = BBGraph::new();

                        let (_, first_edge) = g.line(Vec2::new(-5., 0.), Vec2::new(-2., -2.));
                        let (_, edge) = g.line_from(first_edge.end_idx(), Vec2::new(0., -4.));
                        let (_, edge) = g.line_from(edge.end_idx(), Vec2::new(2., -2.));
                        let (_, edge) = g.line_from_to(first_edge.end_idx(), edge.end_idx());
                        let (_, edge) = g.line_from(edge.end_idx(), Vec2::new(5., 0.));
                        let (_, edge) = g.line_from(edge.end_idx(), Vec2::new(3., 3.));
                        let (_, fork_edge) = g.line_from(edge.end_idx(), Vec2::new(0., 1.));
                        let (_, edge) = g.line_from(edge.end_idx(), Vec2::new(-3., 3.));
                        g.line_from_to(fork_edge.end_idx(), edge.end_idx());
                        g.line_from_to(edge.end_idx(), first_edge.start_idx());
                        
                        let source = BBEdgeIndex(0);
                        debug_graph(&g, source);
                    }),
                },
                Test {
                    name: "Prong 1".to_string(),
                    executor: Box::new(|| {
                        let mut g = BBGraph::new();


                        let (e0, first_edge) = g.line(Vec2::new(-5., 5.), Vec2::new(-4., 0.));
                        let (e1, edge) = g.line_from(first_edge.end_idx(), Vec2::new(-0., 0.));
                        let (e2, edge) = g.line_from(edge.end_idx(), Vec2::new(5., 0.));
                        let (e3, edge) = g.line_from(edge.end_idx(), Vec2::new(5., 5.));
                        let (e4, edge) = g.line_from_to(edge.end_idx(), first_edge.start_idx());


                        let (e5, edge) = g.line_from(first_edge.start_idx(), Vec2::new(-3., 4.));
                        let (e6, edge) = g.line_from(edge.end_idx(), Vec2::new(-3., 2.));
                        let (e7, edge) = g.line_from_to(edge.end_idx(), first_edge.start_idx());
                        
                        let source = BBEdgeIndex(0);
                        debug_graph(&g, source);

                        // if let Ok(regions) = mcb(&g) {
                        //     comfy::draw_text(&format!("{regions:?}"), Vec2::new(0., -4.), comfy::WHITE, TextAlign::Center);
                        // }
                    }),
                },
                Test {
                    name: "Parallel 1".to_string(),
                    executor: Box::new(|| {
                        let mut g = BBGraph::new();

                        let (_, root_link) = g.line(Vec2::new(-5., 0.), Vec2::new(0., 0.));
                        g.line_from(root_link.end_idx(), Vec2::new(5., 0.));
                        g.cubic_from(
                            root_link.end_idx(),
                            Vec2::new(5., 0.),
                            Vec2::new(5., 2.),
                            Vec2::new(5., 5.),
                        );
                        g.cubic_from(
                            root_link.end_idx(),
                            Vec2::new(5., 0.),
                            Vec2::new(5., -2.),
                            Vec2::new(5., -5.),
                        );
                        g.translate(Vec2::new(8., 0.));

                        let mut g2 = BBGraph::new();

                        let (_, root_link) = g2.line(Vec2::new(5., 0.), Vec2::new(0., 0.));
                        g2.line_from(root_link.end_idx(), Vec2::new(-5., 0.));
                        g2.cubic_from(
                            root_link.end_idx(),
                            Vec2::new(-5., 0.),
                            Vec2::new(-5., 2.),
                            Vec2::new(-5., 5.),
                        );
                        g2.cubic_from(
                            root_link.end_idx(),
                            Vec2::new(-5., 0.),
                            Vec2::new(-5., -2.),
                            Vec2::new(-5., -5.),
                        );
                        g2.translate(Vec2::new(-8., 0.));

                        let source = BBEdgeIndex(0);
                        debug_graph(&g, source);
                        debug_graph(&g2, source);
                    }),
                },
                Test {
                    name: "Parallel 2".to_string(),
                    executor: Box::new(|| {
                        let mut g = BBGraph::new();

                        let (_, root_link) = g.line(Vec2::new(0., -5.), Vec2::new(0., 0.));
                        g.cubic_from(
                            root_link.end_idx(),
                            Vec2::new(0., 5.),
                            Vec2::new(-2., 5.),
                            Vec2::new(-5., 5.),
                        );
                        g.cubic_from(
                            root_link.end_idx(),
                            Vec2::new(0., 5.),
                            Vec2::new(-2., 3.),
                            Vec2::new(-5., 3.),
                        );
                        g.translate(Vec2::new(-8., 0.));

                        let mut g2 = BBGraph::new();

                        let (_, root_link) = g2.line(Vec2::new(0., -5.), Vec2::new(0., 0.));
                        g2.cubic_from(
                            root_link.end_idx(),
                            Vec2::new(0., 5.),
                            Vec2::new(2., 5.),
                            Vec2::new(5., 5.),
                        );
                        g2.cubic_from(
                            root_link.end_idx(),
                            Vec2::new(0., 5.),
                            Vec2::new(2., 3.),
                            Vec2::new(5., 3.),
                        );
                        g2.translate(Vec2::new(8., 0.));

                        let source = BBEdgeIndex(0);
                        debug_graph(&g, source);
                        debug_graph(&g2, source);
                    }),
                },
                Test {
                    name: "Parallel 3".to_string(),
                    executor: Box::new(|| {
                        let source = BBEdgeIndex(0);

                        let mut g = BBGraph::new();
                        let (_, root_link) = g.line(Vec2::new(0., -3.), Vec2::new(0., 0.));
                        g.cubic_from(
                            root_link.end_idx(),
                            Vec2::new(0., 3.),
                            Vec2::new(-1., 3.),
                            Vec2::new(-3., 3.),
                        );
                        g.cubic_from(
                            root_link.end_idx(),
                            Vec2::new(0., -3.),
                            Vec2::new(-1., -3.),
                            Vec2::new(-3., -3.),
                        );
                        g.translate(Vec2::new(-8., 0.));
                        debug_graph(&g, source);

                        let mut g = BBGraph::new();
                        let (_, root_link) = g.line(Vec2::new(0., -3.), Vec2::new(0., 0.));
                        g.cubic_from(
                            root_link.end_idx(),
                            Vec2::new(0., 3.),
                            Vec2::new(-1., 3.),
                            Vec2::new(-3., 3.),
                        );
                        g.cubic_from(
                            root_link.end_idx(),
                            Vec2::new(0., -3.),
                            Vec2::new(1., -3.),
                            Vec2::new(3., -3.),
                        );
                        g.translate(Vec2::new(-4., 0.));
                        debug_graph(&g, source);

                        let mut g = BBGraph::new();
                        let (_, root_link) = g.line(Vec2::new(0., -3.), Vec2::new(0., 0.));
                        g.cubic_from(
                            root_link.end_idx(),
                            Vec2::new(0., 3.),
                            Vec2::new(1., 3.),
                            Vec2::new(3., 3.),
                        );
                        g.cubic_from(
                            root_link.end_idx(),
                            Vec2::new(0., -3.),
                            Vec2::new(-1., -3.),
                            Vec2::new(-3., -3.),
                        );
                        g.translate(Vec2::new(4., 0.));
                        debug_graph(&g, source);

                        let mut g = BBGraph::new();
                        let (_, root_link) = g.line(Vec2::new(0., -3.), Vec2::new(0., 0.));
                        g.cubic_from(
                            root_link.end_idx(),
                            Vec2::new(0., 3.),
                            Vec2::new(1., 3.),
                            Vec2::new(3., 3.),
                        );
                        g.cubic_from(
                            root_link.end_idx(),
                            Vec2::new(0., -3.),
                            Vec2::new(1., -3.),
                            Vec2::new(3., -3.),
                        );
                        g.translate(Vec2::new(8., 0.));
                        debug_graph(&g, source);
                    }),
                },
                Test {
                    name: "Shape 1".to_string(),
                    executor: Box::new(|| {
                        let mut g = BBGraph::new();

                        let (_, first_link) = g.line(Vec2::new(-5., 0.), Vec2::new(0., 0.));
                        let (_, middle_link) =
                            g.line_from(first_link.end_idx(), Vec2::new(0., 5.));
                        let (_, last_link) =
                            g.line_from(middle_link.end_idx(), Vec2::new(-5., 5.));
                        g.line_from_to(last_link.end_idx(), first_link.start_idx());
                        let time = comfy::get_time();
                        if time.sin() > -0.4 {
                            let (_, last_link) =
                                g.line_from(first_link.end_idx(), Vec2::new(5., 0.));
                            if time.sin() > 0.0 {
                                let (_, last_link) =
                                    g.line_from(last_link.end_idx(), Vec2::new(5., 5.));
                                if time.sin() > 0.4 {
                                    g.line_from_to(last_link.end_idx(), middle_link.end_idx());
                                }
                            }
                        }

                        let source = BBEdgeIndex(0);
                        debug_graph(&g, source);
                    }),
                },
                Test {
                    name: "Shape 3".to_string(),
                    executor: Box::new(|| {
                        let mut g = BBGraph::new();
                        let (_, first_edge) = g.line(Vec2::ZERO, Vec2::new(5., 0.));
                        let (_, edge) = g.line_from(first_edge.end_idx(), Vec2::new(5., 5.));
                        let (_, branch_edge) = g.line_from(edge.end_idx(), Vec2::new(0., 5.));
                        // Create the inner nested
                        let (_, edge) = g.line_from(branch_edge.end_idx(), Vec2::new(2., 4.));
                        let (_, edge) = g.line_from(edge.end_idx(), Vec2::new(1., 3.));
                        g.line_from_to(edge.end_idx(), branch_edge.end_idx());

                        let (_, _) = g.line_from_to(branch_edge.end_idx(), first_edge.start_idx());

                        let source = BBEdgeIndex(0);
                        debug_graph(&g, source);
                    }),
                },
                Test {
                    name: "Shape 2".to_string(),
                    executor: Box::new(|| {
                        let mut g = BBGraph::new();

                        let (_, first_link) = g.line(Vec2::new(-5., 0.), Vec2::new(0., 0.));
                        let (_, middle_link) =
                            g.line_from(first_link.end_idx(), Vec2::new(0., 5.));
                        let (_, last_link) =
                            g.line_from(middle_link.end_idx(), Vec2::new(-5., 5.));
                        g.line_from_to(last_link.end_idx(), first_link.start_idx());
                        let (_, top_link) = g.line_to(Vec2::new(5., 5.), middle_link.end_idx());
                        let (_, bottom_link) = g.line_to(Vec2::new(5., 0.), middle_link.start_idx());

                        let time = comfy::get_time();
                        if time.sin() > 0.0 {
                            let _ = g.line_from_to(bottom_link.start_idx(), top_link.start_idx());
                        }

                        let source = BBEdgeIndex(0);
                        debug_graph(&g, source);
                    }),
                },
            ],
            current_test: 0,
        }
    }
}

fn debug_graph(g: &BBGraph, _source_link: BBEdgeIndex) { 
    #[cfg(feature = "debug_draw")]
    {
        let _ = g.debug_draw();

        // let first = *g.edge(source_link).unwrap();
        // let next_links = first.next_links(g);
        //
        // comfy::draw_circle(first.end_pos(g), 0.1, WHITE, 11);
        //
        // let next = first.ccw_most_next_link(g, &next_links[..]);
        // let Some(next) = next else {
        //     comfy::draw_text("No next index", Vec2::ZERO, comfy::RED, TextAlign::Center);
        //     return;
        // };
        // let next = *g.edge(next).unwrap();
        // next.debug_draw_with_color_and_z_index(g, comfy::Color::rgb8(100, 255, 100), 11);
    }
}

fn setup(_state: &mut GameState, _c: &mut EngineContext) {}

fn update(state: &mut GameState, _c: &mut EngineContext) {
    if is_key_pressed(KeyCode::Left) {
        state.current_test = (state.current_test - 1) % state.tests.len() as i32;
    }
    if is_key_pressed(KeyCode::Right) {
        state.current_test = (state.current_test + 1) % state.tests.len() as i32;
    }

    let Some(test) = state.tests.get(state.current_test as usize) else {
        draw_text("Invalid Test", Vec2::new(0., 0.), WHITE, TextAlign::Center);
        return;
    };

    draw_text(
        &format!("<< {} >>", test.name),
        Vec2::new(0., -8.),
        WHITE,
        TextAlign::Center,
    );

    (test.executor)();
}

#[cfg(not(feature = "debug_draw"))]
fn main() {
    panic!("Testbeds require the `--features debug_draw` feature to be enabled.");
}
