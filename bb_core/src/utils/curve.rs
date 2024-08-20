use bevy::math::Vec2;

pub fn quadratic_point_at(p0: Vec2, p1: Vec2, p2: Vec2, t: f32) -> Vec2 {
    let a = p0.lerp(p1, t);
    let b = p1.lerp(p2, t);
    a.lerp(b, t)
}

#[allow(dead_code)]
// Function to calculate the derivative of a quadratic Bezier curve at a given t
pub fn quadratic_derivative_at(p0: Vec2, p1: Vec2, p2: Vec2, t: f32) -> Vec2 {
    let mt = 1.0 - t;
    Vec2 {
        x: 2.0 * mt * (p1.x - p0.x) + 2.0 * t * (p2.x - p1.x),
        y: 2.0 * mt * (p1.y - p0.y) + 2.0 * t * (p2.y - p1.y),
    }
}

#[allow(dead_code)]
// Function to calculate the normal of a quadratic Bezier curve at a given t
pub fn quadratic_normal_at(p0: Vec2, p1: Vec2, p2: Vec2, t: f32) -> Vec2 {
    let derivative = quadratic_derivative_at(p0, p1, p2, t);
    derivative.perp().normalize()
}

pub fn cubic_point_at(p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2, t: f32) -> Vec2 {
    let mut a = p0.lerp(p1, t);
    let mut b = p1.lerp(p2, t);
    let c = p2.lerp(p3, t);
    a = a.lerp(b, t);
    b = b.lerp(c, t);
    a.lerp(b, t)
}

#[allow(dead_code)]
// Function to calculate the derivative of a cubic Bezier curve at a given t
pub fn cubic_derivative_at(p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2, t: f32) -> Vec2 {
    let mt = 1.0 - t;
    Vec2 {
        x: 3.0 * mt * mt * (p1.x - p0.x)
            + 6.0 * mt * t * (p2.x - p1.x)
            + 3.0 * t * t * (p3.x - p2.x),
        y: 3.0 * mt * mt * (p1.y - p0.y)
            + 6.0 * mt * t * (p2.y - p1.y)
            + 3.0 * t * t * (p3.y - p2.y),
    }
}

#[allow(dead_code)]
// Function to calculate the normal of a cubic Bezier curve at a given t
pub fn cubic_normal_at(p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2, t: f32) -> Vec2 {
    let derivative = cubic_derivative_at(p0, p1, p2, p3, t);
    derivative.perp().normalize()
}
