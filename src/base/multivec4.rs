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

/// 4D Multivector template for geometric algebra.
/// 
/// A 4D Multivector describes the linear combination of a scalar `r`, four vectors `x`, `y`, `z` and `w` that describe
/// directions, six bivectors `xy`, `xz`, `xw`, `yz`, `yw` and `zw` that each describe an orientation on a surface, four
/// pseudovectors `xyz`, `xyw`, `xzw` and `yzw` which describe oriented volumes, and a pseudoscalar `xyzw` that describes ...
#[derive(Copy,Clone,Debug)]
pub struct MultiVec4<T> {
    pub r: T,
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
    pub xy: T,
    pub xz: T,
    pub xw: T,
    pub yz: T,
    pub yw: T,
    pub zw: T,
    pub xyz: T,
    pub xyw: T,
    pub xzw: T,
    pub yzw: T,
    pub xyzw: T,
}

macro_rules! multivec4_impl {
    ($($t:ty)+) => {
        $(
            impl MultiVec4<$t> {

            }

            impl Display for MultiVec4<$t> {
                fn fmt(&self,f: &mut Formatter) -> Result {
                    write!(f,"({}, {},{},{},{}, {},{},{},{},{},{}, {},{},{},{}, {})",
                        self.r,
                        self.x,self.y,self.z,self.w,
                        self.xy,self.xz,self.xw,self.yz,self.yw,self.zw,
                        self.xyz,self.xyw,self.xzw,self.yzw,
                        self.xyzw
                    )
                }
            }

            impl PartialEq for MultiVec4<$t> {
                fn eq(&self,other: &Self) -> bool {
                    (self.r == other.r)
                    && (self.x == other.x) && (self.y == other.y) && (self.z == other.z) && (self.w == other.w)
                    && (self.xy == other.xy) && (self.xz == other.xz) && (self.xw == other.xw) && (self.yz == other.yz) && (self.yw == other.yw) && (self.zw == other.zw)
                    && (self.xyz == other.xyz) && (self.xyw == other.xyw) && (self.xzw == other.xzw) && (self.yzw == other.yzw)
                    && (self.xyzw == other.xyzw)
                }
            }

            impl Zero for MultiVec4<$t> {
                const ZERO: MultiVec4<$t> = MultiVec4 {
                    r: <$t>::ZERO,
                    x: <$t>::ZERO,y: <$t>::ZERO,z: <$t>::ZERO,w: <$t>::ZERO,
                    xy: <$t>::ZERO,xz: <$t>::ZERO,xw: <$t>::ZERO,yz: <$t>::ZERO,yw: <$t>::ZERO,zw: <$t>::ZERO,
                    xyz: <$t>::ZERO,xyw: <$t>::ZERO,xzw: <$t>::ZERO,yzw: <$t>::ZERO,
                    xyzw: <$t>::ZERO,
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

multivec4_impl! { f32 f64 }
