use std::any::type_name;
use std::num::ParseFloatError;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::{fmt, str::FromStr};

fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

#[macro_export]
macro_rules! complex{
    ( $x:expr, $y:expr ) => {
        {
            Complex::new($x, $y)
        }
    };
    ( $( $x:expr, $y:expr),+ ) => {
        {
            complex![
                $(
                    Complex::new($x, $y)
                ),*
            ]
        }
    };
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Complex<T> {
    pub re: T,
    pub im: T,
}

impl<T> Complex<T>
where
    T: Conjugate
        + AbsSq
        + Real
        + Copy
        + Add<Output = T>
        + Add<f64, Output = T>
        + Sub<Output = T>
        + Sub<f64, Output = T>
        + Mul<Output = T>
        + Mul<f64, Output = T>
        + Div<Output = T>
        + Div<f64, Output = T>
        + Neg<Output = T>,
{
    pub fn new(re: T, im: T) -> Self {
        Self { re, im }
    }

    pub fn exp(&self) -> Self {
        let real = self.real();
        let imag = *self - real;
        let theta = imag.abs_sq().sqrt();
        let unit_imag: Self;

        if theta == 0. {
            unit_imag = imag * 0.;
        } else {
            unit_imag = imag / theta;
        }

        real.exp() * (theta.cos() + unit_imag * theta.sin())
    }

    pub fn polar_rep(&self) -> (f64, Self) {
        let r = self.abs_sq().sqrt();
        let normed = *self / r;
        let real = normed.real();
        let imag = normed - real;
        let theta = real.acos();
        let sin_theta = theta.sin();
        let imag_exp: Self;

        if sin_theta == 0. {
            imag_exp = 0. * imag;
        } else {
            imag_exp = theta * imag / sin_theta;
        }

        (r, imag_exp)
    }
}

pub trait Conjugate {
    fn conj(&self) -> Self;
}

impl Conjugate for f64 {
    fn conj(&self) -> f64 {
        *self
    }
}

impl<T> Conjugate for Complex<T>
where
    T: Conjugate + Copy + Neg<Output = T>,
{
    fn conj(&self) -> Self {
        Self {
            re: self.re.conj(),
            im: -self.im,
        }
    }
}

pub trait AbsSq {
    fn abs_sq(&self) -> f64;
}

impl AbsSq for f64 {
    fn abs_sq(&self) -> f64 {
        self * self
    }
}

impl<T> AbsSq for Complex<T>
where
    T: AbsSq + Conjugate + Copy + Add<Output = T>,
{
    fn abs_sq(&self) -> f64 {
        self.re.abs_sq() + self.im.abs_sq()
    }
}

pub trait Real {
    fn real(&self) -> f64;
}

impl Real for f64 {
    fn real(&self) -> f64 {
        *self
    }
}

impl<T> Real for Complex<T>
where
    T: Real + Copy,
{
    fn real(&self) -> f64 {
        self.re.real()
    }
}

macro_rules! forward_ref_un_op {
    ($imp:ident, $method:ident, $t:ty, $T:tt) => {
        impl<$T> $imp for &$t
        where
            $T: $imp<Output = $T> + Copy,
        {
            type Output = $t;
            fn $method(self) -> Self::Output {
                $imp::$method(*self)
            }
        }
    };
}

forward_ref_un_op!(Neg, neg, Complex<T>, T);
impl<T> Neg for Complex<T>
where
    T: Neg<Output = T>,
{
    type Output = Complex<T>;
    fn neg(self) -> Self {
        Self {
            re: -self.re,
            im: -self.im,
        }
    }
}

macro_rules! forward_ref_bin_op {
    ($imp:ident, $method:ident, $t:ty, $u:ty, $T:tt) => {
        impl<$T> $imp<&$u> for $t
        where
            $T: Conjugate
                + AbsSq
                + Real
                + Copy
                + Add<Output = $T>
                + Add<f64, Output = $T>
                + Sub<Output = $T>
                + Sub<f64, Output = $T>
                + Mul<Output = $T>
                + Mul<f64, Output = $T>
                + Div<Output = $T>
                + Div<f64, Output = $T>
                + Neg<Output = $T>
                + Copy,
        {
            type Output = <$t as $imp<$u>>::Output;
            fn $method(self, other: &$u) -> Self::Output {
                $imp::$method(self, *other)
            }
        }

        impl<$T> $imp<$u> for &$t
        where
            $T: Conjugate
                + AbsSq
                + Real
                + Copy
                + Add<Output = $T>
                + Add<f64, Output = $T>
                + Sub<Output = $T>
                + Sub<f64, Output = $T>
                + Mul<Output = $T>
                + Mul<f64, Output = $T>
                + Div<Output = $T>
                + Div<f64, Output = $T>
                + Neg<Output = $T>
                + Copy,
        {
            type Output = <$t as $imp<$u>>::Output;
            fn $method(self, other: $u) -> Self::Output {
                $imp::$method(*self, other)
            }
        }

        impl<'a, 'b, $T> $imp<&'b $u> for &'a $t
        where
            $T: Conjugate
                + AbsSq
                + Real
                + Copy
                + Add<Output = $T>
                + Add<f64, Output = $T>
                + Sub<Output = $T>
                + Sub<f64, Output = $T>
                + Mul<Output = $T>
                + Mul<f64, Output = $T>
                + Div<Output = $T>
                + Div<f64, Output = $T>
                + Neg<Output = $T>
                + Copy,
        {
            type Output = <$t as $imp<$u>>::Output;
            fn $method(self, other: &'b $u) -> Self::Output {
                $imp::$method(*self, *other)
            }
        }
    };
}

forward_ref_bin_op!(Add, add, Complex<T>, Complex<T>, T);
impl<T> Add for Complex<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}

forward_ref_bin_op!(Add, add, Complex<T>, f64, T);
impl<T> Add<f64> for Complex<T>
where
    T: Add<f64, Output = T>,
{
    type Output = Self;
    fn add(self, other: f64) -> Self {
        Self {
            re: self.re + other,
            im: self.im,
        }
    }
}

forward_ref_bin_op!(Add, add, f64, Complex<T>, T);
impl<T> Add<Complex<T>> for f64
where
    T: Add<f64, Output = T>,
{
    type Output = Complex<T>;
    fn add(self, other: Complex<T>) -> Complex<T> {
        Complex::<T> {
            re: other.re + self,
            im: other.im,
        }
    }
}

forward_ref_bin_op!(Add, add, Complex<T>, Complex<Complex<T>>, T);
impl<T> Add<Complex<Complex<T>>> for Complex<T>
where
    T: Add<Output = T>,
{
    type Output = Complex<Complex<T>>;
    fn add(self, other: Complex<Complex<T>>) -> Complex<Complex<T>> {
        Complex::<Complex<T>> {
            re: self + other.re,
            im: other.im,
        }
    }
}

forward_ref_bin_op!(Add, add, Complex<Complex<T>>, Complex<T>, T);
impl<T> Add<Complex<T>> for Complex<Complex<T>>
where
    T: Add<Output = T>,
{
    type Output = Complex<Complex<T>>;
    fn add(self, other: Complex<T>) -> Complex<Complex<T>> {
        Complex::<Complex<T>> {
            re: self.re + other,
            im: self.im,
        }
    }
}

forward_ref_bin_op!(Sub, sub, Complex<T>, Complex<T>, T);
impl<T> Sub for Complex<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            re: self.re - other.re,
            im: self.im - other.im,
        }
    }
}

forward_ref_bin_op!(Sub, sub, Complex<T>, f64, T);
impl<T> Sub<f64> for Complex<T>
where
    T: Sub<f64, Output = T>,
{
    type Output = Self;
    fn sub(self, other: f64) -> Self {
        Self {
            re: self.re - other,
            im: self.im,
        }
    }
}

forward_ref_bin_op!(Sub, sub, f64, Complex<T>, T);
impl<T> Sub<Complex<T>> for f64
where
    T: Neg<Output = T> + Add<f64, Output = T>,
{
    type Output = Complex<T>;
    fn sub(self, other: Complex<T>) -> Complex<T> {
        Complex::<T> {
            re: -other.re + self,
            im: -other.im,
        }
    }
}

forward_ref_bin_op!(Sub, sub, Complex<T>, Complex<Complex<T>>, T);
impl<T> Sub<Complex<Complex<T>>> for Complex<T>
where
    T: Sub<Output = T>,
{
    type Output = Complex<Complex<T>>;
    fn sub(self, other: Complex<Complex<T>>) -> Complex<Complex<T>> {
        Complex::<Complex<T>> {
            re: self - other.re,
            im: other.im,
        }
    }
}

forward_ref_bin_op!(Sub, sub, Complex<Complex<T>>, Complex<T>, T);
impl<T> Sub<Complex<T>> for Complex<Complex<T>>
where
    T: Sub<Output = T>,
{
    type Output = Complex<Complex<T>>;
    fn sub(self, other: Complex<T>) -> Complex<Complex<T>> {
        Complex::<Complex<T>> {
            re: self.re - other,
            im: self.im,
        }
    }
}

forward_ref_bin_op!(Mul, mul, Complex<T>, Complex<T>, T);
impl<T> Mul for Complex<T>
where
    T: Conjugate 
        + Copy 
        + Add<Output = T> 
        + Sub<Output = T> 
        + Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self {
            re: self.re * other.re - other.im.conj() * self.im,
            im: other.im * self.re + self.im * other.re.conj(),
        }
    }
}

forward_ref_bin_op!(Mul, mul, Complex<T>, f64, T);
impl<T> Mul<f64> for Complex<T>
where
    T: Mul<f64, Output = T>,
{
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        Self {
            re: self.re * other,
            im: self.im * other,
        }
    }
}

forward_ref_bin_op!(Mul, mul, f64, Complex<T>, T);
impl<T> Mul<Complex<T>> for f64
where
    T: Mul<f64, Output = T>,
{
    type Output = Complex<T>;
    fn mul(self, other: Complex<T>) -> Complex<T> {
        Complex::<T> {
            re: other.re * self,
            im: other.im * self,
        }
    }
}

forward_ref_bin_op!(Div, div, Complex<T>, Complex<T>, T);
impl<T> Div for Complex<T>
where
    Complex<T>: Conjugate 
        + AbsSq 
        + Mul<Output = Complex<T>> 
        + Div<f64, Output = Complex<T>>,
{
    type Output = Self;
    fn div(self, other: Self) -> Self {
        self * other.conj() / other.abs_sq()
    }
}

forward_ref_bin_op!(Div, div, Complex<T>, f64, T);
impl<T> Div<f64> for Complex<T>
where
    Complex<T>: Mul<f64, Output = Complex<T>>,
{
    type Output = Self;
    fn div(self, other: f64) -> Self {
        self * (1. / other)
    }
}

forward_ref_bin_op!(Div, div, f64, Complex<T>, T);
impl<T> Div<Complex<T>> for f64
where
    Complex<T>: Conjugate 
        + AbsSq 
        + Mul<f64, Output = Complex<T>> 
        + Div<f64, Output = Complex<T>>,
{
    type Output = Complex<T>;
    fn div(self, other: Complex<T>) -> Complex<T> {
        other.conj() * (self / other.abs_sq())
    }
}

impl<T> fmt::Display for Complex<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if type_of(&self.re) == "f64" {
            let real = self.re.to_string();
            let imag = self.im.to_string();

            if &imag[0..1] != "-" {
                return write!(f, "{} + {}i", &real, &imag);
            } else {
                return write!(f, "{} - {}i", &real, &imag[1..]);
            }
        } else if type_of(&self.re) == "complex::Complex<f64>" {
            let real = self.re.to_string();
            let imag = self.im.to_string();

            let real_split: Vec<&str> = real.split_whitespace().collect();
            let imag_split: Vec<&str> = imag.split_whitespace().collect();

            let len = imag_split[2].len();

            if &imag_split[0][0..1] != "-" {
                return write!(
                    f,
                    "{} {} {} + {}j {} {}k",
                    &real_split[0],
                    &real_split[1],
                    &real_split[2],
                    &imag_split[0],
                    &imag_split[1],
                    &imag_split[2][..(len - 1)]
                );
            } else {
                return write!(
                    f,
                    "{} {} {} - {}j {} {}k",
                    &real_split[0],
                    &real_split[1],
                    &real_split[2],
                    &imag_split[0][1..],
                    &imag_split[1],
                    &imag_split[2][..(len - 1)]
                );
            }
        } else {
            return write!(f, "({}, {})", &self.re, &self.im);
        }
    }
}

/*
impl FromStr for Complex<T> {
    where T: FromStr
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // suppose "4+2i" or "4+i2" or "2i+4" or "i2+4"
        let coords: Vec<&str> = s.split(|c| {c == '+' || c == '-'}).collect();



        //if coords.len() == 2 {
        let x = (coords[0]).parse::<T>()?;
        let y = (coords[1] ).parse::<T>()?;

        Ok(Complex<T>{ re: x, im: y})
    }
}
*/
