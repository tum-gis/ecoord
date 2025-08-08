/// Additional information for a frame.
///
///
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct TransformInfo {
    pub interpolation_method: InterpolationMethod,
    pub extrapolation_method: ExtrapolationMethod,
}

impl TransformInfo {
    pub fn new(
        interpolation_method: InterpolationMethod,
        extrapolation_method: ExtrapolationMethod,
    ) -> Self {
        Self {
            interpolation_method,
            extrapolation_method,
        }
    }
}

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
