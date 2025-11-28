use crate::transform::TimedTransform;
use chrono::{DateTime, Utc};

pub fn get_previous_transform(
    transforms: &[TimedTransform],
    timestamp: &DateTime<Utc>,
) -> Option<TimedTransform> {
    debug_assert!(
        transforms
            .windows(2)
            .all(|w| w[0].timestamp <= w[1].timestamp),
        "transforms must be sorted by timestamp"
    );

    /*transforms
    .iter()
    .filter(|t| t.1.timestamp <= *timestamp)
    .max_by_key(|t| t.1.timestamp)
    .copied()*/

    let idx = transforms.partition_point(|t| t.timestamp <= *timestamp);

    if idx == 0 {
        None
    } else {
        Some(transforms[idx - 1])
    }
}

pub fn get_previous_and_next_transform(
    transforms: &[TimedTransform],
    timestamp: &DateTime<Utc>,
) -> (Option<TimedTransform>, Option<TimedTransform>) {
    debug_assert!(
        transforms
            .windows(2)
            .all(|w| w[0].timestamp <= w[1].timestamp),
        "transforms must be sorted by timestamp"
    );

    let idx = transforms.partition_point(|t| t.timestamp <= *timestamp);

    let previous_transform = if idx == 0 {
        None
    } else {
        Some(transforms[idx - 1])
    };

    let next_transform = if idx >= transforms.len() {
        None
    } else {
        Some(transforms[idx])
    };

    (previous_transform, next_transform)
}

pub fn get_next_transform(
    transforms: &[TimedTransform],
    timestamp: &DateTime<Utc>,
) -> Option<TimedTransform> {
    debug_assert!(
        transforms
            .windows(2)
            .all(|w| w[0].timestamp <= w[1].timestamp),
        "transforms must be sorted by timestamp"
    );

    /*transforms
    .iter()
    .filter(|t| *timestamp < t.timestamp)
    .min_by_key(|t| t.timestamp)
    .copied()*/

    let idx = transforms.partition_point(|t| t.timestamp <= *timestamp);

    if idx >= transforms.len() {
        None
    } else {
        Some(transforms[idx])
    }
}

#[cfg(test)]
mod test_get_previous {
    use crate::utils::transform_list_utils::get_previous_transform;
    use crate::{TimedTransform, Transform};
    use chrono::{DateTime, TimeZone, Utc};
    use nalgebra::{UnitQuaternion, Vector3};

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
            Utc.timestamp_opt(1, 4000).unwrap(),
            Transform::new(
                Vector3::new(3.0, 6.0, -9.0),
                UnitQuaternion::from_euler_angles(std::f64::consts::PI, 0.0, 0.0),
            ),
        );
        let transforms: Vec<TimedTransform> = vec![transform_a, transform_b];
        let timestamp: DateTime<Utc> = Utc.timestamp_opt(1, 2000).unwrap();
        let result = get_previous_transform(&transforms, &timestamp).unwrap();

        assert_eq!(result.transform.translation, Vector3::new(0.0, 0.0, 0.0));
    }
}
