use nalgebra::{ComplexField, Point3, RealField, Scalar};
use std::fmt;

/// Represents a point in three-dimensional spherical coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SphericalPoint3<T: Scalar + Copy> {
    pub r: T,     // Radial distance
    pub theta: T, // Polar angle (θ) in radians
    pub phi: T,   // Azimuthal angle (φ) in radians
}

impl<T: Scalar + Copy + fmt::Debug> SphericalPoint3<T> {
    pub fn new(r: T, theta: T, phi: T) -> Self {
        Self { r, theta, phi }
    }
}

impl<T: Scalar + Copy + ComplexField> From<SphericalPoint3<T>> for Point3<T> {
    fn from(item: SphericalPoint3<T>) -> Self {
        let x = item.r * item.theta.cos() * item.phi.cos();
        let y = item.r * item.theta.cos() * item.phi.sin();
        let z = item.r * item.theta.sin();
        Point3::new(x, y, z)
    }
}

impl<T: Scalar + Copy + RealField> From<Point3<T>> for SphericalPoint3<T> {
    fn from(item: Point3<T>) -> Self {
        let phi = item.y.atan2(item.x);
        let theta = item.z.atan2((item.x.powi(2) + item.y.powi(2)).sqrt());
        let r = item.coords.norm();

        SphericalPoint3 { r, theta, phi }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use approx::assert_relative_eq;

    #[test]
    fn test_spherical_point_creation() {
        let point = SphericalPoint3 {
            r: 2.0,
            theta: std::f64::consts::PI / 4.0,
            phi: std::f64::consts::PI / 2.0,
        };

        assert_eq!(point.r, 2.0);
        assert_eq!(point.theta, std::f64::consts::PI / 4.0);
        assert_eq!(point.phi, std::f64::consts::PI / 2.0);
    }

    #[test]
    fn test_spherical_point_equality() {
        let point1 = SphericalPoint3 {
            r: 2.0,
            theta: std::f64::consts::PI / 4.0,
            phi: std::f64::consts::PI / 2.0,
        };

        let point2 = SphericalPoint3 {
            r: 2.0,
            theta: std::f64::consts::PI / 4.0,
            phi: std::f64::consts::PI / 2.0,
        };

        assert_eq!(point1, point2);
    }

    #[test]
    fn test_spherical_point_inequality() {
        let point1 = SphericalPoint3 {
            r: 2.0,
            theta: std::f64::consts::PI / 4.0,
            phi: std::f64::consts::PI / 2.0,
        };

        let point2 = SphericalPoint3 {
            r: 1.0,
            theta: std::f64::consts::PI / 4.0,
            phi: std::f64::consts::PI / 2.0,
        };

        assert_ne!(point1, point2);
    }

    #[test]
    fn test_spherical_to_cartesian_conversion() {
        let spherical_point = SphericalPoint3 {
            r: 2.0,
            theta: std::f64::consts::PI / 4.0,
            phi: std::f64::consts::PI / 2.0,
        };
        let expected_cartesian_point = Point3::new(0.0, 2.0f64.sqrt(), 2.0f64.sqrt());

        let converted_cartesian_point: Point3<f64> = spherical_point.into();

        assert_relative_eq!(
            converted_cartesian_point.x,
            expected_cartesian_point.x,
            epsilon = f64::EPSILON
        );
        assert_relative_eq!(
            converted_cartesian_point.y,
            expected_cartesian_point.y,
            epsilon = f64::EPSILON
        );
        assert_relative_eq!(
            converted_cartesian_point.z,
            expected_cartesian_point.z,
            epsilon = f64::EPSILON
        );
    }

    #[test]
    fn test_cartesian_to_spherical_conversion() {
        let cartesian_point = Point3::new(1.0, 1.0, 2.0);
        let expected_spherical_point = SphericalPoint3 {
            r: 2.449489742783178,
            theta: 0.9553166181245093,
            phi: std::f64::consts::FRAC_PI_4,
        };

        let converted_spherical_point: SphericalPoint3<f64> = cartesian_point.into();

        assert_relative_eq!(
            converted_spherical_point.r,
            expected_spherical_point.r,
            epsilon = f64::EPSILON
        );
        assert_relative_eq!(
            converted_spherical_point.theta,
            expected_spherical_point.theta,
            epsilon = f64::EPSILON
        );
        assert_relative_eq!(
            converted_spherical_point.phi,
            expected_spherical_point.phi,
            epsilon = f64::EPSILON
        );
    }
}
