//! # complex
//! 
//! `complex` is a crate implementing Cayley-Dickson construction and algebra
//! for hypercomplex numbers through a recursive construction. This crate 
//! allows any hypercomplex numbers to be manipulated with standard operators
//! in a convenient manner.
use std::any::type_name;
use std::iter::{Product, Sum};
use std::ops::{Add, AddAssign, 
               Div, DivAssign, 
               Mul, MulAssign, 
               Sub, SubAssign,
               Neg, 
               Rem, RemAssign};

pub mod fmt;
pub mod ops;

/// Generates a corresponding `Complex<T>` from floating point numbers either
/// `f32` or `f64` in groupings of powers of the number two.
///
/// # Example
/// 
/// ```
/// use complex::*;
///
/// let quaternion = complex![1., 2., 3., 4.];
/// let z1 = Complex::<f64>::new(1., 2.);
/// let z2 = Complex::<f64>::new(3., 4.);
///
/// assert_eq!(quaternion, Complex::<Complex<f64>>::new(z1, z2));
/// ```
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
                ),+
            ]
        }
    };
}

/// An alias for `Complex<f64>`, implements complex numbers with `f64`.
pub type Complexf64 = Complex<f64>;
/// An alias for `Complex<Complex<f64>>`, implements quaternions with `f64`.
pub type Quaternionf64 = Complex<Complex<f64>>;
/// An alias for `Complex<Complex<Complex<f64>>>`, implements octonions with `f64`.
pub type Octonionf64 = Complex<Complex<Complex<f64>>>;
/// An alias for `Complex<Complex<Complex<Complex<f64>>>>` implements sedenions with 'f64'.
pub type Sedenionf64 = Complex<Complex<Complex<Complex<f64>>>>;
/// An alias for `Complex<Complex<Complex<Complex<Complex<f64>>>>>` implements trigintaduonion with `f64`.
pub type Trigintaduonionf64 = Complex<Complex<Complex<Complex<Complex<f64>>>>>;

/// An alias for `Complex<f32>`, implements complex numbers with `f32`.
pub type Complexf32 = Complex<f32>;
/// An alias for `Complex<Complex<f32>>`, implements quaternions with `f32`.
pub type Quaternionf32 = Complex<Complex<f32>>;
/// An alias for `Complex<Complex<Complex<f32>>>`, implements octonions with `f32`.
pub type Octonionf32 = Complex<Complex<Complex<f32>>>;
/// An alias for `Complex<Complex<Complex<Complex<f32>>>>` implements sedenions with 'f32'.
pub type Sedenionf32 = Complex<Complex<Complex<Complex<f32>>>>;
/// An alias for `Complex<Complex<Complex<Complex<Complex<f32>>>>>` implements trigintaduonion with `f32`.
pub type Trigintaduonionf32 = Complex<Complex<Complex<Complex<Complex<f32>>>>>;

/// Base struct that all complex and hypercomplex types are based off of
/// recursively putting `Complex<T>` within itself for other hypercomplex types
/// like `Complex<Complex<...>>`. `Complex<T>` can only be built out from f32 
/// and f64 at the very root of the structure.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Complex<T> {
    pub re: T,
    pub im: T,
}

impl<T> Complex<T>
where
    T: Conjugate + Copy,
{
    /// Method for creating a new `Complex<T>` instance.
    ///
    /// # Example
    /// 
    /// ```
    /// use complex::*;
    ///
    /// let z1 = Complex::<f64>::new(1., 2.);
    /// let z2 = Complex::<f64> {
    ///     re: 1.,
    ///     im: 2.
    /// };
    ///
    /// assert_eq!(z1, z2);
    /// ```
    pub fn new(re: T, im: T) -> Self {
        Self { re, im }
    }
}

/// Implements several common functions for complex and hypercomplex types.
pub trait Functions<U, V> {
    /// Returns the exponent of a hypercomplex number.
    /// 
    /// # Example
    ///
    /// ```
    /// use complex::*;
    /// use std::f64::consts::PI;
    /// 
    /// let z = complex![0.0, PI];
    /// let expz = z.exp();
    ///
    /// assert_eq!(expz, complex![PI.cos(), PI.sin()]) 
    /// ```
    fn exp(&self) -> Self;
    /// Returns the natural logarithm of a hypercomplex number. It's unique
    /// up to integer multiples of 2π times some imaginary.
    /// 
    /// # Example
    ///
    /// ```
    /// use complex::*;
    /// use std::f64::consts::PI;
    /// 
    /// let z = complex![-1.0, 0.0];
    /// let lnz = z.ln();
    ///
    /// assert!((lnz - complex![0.0, 0.0]).abs_sq() < 1e-10) 
    /// ```
    fn ln(&self) -> Self;
    /// Calculate a hypercomplex number to the power of a floating point.
    /// 
    /// # Example
    ///
    /// ```
    /// use complex::*;
    /// 
    /// let z = complex![0.0, 1.0];
    /// let w = z.powf(3.0);
    ///
    /// assert!((w - complex![0.0, -1.0]).abs_sq() < 1e-10) 
    /// ```
    fn powf(&self, num: U) -> Self;
    /// Calculate a hypercomplex number or float to power of a hypercomplex number.
    /// 
    /// # Example
    /// 
    /// ```
    /// use complex::*;
    /// use std::f64::consts::PI;
    /// 
    /// let z = complex![0.0, 1.0];
    /// let w = z.powz(z);
    /// 
    /// assert_eq!(w, (-PI / 2.).exp() * Complex::<f64>::one())
    /// ```
    fn powz(&self, num: V) -> V;
    /// Tail recursive function for calculating repeated products of hypercomplex numbers.
    /// 
    /// # Example
    ///
    /// ```
    /// use complex::*;
    /// 
    /// let z = complex![0.0, 1.0];
    /// let w = z.powu_tail(3, Complex::<f64>::one());
    ///
    /// assert_eq!(w, complex![0.0, -1.0]) 
    /// ```
    fn powu_tail(&self, num: u32, acc: Self) -> Self;
    /// Calculates the power of a hypercomplex number to a power of an unsigned integer.
    /// 
    /// # Example
    ///
    /// ```
    /// use complex::*;
    /// 
    /// let z = complex![0.0, 1.0];
    /// let w = z.powu(3);
    ///
    /// assert_eq!(w, complex![0.0, -1.0]) 
    /// ```
    fn powu(&self, num: u32) -> Self;
    /// Calculates the power of a hypercomplex number to a power of an signed integer.
    /// 
    /// # Example
    ///
    /// ```
    /// use complex::*;
    /// 
    /// let z = complex![0.0, 1.0];
    /// let w = z.powi(-3);
    ///
    /// assert_eq!(w, complex![0.0, 1.0]) 
    /// ```
    fn powi(&self, num: i32) -> Self;
    /// Returns the hyperbolic sine of a hypercomplex number.
    ///
    /// # Example
    /// 
    /// ```
    /// use complex::*;
    /// use std::f64::consts::PI;
    /// 
    /// let z = complex![0., PI];
    /// let w = z.sinh();
    /// 
    /// assert_eq!(w, Complex::<f64>::i() * PI.sin())
    /// ```
    fn sinh(&self) -> Self;
    /// Returns the hyperbolic cosine of a hypercomplex number.
    ///
    /// # Example
    /// 
    /// ```
    /// use complex::*;
    /// use std::f64::consts::PI;
    /// 
    /// let z = complex![0., PI];
    /// let w = z.cosh();
    /// 
    /// assert_eq!(w, Complex::<f64>::one() * PI.cos())
    /// ```
    fn cosh(&self) -> Self;
    /// Returns the hyperbolic tangent of a hypercomplex number.
    ///
    /// # Example
    /// 
    /// ```
    /// use complex::*;
    /// use std::f64::consts::PI;
    /// 
    /// let z = complex![0., PI];
    /// let w = z.tanh();
    /// 
    /// assert!((w - Complex::<f64>::i() * PI.tan()).abs_sq() < 1e-10)
    /// ```
    fn tanh(&self) -> Self;
    /// Returns the sine of a hypercomplex number.
    ///
    /// # Example
    /// 
    /// ```
    /// use complex::*;
    /// use std::f64::consts::PI;
    /// 
    /// let z = complex![0., PI];
    /// let w = z.sin();
    /// 
    /// assert!((w - Complex::<f64>::i() * PI.sinh()).abs_sq() < 1e-10)
    /// ```
    fn sin(&self) -> Self;
    /// Returns the cosine of a hypercomplex number.
    ///
    /// # Example
    /// 
    /// ```
    /// use complex::*;
    /// use std::f64::consts::PI;
    /// 
    /// let z = complex![0., PI];
    /// let w = z.cos();
    /// 
    /// assert!((w - Complex::<f64>::one() * PI.cosh()).abs_sq() < 1e-10)
    /// ```
    fn cos(&self) -> Self;
    /// Returns the tangent of a hypercomplex number.
    ///
    /// # Example
    /// 
    /// ```
    /// use complex::*;
    /// use std::f64::consts::PI;
    /// 
    /// let z = complex![0., PI];
    /// let w = z.tan();
    /// 
    /// assert!((w - Complex::<f64>::i() * PI.tanh()).abs_sq() < 1e-10)
    /// ```
    fn tan(&self) -> Self;
}

macro_rules! impl_functions_for_float {
    ($($u:ty),* ) => {
        $(
            impl<T> Functions<$u, Complex<T>> for $u
            where Complex<T>: Mul<$u, Output=Complex<T>> + Functions<$u, Complex<T>>,
            {
                fn exp(&self) -> Self {
                    Self::exp(*self)
                }

                fn ln(&self) -> Self {
                    Self::ln(*self)
                }

                fn powf(&self, num: Self) -> Self {
                    Self::powf(*self, num)
                }

                fn powz(&self, num: Complex<T>) -> Complex<T> {
                    (num * Self::ln(*self)).exp()
                }

                fn powu_tail(&self, num: u32, acc: Self) -> Self {
                    if num == 0 {
                        return acc;
                    } else if (num % 2) == 0 {
                        let sq = self * self;
                        return <Self as Functions<$u, Complex<T>>>::powu_tail(&sq, num / 2, acc);
                    } else {
                        let sq = self * self;
                        return <Self as Functions<$u, Complex<T>>>::powu_tail(&sq, (num - 1) / 2, self * acc);
                    }
                }

                fn powu(&self, num: u32) -> Self {
                    <Self as Functions<$u, Complex<T>>>::powu_tail(self, num, Self::one())
                }

                fn powi(&self, num: i32) -> Self {
                    Self::powi(*self, num)
                }
                
                fn sinh(&self) -> Self {
                    Self::sinh(*self)
                }
                
                fn cosh(&self) -> Self {
                    Self::cosh(*self)
                }
                
                fn tanh(&self) -> Self {
                    Self::tanh(*self)
                }
                
                fn sin(&self) -> Self {
                    Self::sin(*self)
                }
                
                fn cos(&self) -> Self {
                    Self::cos(*self)
                }
                
                fn tan(&self) -> Self {
                    Self::tan(*self)
                }
            }

            impl<T> Functions<$u, Complex<T>> for Complex<T>
            where
                T: Conjugate
                    + AbsSq<$u>
                    + Fill<$u>
                    + Real<$u>
                    + Copy
                    + Add<Output = T>
                    + Add<$u, Output = T>
                    + Sub<Output = T>
                    + Sub<$u, Output = T>
                    + Mul<Output = T>
                    + Mul<$u, Output = T>
                    + Div<Output = T>
                    + Div<$u, Output = T>
                    + Neg<Output = T>,
                Complex<T>: 
                    Div<Output = Complex<T>>
                    + Div<$u, Output = Complex<T>>
                    + ImaginaryConstants,
            {
                fn exp(&self) -> Self {
                    let real = self.real();
                    let imag = *self - real;
                    let theta = imag.abs_sq().sqrt();
                    let unit_imag: Self;

                    if theta == 0. {
                        unit_imag = Self::zero();
                    } else {
                        unit_imag = imag / theta;
                    }

                    real.exp() * (theta.cos() + unit_imag * theta.sin())
                }

                fn ln(&self) -> Self {
                    let r = self.abs_sq().sqrt();
                    let normed = *self / r;
                    let real = normed.real();
                    let imag = normed - real;
                    let imag_mag = imag.abs_sq().sqrt();
                    let theta = real.acos();
                    // let sin_theta = theta.sin();
                    let imag_exp: Self;

                    if imag_mag == 0. {
                        imag_exp = Self::zero();
                    } else {
                        imag_exp = theta * imag / imag_mag;
                    }

                    r.ln() + imag_exp
                }

                fn powf(&self, num: $u) -> Self {
                    let ln_z = self.ln();

                    (num * ln_z).exp()
                }

                fn powz(&self, num: Self) -> Self {
                    let ln_z = self.ln();

                    (num * ln_z).exp()
                }

                fn powu_tail(&self, num: u32, acc: Self) -> Self {
                    if num == 0 {
                        return acc;
                    } else if (num % 2) == 0 {
                        let sq = self * self;
                        return <Self as Functions<$u, Complex<T>>>::powu_tail(
                            &sq, num / 2, acc
                        );
                    } else {
                        let sq = self * self;
                        return <Self as Functions<$u, Complex<T>>>::powu_tail(
                            &sq, (num - 1) / 2, self * acc
                        );
                    }
                }

                fn powu(&self, num: u32) -> Self {
                    <Self as Functions<$u, Complex<T>>>::powu_tail(self, num, Self::one())
                }

                fn powi(&self, num: i32) -> Self {
                    if num == 0 {
                        Self::zero()
                    } else if num < 0 {
                        let z = self.powu(-num as u32);
                        1. / z
                    } else {
                        self.powu(num as u32)
                    }
                }
                
                fn sinh(&self) -> Self {
                    let exp = self.exp();
                    
                    (exp - 1. / exp) * 0.5 
                }
                
                fn cosh(&self) -> Self {
                    let exp = self.exp();
                    
                    (exp + 1. / exp) * 0.5 
                }
                
                fn tanh(&self) -> Self {
                    let exp = self.exp();
                    
                    (exp * exp - 1.) / (exp * exp + 1.)
                }
                
                fn sin(&self) -> Self {
                    let i = <Self as ImaginaryConstants>::i();
                    let expiz = (i * *self).exp();
                    
                    (expiz - 1. / expiz) *  0.5 * -i
                }
                
                fn cos(&self) -> Self {
                    let i = <Self as ImaginaryConstants>::i();
                    let expiz = (i * *self).exp();
                    
                    (expiz + 1. / expiz) * 0.5
                }
                
                fn tan(&self) -> Self {
                    let i = <Self as ImaginaryConstants>::i();
                    let expiz = (i * *self).exp();
                    
                    (expiz * expiz - 1.) / (i * expiz * expiz + i)
                }
            }
        )*
    }
}

impl_functions_for_float!(f32, f64);

/// Elementwise rounding and truncation functions
pub trait Rounding {
    fn floor(&self) -> Self;
    fn ceil(&self) -> Self;
    fn round(&self) -> Self;
    fn trunc(&self) -> Self;
    fn fract(&self) -> Self;
}

macro_rules! impl_rounding_for_float {
    ($($u:ty),* ) => {
        $(
            impl Rounding for $u
            {
                fn floor(&self) -> Self {
                    Self::floor(*self)
                }

                fn ceil(&self) -> Self {
                    Self::ceil(*self)
                }

                fn round(&self) -> Self {
                    Self::round(*self)
                }

                fn trunc(&self) -> Self {
                    Self::trunc(*self)
                }

                fn fract(&self) -> Self {
                    Self::fract(*self)
                }
            }
        )*
    }
}

impl_rounding_for_float!(f32, f64);

impl<T> Rounding for Complex<T>
where
    T: Rounding + Copy,
{
    fn floor(&self) -> Self {
        Self {
            re: <T as Rounding>::floor(&self.re),
            im: <T as Rounding>::floor(&self.im)
        }
    }

    fn ceil(&self) -> Self {
        Self {
            re: <T as Rounding>::ceil(&self.re),
            im: <T as Rounding>::ceil(&self.im)
        }
    }

    fn round(&self) -> Self {
        Self {
            re: <T as Rounding>::round(&self.re),
            im: <T as Rounding>::round(&self.im)
        }
    }

    fn trunc(&self) -> Self {
        Self {
            re: <T as Rounding>::trunc(&self.re),
            im: <T as Rounding>::trunc(&self.im)
        }
    }

    fn fract(&self) -> Self {
        Self {
            re: <T as Rounding>::fract(&self.re),
            im: <T as Rounding>::fract(&self.im)
        }
    }
}

/// Gives the additive (zero) and multiplicative (one) identity of the respective 
/// complex and hypercomplex types.
pub trait Identity {
    fn zero() -> Self;
    fn one() -> Self;
}

macro_rules! impl_identity_for_float {
    ( $($u:ty),* ) => {
        $(
            impl Identity for $u {
                fn zero() -> Self {
                    0.0
                }

                fn one() -> Self {
                    1.0
                }
            }
        )*
    };
}

impl_identity_for_float!(f32, f64);

impl<T> Identity for Complex<T>
where
    T: Identity,
{
    fn zero() -> Self {
        Self {
            re: <T as Identity>::zero(),
            im: <T as Identity>::zero(),
        }
    }

    fn one() -> Self {
        Self {
            re: <T as Identity>::one(),
            im: <T as Identity>::zero(),
        }
    }
}

/// Generates imaginaries i, j, k for respective hypercomplex type
pub trait ImaginaryConstants {
    fn i() -> Self;
    fn j() -> Self;
    fn k() -> Self;
}

macro_rules! impl_img_const_for_float {
    ( $($u:ty),* ) => {
        $(
            impl ImaginaryConstants for $u {
                fn i() -> Self {
                    0.
                }
                
                fn j() -> Self {
                    0.
                }
                
                fn k() -> Self {
                    0.
                }
            }
        )*
    };
}

impl_img_const_for_float!(f32, f64);
impl<T> ImaginaryConstants for Complex<T> 
where
    T: Identity + ImaginaryConstants
{
    fn i() -> Self {
        if (type_name::<T>() == "f64") || (type_name::<T>() == "f32") {
            return Self {
                re: <T as Identity>::zero(),
                im: <T as Identity>::one()
            };
        } else {
            return Self {
                re: <T as ImaginaryConstants>::i(),
                im: <T as Identity>::zero()
            };
        }
    }
    
    fn j() -> Self {
        if (type_name::<T>() == "f64") || (type_name::<T>() == "f32") {
            return Self {
                re: <T as Identity>::zero(),
                im: <T as Identity>::one()
            };
        } else if (type_name::<T>() == type_name::<Complexf64>()) || 
            (type_name::<T>() == type_name::<Complexf32>()) {
            return Self {
                re: <T as Identity>::zero(),
                im: <T as Identity>::one()
            };
        } else {
            return Self {
                re: <T as ImaginaryConstants>::j(),
                im: <T as Identity>::zero()
            }
        }
    }
    
    fn k() -> Self {
        if (type_name::<T>() == "f64") || (type_name::<T>() == "f32") {
            return Self {
                re: <T as Identity>::zero(),
                im: <T as Identity>::one()
            };
        } else if (type_name::<T>() == type_name::<Complexf64>()) || 
            (type_name::<T>() == type_name::<Complexf32>()) {
            return Self {
                re: <T as Identity>::zero(),
                im: <T as ImaginaryConstants>::k()
            };
        } else {
            return Self {
                re: <T as ImaginaryConstants>::k(),
                im: <T as Identity>::zero()
            }
        }
    }
}

/// Gives static methods for creating complex and hypercomplex types from arrays
/// and vectors as well as simply filling all values with a single number.
pub trait Fill<U>: Identity {
    fn fill(num: U) -> Self;
    fn from_slice(v: &[U]) -> Self;
    fn from_vec(v: Vec<U>) -> Self;
}

macro_rules! impl_fill_for_float {
    ( $($u:ty),* ) => {
        $(
            impl Fill<$u> for $u {
                fn fill(num: $u) -> Self {
                    num
                }

                fn from_slice(v: &[$u]) -> Self {
                    v[0]
                }

                fn from_vec(v: Vec<$u>) -> Self {
                    v[0]
                }
            }
        )*
    };
}

impl_fill_for_float!(f32, f64);

impl<T, U> Fill<U> for Complex<T>
where
    T: Fill<U>,
    U: Copy,
{
    fn fill(num: U) -> Self {
        Self {
            re: <T as Fill<U>>::fill(num),
            im: <T as Fill<U>>::fill(num),
        }
    }

    fn from_slice(v: &[U]) -> Self {
        let len = v.len();

        let half = len / 2;
        Self {
            re: <T as Fill<U>>::from_slice(&v[..half]),
            im: <T as Fill<U>>::from_slice(&v[half..len]),
        }
    }

    fn from_vec(v: Vec<U>) -> Self {
        let len = v.len();

        let half = len / 2;
        Self {
            re: <T as Fill<U>>::from_vec(v[..half].to_vec()),
            im: <T as Fill<U>>::from_vec(v[half..len].to_vec()),
        }
    }
}

/// Conjugates any complex or hypercomplex number.
pub trait Conjugate {
    fn conj(&self) -> Self;
}

macro_rules! impl_conj_for {
    ( $($u:ty),* ) => {
        $(
            impl Conjugate for $u {
                fn conj(&self) -> Self {
                    *self
                }
            }
        )*
    };
}

impl_conj_for!(f32, f64);

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

/// Returns the modulus square of an instance of a complex or hypercomplex type.
pub trait AbsSq<U> {
    fn abs_sq(&self) -> U;
}

macro_rules! impl_abs_sq_for {
    ( $($u:ty),* ) => {
        $(
            impl AbsSq<$u> for $u {
                fn abs_sq(&self) -> $u {
                    self * self
                }
            }

            impl<T> AbsSq<$u> for Complex<T>
            where
                T: AbsSq<$u> + Copy + Add<Output = T>,
            {
                fn abs_sq(&self) -> $u {
                    self.re.abs_sq() + self.im.abs_sq()
                }
            }
        )*
    };
}

impl_abs_sq_for!(f32, f64);

/// Returns the real part of any complex and hypercomplex type.
pub trait Real<U> {
    fn real(&self) -> U;
}

macro_rules! impl_real_for {
    ( $($u:ty),* ) => {
        $(
            impl Real<$u> for $u {
                fn real(&self) -> $u {
                    *self
                }
            }

            impl<T> Real<$u> for Complex<T>
            where
                T: Real<$u> + Copy,
            {
                fn real(&self) -> $u {
                    self.re.real()
                }
            }
        )*
    };
}

impl_real_for!(f32, f64);
