#![feature(test)]
extern crate test;

extern crate nalgebra;
use nalgebra::*;

fn vector2slope(v: Vector2<f64>) -> f64 { v.y / v.x }

fn ellipse_slope(v: Vector2<f64>) -> f64 { -4.0*v.x/v.y }

fn is_exit_point(point: Vector2<f64>) -> bool {
    -0.01 <= point.x && point.x <= 0.01 && point.y > 0.0
}

// pointing inwards, not necessary normalized
fn normal_vec(point_on_ellipse: Vector2<f64>) -> Vector2<f64> {
    let y = point_on_ellipse.y;
    let slope = ellipse_slope(point_on_ellipse);

    match (slope.is_infinite(), slope.is_sign_positive()) {
        (true, true)  => Vector2::new(1.0, 0.0),
        (true, false) => Vector2::new(-1.0, 0.0),
        (false, _)    => Vector2::new(slope, -1.0) * y.signum(),
    }
}

fn angle_of_incidence(point_on_ellipse: Vector2<f64>, incidence_vec: Vector2<f64>) -> f64 {
    nalgebra::angle(&-incidence_vec, &normal_vec(point_on_ellipse) )
}

// compute next x by p-q-formula
fn next_point(v: Vector2<f64>, beam_direction: Vector2<f64>) -> Vector2<f64> {
    let (x, y) = (v.x, v.y);
    let slope = vector2slope(beam_direction);
    let p = -2.0*slope*(y-slope*x)/(4. + slope*slope);

    let next_x = p - x;
    let next_y = slope*(next_x-x)+y;
    Vector2::new(next_x, next_y)
}

fn main() {
    let mut last_pnt = Vector2::new(0.0, 10.1);
    let mut pnt = Vector2::new(1.4, -9.6);

    let mut n_reflections = 0;
    while !is_exit_point(pnt) {
        n_reflections += 1;
        let beam_direction = pnt - last_pnt;
        let incidence_angle = angle_of_incidence(pnt, beam_direction);
        let new_beam_direction = Rotation2::new(2.0*incidence_angle) * (-beam_direction);
        let next_pnt = next_point(pnt, new_beam_direction);
        last_pnt = pnt;
        pnt = next_pnt;
    }
    print!("{}", n_reflections);
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
