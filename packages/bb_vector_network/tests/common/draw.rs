use std::env;

use tiny_skia::{Paint, PathBuilder, Pixmap, Stroke, Transform};

use bb_vector_network::prelude::*;

pub struct SnapshotCtx {
    pub pixmap: Pixmap,
    pub stroke: Stroke,
    pub stroke_paint: Paint<'static>,
    pub transform: Transform,
}

impl Default for SnapshotCtx {
    fn default() -> Self {
        let mut stroke_paint = Paint::default();
        stroke_paint.set_color_rgba8(50, 127, 150, 255);
        stroke_paint.anti_alias = true;
        Self {
            pixmap: Pixmap::new(200, 200).expect("Unable to create pixmap"),
            stroke: Stroke {
                width: 5.,
                ..Default::default()
            },
            stroke_paint,
            transform: Transform::default(),
        }
    }
}

impl SnapshotCtx {
    pub fn with_stroke_color(&mut self, r: u8, g: u8, b: u8, a: u8) -> &mut Self {
        self.stroke_paint.set_color_rgba8(r, g, b, a);
        self
    }
    pub fn with_stroke_width(&mut self, width: f32) -> &mut Self {
        self.stroke.width = width;
        self
    }
}

impl SnapshotCtx {
    pub fn save_or_threshold_with_disk(&self, path: &str, threshold: f32) -> bool {
        let snapshot = match Pixmap::load_png(path) {
            Ok(snapshot) => snapshot,
            Err(reason) => {
                let cwd = env::current_dir().unwrap();

                self.pixmap.save_png(path).unwrap_or_else(|_| {
                    panic!(
                        "Can't load from disk and can't save from disk uh oh. (cwd {})",
                        cwd.display()
                    )
                });
                return false;
            }
        };

        let total_diff = self
            .pixmap
            .data()
            .iter()
            .zip(snapshot.data().iter())
            .fold(0i32, |acc, (a, b)| acc + (*a as i32 - *b as i32).abs());

        let total_diff_f32 = (total_diff as f64) / (self.pixmap.data().len() * 255) as f64;

        println!("total_diff: {total_diff}, total_diff_f32: {total_diff_f32}.");

        (total_diff_f32 as f32) < threshold
    }
}

pub fn draw_edge(cx: &mut SnapshotCtx, graph: &BBGraph, edge: BBEdgeIndex) -> BBResult<()> {
    let edge = graph.edge(edge)?;
    let mut pb = PathBuilder::new();
    let p0 = edge.start_pos(graph);
    let p1 = edge.end_pos(graph);
    pb.move_to(p0.x, p0.y);
    match edge {
        BBEdge::Line { .. } => {
            pb.line_to(p1.x, p1.y);
        }
        BBEdge::Quadratic { ctrl1, .. } => {
            pb.quad_to(ctrl1.x, ctrl1.y, p1.x, p1.y);
        }
        BBEdge::Cubic { ctrl1, ctrl2, .. } => {
            pb.cubic_to(ctrl1.x, ctrl1.y, ctrl2.x, ctrl2.y, p1.x, p1.y);
        }
    }
    let path = pb
        .finish()
        .expect("draw_edge: Failed to unwrap pathbuilder.finish().");
    cx.pixmap
        .stroke_path(&path, &cx.stroke_paint, &cx.stroke, cx.transform, None);
    Ok(())
}

pub fn draw_all_edges(cx: &mut SnapshotCtx, graph: &BBGraph) -> BBResult<()> {
    let edges: Vec<_> = graph.edges.keys().cloned().collect();
    draw_edge_list(cx, graph, edges.as_slice())?;
    Ok(())
}

pub fn draw_edge_list(
    cx: &mut SnapshotCtx,
    graph: &BBGraph,
    edges: &[BBEdgeIndex],
) -> BBResult<()> {
    for e in edges {
        draw_edge(cx, graph, *e)?;
    }
    Ok(())
}

pub fn draw_graph(cx: &mut SnapshotCtx, graph: &BBGraph) -> BBResult<()> {
    draw_all_edges(cx, graph)?;
    Ok(())
}
