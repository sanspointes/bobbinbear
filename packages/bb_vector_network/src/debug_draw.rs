use comfy::{Mul, BLUE, GRAY, LIME_GREEN, RED, WHITE};
use glam::Vec2;

use crate::traits::Determinate;

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
