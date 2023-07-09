use {
    crate::*,
    std::{
        cmp::PartialEq,
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

#[derive(Copy,Clone,Debug)]
pub struct Complex<T> {
    pub r: T,
    pub i: T,
}

macro_rules! complex_impl {
    ($($t:ty)+) => {
        $(
            impl Complex<$t> {
                pub fn conj(&self) -> Self {
                    Complex {
                        r: self.r,
                        i: -self.i,
                    }
                }

                pub fn arg(&self) -> $t {
                    self.r.atan2(self.i)
                }
            }

            impl Zero for Complex<$t> {
                const ZERO: Self = Complex { r: <$t>::ZERO,i: <$t>::ZERO, };
            }
            
            impl One for Complex<$t> {
                const ONE: Self = Complex { r: <$t>::ONE,i: <$t>::ZERO, };
            }

            impl Display for Complex<$t> {
                fn fmt(&self,f: &mut Formatter) -> Result {
                    let si = if self.i < <$t>::ZERO {
                        format!("{}i",self.i)
                    }
                    else {
                        format!("+{}i",self.i)
                    };
                    write!(f,"{}{}",self.r,si)
                }
            }
            
            impl PartialEq<Complex<$t>> for Complex<$t> {
                fn eq(&self,other: &Complex<$t>) -> bool {
                    (self.r == other.r) && (self.i == other.i)
                }
            }

            impl Add<Complex<$t>> for $t {
                type Output = Complex<$t>;
                fn add(self,other: Complex<$t>) -> Self::Output {
                    Complex {
                        r: self + other.r,
                        i: other.i,
                    }
                }
            }

            impl Add<$t> for Complex<$t> {
                type Output = Self;
                fn add(self,other: $t) -> Self::Output {
                    Complex {
                        r: self.r + other,
                        i: self.i,
                    }
                }
            }

            impl Add<Complex<$t>> for Complex<$t> {
                type Output = Self;
                fn add(self,other: Self) -> Self::Output {
                    Complex {
                        r: self.r + other.r,
                        i: self.i + other.i,
                    }
                }
            }

            impl AddAssign<$t> for Complex<$t> {
                fn add_assign(&mut self,other: $t) {
                    self.r += other;
                }
            }

            impl AddAssign<Complex<$t>> for Complex<$t> {
                fn add_assign(&mut self,other: Self) {
                    self.r += other.r;
                    self.i += other.i;
                }
            }

            impl Sub<Complex<$t>> for $t {
                type Output = Complex<$t>;
                fn sub(self,other: Complex<$t>) -> Self::Output {
                    Complex {
                        r: self - other.r,
                        i: -other.i,
                    }
                }
            }

            impl Sub<$t> for Complex<$t> {
                type Output = Self;
                fn sub(self,other: $t) -> Self::Output {
                    Complex {
                        r: self.r - other,
                        i: self.i,
                    }
                }
            }

            impl Sub<Complex<$t>> for Complex<$t> {
                type Output = Self;
                fn sub(self,other: Self) -> Self::Output {
                    Complex {
                        r: self.r - other.r,
                        i: self.i - other.i,
                    }
                }
            }

            impl SubAssign<$t> for Complex<$t> {
                fn sub_assign(&mut self,other: $t) {
                    self.r -= other;
                }
            }

            impl SubAssign<Complex<$t>> for Complex<$t> {
                fn sub_assign(&mut self,other: Self) {
                    self.r -= other.r;
                    self.i -= other.i;
                }
            }

            impl Mul<Complex<$t>> for $t {
                type Output = Complex<$t>;
                fn mul(self,other: Complex<$t>) -> Self::Output {
                    Complex {
                        r: self * other.r,
                        i: self * other.i,
                    }
                }
            }

            impl Mul<$t> for Complex<$t> {
                type Output = Self;
                fn mul(self,other: $t) -> Self::Output {
                    Complex {
                        r: self.r * other,
                        i: self.i * other,
                    }
                }
            }

            impl Mul<Complex<$t>> for Complex<$t> {
                type Output = Self;
                fn mul(self,other: Self) -> Self::Output {
                    Complex {
                        r: self.r * other.r - self.i * other.i,
                        i: self.r * other.i + self.i * other.r,
                    }
                }
            }

            impl MulAssign<$t> for Complex<$t> {
                fn mul_assign(&mut self,other: $t) {
                    self.r *= other;
                    self.i *= other;
                }
            }

            impl MulAssign<Complex<$t>> for Complex<$t> {
                fn mul_assign(&mut self,other: Self) {
                    let r = self.r * other.r - self.i * other.i;
                    let i = self.r * other.i + self.i * other.r;
                    self.r = r;
                    self.i = i;
                }
            }

            impl Div<Complex<$t>> for $t {
                type Output = Complex<$t>;
                fn div(self,other: Complex<$t>) -> Self::Output {
                    let f = other.r * other.r + other.i * other.i;
                    Complex {
                        r: self * other.r / f,
                        i: -self * other.i / f,
                    }
                }
            }

            impl Div<$t> for Complex<$t> {
                type Output = Self;
                fn div(self,other: $t) -> Self::Output {
                    Complex {
                        r: self.r / other,
                        i: self.i / other,
                    }
                }
            }

            impl Div<Complex<$t>> for Complex<$t> {
                type Output = Self;
                fn div(self,other: Self) -> Self::Output {
                    let f = other.r * other.r + other.i * other.i;
                    Complex {
                        r: (self.r * other.r + self.i * other.i) / f,
                        i: (self.i * other.r - self.r * other.i) / f,
                    }
                }
            }

            impl DivAssign<$t> for Complex<$t> {
                fn div_assign(&mut self,other: $t) {
                    self.r /= other;
                    self.i /= other;
                }
            }

            impl DivAssign<Complex<$t>> for Complex<$t> {
                fn div_assign(&mut self,other: Self) {
                    let f = other.r * other.r + other.i * other.i;
                    let r = (self.r * other.r + self.i * other.i) / f; 
                    let i = (self.i * other.r - self.r * other.i) / f;
                    self.r = r;
                    self.i = i;
                }
            }

            impl Neg for Complex<$t> {
                type Output = Self;
                fn neg(self) -> Self {
                    Complex {
                        r: -self.r,
                        i: -self.i,
                    }
                }
            }

            /*
            impl Unsigned for Complex<$t> {
                // TODO
            }

            impl Signed for Complex<$t> {
                // TODO
            }

            impl Real for Complex<$t> {
                // TODO
            }
            */
        )+
    }
}

complex_impl! { f32 f64 }
