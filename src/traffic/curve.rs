use macroquad::prelude::Vec2;

const BEZIER_CURVE_RESOLUTION: usize = 100;

/// A quadratic Bézier curve.
///
/// https://en.wikipedia.org/wiki/B%C3%A9zier_curve#Quadratic_curves
pub fn quadratic_curve(start: Vec2, control: Vec2, end: Vec2) -> Vec<Vec2> {
    let mut points = Vec::with_capacity(BEZIER_CURVE_RESOLUTION + 1);
    for i in 0..=BEZIER_CURVE_RESOLUTION {
        let t = i as f32 / BEZIER_CURVE_RESOLUTION as f32;
        let x = (1.0 - t).powi(2) * start.x + 2.0 * (1.0 - t) * t * control.x + t.powi(2) * end.x;
        let y = (1.0 - t).powi(2) * start.y + 2.0 * (1.0 - t) * t * control.y + t.powi(2) * end.y;
        points.push(Vec2::new(x, y));
    }
    points
}
