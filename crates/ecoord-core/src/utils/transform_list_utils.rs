use crate::Transform;
use chrono::{DateTime, Utc};

/// Returns true if all transforms are equal or if there is only one.
pub fn is_static(transforms: &[Transform]) -> bool {
    let first_transform = transforms.first().unwrap();
    transforms.iter().all(|t| {
        t.translation == first_transform.translation && t.rotation == first_transform.rotation
    })
}

pub fn get_previous_transform(
    transforms: &[Transform],
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

pub fn get_next_transform(
    transforms: &[Transform],
    timestamp: &DateTime<Utc>,
) -> Option<Transform> {
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
    use crate::utils::transform_list_utils::get_previous_transform;
    use crate::Transform;
    use chrono::{DateTime, TimeZone, Utc};
    use nalgebra::{UnitQuaternion, Vector3};

    #[test]
    fn test_basic_interpolation() {
        let transform_a = Transform::new(
            Utc.timestamp_opt(1, 1000).unwrap(),
            Vector3::new(0.0, 0.0, 0.0),
            UnitQuaternion::from_euler_angles(std::f64::consts::FRAC_PI_4, 0.0, 0.0),
        );
        let transform_b = Transform::new(
            Utc.timestamp_opt(1, 4000).unwrap(),
            Vector3::new(3.0, 6.0, -9.0),
            UnitQuaternion::from_euler_angles(std::f64::consts::PI, 0.0, 0.0),
        );
        let transforms: Vec<Transform> = vec![transform_a, transform_b]; // : &Vec<Transform>, timestamp: &DateTime<Utc>
        let timestamp: DateTime<Utc> = Utc.timestamp_opt(1, 2000).unwrap();
        let result = get_previous_transform(&transforms, &timestamp).unwrap();

        assert_eq!(result.translation, Vector3::new(0.0, 0.0, 0.0));
    }
}
