//! All basic algebraic operations are implemented for complex and hypercomplex
//! types using Cayley-Dickson construction and recursive calls
use crate::*;

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

forward_ref_un_op!(Neg, neg, Complex<T>, T);
impl<T> Neg for Complex<T>
where
    T: Neg<Output = T>,
{
    type Output = Complex<T>;
    fn neg(self) -> Self::Output {
        Self {
            re: -self.re,
            im: -self.im,
        }
    }
}

forward_ref_bin_op!(Add, add, Complex<T>, Complex<T>, T);
impl<T> Add for Complex<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}

forward_ref_bin_op!(Add, add, Complex<T>, Complex<Complex<T>>, T);
impl<T> Add<Complex<Complex<T>>> for Complex<T>
where
    T: Add<Output = T>,
{
    type Output = Complex<Complex<T>>;
    fn add(self, other: Complex<Complex<T>>) -> Self::Output {
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
    fn add(self, other: Complex<T>) -> Self::Output {
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
    fn sub(self, other: Self) -> Self::Output {
        Self {
            re: self.re - other.re,
            im: self.im - other.im,
        }
    }
}

forward_ref_bin_op!(Sub, sub, Complex<T>, Complex<Complex<T>>, T);
impl<T> Sub<Complex<Complex<T>>> for Complex<T>
where
    T: Sub<Output = T>,
{
    type Output = Complex<Complex<T>>;
    fn sub(self, other: Complex<Complex<T>>) -> Self::Output {
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
    fn sub(self, other: Complex<T>) -> Self::Output {
        Complex::<Complex<T>> {
            re: self.re - other,
            im: self.im,
        }
    }
}

forward_ref_bin_op!(Mul, mul, Complex<T>, Complex<T>, T);
impl<T> Mul for Complex<T>
where
    T: Conjugate + Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self {
            re: self.re * other.re - other.im.conj() * self.im,
            im: other.im * self.re + self.im * other.re.conj(),
        }
    }
}

mod inv_real{
    use super::*;
    
    pub trait InvReal {
        fn inv_real(&self) -> Self;
    }

    impl<T> InvReal for Complex<T>
    where
        T: InvReal + Copy,
    {
        fn inv_real(&self) -> Self {
            Self {
                re: self.re.inv_real(),
                im: self.im,
            }
        }
    }

    macro_rules! impl_inv_real_for_float {
        ($($ty:ty),* ) => {
            $(
                impl InvReal for $ty {
                    fn inv_real(&self) -> Self {
                        1. / self
                    }
                }
            )*
        }
    }

    impl_inv_real_for_float!(f32, f64);
}

forward_ref_bin_op!(Div, div, Complex<T>, Complex<T>, T);
impl<T> Div for Complex<T>
where
    Complex<T>: Conjugate + Mul<Output = Complex<T>> + inv_real::InvReal + Copy,
{
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        let other_mod_sq = other * other.conj();
        self * other.conj() * <Complex<T> as inv_real::InvReal>::inv_real(&other_mod_sq)
    }
}

forward_ref_bin_op!(Rem, rem, Complex<T>, Complex<T>, T);
impl<T> Rem for Complex<T>
where
    Complex<T>: Rounding + Mul<Output = Complex<T>> + 
        Div<Output = Complex<T>> + Sub<Output = Complex<T>> + Copy,
{
    type Output = Self;
    fn rem(self, other: Self) -> Self::Output {
        self - other * (self / other).trunc()
    }
}

impl<T> Sum for Complex<T>
where
    Complex<T>: Identity + Add<Output = Complex<T>>,
{
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::zero(), Add::add)
    }
}

impl<'a, T: 'a> Sum<&'a Complex<T>> for Complex<T>
where
    Complex<T>: Identity + Add<Output = Complex<T>> + Copy,
{
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.map(|x| *x).sum::<Self>()
    }
}

impl<T> Product for Complex<T>
where
    Complex<T>: Identity + Mul<Output = Complex<T>>,
{
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::one(), Mul::mul)
    }
}

impl<'a, T: 'a> Product<&'a Complex<T>> for Complex<T>
where
    Complex<T>: Identity + Mul<Output = Complex<T>> + Copy,
{
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.map(|x| *x).product::<Self>()
    }
}

macro_rules! impl_algebra_with_reals {
    ( $($ty:ty),* ) => {
        $(
            forward_ref_bin_op!(Add, add, Complex<T>, $ty, T);
            impl<T> Add<$ty> for Complex<T>
            where
                T: Add<$ty, Output = T>,
            {
                type Output = Self;
                fn add(self, other: $ty) -> Self::Output {
                    Self {
                        re: self.re + other,
                        im: self.im,
                    }
                }
            }

            forward_ref_bin_op!(Add, add, $ty, Complex<T>, T);
            impl<T> Add<Complex<T>> for $ty
            where
                T: Add<$ty, Output = T>,
            {
                type Output = Complex<T>;
                fn add(self, other: Complex<T>) -> Self::Output {
                    Complex::<T> {
                        re: other.re + self,
                        im: other.im,
                    }
                }
            }

            forward_ref_bin_op!(Sub, sub, Complex<T>, $ty, T);
            impl<T> Sub<$ty> for Complex<T>
            where
                T: Sub<$ty, Output = T>,
            {
                type Output = Self;
                fn sub(self, other: $ty) -> Self::Output {
                    Self {
                        re: self.re - other,
                        im: self.im,
                    }
                }
            }

            forward_ref_bin_op!(Sub, sub, $ty, Complex<T>, T);
            impl<T> Sub<Complex<T>> for $ty
            where
                T: Neg<Output = T> + Add<$ty, Output = T>,
            {
                type Output = Complex<T>;
                fn sub(self, other: Complex<T>) -> Self::Output {
                    Complex::<T> {
                        re: -other.re + self,
                        im: -other.im,
                    }
                }
            }


            forward_ref_bin_op!(Mul, mul, Complex<T>, $ty, T);
            impl<T> Mul<$ty> for Complex<T>
            where
                T: Mul<$ty, Output = T>,
            {
                type Output = Self;
                fn mul(self, other: $ty) -> Self::Output {
                    Self {
                        re: self.re * other,
                        im: self.im * other,
                    }
                }
            }

            forward_ref_bin_op!(Mul, mul, $ty, Complex<T>, T);
            impl<T> Mul<Complex<T>> for $ty
            where
                T: Mul<$ty, Output = T>,
            {
                type Output = Complex<T>;
                fn mul(self, other: Complex<T>) -> Self::Output {
                    Complex::<T> {
                        re: other.re * self,
                        im: other.im * self,
                    }
                }
            }

            forward_ref_bin_op!(Div, div, Complex<T>, $ty, T);
            impl<T> Div<$ty> for Complex<T>
            where
                T: Div<$ty, Output = T>,
            {
                type Output = Self;
                fn div(self, other: $ty) -> Self::Output {
                    Self{
                        re: self.re / other,
                        im: self.im / other
                    }
                }
            }

            forward_ref_bin_op!(Div, div, $ty, Complex<T>, T);
            impl<T> Div<Complex<T>> for $ty
            where
                Complex<T>: Conjugate + AbsSq<$ty> + Mul<$ty, Output = Complex<T>> + Div<$ty, Output = Complex<T>>,
            {
                type Output = Complex<T>;
                fn div(self, other: Complex<T>) -> Self::Output {
                    other.conj() * (self / other.abs_sq())
                }
            }
            
            impl<T> Rem<Complex<T>> for $ty
            where
                Complex<T>: Rounding + Mul<Output = Complex<T>> 
                    + Copy,
                $ty: Sub<Complex<T>, Output = Complex<T>> + 
                    Div<Complex<T>, Output = Complex<T>> + Copy
            {
                type Output = Complex<T>;
                fn rem(self, other: Complex<T>) -> Self::Output {
                    self - other * (self / other).trunc()
                }
            }
            
            impl<T> Rem<$ty> for Complex<T>
            where
                Complex<T>: Rounding
                    + Sub<Output = Complex<T>> + 
                    Div<$ty, Output = Complex<T>> + Copy,
                $ty: Mul<Complex<T>, Output = Complex<T>> + Copy
            {
                type Output = Complex<T>;
                fn rem(self, other: $ty) -> Self::Output {
                    self - other * (self / other).trunc()
                }
            }

            bin_op_assign!(AddAssign, add_assign, Add, add, Complex<T>, $ty, T);
            bin_op_assign!(SubAssign, sub_assign, Sub, sub, Complex<T>, $ty, T);
            bin_op_assign!(MulAssign, mul_assign, Mul, mul, Complex<T>, $ty, T);
            bin_op_assign!(DivAssign, div_assign, Div, div, Complex<T>, $ty, T);
            bin_op_assign!(RemAssign, rem_assign, Rem, rem, Complex<T>, $ty, T);
        )*
    }
}

bin_op_assign!(AddAssign, add_assign, Add, add, Complex<T>, Complex<T>, T);
bin_op_assign!(SubAssign, sub_assign, Sub, sub, Complex<T>, Complex<T>, T);
bin_op_assign!(MulAssign, mul_assign, Mul, mul, Complex<T>, Complex<T>, T);
bin_op_assign!(DivAssign, div_assign, Div, div, Complex<T>, Complex<T>, T);
bin_op_assign!(RemAssign, rem_assign, Rem, rem, Complex<T>, Complex<T>, T);
impl_algebra_with_reals!(f32, f64);
