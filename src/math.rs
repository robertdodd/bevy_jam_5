pub fn get_angle_for_arc_length(arc_length: f32, radius: f32) -> f32 {
    // Arc length = rθ × π/180
    // arc_length = radius * angle
    // angle = arc_length / radius
    arc_length / radius
}

#[cfg(test)]
mod tests {
    use std::f32::consts::{PI, TAU};

    use super::*;

    #[test]
    fn get_angle_for_arc_length_works() {
        let radius = 1.;
        let circumference = 2. * PI * radius;
        assert_eq!(TAU, get_angle_for_arc_length(circumference, radius));
    }
}
