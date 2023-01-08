use std::str::FromStr;

/// Additional information for a frame.
///
///
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct TransformInfo {
    pub interpolation_method: Option<InterpolationMethod>,
}

impl TransformInfo {
    pub fn new(interpolation_method: Option<InterpolationMethod>) -> Self {
        Self {
            interpolation_method,
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

impl FromStr for InterpolationMethod {
    type Err = ();

    fn from_str(input: &str) -> Result<InterpolationMethod, Self::Err> {
        match input {
            "step" => Ok(InterpolationMethod::Step),
            "linear" => Ok(InterpolationMethod::Linear),
            _ => Err(()),
        }
    }
}

impl InterpolationMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            InterpolationMethod::Step => "step",
            InterpolationMethod::Linear => "linear",
        }
    }
}
