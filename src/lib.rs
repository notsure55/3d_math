pub mod matrix;
pub mod matrix_four;
pub mod matrix_three;
pub mod vec_three;
pub mod vec_two;

// Used to calculate answers
// https://www.emathhelp.net/linear-algebra-calculator/

#[cfg(test)]
mod tests {
    use crate::vec_three::Vec3;

    #[test]
    fn vector_angle() {
        let origin = Vec3::new(100.0, 90.0, 80.0);
        let other = Vec3::new(150.0, 60.0, 70.0);

        let angle = origin.angle(&other);

        assert!(angle == 19.364285)
    }
    #[test]
    fn triple_product() {
        let origin = Vec3::new(100.0, 90.0, 80.0);
        let other = Vec3::new(150.0, 60.0, 70.0);
        let other1 = Vec3::new(80.0, 50.0, 20.0);

        let product = origin.triple_product(&other, &other1);

        assert!(product == 220000.0)
    }
}
