use crate::utils::transform_list_utils::{get_next_transform, get_previous_transform, is_static};
use crate::Error::{MissingTimestamp, MissingTransforms, TransformsNotSortedByTime};
use crate::{Error, InterpolationMethod, Transform};
use chrono::{DateTime, Duration, Utc};
use nalgebra::Isometry3;

pub fn interpolate_transforms(
    transforms: &Vec<Transform>,
    timestamp: &Option<DateTime<Utc>>,
    method: InterpolationMethod,
) -> Result<Isometry3<f64>, Error> {
    if transforms.is_empty() {
        return Err(MissingTransforms());
    }
    if !transforms
        .windows(2)
        .all(|t| t[0].timestamp < t[1].timestamp)
    {
        return Err(TransformsNotSortedByTime());
    }
    if is_static(transforms) {
        return Ok(transforms.first().unwrap().isometry());
    }

    if timestamp.is_none() {
        return Err(MissingTimestamp());
    }
    let timestamp = timestamp.unwrap();
    if timestamp <= transforms.first().unwrap().timestamp {
        return Ok(transforms.first().unwrap().isometry());
    }
    if transforms.last().unwrap().timestamp <= timestamp {
        return Ok(transforms.last().unwrap().isometry());
    }

    match method {
        InterpolationMethod::Step => interpolate_step_function(transforms, &timestamp),
        InterpolationMethod::Linear => interpolate_linearly(transforms, &timestamp),
    }
}

fn interpolate_step_function(
    transforms: &Vec<Transform>,
    timestamp: &DateTime<Utc>,
) -> Result<Isometry3<f64>, Error> {
    if transforms.is_empty() {
        return Err(MissingTransforms());
    }

    let transform = get_previous_transform(transforms, timestamp)
        .unwrap_or_else(|| transforms.first().unwrap().clone());

    let isometry = Isometry3::from_parts(transform.translation.into(), transform.rotation);
    Ok(isometry)
}

/// Implements a linear interpolation for a vector of transforms.
///
/// If requested [timestamp] is before the first transform in the vector, simply the first
/// transform is returned.
fn interpolate_linearly(
    transforms: &Vec<Transform>,
    timestamp: &DateTime<Utc>,
) -> Result<Isometry3<f64>, Error> {
    if transforms.is_empty() {
        return Err(MissingTransforms());
    }

    let previous_transform = get_previous_transform(transforms, timestamp).unwrap();
    let next_transform = get_next_transform(transforms, timestamp).unwrap(); //.unwrap_or(transforms.last().unwrap().clone());

    let duration: Duration = next_transform.timestamp - previous_transform.timestamp;
    let _duration_num = duration.num_nanoseconds();
    let first_duration = *timestamp - previous_transform.timestamp;
    let _first_duration_num = first_duration.num_nanoseconds();

    let weight: f64 = first_duration.num_nanoseconds().unwrap() as f64
        / duration.num_nanoseconds().unwrap() as f64;

    let translation =
        previous_transform.translation * (1.0 - weight) + next_transform.translation * weight;
    let rotation = previous_transform
        .rotation
        .slerp(&next_transform.rotation, weight);
    let isometry = Isometry3::from_parts(translation.into(), rotation);
    Ok(isometry)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use nalgebra::{Translation3, UnitQuaternion, Vector3};

    #[test]
    fn test_basic_interpolation() {
        let transform_a = Transform::new(
            Utc.timestamp_opt(1, 1000).unwrap(),
            Vector3::new(0.0, 0.0, 0.0),
            UnitQuaternion::from_euler_angles(std::f64::consts::FRAC_PI_4, 0.0, 0.0),
        );
        let transform_b = Transform::new(
            Utc.timestamp_opt(4, 4000).unwrap(),
            Vector3::new(3.0, 6.0, -9.0),
            UnitQuaternion::from_euler_angles(std::f64::consts::PI, 0.0, 0.0),
        );
        let transforms: Vec<Transform> = vec![transform_a, transform_b]; // : &Vec<Transform>, timestamp: &DateTime<Utc>
        let timestamp: DateTime<Utc> = Utc.timestamp_opt(2, 2000).unwrap();
        let result = interpolate_linearly(&transforms, &timestamp).unwrap();

        assert_eq!(result.translation, Translation3::new(1.0, 2.0, -3.0));
        assert_eq!(
            result.rotation.euler_angles(),
            (std::f64::consts::FRAC_PI_2, 0.0, 0.0)
        );
    }
}
