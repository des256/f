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

/// 3D Multivector template for geometric algebra.
/// 
/// A 3D Multivector describes the linear combination of a scalar `r`, a vector with three components `x`, `y` and `z` (like
/// [`Vec3`]), three bivectors `xy`, `xz` and `yz` that each describe orientations in orthogonal planes (like [`Quaternion`]),
/// and a trivector `xyz` that describes an oriented volume or imaginary number.
/// 
/// Uses include:
/// 
/// * Various quantities in physics, analysis of electrodynamics, mechanics, torque, angular momentum.
/// * Mathematical equivalents, complex numbers, quaternions.
#[derive(Copy,Clone,Debug)]
pub struct MultiVec3<T> {
    pub r: T, // scalar, weight, etc.
    pub x: T, // position, speed, acceleration, momentum, force, etc.
    pub y: T,
    pub z: T,
    pub xy: T, // orientation, rotation, rotor, torque, angular momentum, magnetic field, etc.
    pub xz: T,
    pub yz: T,
    pub xyz: T, // imaginary number, magnetic flux, etc.
}

macro_rules! multivec3_impl {
    ($($t:ty)+) => {
        $(
            impl MultiVec3<$t> {

            }

            impl Display for MultiVec3<$t> {
                fn fmt(&self,f: &mut Formatter) -> Result {
                    write!(f,"({}, {},{},{}, {},{},{}, {})",
                        self.r,
                        self.x,self.y,self.z,
                        self.xy,self.xz,self.yz,
                        self.xyz
                    )
                }
            }

            impl PartialEq for MultiVec3<$t> {
                fn eq(&self,other: &Self) -> bool {
                    (self.r == other.r)
                    && (self.x == other.x) && (self.y == other.y) && (self.z == other.z)
                    && (self.xy == other.xy) && (self.xz == other.xz) && (self.yz == other.yz)
                    && (self.xyz == other.xyz)
                }
            }

            impl Zero for MultiVec3<$t> {
                const ZERO: MultiVec3<$t> = MultiVec3 {
                    r: <$t>::ZERO,
                    x: <$t>::ZERO,y: <$t>::ZERO,z: <$t>::ZERO,
                    xy: <$t>::ZERO,xz: <$t>::ZERO,yz: <$t>::ZERO,
                    xyz: <$t>::ZERO,
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

multivec3_impl! { f32 f64 }
