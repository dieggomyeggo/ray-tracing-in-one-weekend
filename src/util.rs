use nalgebra::Vector3;

pub fn unit_vector(v: &Vector3<f64>) -> Vector3<f64> {
    v / v.magnitude()
}
