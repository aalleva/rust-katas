fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::math_utils::basic;
    use super::math_utils::geometry;
    use super::math_utils::statistics;

    #[test]
    fn test_basic_operations() {
        assert_eq!(basic::add(5, 3), 8);
        assert_eq!(basic::subtract(5, 3), 2);
        assert_eq!(basic::multiply(5, 3), 15);
        assert_eq!(basic::divide(6, 3), Some(2));
        assert_eq!(basic::divide(6, 0), None);
    }

    #[test]
    fn test_geometry() {
        assert_eq!(geometry::circle_area(1.0), std::f64::consts::PI);
        assert_eq!(geometry::circle_circumference(1.0), 2.0 * std::f64::consts::PI);
    }

    #[test]
    fn test_statistics() {
        assert_eq!(statistics::mean(&[1.0, 2.0, 3.0]), Some(2.0));
        assert_eq!(statistics::median(&mut [1.0, 3.0, 2.0]), Some(2.0));
    }
}
