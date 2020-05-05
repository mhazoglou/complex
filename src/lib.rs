use std::any::type_name;
use std::num::ParseFloatError;
use std::str::FromStr;
use std::{fmt, ops};

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
    ( $( $x:expr, $y:expr),* ) => {
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
        + Copy
        + ops::Add<Output = T>
        + ops::Sub<Output = T>
        + ops::Mul<Output = T>
        + ops::Neg<Output = T>,
{
    pub fn new(re: T, im: T) -> Self {
        Self { re, im }
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
    T: Conjugate + Copy + ops::Neg<Output = T>,
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
    T: AbsSq + Conjugate + Copy + ops::Add<Output = T>,
{
    fn abs_sq(&self) -> f64 {
        self.re.abs_sq() + self.im.abs_sq()
    }
}

impl<T> ops::Neg for Complex<T>
where
    T: ops::Neg<Output = T>,
{
    type Output = Complex<T>;
    fn neg(self) -> Self {
        Self {
            re: -self.re,
            im: -self.im,
        }
    }
}

impl<T> ops::Add for Complex<T>
where
    T: ops::Add<Output = T>,
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}

impl<T> ops::Add<f64> for Complex<T>
where
    T: ops::Add<f64, Output = T>,
{
    type Output = Self;
    fn add(self, other: f64) -> Self {
        Self {
            re: self.re + other,
            im: self.im,
        }
    }
}

impl<T> ops::Add<Complex<T>> for f64
where
    T: ops::Add<f64, Output = T>,
{
    type Output = Complex<T>;
    fn add(self, other: Complex<T>) -> Complex<T> {
        Complex::<T> {
            re: other.re + self,
            im: other.im,
        }
    }
}

impl<T> ops::Add<Complex<Complex<T>>> for Complex<T>
where
    T: ops::Add<Output = T>,
{
    type Output = Complex<Complex<T>>;
    fn add(self, other: Complex<Complex<T>>) -> Complex<Complex<T>> {
        Complex::<Complex<T>> {
            re: self + other.re,
            im: other.im,
        }
    }
}

impl<T> ops::Add<Complex<T>> for Complex<Complex<T>>
where
    T: ops::Add<Output = T>,
{
    type Output = Complex<Complex<T>>;
    fn add(self, other: Complex<T>) -> Complex<Complex<T>> {
        Complex::<Complex<T>> {
            re: self.re + other,
            im: self.im,
        }
    }
}

impl<T> ops::Sub for Complex<T>
where
    T: ops::Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            re: self.re - other.re,
            im: self.im - other.im,
        }
    }
}

impl<T> ops::Sub<f64> for Complex<T>
where
    T: ops::Sub<f64, Output = T>,
{
    type Output = Self;
    fn sub(self, other: f64) -> Self {
        Self {
            re: self.re - other,
            im: self.im,
        }
    }
}

impl<T> ops::Sub<Complex<T>> for f64
where
    T: ops::Neg<Output = T> + ops::Add<f64, Output = T>,
{
    type Output = Complex<T>;
    fn sub(self, other: Complex<T>) -> Complex<T> {
        Complex::<T> {
            re: -other.re + self,
            im: -other.im,
        }
    }
}

impl<T> ops::Sub<Complex<Complex<T>>> for Complex<T>
where
    T: ops::Sub<Output = T>,
{
    type Output = Complex<Complex<T>>;
    fn sub(self, other: Complex<Complex<T>>) -> Complex<Complex<T>> {
        Complex::<Complex<T>> {
            re: self - other.re,
            im: other.im,
        }
    }
}

impl<T> ops::Sub<Complex<T>> for Complex<Complex<T>>
where
    T: ops::Sub<Output = T>,
{
    type Output = Complex<Complex<T>>;
    fn sub(self, other: Complex<T>) -> Complex<Complex<T>> {
        Complex::<Complex<T>> {
            re: self.re - other,
            im: self.im,
        }
    }
}

impl<T> ops::Mul for Complex<T>
where
    T: Conjugate + Copy + ops::Add<Output = T> + ops::Sub<Output = T> + ops::Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            re: self.re * other.re - other.im.conj() * self.im,
            im: other.im * self.re + self.im * other.re.conj(),
        }
    }
}

impl<T> ops::Mul<f64> for Complex<T>
where
    T: ops::Mul<f64, Output = T>,
{
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        Self {
            re: self.re * other,
            im: self.im * other,
        }
    }
}

impl<T> ops::Mul<Complex<T>> for f64
where
    T: ops::Mul<f64, Output = T>,
{
    type Output = Complex<T>;
    fn mul(self, other: Complex<T>) -> Complex<T> {
        Complex::<T> {
            re: other.re * self,
            im: other.im * self,
        }
    }
}

impl<T> ops::Div for Complex<T>
where
    Complex<T>:
        Conjugate + AbsSq + ops::Mul<Output = Complex<T>> + ops::Div<f64, Output = Complex<T>>,
{
    type Output = Self;
    fn div(self, other: Self) -> Self {
        self * other.conj() / other.abs_sq()
    }
}

impl<T> ops::Div<f64> for Complex<T>
where
    Complex<T>: ops::Mul<f64, Output = Complex<T>>,
{
    type Output = Self;
    fn div(self, other: f64) -> Self {
        self * (1. / other)
    }
}

impl<T> ops::Div<Complex<T>> for f64
where
    Complex<T>:
        Conjugate + AbsSq + ops::Mul<f64, Output = Complex<T>> + ops::Div<f64, Output = Complex<T>>,
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
impl FromStr for complex<T> {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split("+i").collect();

        //if coords.len() == 2 {
        let x = coords[0].parse::<f64>()?;
        let y = coords[1].parse::<f64>()?;

        Ok(c128{ re: x, im: y})
        //}
    }
}
*/
