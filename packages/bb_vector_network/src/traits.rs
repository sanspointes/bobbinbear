use glam::Vec2;

pub trait Determinate<TResult> {
    fn determinate(&self, other: Self) -> TResult;
}

impl Determinate<f32> for Vec2 {
    fn determinate(&self, other: Vec2) -> f32 {
        self.x * other.y - self.y * other.x
    }
}

pub trait AngleBetween {
    fn angle_between_ccw(&self, other: Vec2) -> f32;
    fn angle_between_cw(&self, other: Vec2) -> f32;
}

impl AngleBetween for Vec2 {
    fn angle_between_ccw(&self, other: Vec2) -> f32 {
        // Normalize the vectors
        let v1 = self.normalize();
        let v2 = other.normalize();

        // Calculate the angle in radians between the vectors
        let mut angle = v2.y.atan2(v2.x) - v1.y.atan2(v1.x);

        // Adjust the angle to be in the range [0, 2*PI)
        if angle <= 0.0 {
            angle += 2.0 * std::f32::consts::PI;
        }

        // Return the angle in radians
        angle
    }
    fn angle_between_cw(&self, other: Vec2) -> f32 {
        // Normalize the vectors
        let v1 = self.normalize();
        let v2 = other.normalize();

        // Calculate the angle in radians between the vectors
        let mut angle = v1.y.atan2(v1.x) - v2.y.atan2(v2.x);

        // Adjust the angle to be in the range [0, 2*PI)
        if angle < 0.0 {
            angle += 2.0 * std::f32::consts::PI;
        }

        // Return the angle in radians
        angle
    }
}

mod tests {
    #[allow(unused_imports)]
    mod angle_between_trait {
        use glam::vec2;
        use crate::traits::AngleBetween;

        #[test]
        fn it_calculates_angles_correctly() {
            let a = vec2(1., 0.);
            let b = vec2(0., 1.);
            assert_eq!(a.angle_between_cw(b).to_degrees(), 270.);
            assert_eq!(b.angle_between_cw(a).to_degrees(), 90.);
        }
    }
}
