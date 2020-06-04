use std::iter::{Product, Sum};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub mod fmt;
pub mod ops;


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

macro_rules! forward_ref_bin_op {
    ($imp:ident, $method:ident, $t:ty, $u:ty, $T:tt) => {
        impl<$T> $imp<&$u> for $t
        where
            $t: $imp<$u> + Copy,
            $u: Copy,
        {
            type Output = <$t as $imp<$u>>::Output;
            fn $method(self, other: &$u) -> Self::Output {
                $imp::$method(self, *other)
            }
        }

        impl<$T> $imp<$u> for &$t
        where
            $t: $imp<$u> + Copy,
            $u: Copy,
        {
            type Output = <$t as $imp<$u>>::Output;
            fn $method(self, other: $u) -> Self::Output {
                $imp::$method(*self, other)
            }
        }

        impl<'a, 'b, $T> $imp<&'b $u> for &'a $t
        where
            $t: $imp<$u> + Copy,
            $u: Copy,
        {
            type Output = <$t as $imp<$u>>::Output;
            fn $method(self, other: &'b $u) -> Self::Output {
                $imp::$method(*self, *other)
            }
        }
    };
}

macro_rules! bin_op_assign {
    ($assign:ident, $method:ident, $base_trait:ident,
     $base_method:ident, $t:ty, $u:ty, $T:tt) => {
        impl<$T> $assign<$u> for $t
        where
            $t: $base_trait<$u, Output = $t> + Copy,
        {
            fn $method(&mut self, other: $u) {
                let cp = *self;
                *self = $base_trait::$base_method(cp, other);
            }
        }
    };
}

pub type Complexf64 = Complex<f64>;
pub type Quaternionf64 = Complex<Complex<f64>>;
pub type Octonionf64 = Complex<Complex<Complex<f64>>>;
pub type Sedenionf64 = Complex<Complex<Complex<Complex<f64>>>>;
pub type Trigintaduonionf64 = Complex<Complex<Complex<Complex<Complex<f64>>>>>;

pub type Complexf32 = Complex<f32>;
pub type Quaternionf32 = Complex<Complex<f32>>;
pub type Octonionf32 = Complex<Complex<Complex<f32>>>;
pub type Sedenionf32 = Complex<Complex<Complex<Complex<f32>>>>;
pub type Trigintaduonionf32 = Complex<Complex<Complex<Complex<Complex<f32>>>>>;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Complex<T> {
    pub re: T,
    pub im: T,
}

impl<T> Complex<T> 
where T: Conjugate + Copy
{
    pub fn new(re: T, im: T) -> Self {
        Self { re, im }
    }
}

pub trait Functions<U, V> {
    fn exp(&self) -> Self;
    fn ln(&self) -> Self;
    fn powf(&self, num: U) -> Self;
    fn powz(&self, num: V) -> V;
    fn powu_tail(&self, num: u32, acc: Self) -> Self;
    fn powu(&self, num: u32) -> Self;
    fn powi(&self, num: i32) -> Self;
}

macro_rules! impl_functions_for_float {
    ($($ty:ty),* ) => {
        $(
            impl<T> Functions<$ty, Complex<T>> for $ty 
            where Complex<T>: Mul<$ty, Output=Complex<T>> + Functions<$ty, Complex<T>>,
            {
                fn exp(&self) -> Self {
                    self.exp()
                }
                
                fn ln(&self) -> Self {
                    self.ln()
                }
                
                fn powf(&self, num: Self) -> Self {
                    self.powf(num)
                }
                
                fn powz(&self, num: Complex<T>) -> Complex<T> {
                    (num * self.ln()).exp()
                }
                
                fn powu_tail(&self, num: u32, acc: Self) -> Self {
                    if num == 0 {
                        return acc;
                    }

                    <Self as Functions<$ty, Complex<T>>>::powu_tail(self, num - 1, self * acc)
                }
                
                fn powu(&self, num: u32) -> Self {
                    <Self as Functions<$ty, Complex<T>>>::powu_tail(self, num, Self::one())
                }
                
                fn powi(&self, num: i32) -> Self {
                    self.powi(num)
                }
            }
            
            impl<T> Functions<$ty, Complex<T>> for Complex<T>
            where
                T: Conjugate
                    + AbsSq<$ty>
                    + Fill<$ty>
                    + Real<$ty>
                    + Copy
                    + Add<Output = T>
                    + Add<$ty, Output = T>
                    + Sub<Output = T>
                    + Sub<$ty, Output = T>
                    + Mul<Output = T>
                    + Mul<$ty, Output = T>
                    + Div<Output = T>
                    + Div<$ty, Output = T>
                    + Neg<Output = T>,
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
                    let theta = real.acos();
                    let sin_theta = theta.sin();
                    let imag_exp: Self;

                    if sin_theta == 0. {
                        imag_exp = Self::zero();
                    } else {
                        imag_exp = theta * imag / sin_theta;
                    }

                    r.ln() + imag_exp
                }
                
                fn powf(&self, num: $ty) -> Self {
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
                    }

                    <Self as Functions<$ty, Complex<T>>>::powu_tail(self, num - 1, self * acc)
                }

                fn powu(&self, num: u32) -> Self {
                    <Self as Functions<$ty, Complex<T>>>::powu_tail(self, num, Self::one())
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
            }
        )*
    }
}

impl_functions_for_float!(f32, f64);

pub trait Fill<U> {
    fn zero() -> Self;
    fn one() -> Self;
    fn fill(num: U) -> Self;
    fn from_slice(v: &[U]) -> Self;
    fn from_vec(v: Vec<U>) -> Self;
}

macro_rules! impl_fill_for_float {
    ( $($ty:ty),* ) => {
        $(
            impl Fill<$ty> for $ty {
                fn zero() -> Self {
                    0.0
                }

                fn one() -> Self {
                    1.0
                }

                fn fill(num: $ty) -> Self {
                    num
                }
                
                fn from_slice(v: &[$ty]) -> Self {
                    v[0]
                }
                
                fn from_vec(v: Vec<$ty>) -> Self {
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
    fn zero() -> Self {
        Self {
            re: <T as Fill<U>>::zero(),
            im: <T as Fill<U>>::zero(),
        }
    }

    fn one() -> Self {
        Self {
            re: <T as Fill<U>>::one(),
            im: <T as Fill<U>>::zero(),
        }
    }

    fn fill(num: U) -> Self {
        Self {
            re: <T as Fill<U>>::fill(num),
            im: <T as Fill<U>>::fill(num),
        }
    }
    
    fn from_slice(v: &[U]) -> Self {
        let len = v.len();
        
        let half = len / 2;
        Self{
            re: <T as Fill<U>>::from_slice(&v[..half]), 
            im: <T as Fill<U>>::from_slice(&v[half..len])
        }
    }
    
    fn from_vec(v: Vec<U>) -> Self {
        let len = v.len();
        
        let half = len / 2;
        Self{
            re: <T as Fill<U>>::from_vec(v[..half].to_vec()), 
            im: <T as Fill<U>>::from_vec(v[half..len].to_vec())
        }
    }
}

pub trait Conjugate {
    fn conj(&self) -> Self;
}

macro_rules! impl_conj_for {
    ( $($ty:ty),* ) => {
        $(
            impl Conjugate for $ty {
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

pub trait AbsSq<U> {
    fn abs_sq(&self) -> U;
}

macro_rules! impl_abs_sq_for {
    ( $($ty:ty),* ) => {
        $(
            impl AbsSq<$ty> for $ty {
                fn abs_sq(&self) -> $ty {
                    self * self
                }
            }

            impl<T> AbsSq<$ty> for Complex<T>
            where
                T: AbsSq<$ty> + Copy + Add<Output = T>,
            {
                fn abs_sq(&self) -> $ty {
                    self.re.abs_sq() + self.im.abs_sq()
                }
            }
        )*
    };
}

impl_abs_sq_for!(f32, f64);

pub trait Real<U> {
    fn real(&self) -> U;
}

macro_rules! impl_real_for {
    ( $($ty:ty),* ) => {
        $(
            impl Real<$ty> for $ty {
                fn real(&self) -> $ty {
                    *self
                }
            }

            impl<T> Real<$ty> for Complex<T>
            where
                T: Real<$ty> + Copy,
            {
                fn real(&self) -> $ty {
                    self.re.real()
                }
            }
        )*
    };
}

impl_real_for!(f32, f64);