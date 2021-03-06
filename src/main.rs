use complex::*;

fn main() {
    println!("{}", 2.0_f64.exp());
    let z = Complex::new(8.0_f64, 2.0_f64);
    let zomg = z.sin();
    println!("Zoh my god: {}", zomg);
    let z2 = complex![2., 1.];
    let z3 = z / z2;
    let z4 = z % z2;
    println!("{}", z - z2 * z3.trunc());
    println!("ratio: {}", z4);
    let z_star = z.conj();
    let v = vec![z; 3];
    let vec = vec![
        1., 2., 3., 4., 5., 6., 7., 8., 1., 2., 3., 4., 5., 6., 7., 8.,
    ];
    let sede = Sedenionf64::from_slice(&vec[..]);
    let sede2 = Sedenionf64::from_vec(vec);
    assert_eq!(sede, sede2);
    println!("{}", sede);

    let mut zero = Octonionf64::zero();
    zero += Octonionf64::one();
    println!("{}", zero);

    println!("{}", z.powu(3));
    println!("{}", z.powi(-3));
    println!("{}", z.powf(3.0));
    println!(
        "Sum works on complex types too: {}",
        v.iter().sum::<Complex<f64>>()
    );

    println!("{}", z * z_star);
    println!("{}", z.abs_sq());

    println!("Add some complex numbers");
    println!("{}", z + z_star);
    println!("Add a complex number and a float (real)");
    println!("{}", z + 2.);
    println!("Add a float and a complex number");
    println!("{}", 2.0_f64 + z);
    println!("Subtract a float from a complex number");
    println!("{}", z - 2.);
    println!("Subtract a complex number from a float");
    println!("{}\n", 2. - z);

    let q = Complex::<Complex<f64>>::new(z, z_star);
    let kyu = Complex::<Complex<f64>>::new(z_star, z);
    println!("Build some quaternions recursively");
    println!("{}", q);
    println!("{}\n", kyu);
    println!("Add some quaternions using recursion");
    println!("{}", q + kyu);
    println!("Add a quaternion with a complex number");
    println!("{}", q + z);
    println!("And it commutes");
    println!("{}\n", z + q);
    println!("Conjugate a quaternion using recursion");
    println!("{}\n", q.conj());
    println!("Multiply quaternions using recursion");
    println!("{}", q * q.conj());
    println!("This does not commute:");
    println!("{}", q * kyu);
    println!("{}", kyu * q);
    println!("Commutator:");
    println!("{\n}", q * kyu - kyu * q);
    // println!("{}", q.powz(q));

    println!("Norm Squared:");
    println!("{}\n", kyu.abs_sq());
    println!("Inverse of a quaternion:");
    println!("{}\n", 1. / kyu);

    println!("Macros are great for constructing the");
    // macros to construct Complex type easily only work
    // if the number of values is a power of 2
    let z_z_top = complex![-1.0_f64, -5_f64];
    println!("{}", z_z_top);
    let quaternion = complex![1.0_f64, 2.0_f64, 3.0_f64, 4.0_f64];
    println!("{}", quaternion);
    let octonion = complex![1.0_f64, 2.0_f64, 3.0_f64, 4.0_f64, 5.0_f64, 6.0_f64, 7.0_f64, 8.0_f64];
    println!("{}", octonion);
    let polar_octo = octonion.ln();
    println!("Logarithm");
    println!("{}", polar_octo);
    println!("exp(polar_octo)");
    println!("{}", polar_octo.exp());

    let poop = Sedenionf64::i();
    println!("IMAGINARIES YEAH: {}", poop);
}
