use rapier2d::na::Vector2;
use std::f64::consts::PI;

pub fn angle_between_positions(a: Vector2<f32>, b: Vector2<f32>) -> f32 {
    let b_relative_to_a = Vector2::new(b.x - a.x, b.y - a.y);
    let up = Vector2::new(0.0, -1.0);

    let angle_from_up = Vector2::angle(&up, &b_relative_to_a);

    if b_relative_to_a.x > 0.0 {
        return angle_from_up;
    } else {
        return -angle_from_up;
    }
}

pub fn delta_angle(from: f32, to: f32) -> f32 {
    let total = 2.0 * PI as f32;

    let a = if from.is_sign_positive() {
        from
    } else {
        total - from.abs()
    };
    let b = if to.is_sign_positive() {
        to
    } else {
        total - to.abs()
    };

    let x = if b > a { total - (b - a) } else { a - b };

    if x > (PI as f32) {
        return total - x;
    } else {
        return -x;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delta_angle_neg160_to_160() {
        let from_angle: f32 = -160.0;
        let to_angle: f32 = 160.0;
        assert_eq!(
            delta_angle(from_angle.to_radians(), to_angle.to_radians())
                .to_degrees()
                .round(),
            -40.0
        );
    }

    #[test]
    fn delta_angle_20_to_neg10() {
        let from_angle: f32 = 20.0;
        let to_angle: f32 = -10.0;
        assert_eq!(
            delta_angle(from_angle.to_radians(), to_angle.to_radians())
                .to_degrees()
                .round(),
            -30.0
        );
    }

    #[test]
    fn delta_angle_110_to_160() {
        let from_angle: f32 = 110.0;
        let to_angle: f32 = 160.0;
        assert_eq!(
            delta_angle(from_angle.to_radians(), to_angle.to_radians())
                .to_degrees()
                .round(),
            50.0
        );
    }

    #[test]
    fn delta_angle_neg12_t0_2() {
        let from_angle: f32 = -12.0;
        let to_angle: f32 = 2.0;
        assert_eq!(
            delta_angle(from_angle.to_radians(), to_angle.to_radians())
                .to_degrees()
                .round(),
            14.0
        );
    }
}
