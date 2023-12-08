use std::ops::Add;

use comfy::{Mul, Vec2Extensions, BLUE, GRAY, LIME, LIME_GREEN, PURPLE, RED, WHITE};
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

fn cw_of_convex_node(dcurr: Vec2, da: Vec2, db: Vec2) -> bool {
    dcurr.determinate(da) >= 0. || db.determinate(da) >= 0.
}
fn cw_of_concave_node(dcurr: Vec2, da: Vec2, db: Vec2) -> bool {
    dcurr.determinate(da) >= 0. || db.determinate(da) >= 0.
}

pub fn draw_det_arc(pos: Vec2, radius: f32, curr_dir: Vec2, el_dir: Vec2, next_dir: Vec2) {
    let count = (radius * 30.) as usize;
    for v in 0..count {
        let el_dir = Vec2::new(0., 1.);
        let theta = (v as f32) / (count as f32) * comfy::PI * 2.;
        let el_dir = Vec2::new(
            el_dir.x * theta.cos() - el_dir.y * theta.sin(),
            el_dir.x * theta.sin() + el_dir.y * theta.cos(),
        );

        let is_convex = next_dir.determinate(curr_dir) > 0.;
        let ccw_of_curr = curr_dir.determinate(el_dir) >= 0.;
        let ccw_of_next = next_dir.determinate(el_dir) >= 0.;

        let color = if (!is_convex && ccw_of_curr && ccw_of_next)
            || (is_convex && (ccw_of_curr || ccw_of_next))
        {
            LIME_GREEN
        } else {
            RED
        };
        // let color = if (!is_convex && ccw_of_curr && ccw_of_next)
        //     || (is_convex && ccw_of_curr || ccw_of_next)
        // {
        //     LIME_GREEN
        // } else {
        //     RED
        // };

        let color = if is_convex { color.mul(0.5) } else { color };

        comfy::draw_circle(pos + el_dir * radius, 0.05, color, 50);

        let ccw_of_curr_color = if ccw_of_curr { LIME_GREEN } else { RED };
        let ccw_of_next_color = if ccw_of_next { LIME_GREEN } else { RED };
        comfy::draw_circle(pos + el_dir * (radius - 0.08), 0.04, ccw_of_curr_color, 50);
        comfy::draw_circle(pos + el_dir * (radius - 0.16), 0.04, ccw_of_next_color, 50);
    }

    let is_convex = next_dir.determinate(curr_dir);
    let ccw_of_curr = curr_dir.determinate(el_dir);
    let ccw_of_next = next_dir.determinate(el_dir);
    comfy::draw_circle(pos + next_dir.normalize().mul(radius), 0.08, GRAY, 100);
    comfy::draw_circle(pos + el_dir.normalize().mul(radius), 0.08, BLUE, 100);
    comfy::draw_text(
        &format!(
            "convex:{:.1}\nccw_curr:{:.1}\nccw_next:{:.1}",
            is_convex, ccw_of_curr, ccw_of_next
        ),
        pos + el_dir,
        WHITE,
        comfy::TextAlign::Center,
    );
}
