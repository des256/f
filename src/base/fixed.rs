use {
    crate::*,
    std::{
        cmp::{
            PartialEq,
            PartialOrd,
            Ordering,
        },
        fmt::{
            Display,
            Debug,
            Formatter,
            Result,
        },
        ops::{
            Add,
            Sub,
            Mul,
            Div,
            AddAssign,
            SubAssign,
            MulAssign,
            DivAssign,
            Neg,
        },
    },
};

/// fixed-point real number
#[derive(Copy,Clone,Debug)]
pub struct Fixed<T,const B: usize>(T);

macro_rules! fixed_impl {
    ($(($t:ty,$b:expr))+) => {
        $(
            impl Fixed<$t,$b> {
                pub const BITS: usize = $b;
            }
            
            impl Zero for Fixed<$t,$b> {
                const ZERO: Self = Fixed(<$t>::ZERO);
            }
            
            impl One for Fixed<$t,$b> {
                const ONE: Self = Fixed(<$t>::ONE << $b);
            }

            impl Display for Fixed<$t,$b> {
                fn fmt(&self,f: &mut Formatter) -> Result {
                    write!(f,"{}:{}",self.0,$b)
                }
            }
            
            impl PartialEq<Fixed<$t,$b>> for Fixed<$t,$b> {
                fn eq(&self,other: &Fixed<$t,$b>) -> bool {
                    self.0 == other.0
                }
            }
            
            impl PartialOrd<Fixed<$t,$b>> for Fixed<$t,$b> {
                fn partial_cmp(&self,other: &Fixed<$t,$b>) -> Option<Ordering> {
                    self.0.partial_cmp(&other.0)
                }
            }

            impl Add<Fixed<$t,$b>> for Fixed<$t,$b> {
                type Output = Self;
                fn add(self,other: Self) -> Self::Output {
                    Fixed(self.0 + other.0)
                }
            }

            impl AddAssign<Fixed<$t,$b>> for Fixed<$t,$b> {
                fn add_assign(&mut self,other: Self) {
                    self.0 += other.0;
                }
            }

            impl Sub<Fixed<$t,$b>> for Fixed<$t,$b> {
                type Output = Self;
                fn sub(self,other: Self) -> Self::Output {
                    Fixed(self.0 - other.0)
                }
            }

            impl SubAssign<Fixed<$t,$b>> for Fixed<$t,$b> {
                fn sub_assign(&mut self,other: Self) {
                    self.0 -= other.0;
                }
            }
            
            impl Mul<Fixed<$t,$b>> for Fixed<$t,$b> {
                type Output = Self;
                fn mul(self,other: Self) -> Self::Output {
                    Fixed((self.0 * other.0) >> $b)
                }
            }

            impl MulAssign<Fixed<$t,$b>> for Fixed<$t,$b> {
                fn mul_assign(&mut self,other: Self) {
                    self.0 *= other.0;
                    self.0 >>= $b;
                }
            }

            impl Div<Fixed<$t,$b>> for Fixed<$t,$b> {
                type Output = Self;
                fn div(self,other: Self) -> Self::Output {
                    Fixed((self.0 << $b) / other.0)
                }
            }

            impl DivAssign<Fixed<$t,$b>> for Fixed<$t,$b> {
                fn div_assign(&mut self,other: Self) {
                    self.0 <<= $b;
                    self.0 /= other.0;
                }
            }
            
            impl Unsigned for Fixed<$t,$b> {
                const MIN: Self = Self::ZERO;
                const MAX: Self = Fixed(<$t>::MAX);

                fn div_euclid(self,_other: Self) -> Self {
                    // TODO
                    Self::ZERO
                }
            
                fn rem_euclid(self,_other: Self) -> Self {
                    // TODO
                    Self::ZERO
                }
            
                fn min(self,other: Self) -> Self {
                    if other < self {
                        other
                    }
                    else {
                        self
                    }
                }
            
                fn max(self,other: Self) -> Self {
                    if other > self {
                        other
                    }
                    else {
                        self
                    }
                }
            
                fn clamp(self,min: Self,max: Self) -> Self {
                    if max < self {
                        max
                    }
                    else if min > self {
                        min
                    }
                    else {
                        self
                    }
                }
            
                fn mul_add(self,b: Self,c: Self) -> Self {
                    self * b + c
                }
            
                fn powi(self,_n: i32) -> Self {
                    // TODO
                    Self::ZERO
                }            
            }
        )+
    }
}

//fixed_impl! { (u8,8) (u16,8) (u32,16) (u64,32) (u128,64) (i8,8) (i16,8) (i32,16) (i64,32) (i128,64) }
fixed_impl! { (u16,8) (u32,16) (u64,32) (u128,64) (i16,8) (i32,16) (i64,32) (i128,64) }

macro_rules! fixed_impl_signed {
    ($(($t:ty,$b:expr))+) => {
        $(
            impl Neg for Fixed<$t,$b> {
                type Output = Self;
                fn neg(self) -> Self::Output {
                    Fixed(-self.0)
                }
            }

            impl Signed for Fixed<$t,$b> {
                fn abs(self) -> Self {
                    if self < Self::ZERO {
                        -self
                    }
                    else {
                        self
                    }
                }
            
                fn signum(self) -> Self {
                    if self < Self::ZERO {
                        -Self::ONE
                    }
                    else {
                        Self::ONE
                    }
                }
            
                fn is_negative(self) -> bool {
                    self < Self::ZERO
                }
            
                fn copysign(self,sign: Self) -> Self {
                    if sign < Self::ZERO {
                        if self < Self::ZERO {
                            self
                        }
                        else {
                            -self
                        }
                    }
                    else {
                        if self < Self::ZERO {
                            -self
                        }
                        else {
                            self
                        }
                    }
                }            
            }

            impl Real for Fixed<$t,$b> {
                const PI: Self = Fixed((std::f64::consts::PI * (Self::ONE.0 as f64)) as $t);

                fn floor(self) -> Self {
                    Fixed(self.0 & !(Self::ONE.0 - 1))
                }

                fn ceil(self) -> Self {
                    self.floor() + Self::ONE
                }

                fn round(self) -> Self { 
                    let one = Self::ONE.0 as f64;
                    let x = (self.0 as f64) / one;
                    let r = x.round();
                    let y = (r * one) as $t;
                    Fixed(y)
                }

                fn trunc(self) -> Self {
                    let one = Self::ONE.0 as f64;
                    let x = (self.0 as f64) / one;
                    let r = x.trunc();
                    let y = (r * one) as $t;
                    Fixed(y)
                }

                fn fract(self) -> Self {
                    let one = Self::ONE.0 as f64;
                    let x = (self.0 as f64) / one;
                    let r = x.fract();
                    let y = (r * one) as $t;
                    Fixed(y)
                }

                fn powf(self,n: Self) -> Self {
                    let one = Self::ONE.0 as f64;
                    let x = (self.0 as f64) / one;
                    let r = x.powf((n.0 as f64) / one);
                    let y = (r * one) as $t;
                    Fixed(y)
                }

                fn sqrt(self) -> Self {
                    let one = Self::ONE.0 as f64;
                    let x = (self.0 as f64) / one;
                    let r = x.sqrt();
                    let y = (r * one) as $t;
                    Fixed(y)
                }

                fn exp(self) -> Self {
                    let one = Self::ONE.0 as f64;
                    let x = (self.0 as f64) / one;
                    let r = x.exp();
                    let y = (r * one) as $t;
                    Fixed(y)
                }

                fn exp2(self) -> Self {
                    let one = Self::ONE.0 as f64;
                    let x = (self.0 as f64) / one;
                    let r = x.exp2();
                    let y = (r * one) as $t;
                    Fixed(y)
                }

                fn ln(self) -> Self {
                    let one = Self::ONE.0 as f64;
                    let x = (self.0 as f64) / one;
                    let r = x.ln();
                    let y = (r * one) as $t;
                    Fixed(y)
                }

                fn log(self,base: Self) -> Self {
                    let one = Self::ONE.0 as f64;
                    let x = (self.0 as f64) / one;
                    let r = x.log((base.0 as f64) / one);
                    let y = (r * one) as $t;
                    Fixed(y)
                }

                fn log2(self) -> Self {
                    let one = Self::ONE.0 as f64;
                    let x = (self.0 as f64) / one;
                    let r = x.log2();
                    let y = (r * one) as $t;
                    Fixed(y)
                }

                fn log10(self) -> Self {
                    let one = Self::ONE.0 as f64;
                    let x = (self.0 as f64) / one;
                    let r = x.log10();
                    let y = (r * one) as $t;
                    Fixed(y)
                }

                fn cbrt(self) -> Self {
                    let one = Self::ONE.0 as f64;
                    let x = (self.0 as f64) / one;
                    let r = x.cbrt();
                    let y = (r * one) as $t;
                    Fixed(y)
                }

                fn hypot(self,other: Self) -> Self {
                    let one = Self::ONE.0 as f64;
                    let x = (self.0 as f64) / one;
                    let r = x.hypot((other.0 as f64) / one);
                    let y = (r * one) as $t;
                    Fixed(y)
                }

                fn sin(self) -> Self {
                    let one = Self::ONE.0 as f64;
                    let x = (self.0 as f64) / one;
                    let r = x.sin();
                    let y = (r * one) as $t;
                    Fixed(y)
                }

                fn cos(self) -> Self {
                    let one = Self::ONE.0 as f64;
                    let x = (self.0 as f64) / one;
                    let r = x.cos();
                    let y = (r * one) as $t;
                    Fixed(y)
                }

                fn tan(self) -> Self {
                    let one = Self::ONE.0 as f64;
                    let x = (self.0 as f64) / one;
                    let r = x.tan();
                    let y = (r * one) as $t;
                    Fixed(y)
                }

                fn asin(self) -> Self {
                    let one = Self::ONE.0 as f64;
                    let x = (self.0 as f64) / one;
                    let r = x.asin();
                    let y = (r * one) as $t;
                    Fixed(y)
                }

                fn acos(self) -> Self {
                    let one = Self::ONE.0 as f64;
                    let x = (self.0 as f64) / one;
                    let r = x.acos();
                    let y = (r * one) as $t;
                    Fixed(y)
                }

                fn atan(self) -> Self {
                    let one = Self::ONE.0 as f64;
                    let x = (self.0 as f64) / one;
                    let r = x.atan();
                    let y = (r * one) as $t;
                    Fixed(y)
                }

                fn atan2(self,other: Self) -> Self {
                    let one = Self::ONE.0 as f64;
                    let x = (self.0 as f64) / one;
                    let r = x.atan2((other.0 as f64) / one);
                    let y = (r * one) as $t;
                    Fixed(y)
                }

                fn sin_cos(self) -> (Self,Self) {
                    let one = Self::ONE.0 as f64;
                    let x = (self.0 as f64) / one;
                    let s = x.sin();
                    let c = x.cos();
                    let ys = (s * one) as $t;
                    let yc = (c * one) as $t;
                    (Fixed(ys),Fixed(yc))
                }

                fn exp_m1(self) -> Self {
                    let one = Self::ONE.0 as f64;
                    let x = (self.0 as f64) / one;
                    let r = x.exp_m1();
                    let y = (r * one) as $t;
                    Fixed(y)
                }

                fn ln_1p(self) -> Self {
                    let one = Self::ONE.0 as f64;
                    let x = (self.0 as f64) / one;
                    let r = x.ln_1p();
                    let y = (r * one) as $t;
                    Fixed(y)
                }

                fn sinh(self) -> Self {
                    let one = Self::ONE.0 as f64;
                    let x = (self.0 as f64) / one;
                    let r = x.sinh();
                    let y = (r * one) as $t;
                    Fixed(y)
                }

                fn cosh(self) -> Self {
                    let one = Self::ONE.0 as f64;
                    let x = (self.0 as f64) / one;
                    let r = x.cosh();
                    let y = (r * one) as $t;
                    Fixed(y)
                }

                fn tanh(self) -> Self {
                    let one = Self::ONE.0 as f64;
                    let x = (self.0 as f64) / one;
                    let r = x.tanh();
                    let y = (r * one) as $t;
                    Fixed(y)
                }

                fn asinh(self) -> Self {
                    let one = Self::ONE.0 as f64;
                    let x = (self.0 as f64) / one;
                    let r = x.asinh();
                    let y = (r * one) as $t;
                    Fixed(y)
                }

                fn acosh(self) -> Self {
                    let one = Self::ONE.0 as f64;
                    let x = (self.0 as f64) / one;
                    let r = x.acosh();
                    let y = (r * one) as $t;
                    Fixed(y)
                }

                fn atanh(self) -> Self {
                    let one = Self::ONE.0 as f64;
                    let x = (self.0 as f64) / one;
                    let r = x.atanh();
                    let y = (r * one) as $t;
                    Fixed(y)
                }

                fn inv(self) -> Self {
                    let one = Self::ONE.0 as f64;
                    let x = (self.0 as f64) / one;
                    let r = x.inv();
                    let y = (r * one) as $t;
                    Fixed(y)
                }

                fn to_degrees(self) -> Self {
                    let one = Self::ONE.0 as f64;
                    let x = (self.0 as f64) / one;
                    let r = x.to_degrees();
                    let y = (r * one) as $t;
                    Fixed(y)
                }

                fn to_radians(self) -> Self {
                    let one = Self::ONE.0 as f64;
                    let x = (self.0 as f64) / one;
                    let r = x.to_radians();
                    let y = (r * one) as $t;
                    Fixed(y)
                }
            }
        )+
    }
}

//fixed_impl_signed! { (i8,8) (i16,8) (i32,16) (i64,32) (i128,64) }
fixed_impl_signed! { (i16,8) (i32,16) (i64,32) (i128,64) }

//#[allow(non_camel_case_types)]
//pub type u88 = Fixed<u8,8>;

#[allow(non_camel_case_types)]
pub type u168 = Fixed<u16,8>;

#[allow(non_camel_case_types)]
pub type u3216 = Fixed<u32,16>;

#[allow(non_camel_case_types)]
pub type u6432 = Fixed<u64,32>;

#[allow(non_camel_case_types)]
pub type u12864 = Fixed<u128,64>;

#[allow(non_camel_case_types)]
pub type i88 = Fixed<i8,8>;

#[allow(non_camel_case_types)]
pub type i168 = Fixed<i16,8>;

#[allow(non_camel_case_types)]
pub type i3216 = Fixed<i32,16>;

#[allow(non_camel_case_types)]
pub type i6432 = Fixed<i64,32>;

#[allow(non_camel_case_types)]
pub type i12864 = Fixed<i128,64>;
