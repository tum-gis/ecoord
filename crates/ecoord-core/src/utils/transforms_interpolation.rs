use crate::Transform;
use crate::transform::TimedTransform;
use crate::utils::transform_list_utils::{get_previous_and_next_transform, get_previous_transform};
use chrono::{DateTime, Duration, Utc};
use nalgebra::{UnitQuaternion, Vector3};

pub(crate) fn extrapolate_constant(
    transforms: &Vec<TimedTransform>,
    timestamp: &DateTime<Utc>,
) -> Transform {
    let first_transform = transforms
        .first()
        .expect("at least one transform must be present");
    if *timestamp <= first_transform.timestamp {
        return first_transform.transform;
    }

    let last_transform = transforms
        .last()
        .expect("at least one transform must be present");
    if last_transform.timestamp <= *timestamp {
        return last_transform.transform;
    }

    panic!(
        "timestamp is located within duration of the transforms and must therefore be interpolated"
    );
}

pub(crate) fn extrapolate_linear(
    transforms: &Vec<TimedTransform>,
    timestamp: &DateTime<Utc>,
) -> Transform {
    let first_timed_transform = transforms.first().expect("should work");
    let last_timed_transform = transforms.last().expect("should work");

    // early returns
    if *timestamp == first_timed_transform.timestamp {
        return first_timed_transform.transform;
    }
    if *timestamp == last_timed_transform.timestamp {
        return last_timed_transform.transform;
    }

    // select interpolation candidates
    let (t1, t2) = if *timestamp < first_timed_transform.timestamp {
        (&transforms[0], &transforms[1])
    } else if last_timed_transform.timestamp < *timestamp {
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

    let translation: Vector3<f64> =
        t1.transform.translation + alpha * (t2.transform.translation - t1.transform.translation);
    let rotation: UnitQuaternion<f64> = t1.transform.rotation.slerp(&t2.transform.rotation, alpha);

    Transform::new(translation, rotation)
}

pub(crate) fn interpolate_step_function(
    transforms: &Vec<TimedTransform>,
    timestamp: &DateTime<Utc>,
) -> Transform {
    /*transforms
    .binary_search_by_key(&13, |&(a, b)| b)
    .expect("TODO: panic message");*/

    let timed_transform = get_previous_transform(transforms, timestamp).unwrap_or_else(|| {
        *transforms
            .first()
            .expect("at least one transform must be present")
    });

    timed_transform.transform
}

/// Implements a linear interpolation for a vector of transforms.
///
/// If requested [timestamp] is before the first transform in the vector, simply the first
/// transform is returned.
pub(crate) fn interpolate_linearly(
    transforms: &Vec<TimedTransform>,
    timestamp: &DateTime<Utc>,
) -> Transform {
    let (previous_transform, next_transform) =
        get_previous_and_next_transform(transforms, timestamp);

    let previous_transform = previous_transform.expect("previous transform must be present");
    let next_transform = next_transform.expect("next transform must be present");

    let duration: Duration = next_transform.timestamp - previous_transform.timestamp;
    let first_duration = *timestamp - previous_transform.timestamp;

    let weight: f64 = first_duration
        .num_nanoseconds()
        .expect("nanoseconds should be derivable") as f64
        / duration
            .num_nanoseconds()
            .expect("nanoseconds should be derivable") as f64;

    let translation = previous_transform.transform.translation * (1.0 - weight)
        + next_transform.transform.translation * weight;
    let rotation = previous_transform
        .transform
        .rotation
        .slerp(&next_transform.transform.rotation, weight);
    Transform::new(translation, rotation)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Transform;
    use approx::relative_eq;
    use chrono::TimeZone;
    use nalgebra::{Translation3, UnitQuaternion, Vector3};

    #[test]
    fn test_basic_interpolation() {
        let transform_a = TimedTransform::new(
            Utc.timestamp_opt(1, 1000).unwrap(),
            Transform::new(
                Vector3::new(0.0, 0.0, 0.0),
                UnitQuaternion::from_euler_angles(std::f64::consts::FRAC_PI_4, 0.0, 0.0),
            ),
        );
        let transform_b = TimedTransform::new(
            Utc.timestamp_opt(4, 4000).unwrap(),
            Transform::new(
                Vector3::new(3.0, 6.0, -9.0),
                UnitQuaternion::from_euler_angles(std::f64::consts::PI, 0.0, 0.0),
            ),
        );
        let transforms: Vec<TimedTransform> = vec![transform_a, transform_b]; // : &Vec<Transform>, timestamp: &DateTime<Utc>
        let timestamp: DateTime<Utc> = Utc.timestamp_opt(2, 2000).unwrap();
        let result = interpolate_linearly(&transforms, &timestamp);

        assert_eq!(result.translation, Vector3::new(1.0, 2.0, -3.0));
        assert_eq!(
            result.rotation.euler_angles(),
            (std::f64::consts::FRAC_PI_2, 0.0, 0.0)
        );
    }

    #[test]
    fn test_linear_extrapolation_after() {
        let transform_a = TimedTransform::new(
            Utc.timestamp_opt(1, 0).unwrap(),
            Transform::new(
                Vector3::new(1.0, 2.0, 3.0),
                UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
            ),
        );
        let transform_b = TimedTransform::new(
            Utc.timestamp_opt(2, 0).unwrap(),
            Transform::new(
                Vector3::new(2.0, 3.0, 4.0),
                UnitQuaternion::from_euler_angles(std::f64::consts::FRAC_PI_2, 0.0, 0.0),
            ),
        );
        let transforms: Vec<TimedTransform> = vec![transform_a, transform_b];
        let timestamp: DateTime<Utc> = Utc.timestamp_opt(3, 0).unwrap();
        let result = extrapolate_linear(&transforms, &timestamp);

        assert_eq!(result.translation, Vector3::new(3.0, 4.0, 5.0));
        relative_eq!(result.rotation.euler_angles().0, std::f64::consts::PI,);
    }

    #[test]
    fn test_linear_extrapolation_before() {
        let transform_a = TimedTransform::new(
            Utc.timestamp_opt(1, 0).unwrap(),
            Transform::new(
                Vector3::new(1.0, 2.0, 3.0),
                UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
            ),
        );
        let transform_b = TimedTransform::new(
            Utc.timestamp_opt(2, 0).unwrap(),
            Transform::new(
                Vector3::new(2.0, 3.0, 4.0),
                UnitQuaternion::from_euler_angles(std::f64::consts::FRAC_PI_2, 0.0, 0.0),
            ),
        );
        let transforms: Vec<TimedTransform> = vec![transform_a, transform_b];
        let timestamp: DateTime<Utc> = Utc.timestamp_opt(0, 0).unwrap();
        let result = extrapolate_linear(&transforms, &timestamp);

        assert_eq!(result.translation, Vector3::new(0.0, 1.0, 2.0));
        relative_eq!(
            result.rotation.euler_angles().0,
            -std::f64::consts::FRAC_PI_2,
        );
    }
}
