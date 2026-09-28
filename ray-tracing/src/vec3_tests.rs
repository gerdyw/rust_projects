
#[cfg(test)]
mod tests {
    use crate::vec3::{Vec3, vec3};

    #[test]
    fn adds_two_vectors() {
        let a = vec3(1.0, 2.0, 3.0);
        let b = vec3(4.0, 5.0, 6.0);

        assert_eq!(a + b, vec3(5.0, 7.0, 9.0));
    }

    #[test]
    fn subtracts_two_vectors() {
        let a = vec3(4.0, 5.0, 6.0);
        let b = vec3(1.0, 2.0, 3.0);

        assert_eq!(a - b, vec3(3.0, 3.0, 3.0));
    }

    #[test]
    fn adding_zero_vector_returns_original() {
        let v = vec3(2.5, -1.0, 8.0);

        assert_eq!(v + vec3(0.0, 0.0, 0.0), v);
    }

    #[test]
    fn subtracting_self_returns_zero_vector() {
        let v = vec3(7.0, -3.0, 1.5);

        assert_eq!(v - v, vec3(0.0, 0.0, 0.0));
    }

    #[test]
    fn addition_is_component_wise() {
        assert_eq!(
            vec3(-1.0, 2.0, -3.0) + vec3(4.0, -5.0, 6.0),
            vec3(3.0, -3.0, 3.0)
        );
    }

    #[test]
    fn subtraction_is_component_wise() {
        assert_eq!(
            vec3(-1.0, 2.0, -3.0) - vec3(4.0, -5.0, 6.0),
            vec3(-5.0, 7.0, -9.0)
        );
    }

    #[test]
    fn negates_vector() {
        let v = vec3(1.0, -2.0, 3.0);

        assert_eq!(-v, vec3(-1.0, 2.0, -3.0));
    }

    #[test]
    fn multiplies_vector_by_scalar() {
        let v = vec3(1.0, -2.0, 3.0);

        assert_eq!(v * 2.0, vec3(2.0, -4.0, 6.0));
    }

    #[test]
    fn divides_vector_by_scalar() {
        let v = vec3(2.0, -4.0, 6.0);

        assert_eq!(v / 2.0f64, vec3(1.0, -2.0, 3.0));
    }

    #[test]
    fn scalar_multiplication_by_one_returns_original() {
        let v = vec3(1.5, -3.0, 7.0);

        assert_eq!(v * 1.0, v);
    }

    #[test]
    fn scalar_multiplication_by_zero_returns_zero() {
        let v = vec3(1.5, -3.0, 7.0);

        assert_eq!(v * 0.0, vec3(0.0, 0.0, 0.0));
    }

    #[test]
    fn vector_addition_is_commutative() {
        let a = vec3(1.0, 2.0, 3.0);
        let b = vec3(4.0, 5.0, 6.0);

        assert_eq!(a + b, b + a);
    }

    #[test]
    fn vector_addition_is_associative() {
        let a = vec3(1.0, 2.0, 3.0);
        let b = vec3(4.0, 5.0, 6.0);
        let c = vec3(7.0, 8.0, 9.0);

        assert_eq!((a + b) + c, a + (b + c));
    }

    #[test]
    fn subtraction_is_addition_of_negative() {
        let a = vec3(5.0, 6.0, 7.0);
        let b = vec3(1.0, 2.0, 3.0);

        assert_eq!(a - b, a + -b);
    }

    #[test]
    fn zero_vector_is_identity_for_addition() {
        let v = vec3(3.0, -4.0, 5.0);
        let zero = vec3(0.0, 0.0, 0.0);

        assert_eq!(v + zero, v);
    }

    #[test]
    fn indexing_returns_components() {
        let v = vec3(10.0, 20.0, 30.0);

        assert_eq!(v[0], 10.0);
        assert_eq!(v[1], 20.0);
        assert_eq!(v[2], 30.0);
    }

    fn assert_vec_approx_eq(a: Vec3, b: Vec3) {
        let epsilon = 1e-12;

        assert!((a[0] - b[0]).abs() < epsilon, "x differs: {} != {}", a[0], b[0]);
        assert!((a[1] - b[1]).abs() < epsilon, "y differs: {} != {}", a[1], b[1]);
        assert!((a[2] - b[2]).abs() < epsilon, "z differs: {} != {}", a[2], b[2]);
    }

    #[test]
    fn dot_product_of_parallel_vectors() {
        let a = vec3(1.0, 2.0, 3.0);
        let b = vec3(2.0, 4.0, 6.0);

        assert_eq!(a.dot(b), 28.0);
    }

    #[test]
    fn dot_product_of_perpendicular_vectors_is_zero() {
        let a = vec3(1.0, 0.0, 0.0);
        let b = vec3(0.0, 1.0, 0.0);

        assert_eq!(a.dot(b), 0.0);
    }

    #[test]
    fn dot_product_is_commutative() {
        let a = vec3(1.0, 2.0, 3.0);
        let b = vec3(4.0, 5.0, 6.0);

        assert_eq!(a.dot(b), b.dot(a));
    }

    #[test]
    fn cross_product_of_basis_vectors() {
        let x = vec3(1.0, 0.0, 0.0);
        let y = vec3(0.0, 1.0, 0.0);

        assert_eq!(x.cross(y), vec3(0.0, 0.0, 1.0));
    }

    #[test]
    fn cross_product_reverses_sign_when_order_swaps() {
        let a = vec3(1.0, 2.0, 3.0);
        let b = vec3(4.0, 5.0, 6.0);

        assert_eq!(a.cross(b), -b.cross(a));
    }

    #[test]
    fn cross_product_is_perpendicular_to_both_vectors() {
        let a = vec3(1.0, 2.0, 3.0);
        let b = vec3(4.0, 5.0, 6.0);

        let cross = a.cross(b);

        assert!((cross.dot(a)).abs() < 1e-12);
        assert!((cross.dot(b)).abs() < 1e-12);
    }

    #[test]
    fn cross_product_with_self_is_zero() {
        let a = vec3(1.0, 2.0, 3.0);

        assert_eq!(a.cross(a), vec3(0.0, 0.0, 0.0));
    }

    #[test]
    fn unit_vector_has_length_one() {
        let v = vec3(3.0, 4.0, 0.0);

        assert!((v.unit().length() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn unit_vector_preserves_direction() {
        let v = vec3(3.0, 4.0, 0.0);

        assert_vec_approx_eq(
            v.unit(),
            vec3(0.6, 0.8, 0.0),
        );
    }

    #[test]
    fn unit_vector_of_axis_vector_is_same_axis() {
        let v = vec3(0.0, 0.0, 5.0);

        assert_vec_approx_eq(
            v.unit(),
            vec3(0.0, 0.0, 1.0),
        );
    }
}
