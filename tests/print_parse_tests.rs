use complex::*;

#[test]
fn test_print_complexf32() {
    let z = Complex::new(1.0_f32, 2.0_f32);
    assert_eq!(z.to_string(), "1 + 2i");
}

#[test]
fn test_print_complexf64() {
    let z = Complex::new(1.0_f64, 2.0_f64);
    assert_eq!(z.to_string(), "1 + 2i");
}

#[test]
fn test_parse_complexf32() {
    let z = match "4+1i".parse::<Complex<f32>>() {
        Ok(num) => num,
        Err(_err) => panic!("Didn't parse complex number correctly"),
    };

    assert_eq!(z, Complex::new(4.0_f32, 1.0_f32));
}

#[test]
#[should_panic]
fn test_parse_complexf32_err() {
    let z = match "4b+1i".parse::<Complex<f32>>() {
        Ok(num) => num,
        Err(_err) => panic!("Didn't parse complex number correctly"),
    };

    assert_eq!(z, Complex::new(4.0_f32, 1.0_f32));
}

#[test]
fn test_parse_complexf64() {
    let z = match "4+1i".parse::<Complex<f64>>() {
        Ok(num) => num,
        Err(_err) => panic!("Didn't parse complex number correctly"),
    };

    assert_eq!(z, Complex::new(4.0_f64, 1.0_f64));
}

#[test]
#[should_panic]
fn test_parse_complexf64_err() {
    let z = match "4b+1i".parse::<Complex<f64>>() {
        Ok(num) => num,
        Err(_err) => panic!("Didn't parse complex number correctly"),
    };

    assert_eq!(z, Complex::new(4.0_f64, 1.0_f64));
}

#[test]
fn test_parse_complexf32_no_imaginary_part() {
    let z = match "4".parse::<Complex<f32>>() {
        Ok(num) => num,
        Err(_err) => panic!("Didn't parse complex number correctly"),
    };

    assert_eq!(z, Complex::new(4.0_f32, 0.0_f32));
}

#[test]
fn test_parse_complexf64_no_imaginary_part() {
    let z = match "4".parse::<Complex<f64>>() {
        Ok(num) => num,
        Err(_err) => panic!("Didn't parse complex number correctly"),
    };

    assert_eq!(z, Complex::new(4.0_f64, 0.0_f64));
}

#[test]
fn test_parse_complexf32_no_real_part() {
    let z = match "1i".parse::<Complex<f32>>() {
        Ok(num) => num,
        Err(_err) => panic!("Didn't parse complex number correctly"),
    };

    assert_eq!(z, Complex::new(0.0_f32, 1.0_f32));
}

#[test]
fn test_parse_complexf64_no_real_part() {
    let z = match "1i".parse::<Complex<f64>>() {
        Ok(num) => num,
        Err(_err) => panic!("Didn't parse complex number correctly"),
    };

    assert_eq!(z, Complex::new(0.0_f64, 1.0_f64));
}

#[test]
fn test_parse_quaternionf64() {
    let z = match "1+2i+3j+4k".parse::<Complex<Complex<f64>>>() {
        Ok(num) => num,
        Err(_err) => panic!("Didn't parse complex number correctly"),
    };

    assert_eq!(z, complex!(1.0_f64, 2.0_f64, 3.0_f64, 4.0_f64));
}

#[test]
fn test_parse_octonionf64() {
    let z = match "(1+2i+3j+4k, 5+6i+7j+8k)".parse::<Complex<Complex<Complex<f64>>>>() {
        Ok(num) => num,
        Err(_err) => panic!("Didn't parse complex number correctly"),
    };

    assert_eq!(
        z,
        complex!(1.0_f64, 2.0_f64, 3.0_f64, 4.0_f64, 5.0_f64, 6.0_f64, 7.0_f64, 8.0_f64)
    );
}
