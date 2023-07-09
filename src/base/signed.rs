use crate::*;

pub trait Signed: Unsigned {
    fn abs(self) -> Self;
    fn signum(self) -> Self;
    fn is_negative(self) -> bool;
    fn copysign(self,sign: Self) -> Self;
}

macro_rules! signed_impl {
    ($($t:ty)+) => {
        $(
            impl Signed for $t {
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

signed_impl! { isize i8 i16 i32 i64 i128 f32 f64 }
