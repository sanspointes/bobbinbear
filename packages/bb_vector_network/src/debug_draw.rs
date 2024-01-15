//! ## Debug Draw
//!
//! This package is designed to be debugged/tested interactively using [comfy engine](https://github.com/darthdeus/comfy)
//! and can create visualisations of the underlying algorithms when the `debug_draw` feature flag,
//! and subsequent runtime flags are enabled.
//!

use comfy::*;
use glam::Vec2;
use lyon_tessellation::{FillVertexConstructor, FillVertex, VertexBuffers, BuffersBuilder, FillTessellator, FillOptions, StrokeVertexConstructor, StrokeVertex, StrokeTessellator, StrokeOptions};

use crate::{prelude::*, traits::AngleBetween};

pub static DEBUG_DRAW_CCW: Lazy<AtomicRefCell<bool>> = Lazy::new(|| AtomicRefCell::new(false));
pub static DEBUG_DRAW_TRAVERSAL: Lazy<AtomicRefCell<bool>> =
    Lazy::new(|| AtomicRefCell::new(false));

pub trait Vec2Utils {
    fn rotate_angle(&self, angle: f32) -> Vec2;
    fn rotate_radians(&self, angle: f32) -> Vec2;

    fn draw_angle_between_ccw(&self, other: Vec2, pos: Vec2);
    fn draw_angle_between_cw(&self, other: Vec2, pos: Vec2);

}

impl Vec2Utils for Vec2 {
    fn rotate_angle(&self, angle: f32) -> Vec2 {
        self.rotate_radians(angle.to_radians())
    }
    fn rotate_radians(&self, theta: f32) -> Vec2 {
        let cs = theta.cos();
        let sn = theta.sin();

        Vec2::new(self.x * cs - self.y * sn, self.x * sn + self.y * cs)
    }

    fn draw_angle_between_ccw(&self, other: Vec2, pos: Vec2) {
        let angle_delta = self.normalize().angle_between_ccw(other.normalize());

        comfy::draw_text_ex(
            &format!("ccw {angle_delta}"),
            self.normalize() + pos,
            comfy::TextAlign::TopLeft,
            DBG_TEXT_PARAMS.clone(),
        );

        let mut prev_dir = self.normalize(); 
        for i in 0..12 {
            let next_dir = prev_dir.rotate_radians(angle_delta / 12.);
            if i == 11 {
                comfy::draw_arrow(prev_dir + pos, next_dir + pos, 0.05, comfy::BLUE, 500);
            } else {
                comfy::draw_line(prev_dir + pos, next_dir + pos, 0.05, comfy::BLUE, 500);
            }
            prev_dir = next_dir;
        }
    }

    fn draw_angle_between_cw(&self, other: Vec2, pos: Vec2) {
        let angle_delta = self.normalize().angle_between_cw(other.normalize());

        comfy::draw_text_ex(
            &format!("cw {angle_delta}"),
            self.normalize() + pos,
            comfy::TextAlign::TopLeft,
            DBG_TEXT_PARAMS.clone(),
        );

        let mut prev_dir = self.normalize(); 
        for i in 0..12 {
            let next_dir = prev_dir.rotate_radians(-angle_delta / 12.);
            if i == 11 {
                comfy::draw_arrow(prev_dir + pos, next_dir + pos, 0.05, comfy::BLUE, 500);
            } else {
                comfy::draw_line(prev_dir + pos, next_dir + pos, 0.05, comfy::BLUE, 500);
            }
            prev_dir = next_dir;
        }
    }
}

pub static DBG_TEXT_PARAMS: Lazy<TextParams> = Lazy::new(|| TextParams {
    color: LIME_GREEN,
    font: comfy::egui::FontId::new(12.0, comfy::egui::FontFamily::Name("comfy-font".into())),
    ..Default::default()
});

struct ComfyFillVertexConstructor(Color);

impl FillVertexConstructor<SpriteVertex> for ComfyFillVertexConstructor {
    fn new_vertex(&mut self, vertex: FillVertex) -> SpriteVertex {
        let p = vertex.position();
        let c = self.0;
        SpriteVertex {
            position: [p.x, p.y, 0.],
            color: [c.r, c.g, c.b, c.a],
            tex_coords: [p.x, p.y],
        }
    }
}

pub fn tessellate_fill(graph: &BBGraph) -> BBResult<Mesh> {
    let path_result = graph.generate_fill_path();

    let Ok(path) = path_result else {
        let reason = path_result.unwrap_err();
        return Err(reason);
    };

    let mut buffers: VertexBuffers<SpriteVertex, u32> = VertexBuffers::new();
    // let mut vertex_builder = simple_builder(&mut buffers);
    let mut vertex_builder = BuffersBuilder::new(&mut buffers, ComfyFillVertexConstructor(DARK_GRAY));
    let mut tess = FillTessellator::new();


    let _ = tess.tessellate(
            &path,          // PositionStore
            &FillOptions::default(),
            &mut vertex_builder
        );
    let m = Mesh {
        vertices: buffers.vertices.into(),
        indices: buffers.indices.into(),
        z_index: 0,
        texture: None,
    };

    Ok(m)
}

pub fn draw_graph_fill(graph: &BBGraph) -> BBResult<()> {
    let mesh = tessellate_fill(graph)?;
    draw_mesh(mesh);
    Ok(())
}

struct ComfyStrokeVertexConstructor(Color);

impl StrokeVertexConstructor<SpriteVertex> for ComfyStrokeVertexConstructor {
    fn new_vertex(&mut self, vertex: StrokeVertex) -> SpriteVertex {
        let p = vertex.position();
        let c = self.0;
        SpriteVertex {
            position: [p.x, p.y, 0.],
            color: [c.r, c.g, c.b, c.a],
            tex_coords: [p.x, p.y],
        }
    }
}

pub fn tessellate_stroke(graph: &BBGraph) -> BBResult<Mesh> {
    let path_result = graph.generate_stroke_path();

    let Ok(path) = path_result else {
        let reason = path_result.unwrap_err();
        return Err(reason);
    };

    let mut buffers: VertexBuffers<SpriteVertex, u32> = VertexBuffers::new();
    // let mut vertex_builder = simple_builder(&mut buffers);
    let mut vertex_builder = BuffersBuilder::new(&mut buffers, ComfyStrokeVertexConstructor(WHITE));
    let mut tess = StrokeTessellator::new();


    let _ = tess.tessellate(
            &path,          // PositionStore
            &StrokeOptions::default().with_line_width(0.08),
            &mut vertex_builder
        );

    let m = Mesh {
        vertices: buffers.vertices.into(),
        indices: buffers.indices.into(),
        z_index: 0,
        texture: None,
    };

    Ok(m)
}

pub fn draw_graph_stroke(graph: &BBGraph) -> BBResult<()> {
    let mesh = tessellate_stroke(graph)?;
    draw_mesh(mesh);
    Ok(())
}
