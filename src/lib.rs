pub mod gon;
pub mod hedron;
pub mod line;
pub mod pose3;
pub mod quaternion;
pub mod ray;
pub mod rotation3;
pub mod simple_plane;
pub mod simple_tri;
pub mod utils;
pub mod vectors;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hedron::Tetrahedron;
    use crate::pose3::Pose3;
    use crate::ray::Ray;
    use crate::rotation3::Rotation3;
    use crate::simple_plane::{Intersection, SimplePlane};
    use crate::vectors::{Vector2, Vector3};
    use gon::Polygon;

    #[test]
    fn test_vectors() {
        let i = Vector3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let j = Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let k = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };

        assert_eq!(
            i + j,
            Vector3 {
                x: 1.0,
                y: 1.0,
                z: 0.0
            }
        );
        assert_eq!(
            i - j,
            Vector3 {
                x: 1.0,
                y: -1.0,
                z: 0.0
            }
        );
        assert_eq!(i.dot(&j), 0.0);
        assert_eq!(j.dot(&k), 0.0);
        assert_eq!(i.cross(&j), k);
        assert_eq!(j.cross(&i), -k);
        assert_eq!(i.cross(&(j * 2.0)), k * 2.0)
    }
    #[test]
    fn test_ray() {
        let plane = SimplePlane::new(Vector3::new(1.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0));

        assert_eq!(
            plane.ray_intersects(Ray::new(
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(0.0, 1.0, 0.0)
            )),
            Intersection::Never
        );
        assert_eq!(
            plane.ray_intersects(Ray::new(
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(-1.0, 0.0, 0.0)
            )),
            Intersection::Never
        );
        assert_eq!(
            plane.ray_intersects(Ray::new(
                Vector3::new(0.0, 2.0, 1.0),
                Vector3::new(1.0, 5.0, 3.0)
            )),
            Intersection::Once(Vector3::new(1.0, 7.0, 4.0))
        )
    }
    #[test]
    fn test_hedron() {
        let hedr = Tetrahedron::new(
            Vector3::zero(),
            Vector3::i_hat() * 3.0,
            Vector3::j_hat() * 3.0,
            -Vector3::k_hat() * 3.0,
        );
        assert_eq!(hedr.volume(), -4.5);
    }
    #[test]
    fn test_triangle() {
        let poly = Polygon::new(vec![
            Vector2::new(1.0, 0.0),
            Vector2::new(2.0, 0.0),
            Vector2::new(2.0, 1.0),
            Vector2::new(1.0, 1.0),
        ]);
        assert_eq!(poly.area(), 1.0);
    }
}
