use complex::*;

fn main() {
    let z = Complex::new(1.0_f64, -1.0_f64);
    let z_star = z.conj();

    println!("{}", &z);

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
    let Q = Complex::<Complex<f64>>::new(z_star, z);
    println!("Build some quaternions recursively");
    println!("{}", q);
    println!("{}\n", Q);
    println!("Add some quaternions using recursion");
    println!("{}", q + Q);
    println!("Add a quaternion with a complex number");
    println!("{}", q + z);
    println!("And it commutes");
    println!("{}\n", z + q);
    println!("Conjugate a quaternion using recursion");
    println!("{}\n", q.conj());
    println!("Multiply quaternions using recursion");
    println!("{}", q * q.conj());
    println!("This does not commute:");
    println!("{}", q * Q);
    println!("{}", Q * q);
    println!("Commutator:");
    println!("{\n}", q * Q - Q * q);

    println!("Norm Squared:");
    println!("{}\n", Q.abs_sq());
    println!("Inverse of a quaternion:");
    println!("{}\n", 1. / Q);

    println!("Macros are great for constructing the");
    // macros to construct Complex type easily only work
    // if the number of values is a power of 2
    let z_z_top = complex![-1.0_f64, -5_f64];
    println!("{}", z_z_top);
    let quaternion = complex![1.0_f64, 2.0_f64, 3.0_f64, 4.0_f64];
    println!("{}", quaternion);
    let octonion = complex![1.0_f64, 2.0_f64, 3.0_f64, 4.0_f64, 5.0_f64, 6.0_f64, 7.0_f64, 8.0_f64];
    println!("{}", octonion);
    let (r, polar_octo) = octonion.polar_rep();
    println!("Polar representation");
    println!("{}, {}", r, polar_octo);
    println!("r*exp(polar_octo)");
    println!("{}", r * polar_octo.exp());
}
