extern crate cgmath;
use cgmath::{Array, Matrix, Matrix2, Matrix3, Matrix4, SquareMatrix, Vector1, Vector2, Vector3, Vector4, Point1, Point2, Point3};
use std::panic::{catch_unwind, AssertUnwindSafe};

macro_rules! matrix_tests {
    ($module:ident, $type:ident, $size:expr) => { mod $module {
        use super::*;
        fn populated() -> $type<f64> {
            let mut m = $type::identity();
            for c in 0..$size { for r in 0..$size { m[c][r] = (c * $size + r + 1) as f64; } }
            m
        }
        #[test] fn columns_equal_and_distinct() {
            let original = populated();
            for a in 0..$size { for b in 0..$size {
                let mut m = original; m.swap_columns(a, b);
                for c in 0..$size { for r in 0..$size {
                    let source = if c == a { b } else if c == b { a } else { c };
                    assert_eq!(m[c][r], original[source][r]);
                } }
            } }
        }
        #[test] fn elements_equal_and_distinct() {
            let original = populated();
            for ac in 0..$size { for ar in 0..$size { for bc in 0..$size { for br in 0..$size {
                let a = (ac, ar); let b = (bc, br); let mut m = original; m.swap_elements(a, b);
                for c in 0..$size { for r in 0..$size {
                    let source = if (c, r) == a { b } else if (c, r) == b { a } else { (c, r) };
                    assert_eq!(m[c][r], original[source.0][source.1]);
                } }
            } } } }
        }
        #[test] fn rows_equal_and_distinct() {
            let original = populated();
            for a in 0..$size { for b in 0..$size {
                let mut m = original; m.swap_rows(a, b);
                for c in 0..$size { for r in 0..$size {
                    let source = if r == a { b } else if r == b { a } else { r };
                    assert_eq!(m[c][r], original[c][source]);
                } }
            } }
        }
        #[test] fn invalid_indices_panic_without_mutation() {
            let original = populated();
            for &(a, b) in &[(0, $size), ($size, 0), ($size, $size)] {
                let mut m = original;
                assert!(catch_unwind(AssertUnwindSafe(|| m.swap_columns(a, b))).is_err());
                assert_eq!(m, original);
                let mut m = original;
                assert!(catch_unwind(AssertUnwindSafe(|| m.swap_rows(a, b))).is_err());
                assert_eq!(m, original);
            }
            for &(a, b) in &[((0,0),($size,0)), ((0,0),(0,$size)), (($size,0),($size,0)), ((0,$size),(0,$size))] {
                let mut m = original;
                assert!(catch_unwind(AssertUnwindSafe(|| m.swap_elements(a, b))).is_err());
                assert_eq!(m, original);
            }
        }
    } }
}
matrix_tests!(matrix2, Matrix2, 2);
matrix_tests!(matrix3, Matrix3, 3);
matrix_tests!(matrix4, Matrix4, 4);

fn check_array<A: Array<Element = i32> + Copy>(original: A) {
    for a in 0..A::len() { for b in 0..A::len() {
        let mut value = original; value.swap_elements(a, b);
        for i in 0..A::len() {
            let source = if i == a { b } else if i == b { a } else { i };
            assert_eq!(value[i], original[source]);
        }
    } }
    for &(a,b) in &[(0,A::len()), (A::len(),0), (A::len(),A::len())] {
        let mut value = original;
        assert!(catch_unwind(AssertUnwindSafe(|| value.swap_elements(a,b))).is_err());
        for i in 0..A::len() { assert_eq!(value[i], original[i]); }
    }
}
#[test] fn vector_arrays() {
    check_array(Vector1::new(1)); check_array(Vector2::new(1,2));
    check_array(Vector3::new(1,2,3)); check_array(Vector4::new(1,2,3,4));
}
#[test] fn point_arrays() {
    check_array(Point1::new(1)); check_array(Point2::new(1,2)); check_array(Point3::new(1,2,3));
}
