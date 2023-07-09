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
    },
};

/// 2D Multivector template for geometric algebra.
/// 
/// A 2D Multivector describes the linear combination of a scalar `r`, a vector with components `x` and `y` (like ['Vec2']),
/// and a bivector `xy` that describes an orientation or area, or imaginary number (`r` and `xy` together are like [`Complex`]).
#[derive(Copy,Clone,Debug)]
pub struct MultiVec2<T> {
    pub r: T,
    pub x: T,
    pub y: T,
    pub xy: T,
}

macro_rules! multivec2_impl {
    ($($t:ty)+) => {
        $(
            impl MultiVec2<$t> {

            }

            impl Display for MultiVec2<$t> {
                fn fmt(&self,f: &mut Formatter) -> Result {
                    write!(f,"({}, {},{}, {})",
                        self.r,
                        self.x,self.y,
                        self.xy
                    )
                }
            }

            impl PartialEq for MultiVec2<$t> {
                fn eq(&self,other: &Self) -> bool {
                    (self.r == other.r)
                    && (self.x == other.x) && (self.y == other.y)
                    && (self.xy == other.xy)
                }
            }

            impl Zero for MultiVec2<$t> {
                const ZERO: MultiVec2<$t> = MultiVec2 {
                    r: <$t>::ZERO,
                    x: <$t>::ZERO,y: <$t>::ZERO,
                    xy: <$t>::ZERO,
                };
            }

            // multivector + multivector

            // multivector += multivector

            // multivector - multivector

            // multivector -= multivector
            
            // multivector * scalar

            // scalar * multivector

            // multivector * multivector

            // multivector *= scalar

            // multivector *= multivector

            // multivector / scalar

            // multivector / multivector

            // multivector /= scalar

            // multivector /= multivector
        )+
    }
}

multivec2_impl! { f32 f64 }
