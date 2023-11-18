use glam::Vec2;

pub trait Determinate<TResult> {
    fn determinate(&self, other: Self) -> TResult;
}

impl Determinate<f32> for Vec2 {
    fn determinate(&self, other: Vec2) -> f32 {
        self.x * other.y - self.y * other.x
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec2;

    use super::Determinate;

    #[test]
    fn test_determinate() {
        let v_up = Vec2::new(0., 1.);
        let v_left = Vec2::new(-1., 0.);
        let v_right = Vec2::new(1., 0.);
        let v_down = Vec2::new(0., -1.);

        let up_left_det = v_up.determinate(v_left);
        assert_eq!(up_left_det, 1., "up_left_det");
        let up_right_det = v_up.determinate(v_right);
        assert_eq!(up_right_det, -1., "up_right_det");
        let up_down_det = v_up.determinate(v_down);
        assert_eq!(up_down_det, -0., "up_down_det");

        let left_down_det = v_left.determinate(v_down);
        assert_eq!(left_down_det, 1., "left_down_det");

        let left_right_det = v_left.determinate(v_right);
        assert_eq!(left_right_det, -0., "left_right_det");

        let down_right_det = v_down.determinate(v_right);
        assert_eq!(down_right_det, 1., "down_right_det");
    }
}
