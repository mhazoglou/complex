use complex::*;

#[test]
fn test_add_two_complex() {
    let z1 = Complex::<f64>::new(1., 2.);
    let z2 = Complex::<f64>::new(2., 3.);
    assert_eq!(z1 + z2, Complex::<f64>::new(3., 5.));
}

#[test]
fn test_add_complex_and_f64() {
    let z1 = Complex::<f64>::new(1., 2.);
    assert_eq!(z1 + 3.0_f64, Complex::<f64>::new(4., 2.));
    assert_eq!(3.0_f64 + z1, Complex::<f64>::new(4., 2.));
}

#[test]
fn test_sub_two_complex() {
    let z1 = Complex::<f64>::new(1., 2.);
    let z2 = Complex::<f64>::new(2., 3.);
    assert_eq!(z1 - z2, Complex::<f64>::new(-1., -1.));
    assert_eq!(z2 - z1, Complex::<f64>::new(1., 1.));
}

#[test]
fn test_sub_complex_and_f64() {
    let z1 = Complex::<f64>::new(1., 2.);
    assert_eq!(z1 - 3.0_f64, Complex::<f64>::new(-2., 2.));
    assert_eq!(3.0_f64 - z1, Complex::<f64>::new(2., -2.));
}

#[test]
fn test_mul_two_complex() {
    let z1 = Complex::<f64>::new(1., 2.);
    let z2 = Complex::<f64>::new(1., -2.);
    assert_eq!(z1 * z2, Complex::<f64>::new(5., 0.));
}

#[test]
fn test_mul_complex_and_f64() {
    let z1 = Complex::<f64>::new(1., 2.);
    assert_eq!(z1 * 3.0_f64, Complex::<f64>::new(3., 6.));
    assert_eq!(3.0_f64 * z1, Complex::<f64>::new(3., 6.));
}

#[test]
fn test_div_two_complex() {
    let z1 = Complex::<f64>::new(1., 2.);
    let z2 = Complex::<f64>::new(1., 2.);
    assert_eq!(z1 / z2, Complex::<f64>::new(1., 0.));
}

#[test]
fn test_div_complex_and_f64() {
    let z1 = Complex::<f64>::new(1., 1.);
    assert_eq!(z1 / 2.0_f64, Complex::<f64>::new(0.5, 0.5));
    assert_eq!(2.0_f64 / z1, Complex::<f64>::new(1., -1.));
}

#[test]
fn test_neg_complex() {
    let z1 = Complex::<f64>::new(1., -2.);
    assert_eq!(-z1, Complex::<f64>::new(-1., 2.));
}

// #[test]
// fn test_exp_complex() {
// let z1 = Complex::<f64>::new(0., 1.);
// assert_eq!(z1.exp(), Complex::<f64>::new(1_f64.cos(), 1_f64.sin()));
// }

#[test]
fn test_add_two_quaternions() {
    let z1 = complex![1., 2., 3., 4.];
    let z2 = complex![2., 3., 4., 5.];
    assert_eq!(z1 + z2, complex![3., 5., 7., 9.]);
}

#[test]
fn test_add_quaternion_and_f64() {
    let z1 = complex![2., -1., 3., 4.];
    assert_eq!(z1 + 3.0_f64, complex![5., -1., 3., 4.]);
    assert_eq!(3.0_f64 + z1, complex![5., -1., 3., 4.]);
}

#[test]
fn test_sub_two_quaternion() {
    let z1 = complex![1., 2., 3., 4.];
    let z2 = complex![2., 3., 4., 5.];
    assert_eq!(z1 - z2, complex![-1.0, -1.0, -1.0, -1.0]);
    assert_eq!(z2 - z1, complex![1.0, 1.0, 1.0, 1.0]);
}

#[test]
fn test_sub_quaternion_and_f64() {
    let z1 = complex![1., 2., 3., 4.];
    assert_eq!(z1 - 3.0_f64, complex![-2., 2., 3., 4.]);
    assert_eq!(3.0_f64 - z1, complex![2., -2., -3., -4.]);
}

#[test]
fn test_mul_two_quaternion() {
    let z1 = complex![1., 0., 2., 3.];
    let z2 = complex![1., 0., -2., -3.];
    assert_eq!(z1 * z2, complex!(14., 0., 0., 0.));
}

#[test]
fn test_commutator_two_quaternion() {
    let z1 = complex![1., -1., 1., 1.];
    let z2 = complex![1., 1., 1., -1.];
    assert_eq!(z1 * z2 - z2 * z1, complex!(0., -4., 0., -4.));
}

#[test]
fn test_mul_quaternion_and_f64() {
    let z1 = complex![1., 2., 3., 4.];
    assert_eq!(z1 * 3.0_f64, complex!(3., 6., 9., 12.));
    assert_eq!(3.0_f64 * z1, complex!(3., 6., 9., 12.));
}

#[test]
fn test_div_two_quaternion() {
    let z1 = complex![1., 2., 3., 4.];
    let z2 = complex![1., 2., 3., 4.];
    assert_eq!(z1 / z2, complex!(1., 0., 0., 0.));
}

#[test]
fn test_div_quaternion_and_f64() {
    let z1 = complex![1., 1., 1., 1.];
    assert_eq!(z1 / 2.0_f64, complex!(0.5, 0.5, 0.5, 0.5));
    assert_eq!(2.0_f64 / z1, complex!(0.5, -0.5, -0.5, -0.5));
}

#[test]
fn test_neg_quaternion() {
    let z1 = complex![1., -2., 3., -4.];
    assert_eq!(-z1, complex!(-1., 2., -3., 4.));
}

// #[test]
// fn test_exp_quaternion() {
// let z1 = Complex::<f64>::new(0., 1.);
// assert_eq!(z1.exp(), Complex::<f64>::new(1_f64.cos(), 1_f64.sin()));
// }

#[test]
fn test_add_two_octonions() {
    let z1 = complex![1., 2., 3., 4., 1., 2., 3., 4.];
    let z2 = complex![2., 3., 4., 5., -1., -2., -3., -4.];
    assert_eq!(z1 + z2, complex![3., 5., 7., 9., 0., 0., 0., 0.]);
}

#[test]
fn test_add_octonion_and_f64() {
    let z1 = complex![2., -1., 3., 4., 2., -1., 3., 4.];
    assert_eq!(z1 + 3.0_f64, complex![5., -1., 3., 4., 2., -1., 3., 4.]);
    assert_eq!(3.0_f64 + z1, complex![5., -1., 3., 4., 2., -1., 3., 4.]);
}

#[test]
fn test_sub_two_octonion_f64() {
    let z1 = complex![1., 2., 3., 4., 1., 2., 3., 4.];
    let z2 = complex![2., 3., 4., 5., 1., 2., 3., 4.];
    assert_eq!(
        z1 - z2,
        complex![-1.0, -1.0, -1.0, -1.0, 0.0, 0.0, 0.0, 0.0]
    );
    assert_eq!(z2 - z1, complex![1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0]);
}

#[test]
fn test_sub_octonion_and_f64() {
    let z1 = complex![1., 2., 3., 4., 1., 2., 3., 4.];
    assert_eq!(z1 - 3.0_f64, complex![-2., 2., 3., 4., 1., 2., 3., 4.]);
    assert_eq!(
        3.0_f64 - z1,
        complex![2., -2., -3., -4., -1., -2., -3., -4.]
    );
}

#[test]
fn test_mul_two_octonion() {
    let z1 = complex![1., 0., 2., 3., 2., 0., 1., 3.];
    let z2 = complex![1., 0., -2., -3., -2., 0., -1., -3.];
    assert_eq!(z1 * z2, complex!(28., 0., 0., 0., 0., 0., 0., 0.));
}

#[test]
fn test_commutator_two_octonion() {
    let z1 = complex![1., -1., 1., 1., 0., 0., 0., 0.];
    let z2 = complex![1., 1., 1., -1., 0., 0., 0., 0.];
    assert_eq!(
        z1 * z2 - z2 * z1,
        complex!(0., -4., 0., -4., 0., 0., 0., 0.)
    );
}

#[test]
fn test_associator_two_octonion() {
    let z1 = complex![1., -1., 1., 1., 0., 1., 0., 0.];
    let z2 = complex![1., 1., 1., -1., 0., 0., 0., 1.];
    let z3 = complex![1., 1., -1., 1., 0., 0., 1., 0.];
    assert_ne!(
        (z1 * z2) * z3 - z1 * (z2 * z3),
        complex!(0., 0., 0., 0., 0., 0., 0., 0.),
        "General associator for octonions is non-zero."
    );
}

#[test]
fn test_mul_octonion_and_f64() {
    let z1 = complex![1., 2., 3., 4., 1., 2., 3., 4.];
    assert_eq!(z1 * 3.0_f64, complex!(3., 6., 9., 12., 3., 6., 9., 12.));
    assert_eq!(3.0_f64 * z1, complex!(3., 6., 9., 12., 3., 6., 9., 12.));
}

#[test]
fn test_div_two_octonion() {
    let z1 = complex![1., 2., 3., 4., 1., 2., 3., 4.];
    let z2 = complex![1., 2., 3., 4., 1., 2., 3., 4.];
    assert_eq!(z1 / z2, complex!(1., 0., 0., 0., 0., 0., 0., 0.));
}

#[test]
fn test_div_octonion_and_f64() {
    let z1 = complex![1., 1., 1., 1., 1., 1., 1., 1.];
    assert_eq!(
        z1 / 2.0_f64,
        complex!(0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5)
    );
    assert_eq!(
        2.0_f64 / z1,
        complex!(0.25, -0.25, -0.25, -0.25, -0.25, -0.25, -0.25, -0.25)
    );
}

#[test]
fn test_neg_octonion() {
    let z1 = complex![1., -2., 3., -4., 1., -2., 3., -4.];
    assert_eq!(-z1, complex!(-1., 2., -3., 4., -1., 2., -3., 4.));
}

// #[test]
// fn test_exp_quaternion() {
// let z1 = Complex::<f64>::new(0., 1.);
// assert_eq!(z1.exp(), Complex::<f64>::new(1_f64.cos(), 1_f64.sin()));
// }
