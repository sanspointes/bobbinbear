use std::ops::Add;

use comfy::{Mul, Vec2Extensions, LIME_GREEN, PURPLE, RED, WHITE, GRAY};
use glam::Vec2;

use crate::traits::Determinate;

pub fn draw_determinate(pos: Vec2, v1: Vec2, v2: Vec2, color: comfy::Color, z_index: i32) {
    let det = v1.determinate(v2);
    let color = color.mul(if det > 0. { 1.5 } else { 0.5 });

    comfy::draw_line(pos, pos + v1, 0.02, color, 100);
    comfy::draw_line(pos + v1, pos + v1 + v2, 0.02, color, 100);
    comfy::draw_line(pos, pos + v2, 0.02, color, 100);
    comfy::draw_line(pos + v2, pos + v1 + v2, 0.02, color, 100);
    // comfy::draw_line(pos, Vec2::new(pos.x + v2.x, pos.y), 0.02, color, 100);
    // comfy::draw_line(Vec2::new(pos.x + v2.x, pos.y + v2.y), Vec2::new(pos.x, pos.y + v1.y), 0.02, color, 100);
}

pub fn draw_det_arc(pos: Vec2, radius: f32, dcurr: Vec2, da: Vec2, db: Vec2) {
    let count = (radius * 30.) as usize;
    for v in 0..count {
        let da = Vec2::new(0., 1.);
        let theta = (v as f32) / (count as f32) * comfy::PI * 2.;
        let da = Vec2::new(
            da.x * theta.cos() - da.y * theta.sin(),
            da.x * theta.sin() + da.y * theta.cos(),
        );

        let is_convex = dcurr.determinate(db) > 0.;
        let ccw_of_prev = dcurr.determinate(da) > 0.;
        let ccw_of_next = da.determinate(db) > 0.;

        let color = if (!is_convex && ccw_of_prev && ccw_of_next) || (is_convex && ccw_of_prev || ccw_of_next) {
            LIME_GREEN
        } else {
            RED
        };

        let color = if is_convex {
            color.mul(0.5)
        } else {
            color
        };


        comfy::draw_circle(pos + da * radius, 0.05, color, 50);
    }

    let is_convex = db.determinate(dcurr) > 0.;
    let ccw_of_prev = da.determinate(dcurr) > 0.;
    let ccw_of_next = db.determinate(da) > 0.;
    comfy::draw_circle(pos + dcurr.normalize().mul(radius), 0.08, GRAY, 100);
    comfy::draw_circle(pos + da.normalize().mul(radius), 0.08, WHITE, 100);
    comfy::draw_text(
        &format!("[{},{},{}]", is_convex, ccw_of_prev, ccw_of_next),
        pos + da,
        WHITE,
        comfy::TextAlign::Center,
    );
}
