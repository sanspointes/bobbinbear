use bevy_math::Vec2;
use lyon_tessellation::math::Point;

pub trait ToPoint {
    fn to_point(&self) -> Point;
}

impl ToPoint for Vec2 {
    fn to_point(&self) -> Point {
        Point::new(self.x, self.y)
    }
}
