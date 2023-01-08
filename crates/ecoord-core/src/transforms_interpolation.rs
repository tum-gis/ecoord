use crate::{InterpolationMethod, Transform};
use chrono::{DateTime, Duration, Utc};
use nalgebra::Isometry3;

pub fn interpolate_transforms(
    transforms: &Vec<Transform>,
    timestamp: &Option<DateTime<Utc>>,
    method: InterpolationMethod,
) -> Isometry3<f64> {
    assert!(
        !transforms.is_empty(),
        "At least one transform must be provided."
    );
    assert!(
        transforms
            .windows(2)
            .all(|t| t[0].timestamp < t[1].timestamp),
        "Transforms must be sorted by time."
    );

    if is_static(transforms) {
        return transforms.first().unwrap().isometry();
    }

    assert!(
        timestamp.is_some(),
        "Timestamp is required, if it is not set."
    );
    let timestamp = timestamp.unwrap();
    if timestamp <= transforms.first().unwrap().timestamp {
        return transforms.first().unwrap().isometry();
    }
    if transforms.last().unwrap().timestamp <= timestamp {
        return transforms.last().unwrap().isometry();
    }

    match method {
        InterpolationMethod::Step => interpolate_step_function(transforms, &timestamp),
        InterpolationMethod::Linear => interpolate_linearly(transforms, &timestamp),
    }
}

fn is_static(transforms: &Vec<Transform>) -> bool {
    let first_transform = transforms.first().unwrap();
    transforms.iter().all(|t| {
        t.translation == first_transform.translation && t.rotation == first_transform.rotation
    })
}

fn interpolate_step_function(
    transforms: &Vec<Transform>,
    timestamp: &DateTime<Utc>,
) -> Isometry3<f64> {
    assert!(
        !transforms.is_empty(),
        "At least one transform must be provided."
    );

    let transform = get_previous_transform(transforms, timestamp)
        .unwrap_or(transforms.first().unwrap().clone());

    Isometry3::from_parts(transform.translation.into(), transform.rotation)
}

fn get_previous_transform(
    transforms: &Vec<Transform>,
    timestamp: &DateTime<Utc>,
) -> Option<Transform> {
    let previous_timestamps: Vec<&Transform> = transforms
        .iter()
        .filter(|t| t.timestamp.timestamp_nanos() <= timestamp.timestamp_nanos())
        .collect();

    let previous = previous_timestamps
        .iter()
        .max_by_key(|t| t.timestamp.timestamp_nanos())
        .map(|&t| t.clone());

    previous
}

fn get_next_transform(transforms: &Vec<Transform>, timestamp: &DateTime<Utc>) -> Option<Transform> {
    let next_timestamps: Vec<&Transform> = transforms
        .iter()
        .filter(|t| timestamp.timestamp_nanos() < t.timestamp.timestamp_nanos())
        .collect();

    let next = next_timestamps
        .iter()
        .min_by_key(|t| t.timestamp.timestamp_nanos())
        .map(|&t| t.clone());

    next
}

#[cfg(test)]
mod test_get_previous {
    use crate::transforms_interpolation::get_previous_transform;
    use crate::Transform;
    use chrono::{DateTime, TimeZone, Utc};
    use nalgebra::{UnitQuaternion, Vector3};

    #[test]
    fn test_basic_interpolation() {
        let transform_a = Transform::new(
            Utc.timestamp_opt(1, 1000).unwrap(),
            None,
            Vector3::new(0.0, 0.0, 0.0),
            UnitQuaternion::from_euler_angles(std::f64::consts::FRAC_PI_4, 0.0, 0.0),
        );
        let transform_b = Transform::new(
            Utc.timestamp_opt(1, 4000).unwrap(),
            None,
            Vector3::new(3.0, 6.0, -9.0),
            UnitQuaternion::from_euler_angles(std::f64::consts::PI, 0.0, 0.0),
        );
        let transforms: Vec<Transform> = vec![transform_a, transform_b]; // : &Vec<Transform>, timestamp: &DateTime<Utc>
        let timestamp: DateTime<Utc> = Utc.timestamp_opt(1, 2000).unwrap();
        let result = get_previous_transform(&transforms, &timestamp).unwrap();

        assert_eq!(result.translation, Vector3::new(0.0, 0.0, 0.0));
    }
}

/// Implements a linear interpolation for a vector of transforms.
///
/// If requested [timestamp] is before the first transform in the vector, simply the first
/// transform is returned.
fn interpolate_linearly(transforms: &Vec<Transform>, timestamp: &DateTime<Utc>) -> Isometry3<f64> {
    assert!(
        !transforms.is_empty(),
        "At least one transform must be provided."
    );

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
    Isometry3::from_parts(translation.into(), rotation)
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
            None,
            Vector3::new(0.0, 0.0, 0.0),
            UnitQuaternion::from_euler_angles(std::f64::consts::FRAC_PI_4, 0.0, 0.0),
        );
        let transform_b = Transform::new(
            Utc.timestamp_opt(4, 4000).unwrap(),
            None,
            Vector3::new(3.0, 6.0, -9.0),
            UnitQuaternion::from_euler_angles(std::f64::consts::PI, 0.0, 0.0),
        );
        let transforms: Vec<Transform> = vec![transform_a, transform_b]; // : &Vec<Transform>, timestamp: &DateTime<Utc>
        let timestamp: DateTime<Utc> = Utc.timestamp_opt(2, 2000).unwrap();
        let result = interpolate_linearly(&transforms, &timestamp);

        assert_eq!(result.translation, Translation3::new(1.0, 2.0, -3.0));
        assert_eq!(
            result.rotation.euler_angles(),
            (std::f64::consts::FRAC_PI_2, 0.0, 0.0)
        );
    }
}
