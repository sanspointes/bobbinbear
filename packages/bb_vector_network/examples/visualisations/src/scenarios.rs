use std::borrow::BorrowMut;
use std::fmt::format;
use std::ops::Deref;

use bb_vector_network::bb_graph::ClosedWalkResult;
use bb_vector_network::debug_draw::*;
use bb_vector_network::prelude::*;
use comfy::*;

use crate::draw::assert_edge;
use crate::draw::draw_graph;

pub enum ScenarioOutcome {
    Pass,
    Fail,
}

pub struct Scenario {
    pub name: String,
    pub executor: Box<dyn Fn() -> BBResult<ScenarioOutcome>>,
}

pub fn build_scenerios() -> Vec<Scenario> {
    vec![
        Scenario {
            name: "graph_fill".to_string(),
            executor: Box::new(|| {
                *DEBUG_DRAW_CCW.deref().borrow_mut() = true;

                let mut g = BBGraph::new();

                let (e0, f) = g.line(vec2(0., 5.), vec2(0., 0.));
                let (e1, e) = g.line_from(f.end_idx(), vec2(5., 0.));
                let (e2, m) = g.line_from_to(e.end_idx(), f.start_idx());

                let (e3, e) = g.line_from(m.end_idx(), vec2(6., 5.));
                let (e4, _) = g.line_from_to(e.end_idx(), m.start_idx());

                g.update_regions()?;
                println!("{:?}", g.regions);
                draw_graph(&g)?;

                *DEBUG_DRAW_CCW.deref().borrow_mut() = false;
                Ok(ScenarioOutcome::Pass)
            }),
        },
        Scenario {
            name: "get_cw_edge_of_node:1a".to_string(),
            executor: Box::new(|| {
                *DEBUG_DRAW_CCW.deref().borrow_mut() = true;

                let mut g = BBGraph::new();

                let (e0, f) = g.line(vec2(-5., 0.), vec2(0., 0.));
                let (e1, _) = g.line_from(f.end_idx(), vec2(get_time().sin() as f32 * 5., 5.));
                let (e2, _) = g.line_from(f.end_idx(), vec2(get_time().sin() as f32 * 5., -5.));

                draw_graph(&g)?;

                let next = g.get_cw_edge_of_node(f.end_idx(), vec2(1., 0.), Some(e0))?;

                *DEBUG_DRAW_CCW.deref().borrow_mut() = false;
                Ok(assert_edge(next, e1))
            }),
        },
        Scenario {
            name: "get_cw_edge_of_node:1b".to_string(),
            executor: Box::new(|| {
                *DEBUG_DRAW_CCW.deref().borrow_mut() = true;

                let mut g = BBGraph::new();

                let (e0, f) = g.line(vec2(-5., 0.), vec2(0., 0.));
                let (e2, _) = g.line_from(f.end_idx(), vec2(get_time().sin() as f32 * 5., -5.));
                let (e1, _) = g.line_from(f.end_idx(), vec2(get_time().sin() as f32 * 5., 5.));

                draw_graph(&g)?;

                let next = g.get_cw_edge_of_node(f.end_idx(), vec2(1., 0.), Some(e0))?;

                *DEBUG_DRAW_CCW.deref().borrow_mut() = false;
                Ok(assert_edge(next, e1))
            }),
        },
        Scenario {
            name: "get_cw_edge_of_node:2a".to_string(),
            executor: Box::new(|| {
                *DEBUG_DRAW_CCW.deref().borrow_mut() = true;

                let mut g = BBGraph::new();

                let (e0, f) = g.line(vec2(-5., 0.), vec2(0., 0.));
                let (e1, e1v) = g.line_from(f.end_idx(), vec2(get_time().sin() as f32 * 5., 5.));
                let (e2, e2v) = g.line_from(f.end_idx(), vec2(get_time().cos() as f32 * 5., 5.));

                draw_graph(&g)?;

                let next = g.get_cw_edge_of_node(f.end_idx(), vec2(1., 0.), Some(e0))?;
                let expected_edge = if e1v.end_pos(&g).x < e2v.end_pos(&g).x {
                    e1
                } else {
                    e2
                };

                *DEBUG_DRAW_CCW.deref().borrow_mut() = false;
                Ok(assert_edge(next, expected_edge))
            }),
        },
        Scenario {
            name: "get_cw_edge_of_node:2b".to_string(),
            executor: Box::new(|| {
                *DEBUG_DRAW_CCW.deref().borrow_mut() = true;

                let mut g = BBGraph::new();

                let (e0, f) = g.line(vec2(-5., 0.), vec2(0., 0.));
                let (e2, e2v) = g.line_from(f.end_idx(), vec2(get_time().cos() as f32 * 5., 5.));
                let (e1, e1v) = g.line_from(f.end_idx(), vec2(get_time().sin() as f32 * 5., 5.));

                draw_graph(&g)?;

                let next = g.get_cw_edge_of_node(f.end_idx(), vec2(1., 0.), Some(e0))?;
                let expected_edge = if e1v.end_pos(&g).x < e2v.end_pos(&g).x {
                    e1
                } else {
                    e2
                };

                *DEBUG_DRAW_CCW.deref().borrow_mut() = false;
                Ok(assert_edge(next, expected_edge))
            }),
        },
        Scenario {
            name: "get_cw_edge_of_node:3".to_string(),
            executor: Box::new(|| {
                *DEBUG_DRAW_CCW.deref().borrow_mut() = true;

                let mut g = BBGraph::new();

                let (e0, f) = g.line(vec2(-5., 0.), vec2(0., 0.));
                g.line_from(f.end_idx(), vec2(get_time().sin() as f32 * 5., 5.));
                g.line_from(f.end_idx(), vec2(get_time().cos() as f32 * 5., 5.));
                let (e3, e3v) = g.line_from(f.end_idx(), vec2(get_time().cos() as f32 * 5., -5.));
                let (e4, e4v) = g.line_from(f.end_idx(), vec2(get_time().sin() as f32 * 5., -5.));

                draw_graph(&g)?;

                let next = g.get_cw_edge_of_node(f.end_idx(), vec2(1., 0.), Some(e0))?;
                let expected_edge = if e3v.end_pos(&g).x < e4v.end_pos(&g).x {
                    e3
                } else {
                    e4
                };

                *DEBUG_DRAW_CCW.deref().borrow_mut() = false;
                Ok(assert_edge(next, expected_edge))
            }),
        },
        Scenario {
            name: "get_cw_edge_of_node:4a".to_string(),
            executor: Box::new(|| {
                *DEBUG_DRAW_CCW.deref().borrow_mut() = true;

                let mut g = BBGraph::new();

                let (e0, f) = g.line(vec2(-5., 0.), vec2(0., 0.));
                let (e1, _) = g.cubic_from(f.end_idx(), vec2(5., 0.), vec2(5., 0.), vec2(5., 5.));
                let (e2, _) = g.cubic_from(f.end_idx(), vec2(5., 0.), vec2(5., 0.), vec2(5., -5.));


                draw_graph(&g)?;

                let next = g.get_cw_edge_of_node(f.end_idx(), vec2(1., 0.), Some(e0))?;
                let expected_edge = e2;

                *DEBUG_DRAW_CCW.deref().borrow_mut() = false;
                Ok(assert_edge(next, expected_edge))
            }),
        },
        Scenario {
            name: "get_cw_edge_of_node:4b".to_string(),
            executor: Box::new(|| {
                *DEBUG_DRAW_CCW.deref().borrow_mut() = true;

                let mut g = BBGraph::new();

                let (e0, f) = g.line(vec2(-5., 0.), vec2(0., 0.));
                let (e2, _) = g.cubic_from(f.end_idx(), vec2(5., 0.), vec2(5., 0.), vec2(5., -5.));
                let (e1, _) = g.cubic_from(f.end_idx(), vec2(5., 0.), vec2(5., 0.), vec2(5., 5.));


                draw_graph(&g)?;

                let next = g.get_cw_edge_of_node(f.end_idx(), vec2(1., 0.), Some(e0))?;
                let expected_edge = e2;
                ;

                *DEBUG_DRAW_CCW.deref().borrow_mut() = false;
                Ok(assert_edge(next, expected_edge))
            }),
        },
        Scenario {
            name: "get_cw_edge_of_node:5a".to_string(),
            executor: Box::new(|| {
                *DEBUG_DRAW_CCW.deref().borrow_mut() = true;

                let mut g = BBGraph::new();

                let (e0, f) = g.line(vec2(-5., 0.), vec2(0., 0.));
                let (e2, _) = g.cubic_from(f.end_idx(), vec2(3., 3.), vec2(3., 3.), vec2(5., -5.));
                let (e1, _) = g.cubic_from(f.end_idx(), vec2(3., -3.), vec2(3., -3.), vec2(5., 5.));


                draw_graph(&g)?;

                let next = g.get_cw_edge_of_node(f.end_idx(), vec2(1., 0.), Some(e0))?;
                let expected_edge = e2;

                *DEBUG_DRAW_CCW.deref().borrow_mut() = false;
                Ok(assert_edge(next, expected_edge))
            }),
        },
        Scenario {
            name: "get_cw_edge_of_node:6a".to_string(),
            executor: Box::new(|| {
                *DEBUG_DRAW_CCW.deref().borrow_mut() = true;

                let mut g = BBGraph::new();

                let (e0, f) = g.line(vec2(-5., 0.), vec2(0., 0.));
                let (e2, _) = g.cubic_from(f.end_idx(), vec2(3., 3.), vec2(-3., 3.), vec2(-2., -5.));
                let (e1, _) = g.cubic_from(f.end_idx(), vec2(3., -3.), vec2(3., -3.), vec2(5., 5.));


                draw_graph(&g)?;

                let next = g.get_cw_edge_of_node(f.end_idx(), vec2(1., 0.), Some(e0))?;
                let expected_edge = e2;

                *DEBUG_DRAW_CCW.deref().borrow_mut() = false;
                Ok(assert_edge(next, expected_edge))
            }),
        },
        Scenario {
            name: "get_ccw_edge_of_node:1a".to_string(),
            executor: Box::new(|| {
                *DEBUG_DRAW_CCW.deref().borrow_mut() = true;

                let mut g = BBGraph::new();

                let (e0, f) = g.line(vec2(-5., 0.), vec2(0., 0.));
                let (e1, _) = g.line_from(f.end_idx(), vec2(get_time().sin() as f32 * 5., 5.));
                let (e2, _) = g.line_from(f.end_idx(), vec2(get_time().sin() as f32 * 5., -5.));

                draw_graph(&g)?;

                let next = g.get_ccw_edge_of_node(f.end_idx(), vec2(1., 0.), Some(e0))?;

                *DEBUG_DRAW_CCW.deref().borrow_mut() = false;
                Ok(assert_edge(next, e2))
            }),
        },
        Scenario {
            name: "get_ccw_edge_of_node:1b".to_string(),
            executor: Box::new(|| {
                *DEBUG_DRAW_CCW.deref().borrow_mut() = true;

                let mut g = BBGraph::new();

                let (e0, f) = g.line(vec2(-5., 0.), vec2(0., 0.));
                let (e2, _) = g.line_from(f.end_idx(), vec2(get_time().sin() as f32 * 5., -5.));
                let (e1, _) = g.line_from(f.end_idx(), vec2(get_time().sin() as f32 * 5., 5.));

                draw_graph(&g)?;

                let next = g.get_ccw_edge_of_node(f.end_idx(), vec2(1., 0.), Some(e0))?;

                *DEBUG_DRAW_CCW.deref().borrow_mut() = false;
                Ok(assert_edge(next, e2))
            }),
        },
        Scenario {
            name: "get_ccw_edge_of_node:2".to_string(),
            executor: Box::new(|| {
                *DEBUG_DRAW_CCW.deref().borrow_mut() = true;

                let mut g = BBGraph::new();

                let (e0, f) = g.line(vec2(-5., 0.), vec2(0., 0.));
                let (e3, e3v) = g.line_from(f.end_idx(), vec2(get_time().cos() as f32 * 5., -5.));
                let (e4, e4v) = g.line_from(f.end_idx(), vec2(get_time().sin() as f32 * 5., -5.));

                draw_graph(&g)?;

                let next = g.get_ccw_edge_of_node(f.end_idx(), vec2(1., 0.), Some(e0))?;

                let expected_edge = if e3v.end_pos(&g).x < e4v.end_pos(&g).x {
                    e3
                } else {
                    e4
                };

                *DEBUG_DRAW_CCW.deref().borrow_mut() = false;
                Ok(assert_edge(next, expected_edge))
            }),
        },
        Scenario {
            name: "get_ccw_edge_of_node:3".to_string(),
            executor: Box::new(|| {
                *DEBUG_DRAW_CCW.deref().borrow_mut() = true;

                let mut g = BBGraph::new();

                let (e0, f) = g.line(vec2(-5., 0.), vec2(0., 0.));
                let (e1, e1v) = g.line_from(f.end_idx(), vec2(get_time().sin() as f32 * 5., 5.));
                let (e2, e2v) = g.line_from(f.end_idx(), vec2(get_time().cos() as f32 * 5., 5.));
                g.line_from(f.end_idx(), vec2(get_time().cos() as f32 * 5., -5.));
                g.line_from(f.end_idx(), vec2(get_time().sin() as f32 * 5., -5.));

                draw_graph(&g)?;

                let next = g.get_ccw_edge_of_node(f.end_idx(), vec2(1., 0.), Some(e0))?;

                let expected_edge = if e1v.end_pos(&g).x < e2v.end_pos(&g).x {
                    e1
                } else {
                    e2
                };

                *DEBUG_DRAW_CCW.deref().borrow_mut() = false;
                Ok(assert_edge(next, expected_edge))
            }),
        },
        Scenario {
            name: "get_ccw_edge_of_node:4".to_string(),
            executor: Box::new(|| {
                *DEBUG_DRAW_CCW.deref().borrow_mut() = true;

                let mut g = BBGraph::new();

                let (e0, f) = g.line(vec2(-5., 0.), vec2(0., 0.));
                let (e1, _) = g.cubic_from(f.end_idx(), vec2(5., 0.), vec2(5., 0.), vec2(5., 5.));
                let (e2, _) = g.cubic_from(f.end_idx(), vec2(5., 0.), vec2(5., 0.), vec2(5., -5.));


                draw_graph(&g)?;

                let next = g.get_cw_edge_of_node(f.end_idx(), vec2(1., 0.), Some(e0))?;
                let expected_edge = e2;

                *DEBUG_DRAW_CCW.deref().borrow_mut() = false;
                Ok(assert_edge(next, expected_edge))
            }),
        },
        Scenario {
            name: "closed_walk_with_cw_start_and_ccw_traverse:1".to_string(),
            executor: Box::new(|| {
                *DEBUG_DRAW_CCW.deref().borrow_mut() = true;

                let mut g = BBGraph::new();

                let (_, f) = g.line(vec2(-8., 6.), vec2(0., 6.));
                let (_, m) = g.line_from(f.end_idx(), vec2(0., -6.));
                let (_, e) = g.line_from(m.end_idx(), vec2(-8., -6.));
                let (_, e) = g.line_from_to(e.end_idx(), f.start_idx());

                let (_, e) = g.line_from(m.start_idx(), vec2(8., 6.));
                let (_, e) = g.line_from(e.end_idx(), vec2(8., -6.));
                let (_, e) = g.line_from_to(e.end_idx(), m.end_idx());

                let left_most = g.get_left_most_node_index().unwrap();

                let ClosedWalkResult { outer_edge, edges } = g
                    .closed_walk_with_cw_start_and_ccw_traverse(left_most)
                    .unwrap();

                draw_graph(&g).unwrap();

                draw_circle(g.node(left_most).unwrap().position(), 0.2, WHITE.alpha(0.5), 500);

                let outer_edge = g.edge(outer_edge).unwrap();
                draw_line(outer_edge.start_pos(&g), outer_edge.end_pos(&g), 0.2, RED.alpha(0.5), 500);

                let directed = g.edges_directed(&edges)?;
                for (_, edge) in directed {
                    draw_arrow(edge.t_point(&g, 0.49), edge.t_point(&g, 0.51), 0.1, RED, 500);
                }

                Ok(ScenarioOutcome::Pass)
            }),
        },
        Scenario {
            name: "closed_walk_with_ccw_start_and_ccw_traverse:1".to_string(),
            executor: Box::new(|| {
                *DEBUG_DRAW_CCW.deref().borrow_mut() = true;

                let mut g = BBGraph::new();

                let (_, f) = g.line(vec2(-8., 6.), vec2(0., 6.));
                let (_, m) = g.line_from(f.end_idx(), vec2(0., -6.));
                let (_, e) = g.line_from(m.end_idx(), vec2(-8., -6.));
                let (_, e) = g.line_from_to(e.end_idx(), f.start_idx());

                let (_, e) = g.line_from(m.start_idx(), vec2(8., 6.));
                let (_, e) = g.line_from(e.end_idx(), vec2(8., -6.));
                let (_, e) = g.line_from_to(e.end_idx(), m.end_idx());

                let left_most = g.get_left_most_node_index().unwrap();

                let ClosedWalkResult { outer_edge, edges } = g
                    .closed_walk_with_ccw_start_and_ccw_traverse(left_most)
                    .unwrap();

                draw_graph(&g).unwrap();

                draw_circle(g.node(left_most).unwrap().position(), 0.2, WHITE.alpha(0.5), 500);

                let outer_edge = g.edge(outer_edge).unwrap();
                draw_line(outer_edge.start_pos(&g), outer_edge.end_pos(&g), 0.2, RED.alpha(0.5), 500);

                let directed = g.edges_directed(&edges)?;
                for (_, edge) in directed {
                    draw_arrow(edge.t_point(&g, 0.49), edge.t_point(&g, 0.51), 0.1, RED, 500);
                }

                Ok(ScenarioOutcome::Pass)
            }),
        },
    ]
}
