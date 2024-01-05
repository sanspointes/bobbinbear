use glam::Vec2;

pub trait Determinate<TResult> {
    fn determinate(&self, other: Self) -> TResult;
}

impl Determinate<f32> for Vec2 {
    fn determinate(&self, other: Vec2) -> f32 {
        self.x * other.y - self.y * other.x
    }
}
