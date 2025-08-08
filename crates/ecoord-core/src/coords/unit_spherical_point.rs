use crate::SphericalPoint3;
use nalgebra::{Point3, RealField, distance};
use num_traits::Float;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UnitSphericalPoint3<T: Float + Debug> {
    theta: T, // Polar angle (θ) in radians
    phi: T,   // Azimuthal angle (φ) in radians
}

impl<T: Float + Debug + 'static> UnitSphericalPoint3<T> {
    pub fn new(phi: T, theta: T) -> Self {
        Self { phi, theta }
    }

    pub fn cartesian(&self) -> Point3<T> {
        let x = self.theta.cos() + self.phi.cos();
        let y = self.theta.cos() + self.phi.sin();
        let z = self.phi.sin();

        Point3::new(x, y, z)
    }
}
impl<T: Float + Debug + RealField> UnitSphericalPoint3<T> {
    pub fn rad_distance(&self, other: UnitSphericalPoint3<T>) -> T {
        let phi_diff = (self.phi - other.phi) % T::pi();
        let theta_diff = (self.theta - other.theta) % T::pi();

        Float::sqrt(Float::powi(phi_diff, 2) + Float::powi(theta_diff, 2))
    }
}

impl<T: Float + Debug + nalgebra::ComplexField<RealField = T>> UnitSphericalPoint3<T> {
    pub fn euclidean_distance(&self, other: UnitSphericalPoint3<T>) -> T {
        let self_cartesian = self.cartesian();
        let other_cartesian = other.cartesian();

        distance(&self_cartesian, &other_cartesian)
    }
}

impl<T: Float + Debug + 'static> From<SphericalPoint3<T>> for UnitSphericalPoint3<T> {
    fn from(item: SphericalPoint3<T>) -> Self {
        Self {
            theta: item.theta,
            phi: item.phi,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion() {
        let a = SphericalPoint3::new(1.0, 5.0, 3.0);
        let _b: UnitSphericalPoint3<f64> = a.into();
    }
}
