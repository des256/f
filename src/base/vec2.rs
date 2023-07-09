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
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

macro_rules! vec2_impl {
    ($($t:ty)+) => {
        $(
            impl Vec2<$t> {
                pub fn dot(self,other: Vec2<$t>) -> $t {
                    self.x * other.x + self.y * other.y
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

                pub fn scale(&self,other: &Vec2<$t>) -> Self {
                    Vec2 {
                        x: self.x * other.x,
                        y: self.y * other.y,
                    }
                }
            }

            impl Display for Vec2<$t> {
                fn fmt(&self,f: &mut Formatter) -> Result {
                    write!(f,"({},{})",self.x,self.y)
                }
            }

            impl PartialEq for Vec2<$t> {
                fn eq(&self,other: &Self) -> bool {
                    (self.x == other.x) && (self.y == other.y)
                }
            }

            impl Zero for Vec2<$t> {
                const ZERO: Vec2<$t> = Vec2 { x: <$t>::ZERO,y: <$t>::ZERO, };
            }

            // vector + vector
            impl Add<Vec2<$t>> for Vec2<$t> {
                type Output = Vec2<$t>;
                fn add(self,other: Self) -> Self {
                    Vec2 {
                        x: self.x + other.x,
                        y: self.y + other.y,
                    }
                }
            }

            // vector += vector
            impl AddAssign<Vec2<$t>> for Vec2<$t> {
                fn add_assign(&mut self,other: Self) {
                    self.x += other.x;
                    self.y += other.y;
                }
            }

            // vector - vector
            impl Sub<Vec2<$t>> for Vec2<$t> {
                type Output = Vec2<$t>;
                fn sub(self,other: Self) -> Self {
                    Vec2 {
                        x: self.x - other.x,
                        y: self.y - other.y,
                    }
                }
            }

            // vector -= vector
            impl SubAssign<Vec2<$t>> for Vec2<$t> {
                fn sub_assign(&mut self,other: Self) {
                    self.x -= other.x;
                    self.y -= other.y;
                }
            }

            // scalar * vector
            impl Mul<Vec2<$t>> for $t {
                type Output = Vec2<$t>;
                fn mul(self,other: Vec2<$t>) -> Self::Output {
                    Vec2 {
                        x: self * other.x,
                        y: self * other.y,
                    }
                }
            }
            
            // vector * scalar
            impl Mul<$t> for Vec2<$t> {
                type Output = Vec2<$t>;
                fn mul(self,other: $t) -> Self::Output {
                    Vec2 {
                        x: self.x * other,
                        y: self.y * other,
                    }
                }
            }

            // vector *= scalar
            impl MulAssign<$t> for Vec2<$t> {
                fn mul_assign(&mut self,other: $t) {
                    self.x *= other;
                    self.y *= other;
                }
            }

            // vector / scalar
            impl Div<$t> for Vec2<$t> {
                type Output = Vec2<$t>;
                fn div(self,other: $t) -> Self::Output {
                    Vec2 {
                        x: self.x / other,
                        y: self.y / other,
                    }
                }
            }

            // vector /= scalar
            impl DivAssign<$t> for Vec2<$t> {
                fn div_assign(&mut self,other: $t) {
                    self.x /= other;
                    self.y /= other;
                }
            }

            // -vector
            impl Neg for Vec2<$t> {
                type Output = Vec2<$t>;
                fn neg(self) -> Self::Output {
                    Vec2 {
                        x: -self.x,
                        y: -self.y,
                    }
                }
            }
        )+
    }
}

vec2_impl! { isize i8 i16 i32 i64 i128 f32 f64 }
