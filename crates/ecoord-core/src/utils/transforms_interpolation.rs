use crate::Error::{MissingTimestamp, NoTransforms};
use crate::utils::transform_list_utils::{get_next_transform, get_previous_transform, is_static};
use crate::{Error, ExtrapolationMethod, InterpolationMethod, Transform};
use chrono::{DateTime, Duration, Utc};
use nalgebra::{Isometry3, Translation3, UnitQuaternion, Vector3};

pub fn inter_and_extrapolate_transforms(
    transforms: &Vec<Transform>,
    timestamp: &Option<DateTime<Utc>>,
    interpolation_method: InterpolationMethod,
    extrapolation_method: ExtrapolationMethod,
) -> Result<Isometry3<f64>, Error> {
    if transforms.is_empty() {
        return Err(NoTransforms());
    }
    debug_assert!(
        transforms.is_sorted_by_key(|t| t.timestamp),
        "transforms must be sorted by timestamp"
    );
    debug_assert!(
        transforms
            .windows(2)
            .all(|t| t[0].timestamp < t[1].timestamp),
        "transforms must be strictly sorted by timestamp"
    );
    if is_static(transforms) {
        return Ok(transforms
            .first()
            .expect("at least one transform must be present")
            .isometry());
    }
    let timestamp = timestamp.ok_or(MissingTimestamp())?;
    let first_transform = transforms
        .first()
        .expect("at least one transform must be present");
    let last_transform = transforms
        .last()
        .expect("at least one transform must be present");
    if timestamp < first_transform.timestamp || last_transform.timestamp < timestamp {
        return match extrapolation_method {
            ExtrapolationMethod::Constant => Ok(extrapolate_constant(transforms, &timestamp)),
            ExtrapolationMethod::Linear => Ok(extrapolate_linear(transforms, &timestamp)),
        };
    }

    match interpolation_method {
        InterpolationMethod::Step => Ok(interpolate_step_function(transforms, &timestamp)),
        InterpolationMethod::Linear => Ok(interpolate_linearly(transforms, &timestamp)),
    }
}

fn extrapolate_constant(transforms: &Vec<Transform>, timestamp: &DateTime<Utc>) -> Isometry3<f64> {
    let first_transform = transforms
        .first()
        .expect("at least one transform must be present");
    if *timestamp <= first_transform.timestamp {
        return first_transform.isometry();
    }

    let last_transform = transforms
        .last()
        .expect("at least one transform must be present");
    if last_transform.timestamp <= *timestamp {
        return last_transform.isometry();
    }

    panic!(
        "timestamp is located within duration of the transforms and must therefore be interpolated"
    );
}

fn extrapolate_linear(transforms: &Vec<Transform>, timestamp: &DateTime<Utc>) -> Isometry3<f64> {
    let (t1, t2) = if *timestamp < transforms.first().expect("should work").timestamp {
        (&transforms[0], &transforms[1])
    } else if transforms.last().expect("should work").timestamp < *timestamp {
        let last_two = &transforms[transforms.len() - 2..];
        (&last_two[0], &last_two[1])
    } else {
        panic!(
            "timestamp is located within duration of the transforms and must therefore be interpolated"
        );
    };

    let difference_time = (t2.timestamp - t1.timestamp)
        .num_nanoseconds()
        .expect("should work") as f64;
    let alpha = (*timestamp - t1.timestamp)
        .num_nanoseconds()
        .expect("should work") as f64
        / difference_time;

    let translation: Vector3<f64> = t1.translation + alpha * (t2.translation - t1.translation);
    let rotation: UnitQuaternion<f64> = t1.rotation.slerp(&t2.rotation, alpha);

    Isometry3::from_parts(Translation3::from(translation), rotation)
}

fn interpolate_step_function(
    transforms: &Vec<Transform>,
    timestamp: &DateTime<Utc>,
) -> Isometry3<f64> {
    let transform = get_previous_transform(transforms, timestamp).unwrap_or_else(|| {
        transforms
            .first()
            .expect("at least one transform must be present")
            .clone()
    });

    Isometry3::from_parts(transform.translation.into(), transform.rotation)
}

/// Implements a linear interpolation for a vector of transforms.
///
/// If requested [timestamp] is before the first transform in the vector, simply the first
/// transform is returned.
fn interpolate_linearly(transforms: &Vec<Transform>, timestamp: &DateTime<Utc>) -> Isometry3<f64> {
    let previous_transform =
        get_previous_transform(transforms, timestamp).expect("previous transform must be present");
    let next_transform =
        get_next_transform(transforms, timestamp).expect("next transform must be present");

    let duration: Duration = next_transform.timestamp - previous_transform.timestamp;
    let first_duration = *timestamp - previous_transform.timestamp;

    let weight: f64 = first_duration
        .num_nanoseconds()
        .expect("nanoseconds should be derivable") as f64
        / duration
            .num_nanoseconds()
            .expect("nanoseconds should be derivable") as f64;

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
    use approx::relative_eq;
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
        let result = interpolate_linearly(&transforms, &timestamp);

        assert_eq!(result.translation, Translation3::new(1.0, 2.0, -3.0));
        assert_eq!(
            result.rotation.euler_angles(),
            (std::f64::consts::FRAC_PI_2, 0.0, 0.0)
        );
    }

    #[test]
    fn test_linear_extrapolation_after() {
        let transform_a = Transform::new(
            Utc.timestamp_opt(1, 0).unwrap(),
            Vector3::new(1.0, 2.0, 3.0),
            UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
        );
        let transform_b = Transform::new(
            Utc.timestamp_opt(2, 0).unwrap(),
            Vector3::new(2.0, 3.0, 4.0),
            UnitQuaternion::from_euler_angles(std::f64::consts::FRAC_PI_2, 0.0, 0.0),
        );
        let transforms: Vec<Transform> = vec![transform_a, transform_b];
        let timestamp: DateTime<Utc> = Utc.timestamp_opt(3, 0).unwrap();
        let result = extrapolate_linear(&transforms, &timestamp);

        assert_eq!(result.translation, Translation3::new(3.0, 4.0, 5.0));
        relative_eq!(result.rotation.euler_angles().0, std::f64::consts::PI,);
    }

    #[test]
    fn test_linear_extrapolation_before() {
        let transform_a = Transform::new(
            Utc.timestamp_opt(1, 0).unwrap(),
            Vector3::new(1.0, 2.0, 3.0),
            UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
        );
        let transform_b = Transform::new(
            Utc.timestamp_opt(2, 0).unwrap(),
            Vector3::new(2.0, 3.0, 4.0),
            UnitQuaternion::from_euler_angles(std::f64::consts::FRAC_PI_2, 0.0, 0.0),
        );
        let transforms: Vec<Transform> = vec![transform_a, transform_b];
        let timestamp: DateTime<Utc> = Utc.timestamp_opt(0, 0).unwrap();
        let result = extrapolate_linear(&transforms, &timestamp);

        assert_eq!(result.translation, Translation3::new(0.0, 1.0, 2.0));
        relative_eq!(
            result.rotation.euler_angles().0,
            -std::f64::consts::FRAC_PI_2,
        );
    }
}
