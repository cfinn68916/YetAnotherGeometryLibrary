mod hedron;
mod line;
mod ray;
mod simple_plane;
mod simple_tri;
mod vectors;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hedron::{Polyhedron, Tetrahedron};
    use crate::ray::Ray;
    use crate::simple_plane::SimplePlane;
    use crate::vectors::{Vector2, Vector3};
    use gon::Polygon;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
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
            Err("Ray is perpendicular to plane".to_string())
        );
        assert_eq!(
            plane.ray_intersects(Ray::new(
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(-1.0, 0.0, 0.0)
            )),
            Err("Ray points away from plane".to_string())
        );
        assert_eq!(
            plane.ray_intersects(Ray::new(
                Vector3::new(0.0, 2.0, 1.0),
                Vector3::new(1.0, 5.0, 3.0)
            )),
            Ok(Vector3::new(1.0, 7.0, 4.0))
        )
    }
    #[test]
    fn test_hedron() {
        let hedr = Tetrahedron::new(
            Vector3::zero(),
            Vector3::ihat() * 3.0,
            Vector3::jhat() * 3.0,
            -Vector3::khat() * 3.0,
        );
        assert_eq!(hedr.volume(), -4.5);
    }
}
