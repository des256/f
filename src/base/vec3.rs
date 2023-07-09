use {
    crate::*,
    std::{
        cmp::PartialEq,
        fmt::{
            Display,
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
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

macro_rules! vec3_impl {
    ($($t:ty)+) => {
        $(
            impl Vec3<$t> {
                pub fn dot(self,other: Vec3<$t>) -> $t {
                    self.x * other.x + self.y * other.y + self.z * other.z
                }

                //pub fn norm(&self) -> $t {
                //    self.dot(&self).sqrt()
                //}

                //pub fn normalize(&mut self) {
                //    let d = self.norm();
                //    if d != <$t>::ZERO {
                //        self.x /= d;
                //        self.y /= d;
                //    }
                //}

                pub fn scale(&self,other: &Vec3<$t>) -> Self {
                    Vec3 {
                        x: self.x * other.x,
                        y: self.y * other.y,
                        z: self.z * other.z,
                    }
                }
            }

            impl Display for Vec3<$t> {
                fn fmt(&self,f: &mut Formatter) -> Result {
                    write!(f,"({},{},{})",self.x,self.y,self.z)
                }
            }

            impl PartialEq for Vec3<$t> {
                fn eq(&self,other: &Self) -> bool {
                    (self.x == other.x) && (self.y == other.y) && (self.z == other.z)
                }
            }

            impl Zero for Vec3<$t> {
                const ZERO: Vec3<$t> = Vec3 { x: <$t>::ZERO,y: <$t>::ZERO,z: <$t>::ZERO, };
            }

            // vector + vector
            impl Add<Vec3<$t>> for Vec3<$t> {
                type Output = Vec3<$t>;
                fn add(self,other: Self) -> Self {
                    Vec3 {
                        x: self.x + other.x,
                        y: self.y + other.y,
                        z: self.z + other.z,
                    }
                }
            }

            // vector += vector
            impl AddAssign<Vec3<$t>> for Vec3<$t> {
                fn add_assign(&mut self,other: Self) {
                    self.x += other.x;
                    self.y += other.y;
                    self.z += other.z;
                }
            }

            // vector - vector
            impl Sub<Vec3<$t>> for Vec3<$t> {
                type Output = Vec3<$t>;
                fn sub(self,other: Self) -> Self {
                    Vec3 {
                        x: self.x - other.x,
                        y: self.y - other.y,
                        z: self.z - other.z,
                    }
                }
            }

            // vector -= vector
            impl SubAssign<Vec3<$t>> for Vec3<$t> {
                fn sub_assign(&mut self,other: Self) {
                    self.x -= other.x;
                    self.y -= other.y;
                    self.z -= other.z;
                }
            }

            // scalar * vector
            impl Mul<Vec3<$t>> for $t {
                type Output = Vec3<$t>;
                fn mul(self,other: Vec3<$t>) -> Self::Output {
                    Vec3 {
                        x: self * other.x,
                        y: self * other.y,
                        z: self * other.z,
                    }
                }
            }
            
            // vector * scalar
            impl Mul<$t> for Vec3<$t> {
                type Output = Vec3<$t>;
                fn mul(self,other: $t) -> Self::Output {
                    Vec3 {
                        x: self.x * other,
                        y: self.y * other,
                        z: self.z * other,
                    }
                }
            }

            // vector *= scalar
            impl MulAssign<$t> for Vec3<$t> {
                fn mul_assign(&mut self,other: $t) {
                    self.x *= other;
                    self.y *= other;
                    self.z *= other;
                }
            }

            // vector / scalar
            impl Div<$t> for Vec3<$t> {
                type Output = Vec3<$t>;
                fn div(self,other: $t) -> Self::Output {
                    Vec3 {
                        x: self.x / other,
                        y: self.y / other,
                        z: self.z / other,
                    }
                }
            }

            // vector /= scalar
            impl DivAssign<$t> for Vec3<$t> {
                fn div_assign(&mut self,other: $t) {
                    self.x /= other;
                    self.y /= other;
                    self.z /= other;
                }
            }

            // -vector
            impl Neg for Vec3<$t> {
                type Output = Vec3<$t>;
                fn neg(self) -> Self::Output {
                    Vec3 {
                        x: -self.x,
                        y: -self.y,
                        z: -self.z,
                    }
                }
            }
        )+
    }
}

vec3_impl! { isize i8 i16 i32 i64 i128 f32 f64 }
