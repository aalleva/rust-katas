pub fn circle_area(radius: f64) -> f64 {
    std::f64::consts::PI * radius.powi(2)
}

pub fn circle_circumference(radius: f64) -> f64 {
    2.0 * std::f64::consts::PI * radius
}
