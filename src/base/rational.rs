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
            Rem,
            AddAssign,
            SubAssign,
            MulAssign,
            DivAssign,
            Neg,
        },
    },
};

fn _gcd<D: Copy + Zero + PartialEq + Rem<Output=D>>(mut a: D,mut b: D) -> D {
    while b != D::ZERO {
        let c = b;
        b = a % b;
        a = c;
    }
    a
}

/// rational construct
#[derive(Copy,Clone,Debug)]
pub struct Rational<N,D> {
    n: N,
    d: D,
}

macro_rules! rational_impl {
    ($(($n:ty,$d:ty))+) => {
        $(
            impl Rational<$n,$d> {
                fn _reduce(&mut self) {
                    let gcd = _gcd(self.n as $d,self.d);
                    self.n /= gcd as $n;
                    self.d /= gcd;
                }
            }

            impl Zero for Rational<$n,$d> {
                const ZERO: Self = Rational { n: <$n>::ZERO,d: <$d>::ONE, };
            }

            impl One for Rational<$n,$d> {
                const ONE: Self = Rational { n: <$n>::ONE,d: <$d>::ONE, };
            }

            impl Display for Rational<$n,$d> {
                fn fmt(&self,f: &mut Formatter) -> Result {
                    write!(f,"{}/{}",self.n,self.d)
                }
            }

            impl PartialEq<Rational<$n,$d>> for $n {
                fn eq(&self,other: &Rational<$n,$d>) -> bool {
                    (*self == other.n) && (other.d == <$d>::ONE)
                }
            }
            
            impl PartialEq<$n> for Rational<$n,$d> {
                fn eq(&self,other: &$n) -> bool {
                    (self.n == *other) && (self.d == <$d>::ONE)
                }
            }

            impl PartialEq<Rational<$n,$d>> for Rational<$n,$d> {
                fn eq(&self,other: &Rational<$n,$d>) -> bool {
                    (self.n == other.n) && (self.d == other.d)
                }
            }
            
            impl PartialOrd<Rational<$n,$d>> for $n {
                fn partial_cmp(&self,other: &Rational<$n,$d>) -> Option<Ordering> {
                    (self * (other.d as $n)).partial_cmp(&other.n)
                }
            }

            impl PartialOrd<$n> for Rational<$n,$d> {
                fn partial_cmp(&self,other: &$n) -> Option<Ordering> {
                    self.n.partial_cmp(&(*other * (self.d as $n)))
                }
            }
            
            impl PartialOrd<Rational<$n,$d>> for Rational<$n,$d> {
                fn partial_cmp(&self,other: &Rational<$n,$d>) -> Option<Ordering> {
                    (self.n * (other.d as $n)).partial_cmp(&(other.n * (self.d as $n)))
                }
            }            

            impl Add<Rational<$n,$d>> for $n {
                type Output = Rational<$n,$d>;
                fn add(self,other: Rational<$n,$d>) -> Self::Output {
                    let mut result = Rational {
                        n: self * (other.d as $n) + other.n,
                        d: other.d,            
                    };
                    result._reduce();
                    result
                }
            }            

            impl Add<$n> for Rational<$n,$d> {
                type Output = Rational<$n,$d>;
                fn add(self,other: $n) -> Self::Output {
                    let mut result = Rational {
                        n: self.n + (self.d as $n) * other,
                        d: self.d,
                    };
                    result._reduce();
                    result
                }
            }

            impl Add<Rational<$n,$d>> for Rational<$n,$d> {
                type Output = Rational<$n,$d>;
                fn add(self,other: Rational<$n,$d>) -> Self::Output {
                    let mut result = Rational {
                        n: self.n * (other.d as $n) + other.n * (self.d as $n),
                        d: self.d * other.d,
                    };
                    result._reduce();
                    result
                }
            }

            impl AddAssign<$n> for Rational<$n,$d> {
                fn add_assign(&mut self,other: $n) {
                    self.n += other * (self.d as $n);
                    self._reduce();
                }
            }

            impl AddAssign<Rational<$n,$d>> for Rational<$n,$d> {
                fn add_assign(&mut self,other: Rational<$n,$d>) {
                    self.n *= other.d as $n;
                    self.n += other.n * (self.d as $n);
                    self.d *= other.d;
                    self._reduce();
                }
            }

            impl Sub<Rational<$n,$d>> for $n {
                type Output = Rational<$n,$d>;
                fn sub(self,other: Rational<$n,$d>) -> Self::Output {
                    let mut result = Rational {
                        n: self * (other.d as $n) - other.n,
                        d: other.d,            
                    };
                    result._reduce();
                    result
                }
            }            

            impl Sub<$n> for Rational<$n,$d> {
                type Output = Rational<$n,$d>;
                fn sub(self,other: $n) -> Self::Output {
                    let mut result = Rational {
                        n: self.n - (self.d as $n) * other,
                        d: self.d,
                    };
                    result._reduce();
                    result
                }
            }

            impl Sub<Rational<$n,$d>> for Rational<$n,$d> {
                type Output = Rational<$n,$d>;
                fn sub(self,other: Rational<$n,$d>) -> Self::Output {
                    let mut result = Rational {
                        n: self.n * (other.d as $n) - other.n * (self.d as $n),
                        d: self.d * other.d,
                    };
                    result._reduce();
                    result
                }
            }

            impl SubAssign<$n> for Rational<$n,$d> {
                fn sub_assign(&mut self,other: $n) {
                    self.n -= other * (self.d as $n);
                    self._reduce();
                }
            }

            impl SubAssign<Rational<$n,$d>> for Rational<$n,$d> {
                fn sub_assign(&mut self,other: Rational<$n,$d>) {
                    self.n *= other.d as $n;
                    self.n -= other.n * (self.d as $n);
                    self.d *= other.d;
                    self._reduce();
                }
            }

            impl Mul<Rational<$n,$d>> for $n {
                type Output = Rational<$n,$d>;
                fn mul(self,other: Rational<$n,$d>) -> Self::Output {
                    let mut result = Rational {
                        n: self * other.n,
                        d: other.d,            
                    };
                    result._reduce();
                    result
                }
            }            

            impl Mul<$n> for Rational<$n,$d> {
                type Output = Rational<$n,$d>;
                fn mul(self,other: $n) -> Self::Output {
                    let mut result = Rational {
                        n: self.n * other,
                        d: self.d,
                    };
                    result._reduce();
                    result
                }
            }

            impl Mul<Rational<$n,$d>> for Rational<$n,$d> {
                type Output = Rational<$n,$d>;
                fn mul(self,other: Rational<$n,$d>) -> Self::Output {
                    let mut result = Rational {
                        n: self.n * other.n,
                        d: self.d * other.d,
                    };
                    result._reduce();
                    result
                }
            }

            impl MulAssign<$n> for Rational<$n,$d> {
                fn mul_assign(&mut self,other: $n) {
                    self.n *= other;
                    self._reduce();
                }
            }

            impl MulAssign<Rational<$n,$d>> for Rational<$n,$d> {
                fn mul_assign(&mut self,other: Rational<$n,$d>) {
                    self.n *= other.n;
                    self.d *= other.d;
                    self._reduce();
                }
            }

            impl Unsigned for Rational<$n,$d> {
                const MIN: Rational<$n,$d> = Rational { n: <$n>::MIN,d: <$d>::MAX, };
                const MAX: Rational<$n,$d> = Rational { n: <$n>::MAX,d: <$d>::ONE, };

                fn div_euclid(self,_other: Rational<$n,$d>) -> Rational<$n,$d> {
                    // TODO
                    Self::ZERO
                }
            
                fn rem_euclid(self,_other: Rational<$n,$d>) -> Rational<$n,$d> {
                    // TODO
                    Self::ZERO
                }
            
                fn min(self,other: Rational<$n,$d>) -> Rational<$n,$d> {
                    if other < self {
                        other
                    }
                    else {
                        self
                    }
                }
            
                fn max(self,other: Rational<$n,$d>) -> Rational<$n,$d> {
                    if other > self {
                        other
                    }
                    else {
                        self
                    }
                }
            
                fn clamp(self,min: Rational<$n,$d>,max: Rational<$n,$d>) -> Rational<$n,$d> {
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
            
                fn mul_add(self,b: Rational<$n,$d>,c: Rational<$n,$d>) -> Rational<$n,$d> {
                    self * b + c
                }
            
                fn powi(self,_n: i32) -> Rational<$n,$d> {
                    // TODO
                    Self::ZERO
                }
            }
        )+
    }
}

rational_impl! { (usize,usize) (u8,u8) (u16,u16) (u32,u32) (u64,u64) (u128,u128) (isize,usize) (i8,u8) (i16,u16) (i32,u32) (i64,u64) (i128,u128) }

macro_rules! rational_impl_unsigned {
    ($(($n:ty,$d:ty))+) => {
        $(
            impl Div<Rational<$n,$d>> for $n {
                type Output = Rational<$n,$d>;
                fn div(self,other: Rational<$n,$d>) -> Self::Output {
                    let mut result = Rational {
                        n: self * (other.d as $n),
                        d: other.n as $d,            
                    };
                    result._reduce();
                    result
                }
            }            
            
            impl Div<$n> for Rational<$n,$d> {
                type Output = Rational<$n,$d>;
                fn div(self,other: $n) -> Self::Output {
                    let mut result = Rational {
                        n: self.n,
                        d: self.d * (other as $d),
                    };
                    result._reduce();
                    result
                }
            }
            
            impl Div<Rational<$n,$d>> for Rational<$n,$d> {
                type Output = Rational<$n,$d>;
                fn div(self,other: Rational<$n,$d>) -> Self::Output {
                    let mut result = Rational {
                        n: self.n * (other.d as $n),
                        d: self.d * (other.n as $d),
                    };
                    result._reduce();
                    result
                }
            }

            impl DivAssign<$n> for Rational<$n,$d> {
                fn div_assign(&mut self,other: $n) {
                    self.d *= other as $d;
                    self._reduce();
                }
            }

            impl DivAssign<Rational<$n,$d>> for Rational<$n,$d> {
                fn div_assign(&mut self,other: Rational<$n,$d>) {
                    self.n *= other.d as $n;
                    self.d *= other.n as $d;
                    self._reduce();
                }
            }
        )+
    }
}

rational_impl_unsigned! { (usize,usize) (u8,u8) (u16,u16) (u32,u32) (u64,u64) (u128,u128) }

macro_rules! rational_impl_signed {
    ($(($n:ty,$d:ty))+) => {
        $(
            impl Div<Rational<$n,$d>> for $n {
                type Output = Rational<$n,$d>;
                fn div(self,other: Rational<$n,$d>) -> Self::Output {
                    let mut result = if other.n < <$n>::ZERO {
                        Rational {
                            n: -(self * (other.d as $n)),
                            d: -other.n as $d,
                        }
                    }
                    else {
                        Rational {
                            n: self * (other.d as $n),
                            d: other.n as $d,            
                        }
                    };
                    result._reduce();
                    result
                }
            }            
            
            impl Div<$n> for Rational<$n,$d> {
                type Output = Rational<$n,$d>;
                fn div(self,other: $n) -> Self::Output {
                    let mut result = if other < <$n>::ZERO {
                        Rational {
                            n: -self.n,
                            d: self.d * (-other as $d),
                        }
                    }
                    else {
                        Rational {
                            n: self.n,
                            d: self.d * (other as $d),
                        }
                    };
                    result._reduce();
                    result
                }
            }
            
            impl Div<Rational<$n,$d>> for Rational<$n,$d> {
                type Output = Rational<$n,$d>;
                fn div(self,other: Rational<$n,$d>) -> Self::Output {
                    let mut result = if other.n < 0 {
                        Rational {
                            n: -self.n * (other.d as $n),
                            d: self.d * (-other.n as $d),
                        }
                    }
                    else {
                        Rational {
                            n: self.n * (other.d as $n),
                            d: self.d * (other.n as $d),
                        }
                    };
                    result._reduce();
                    result
                }
            }

            impl DivAssign<$n> for Rational<$n,$d> {
                fn div_assign(&mut self,other: $n) {
                    if other < <$n>::ZERO {
                        self.n = -self.n;
                        self.d *= -other as $d;
                    }
                    else {
                        self.d *= other as $d;
                    }
                    self._reduce();
                }
            }

            impl DivAssign<Rational<$n,$d>> for Rational<$n,$d> {
                fn div_assign(&mut self,other: Rational<$n,$d>) {
                    if other.n < <$n>::ZERO {
                        self.n *= -(other.d as $n);
                        self.d *= -other.n as $d;
                    }
                    else {
                        self.n *= other.d as $n;
                        self.d *= other.n as $d;
                    }
                    self._reduce();
                }
            }

            impl Neg for Rational<$n,$d> {
                type Output = Self;
                fn neg(self) -> Self::Output {
                    Rational {
                        n: -self.n,
                        d: self.d,
                    }
                }
            }

            impl Signed for Rational<$n,$d> {
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
        )+
    }
}

rational_impl_signed! { (isize,usize) (i8,u8) (i16,u16) (i32,u32) (i64,u64) (i128,u128) }

#[allow(non_camel_case_types)]
pub type rsize = Rational<isize,usize>;

#[allow(non_camel_case_types)]
pub type r8 = Rational<i8,u8>;

#[allow(non_camel_case_types)]
pub type r16 = Rational<i16,u16>;

#[allow(non_camel_case_types)]
pub type r32 = Rational<i32,u32>;

#[allow(non_camel_case_types)]
pub type r64 = Rational<i64,u64>;

#[allow(non_camel_case_types)]
pub type r128 = Rational<i128,u128>;
