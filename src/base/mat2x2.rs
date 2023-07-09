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
pub struct Mat2x2<T> {
    pub x: Vec2<T>,
    pub y: Vec2<T>,
}

macro_rules! mat2x2_impl {
    ($($t:ty)+) => {
        $(
            impl Mat2x2<$t> {
                pub fn transpose(self) -> Mat2x2<$t> {
                    Mat2x2 {
                        x: Vec2 {
                            x: self.x.x,
                            y: self.y.x,
                        },
                        y: Vec2 {
                            x: self.x.y,
                            y: self.y.y,
                        },
                    }
                }

                pub fn det(self) -> $t {
                    let a = self.x.x;
                    let b = self.y.x;
                    let c = self.x.y;
                    let d = self.y.y;
                    let aa = d;
                    let ab = c;
                    a * aa - b * ab
                }
            }

            impl Zero for Mat2x2<$t> {
                const ZERO: Mat2x2<$t> = Mat2x2 {
                    x: Vec2::ZERO,
                    y: Vec2::ZERO,
                };
            }

            impl One for Mat2x2<$t> {
                const ONE: Mat2x2<$t> = Mat2x2 {
                    x: Vec2 {
                        x: <$t>::ONE,
                        y: <$t>::ZERO,
                    },
                    y: Vec2 {
                        x: <$t>::ZERO,
                        y: <$t>::ONE,
                    },
                };
            }

            impl PartialEq for Mat2x2<$t> {
                fn eq(&self,other: &Self) -> bool {
                    (self.x == other.x) && (self.y == other.y)
                }
            }

            impl Display for Mat2x2<$t> {
                fn fmt(&self,f: &mut Formatter) -> Result {
                    write!(f,"[{},{}]",self.x,self.y)
                }
            }

            // matrix + matrix
            impl Add<Mat2x2<$t>> for Mat2x2<$t> {
                type Output = Self;
                fn add(self,other: Self) -> Self::Output {
                    Mat2x2 {
                        x: self.x + other.x,
                        y: self.y + other.y,
                    }
                }
            }

            // matrix += matrix
            impl AddAssign<Mat2x2<$t>> for Mat2x2<$t> {
                fn add_assign(&mut self,other: Self) {
                    self.x += other.x;
                    self.y += other.y;
                }
            }

            // matrix - matrix
            impl Sub<Mat2x2<$t>> for Mat2x2<$t> {
                type Output = Self;
                fn sub(self,other: Self) -> Self::Output {
                    Mat2x2 {
                        x: self.x - other.x,
                        y: self.y - other.y,
                    }
                }
            }

            // matrix -= matrix
            impl SubAssign<Mat2x2<$t>> for Mat2x2<$t> {
                fn sub_assign(&mut self,other: Self) {
                    self.x -= other.x;
                    self.y -= other.y;
                }
            }

            // scalar * matrix
            impl Mul<Mat2x2<$t>> for $t {
                type Output = Mat2x2<$t>;
                fn mul(self,other: Mat2x2<$t>) -> Self::Output {
                    Mat2x2 {
                        x: self * other.x,
                        y: self * other.y,
                    }
                }
            }

            // matrix * scalar
            impl Mul<$t> for Mat2x2<$t> {
                type Output = Mat2x2<$t>;
                fn mul(self,other: $t) -> Self::Output {
                    Mat2x2 {
                        x: self.x * other,
                        y: self.y * other,
                    }
                }
            }

            // matrix * vector
            impl Mul<Vec2<$t>> for Mat2x2<$t> {
                type Output = Vec2<$t>;
                fn mul(self,other: Vec2<$t>) -> Self::Output {
                    Vec2 {
                        x: self.x.x * other.x + self.x.y * other.y,
                        y: self.y.x * other.x + self.y.y * other.y,
                    }
                }
            }

            // matrix * matrix
            impl Mul<Mat2x2<$t>> for Mat2x2<$t> {
                type Output = Mat2x2<$t>;
                fn mul(self,other: Mat2x2<$t>) -> Self::Output {
                    Mat2x2 {
                        x: Vec2 {
                            x: self.x.x * other.x.x + self.x.y * other.y.x,
                            y: self.x.x * other.x.y + self.x.y * other.y.y,
                        },
                        y: Vec2 {
                            x: self.y.x * other.x.x + self.y.y * other.y.x,
                            y: self.y.x * other.x.y + self.y.y * other.y.y,
                        },
                    }
                }
            }

            // matrix *= scalar
            impl MulAssign<$t> for Mat2x2<$t> {
                fn mul_assign(&mut self,other: $t) {
                    self.x.x *= other;
                    self.x.y *= other;
                    self.y.x *= other;
                    self.y.y *= other;
                }
            }

            // matrix *= matrix
            impl MulAssign<Mat2x2<$t>> for Mat2x2<$t> {
                fn mul_assign(&mut self,other: Mat2x2<$t>) {
                    let xx = self.x.x * other.x.x + self.x.y * other.y.x;
                    let xy = self.x.x * other.x.y + self.x.y * other.y.y;
                    let yx = self.y.x * other.x.x + self.y.y * other.y.x;
                    let yy = self.y.x * other.x.y + self.y.y * other.y.y;
                    self.x = Vec2 { x: xx,y: xy, };
                    self.y = Vec2 { x: yx,y: yy, };
                }
            }

            // matrix / scalar
            impl Div<$t> for Mat2x2<$t> {
                type Output = Mat2x2<$t>;
                fn div(self,other: $t) -> Self::Output {
                    Mat2x2 {
                        x: self.x / other,
                        y: self.y / other,
                    }
                }
            }

            // matrix /= scalar
            impl DivAssign<$t> for Mat2x2<$t> {
                fn div_assign(&mut self,other: $t) {
                    self.x /= other;
                    self.y /= other;
                }
            }

            // -matrix
            impl Neg for Mat2x2<$t> {
                type Output = Mat2x2<$t>;
                fn neg(self) -> Self::Output {
                    Mat2x2 {
                        x: -self.x,
                        y: -self.y,
                    }
                }
            }
        )+
    }
}

mat2x2_impl! { isize i8 i16 i32 i64 i128 f32 f64 }

macro_rules! mat2x2_real_impl {
    ($($t:ty)+) => {
        $(
            impl Mat2x2<$t> {
                pub fn inv(self) -> Self {
                    let a = self.x.x;
                    let b = self.y.x;
                    let c = self.x.y;
                    let d = self.y.y;
                    let aa = d;
                    let ab = c;
                    let det = a * aa - b * ab;
                    if det == 0.0 {
                        return self;
                    }
                    let ac = b;
                    let ad = a;
                    Mat2x2 {
                        x: Vec2 { x: aa,y: -ac, },
                        y: Vec2 { x: -ab,y: ad, },
                    } / det
                }
            }

            // scalar / matrix
            impl Div<Mat2x2<$t>> for $t {
                type Output = Mat2x2<$t>;
                fn div(self,other: Mat2x2<$t>) -> Self::Output {
                    other.inv() * self
                }
            }

            // matrix / matrix
            impl Div<Mat2x2<$t>> for Mat2x2<$t> {
                type Output = Mat2x2<$t>;
                fn div(self,other: Mat2x2<$t>) -> Self::Output {
                    self * other.inv()
                }
            }

            // matrix /= matrix
            impl DivAssign<Mat2x2<$t>> for Mat2x2<$t> {
                fn div_assign(&mut self,other: Mat2x2<$t>) {
                    *self *= other.inv()
                }
            }
        )+
    }
}

mat2x2_real_impl! { f32 f64 }
