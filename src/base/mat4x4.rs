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
pub struct Mat4x4<T> {
    pub x: Vec4<T>,
    pub y: Vec4<T>,
    pub z: Vec4<T>,
    pub w: Vec4<T>,
}

macro_rules! mat4x4_impl {
    ($($t:ty)+) => {
        $(
            impl Mat4x4<$t> {
                pub fn from_mv(m: Mat3x3<$t>,v: Vec3<$t>) -> Mat4x4<$t> {
                    Mat4x4 {
                        x: Vec4 {
                            x: m.x.x,
                            y: m.x.y,
                            z: m.x.z,
                            w: <$t>::ZERO,
                        },
                        y: Vec4 {
                            x: m.y.x,
                            y: m.y.y,
                            z: m.y.z,
                            w: <$t>::ZERO,
                        },
                        z: Vec4 {
                            x: m.z.x,
                            y: m.z.y,
                            z: m.z.z,
                            w: <$t>::ZERO,
                        },
                        w: Vec4 {
                            x: v.x,
                            y: v.y,
                            z: v.z,
                            w: <$t>::ONE,
                        },
                    }
                }

                pub fn transpose(self) -> Mat4x4<$t> {
                    Mat4x4 {
                        x: Vec4 {
                            x: self.x.x,
                            y: self.y.x,
                            z: self.z.x,
                            w: self.w.x,
                        },
                        y: Vec4 {
                            x: self.x.y,
                            y: self.y.y,
                            z: self.z.y,
                            w: self.w.y,
                        },
                        z: Vec4 {
                            x: self.x.z,
                            y: self.y.z,
                            z: self.z.z,
                            w: self.w.z,
                        },
                        w: Vec4 {
                            x: self.x.w,
                            y: self.y.w,
                            z: self.z.w,
                            w: self.w.w,
                        },
                    }
                }

                pub fn det(self) -> $t {
                    let a = self.x.x;
                    let e = self.x.y;
                    let i = self.x.z;
                    let m = self.x.w;
                    let b = self.y.x;
                    let f = self.y.y;
                    let j = self.y.z;
                    let n = self.y.w;
                    let c = self.z.x;
                    let g = self.z.y;
                    let k = self.z.z;
                    let o = self.z.w;
                    let d = self.w.x;
                    let h = self.w.y;
                    let l = self.w.z;
                    let p = self.w.w;
                    let kplo = k * p - l * o;
                    let jpln = j * p - l * n;
                    let jokn = j * o - k * n;
                    let iplm = i * p - l * m;
                    let iokm = i * o - k * m;
                    let injm = i * n - j * m;
                    let aa = f * kplo - g * jpln + h * jokn;
                    let ab = e * kplo - g * iplm + h * iokm;
                    let ac = e * jpln - f * iplm + h * injm;
                    let ad = e * jokn - f * iokm + g * injm;
                    a * aa - b * ab + c * ac - d * ad
                }
            }

            impl PartialEq for Mat4x4<$t> {
                fn eq(&self,other: &Self) -> bool {
                    (self.x == other.x) && (self.y == other.y) && (self.z == other.z) && (self.w == other.w)
                }
            }

            impl Display for Mat4x4<$t> {
                fn fmt(&self,f: &mut Formatter) -> Result {
                    write!(f,"[{},{},{},{}]",self.x,self.y,self.z,self.w)
                }
            }

            // matrix + matrix
            impl Add<Mat4x4<$t>> for Mat4x4<$t> {
                type Output = Self;
                fn add(self,other: Self) -> Self::Output {
                    Mat4x4 {
                        x: self.x + other.x,
                        y: self.y + other.y,
                        z: self.z + other.z,
                        w: self.w + other.w,
                    }
                }
            }

            // matrix += matrix
            impl AddAssign<Mat4x4<$t>> for Mat4x4<$t> {
                fn add_assign(&mut self,other: Self) {
                    self.x += other.x;
                    self.y += other.y;
                    self.z += other.z;
                    self.w += other.w;
                }
            }

            // matrix - matrix
            impl Sub<Mat4x4<$t>> for Mat4x4<$t> {
                type Output = Self;
                fn sub(self,other: Self) -> Self::Output {
                    Mat4x4 {
                        x: self.x - other.x,
                        y: self.y - other.y,
                        z: self.z - other.z,
                        w: self.w - other.w,
                    }
                }
            }

            // matrix -= matrix
            impl SubAssign<Mat4x4<$t>> for Mat4x4<$t> {
                fn sub_assign(&mut self,other: Self) {
                    self.x -= other.x;
                    self.y -= other.y;
                    self.z -= other.z;
                    self.w -= other.w;
                }
            }

            // scalar * matrix
            impl Mul<Mat4x4<$t>> for $t {
                type Output = Mat4x4<$t>;
                fn mul(self,other: Mat4x4<$t>) -> Self::Output {
                    Mat4x4 {
                        x: self * other.x,
                        y: self * other.y,
                        z: self * other.z,
                        w: self * other.w,
                    }
                }
            }

            // matrix * scalar
            impl Mul<$t> for Mat4x4<$t> {
                type Output = Mat4x4<$t>;
                fn mul(self,other: $t) -> Self::Output {
                    Mat4x4 {
                        x: self.x * other,
                        y: self.y * other,
                        z: self.z * other,
                        w: self.w * other,
                    }
                }
            }

            // matrix * vector
            impl Mul<Vec4<$t>> for Mat4x4<$t> {
                type Output = Vec4<$t>;
                fn mul(self,other: Vec4<$t>) -> Self::Output {
                    Vec4 {
                        x: self.x.x * other.x + self.x.y * other.y + self.x.z * other.z + self.x.w * other.w,
                        y: self.y.x * other.x + self.y.y * other.y + self.y.z * other.z + self.y.w * other.w,
                        z: self.z.x * other.x + self.z.y * other.y + self.z.z * other.z + self.z.w * other.w,
                        w: self.w.x * other.x + self.w.y * other.y + self.w.z * other.z + self.w.w * other.w,
                    }
                }
            }

            // matrix * matrix
            impl Mul<Mat4x4<$t>> for Mat4x4<$t> {
                type Output = Mat4x4<$t>;
                fn mul(self,other: Mat4x4<$t>) -> Self::Output {
                    Mat4x4 {
                        x: Vec4 {
                            x: self.x.x * other.x.x + self.x.y * other.y.x + self.x.z * other.z.x + self.x.w * other.w.x,
                            y: self.x.x * other.x.y + self.x.y * other.y.y + self.x.z * other.z.y + self.x.w * other.w.y,
                            z: self.x.x * other.x.z + self.x.y * other.y.z + self.x.z * other.z.z + self.x.w * other.w.z,
                            w: self.x.x * other.x.w + self.x.y * other.y.w + self.x.z * other.z.w + self.x.w * other.w.w,
                        },
                        y: Vec4 {
                            x: self.y.x * other.x.x + self.y.y * other.y.x + self.y.z * other.z.x + self.y.w * other.w.x,
                            y: self.y.x * other.x.y + self.y.y * other.y.y + self.y.z * other.z.y + self.y.w * other.w.y,
                            z: self.y.x * other.x.z + self.y.y * other.y.z + self.y.z * other.z.z + self.y.w * other.w.z,
                            w: self.y.x * other.x.w + self.y.y * other.y.w + self.y.z * other.z.w + self.y.w * other.w.w,
                        },
                        z: Vec4 {
                            x: self.z.x * other.x.x + self.z.y * other.y.x + self.z.z * other.z.x + self.z.w * other.w.x,
                            y: self.z.x * other.x.y + self.z.y * other.y.y + self.z.z * other.z.y + self.z.w * other.w.y,
                            z: self.z.x * other.x.z + self.z.y * other.y.z + self.z.z * other.z.z + self.z.w * other.w.z,
                            w: self.z.x * other.x.w + self.z.y * other.y.w + self.z.z * other.z.w + self.z.w * other.w.w,
                        },
                        w: Vec4 {
                            x: self.w.x * other.x.x + self.w.y * other.y.x + self.w.z * other.z.x + self.w.w * other.w.x,
                            y: self.w.x * other.x.y + self.w.y * other.y.y + self.w.z * other.z.y + self.w.w * other.w.y,
                            z: self.w.x * other.x.z + self.w.y * other.y.z + self.w.z * other.z.z + self.w.w * other.w.z,
                            w: self.w.x * other.x.w + self.w.y * other.y.w + self.w.z * other.z.w + self.w.w * other.w.w,
                        }
                    }
                }
            }

            // matrix *= scalar
            impl MulAssign<$t> for Mat4x4<$t> {
                fn mul_assign(&mut self,other: $t) {
                    self.x.x *= other;
                    self.x.y *= other;
                    self.x.z *= other;
                    self.x.w *= other;
                    self.y.x *= other;
                    self.y.y *= other;
                    self.y.z *= other;
                    self.y.w *= other;
                    self.z.x *= other;
                    self.z.y *= other;
                    self.z.z *= other;
                    self.z.w *= other;
                    self.w.x *= other;
                    self.w.y *= other;
                    self.w.z *= other;
                    self.w.w *= other;
                }
            }

            // matrix *= matrix
            impl MulAssign<Mat4x4<$t>> for Mat4x4<$t> {
                fn mul_assign(&mut self,other: Mat4x4<$t>) {
                    let xx = self.x.x * other.x.x + self.x.y * other.y.x + self.x.z * other.z.x + self.x.w * other.w.x;
                    let xy = self.x.x * other.x.y + self.x.y * other.y.y + self.x.z * other.z.y + self.x.w * other.w.y;
                    let xz = self.x.x * other.x.z + self.x.y * other.y.z + self.x.z * other.z.z + self.x.w * other.w.z;
                    let xw = self.x.x * other.x.w + self.x.y * other.y.w + self.x.z * other.z.w + self.x.w * other.w.w;
                    let yx = self.y.x * other.x.x + self.y.y * other.y.x + self.y.z * other.z.x + self.y.w * other.w.x;
                    let yy = self.y.x * other.x.y + self.y.y * other.y.y + self.y.z * other.z.y + self.y.w * other.w.y;
                    let yz = self.y.x * other.x.z + self.y.y * other.y.z + self.y.z * other.z.z + self.y.w * other.w.z;
                    let yw = self.y.x * other.x.w + self.y.y * other.y.w + self.y.z * other.z.w + self.y.w * other.w.w;
                    let zx = self.z.x * other.x.x + self.z.y * other.y.x + self.z.z * other.z.x + self.z.w * other.w.x;
                    let zy = self.z.x * other.x.y + self.z.y * other.y.y + self.z.z * other.z.y + self.z.w * other.w.y;
                    let zz = self.z.x * other.x.z + self.z.y * other.y.z + self.z.z * other.z.z + self.z.w * other.w.z;
                    let zw = self.z.x * other.x.w + self.z.y * other.y.w + self.z.z * other.z.w + self.z.w * other.w.w;
                    let wx = self.w.x * other.x.x + self.w.y * other.y.x + self.w.z * other.z.x + self.w.w * other.w.x;
                    let wy = self.w.x * other.x.y + self.w.y * other.y.y + self.w.z * other.z.y + self.w.w * other.w.y;
                    let wz = self.w.x * other.x.z + self.w.y * other.y.z + self.w.z * other.z.z + self.w.w * other.w.z;
                    let ww = self.w.x * other.x.w + self.w.y * other.y.w + self.w.z * other.z.w + self.w.w * other.w.w;
                    self.x = Vec4 { x: xx,y: xy,z: xz,w: xw, };
                    self.y = Vec4 { x: yx,y: yy,z: yz,w: yw, };
                    self.z = Vec4 { x: zx,y: zy,z: zz,w: zw, };
                    self.w = Vec4 { x: wx,y: wy,z: wz,w: ww, };
                }
            }

            // matrix /= scalar
            impl DivAssign<$t> for Mat4x4<$t> {
                fn div_assign(&mut self,other: $t) {
                    self.x /= other;
                    self.y /= other;
                    self.z /= other;
                    self.w /= other;
                }
            }

            // -matrix
            impl Neg for Mat4x4<$t> {
                type Output = Mat4x4<$t>;
                fn neg(self) -> Self::Output {
                    Mat4x4 {
                        x: -self.x,
                        y: -self.y,
                        z: -self.z,
                        w: -self.w,
                    }
                }
            }

            impl From<Vec3<$t>> for Mat4x4<$t> {
                fn from(value: Vec3<$t>) -> Self {
                    Mat4x4 {
                        x: Vec4 {
                            x: <$t>::ONE,
                            y: <$t>::ZERO,
                            z: <$t>::ZERO,
                            w: <$t>::ZERO,
                        },
                        y: Vec4 {
                            x: <$t>::ZERO,
                            y: <$t>::ONE,
                            z: <$t>::ZERO,
                            w: <$t>::ZERO,
                        },
                        z: Vec4 {
                            x: <$t>::ZERO,
                            y: <$t>::ZERO,
                            z: <$t>::ONE,
                            w: <$t>::ZERO,
                        },
                        w: Vec4 {
                            x: value.x,
                            y: value.y,
                            z: value.z,
                            w: <$t>::ONE,
                        },
                    }
                }
            }

            impl From<Quaternion<$t>> for Mat4x4<$t> {
                fn from(value: Quaternion<$t>) -> Self {
                    let x2 = value.i + value.i;
                    let y2 = value.j + value.j;
                    let z2 = value.k + value.k;
                    let xx2 = value.i * x2;
                    let yy2 = value.j * y2;
                    let zz2 = value.k * z2;
                    let yz2 = value.j * z2;
                    let wx2 = value.r * x2;
                    let xy2 = value.i * y2;
                    let wz2 = value.r * z2;
                    let xz2 = value.i * z2;
                    let wy2 = value.r * y2;
                    Mat4x4 {
                        x: Vec4 {
                            x: <$t>::ONE - yy2 - zz2,
                            y: xy2 + wz2,
                            z: xz2 - wy2,
                            w: <$t>::ZERO,
                        },
                        y: Vec4 {
                            x: xy2 - wz2,
                            y: <$t>::ONE - xx2 - zz2,
                            z: yz2 + wx2,
                            w: <$t>::ZERO,
                        },
                        z: Vec4 {
                            x: xz2 + wy2,
                            y: yz2 - wx2,
                            z: <$t>::ONE - xx2 - yy2,
                            w: <$t>::ZERO,
                        },
                        w: Vec4 {
                            x: <$t>::ZERO,
                            y: <$t>::ZERO,
                            z: <$t>::ZERO,
                            w: <$t>::ONE,
                        },
                    }
                }
            }
        )+
    }
}

mat4x4_impl! { isize i8 i16 i32 i64 i128 f32 f64 }

macro_rules! mat4x4_real_impl {
    ($($t:ty)+) => {
        $(
            impl Mat4x4<$t> {
                pub fn inv(self) -> Self {
                    let a = self.x.x;
                    let e = self.x.y;
                    let i = self.x.z;
                    let m = self.x.w;
                    let b = self.y.x;
                    let f = self.y.y;
                    let j = self.y.z;
                    let n = self.y.w;
                    let c = self.z.x;
                    let g = self.z.y;
                    let k = self.z.z;
                    let o = self.z.w;
                    let d = self.w.x;
                    let h = self.w.y;
                    let l = self.w.z;
                    let p = self.w.w;
                    let kplo = k * p - l * o;
                    let jpln = j * p - l * n;
                    let jokn = j * o - k * n;
                    let iplm = i * p - l * m;
                    let iokm = i * o - k * m;
                    let injm = i * n - j * m;
                    let aa = f * kplo - g * jpln + k * iokm;
                    let ab = e * kplo - g * iplm + h * iokm;
                    let ac = e * jpln - f * iplm + h * injm;
                    let ad = e * jokn - f * iokm + g * injm;
                    let det = a * aa - b * ab + c * ac - d * ad;
                    if det == 0.0 {
                        return self;
                    }
                    let ae = e * kplo - c * jpln + d * jokn;
                    let af = a * kplo - c * iplm + d * iokm;
                    let ag = a * jpln - b * iplm + d * injm;
                    let ah = a * jokn - b * iokm + c * injm;
                    let chdg = c * h - d * g;
                    let bhdf = b * h - d * f;
                    let bgcf = b * g - c * f;
                    let ahde = a * h - d * e;
                    let agce = a * g - c * e;
                    let afbe = a * f - b * e;
                    let ai = n * chdg - o * bhdf + p * bgcf;
                    let aj = m * chdg - o * ahde + p * agce;
                    let ak = m * bhdf - n * ahde + p * afbe;
                    let al = m * bgcf - n * agce + o * afbe;
                    let am = j * chdg - k * bhdf + l * bgcf;
                    let an = i * chdg - k * ahde + l * agce;
                    let ao = i * bhdf - j * ahde + l * afbe;
                    let ap = i * bgcf - j * agce + k * afbe;
                    Mat4x4 {
                        x: Vec4 { x: aa,y: -ae,z: ai,w: -am, },
                        y: Vec4 { x: -ab,y: af,z: -aj,w: an, },
                        z: Vec4 { x: ac,y: -ag,z: ak,w: -ao, },
                        w: Vec4 { x: -ad,y: ah,z: -al,w: ap, },
                    } / det
                }
            }

            // scalar / matrix
            impl Div<Mat4x4<$t>> for $t {
                type Output = Mat4x4<$t>;
                fn div(self,other: Mat4x4<$t>) -> Self::Output {
                    self * other.inv()
                }
            }

            // matrix / scalar
            impl Div<$t> for Mat4x4<$t> {
                type Output = Mat4x4<$t>;
                fn div(self,other: $t) -> Self::Output {
                    Mat4x4 {
                        x: self.x / other,
                        y: self.y / other,
                        z: self.z / other,
                        w: self.w / other,
                    }
                }
            }

            // matrix / matrix
            impl Div<Mat4x4<$t>> for Mat4x4<$t> {
                type Output = Mat4x4<$t>;
                fn div(self,other: Mat4x4<$t>) -> Self::Output {
                    self * other.inv()
                }
            }

            // matrix /= matrix
            impl DivAssign<Mat4x4<$t>> for Mat4x4<$t> {
                fn div_assign(&mut self,other: Mat4x4<$t>) {
                    *self *= other.inv()
                }
            }
        )+
    }
}

mat4x4_real_impl! { f32 f64 }
