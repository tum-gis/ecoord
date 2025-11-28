/// Methods for interpolating a list of [`Transform`].
///
/// [`Transform`]: crate::Transform
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub enum InterpolationMethod {
    /// Step function interpolation
    #[default]
    Step,
    /// Linear interpolation
    Linear,
}

/// Methods for extrapolating a list of [`Transform`].
///
/// [`Transform`]: crate::Transform
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub enum ExtrapolationMethod {
    /// Step function interpolation
    #[default]
    Constant,
    /// Linear interpolation
    Linear,
}
