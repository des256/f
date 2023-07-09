use crate::*;

/// unsigned number trait
pub trait Unsigned: Sized + Zero + One {
    const MIN: Self;
    const MAX: Self;
    fn div_euclid(self,other: Self) -> Self;
    fn rem_euclid(self,other: Self) -> Self;
    fn min(self,other: Self) -> Self;
    fn max(self,other: Self) -> Self;
    fn clamp(self,min: Self,max: Self) -> Self;
    fn mul_add(self,b: Self,c: Self) -> Self;
    fn powi(self,n: i32) -> Self;
}

macro_rules! unsigned_impl {
    ($($t:ty)+) => {
        $(
            impl Unsigned for $t {
                const MIN: Self = Self::MIN;
                const MAX: Self = Self::MAX;

                fn div_euclid(self,other: Self) -> Self {
                    self.div_euclid(other)
                }
            
                fn rem_euclid(self,other: Self) -> Self {
                    self.rem_euclid(other)
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

unsigned_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }
