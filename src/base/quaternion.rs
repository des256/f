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

/// Quaternion template.
/// 
/// A quaternion is a way to represent 3D orientation and allow for correct rotations without gimbal lock. The concept is
/// similar to [`Complex`], where imaginary numbers are combined with scalars. The [`Quaternion`] adds three separate
/// imaginary numbers, allowing rotations around 3 orthogonal axes.
/// 
/// Can use any scalar underneath (typically [`f32`] or [`f64`]), as well as [`Rational`] and [`Fixed`] types.
#[derive(Copy,Clone,Debug)]
pub struct Quaternion<T> {
    pub r: T,
    pub i: T,
    pub j: T,
    pub k: T,
}

macro_rules! quaternion_impl {
    ($($t:ty)+) => {
        $(
            impl Quaternion<$t> {
                pub fn conj(&self) -> Self {
                    Quaternion {
                        r: self.r,
                        i: -self.i,
                        j: -self.j,
                        k: -self.k,
                    }
                }

                pub fn inv(&self) -> Self {
                    let f = self.r * self.r + self.i * self.i + self.j * self.j + self.k * self.k;
                    Quaternion {
                        r: self.r / f,
                        i: -self.i / f,
                        j: -self.j / f,
                        k: -self.k / f,
                    }
                }
            }

            impl Zero for Quaternion<$t> {
                const ZERO: Self = Quaternion { r: <$t>::ZERO,i: <$t>::ZERO,j: <$t>::ZERO,k: <$t>::ZERO, };
            }

            impl One for Quaternion<$t> {
                const ONE: Self = Quaternion { r: <$t>::ONE,i: <$t>::ZERO,j: <$t>::ZERO,k: <$t>::ZERO, };
            }

            impl Display for Quaternion<$t> {
                fn fmt(&self,f: &mut Formatter) -> Result {
                    let si = if self.i < <$t>::ZERO {
                        format!("{}i",self.i)
                    } else {
                        format!("+{}i",self.i)
                    };
                    let sj = if self.j < <$t>::ZERO {
                        format!("{}j",self.j)
                    } else {
                        format!("+{}j",self.j)
                    };
                    let sk = if self.k < <$t>::ZERO {
                        format!("{}k",self.k)
                    } else {
                        format!("+{}k",self.k)
                    };
                    write!(f,"{}{}{}{}",self.r,si,sj,sk)        
                }            
            }

            impl PartialEq<Quaternion<$t>> for Quaternion<$t> {
                fn eq(&self,other: &Quaternion<$t>) -> bool {
                    (self.r == other.r) && (self.i == other.i) && (self.j == other.j) && (self.k == other.k)
                }
            }

            // scalar + quaternion
            impl Add<Quaternion<$t>> for $t {
                type Output = Quaternion<$t>;
                fn add(self,other: Quaternion<$t>) -> Self::Output {
                    Quaternion {
                        r: self + other.r,
                        i: other.i,
                        j: other.j,
                        k: other.k,
                    }
                }
            }

            // complex + quaternion
            impl Add<Quaternion<$t>> for Complex<$t> {
                type Output = Quaternion<$t>;
                fn add(self,other: Quaternion<$t>) -> Self::Output {
                    Quaternion {
                        r: self.r + other.r,
                        i: self.i + other.i,
                        j: other.j,
                        k: other.k,
                    }
                }
            }

            // quaternion + scalar
            impl Add<$t> for Quaternion<$t> {
                type Output = Quaternion<$t>;
                fn add(self,other: $t) -> Self::Output {
                    Quaternion {
                        r: self.r + other,
                        i: self.i,
                        j: self.j,
                        k: self.k,
                    }
                }
            }

            // quaternion + complex
            impl Add<Complex<$t>> for Quaternion<$t> {
                type Output = Quaternion<$t>;
                fn add(self,other: Complex<$t>) -> Self::Output {
                    Quaternion {
                        r: self.r + other.r,
                        i: self.i + other.i,
                        j: self.j,
                        k: self.k,
                    }
                }
            }

            // quaternion + quaternion
            impl Add<Quaternion<$t>> for Quaternion<$t> {
                type Output = Quaternion<$t>;
                fn add(self,other: Quaternion<$t>) -> Self::Output {
                    Quaternion {
                        r: self.r + other.r,
                        i: self.i + other.i,
                        j: self.j + other.j,
                        k: self.k + other.k,
                    }
                }
            }

            // quaternion += scalar
            impl AddAssign<$t> for Quaternion<$t> {
                fn add_assign(&mut self,other: $t) {
                    self.r += other;
                }
            }

            // quaternion += complex
            impl AddAssign<Complex<$t>> for Quaternion<$t> {
                fn add_assign(&mut self,other: Complex<$t>) {
                    self.r += other.r;
                    self.i += other.i;
                }
            }

            // quaternion += quaternion
            impl AddAssign<Quaternion<$t>> for Quaternion<$t> {
                fn add_assign(&mut self,other: Quaternion<$t>) {
                    self.r += other.r;
                    self.i += other.i;
                    self.j += other.j;
                    self.k += other.k;
                }
            }

            // scalar - quaternion
            impl Sub<Quaternion<$t>> for $t {
                type Output = Quaternion<$t>;
                fn sub(self,other: Quaternion<$t>) -> Self::Output {
                    Quaternion {
                        r: self - other.r,
                        i: -other.i,
                        j: -other.j,
                        k: -other.k,
                    }
                }
            }

            // complex - quaternion
            impl Sub<Quaternion<$t>> for Complex<$t> {
                type Output = Quaternion<$t>;
                fn sub(self,other: Quaternion<$t>) -> Self::Output {
                    Quaternion {
                        r: self.r - other.r,
                        i: self.i - other.i,
                        j: -other.j,
                        k: -other.k,
                    }
                }
            }

            // quaternion - scalar
            impl Sub<$t> for Quaternion<$t> {
                type Output = Quaternion<$t>;
                fn sub(self,other: $t) -> Self::Output {
                    Quaternion {
                        r: self.r - other,
                        i: self.i,
                        j: self.j,
                        k: self.k,
                    }
                }
            }

            // quaternion - complex
            impl Sub<Complex<$t>> for Quaternion<$t> {
                type Output = Quaternion<$t>;
                fn sub(self,other: Complex<$t>) -> Self::Output {
                    Quaternion {
                        r: self.r - other.r,
                        i: self.i - other.i,
                        j: self.j,
                        k: self.k,
                    }
                }
            }

            // quaternion - quaternion
            impl Sub<Quaternion<$t>> for Quaternion<$t> {
                type Output = Quaternion<$t>;
                fn sub(self,other: Quaternion<$t>) -> Self::Output {
                    Quaternion {
                        r: self.r - other.r,
                        i: self.i - other.i,
                        j: self.j - other.j,
                        k: self.k - other.k,
                    }
                }
            }

            // quaternion -= scalar
            impl SubAssign<$t> for Quaternion<$t> {
                fn sub_assign(&mut self,other: $t) {
                    self.r -= other;
                }
            }

            // quaternion -= complex
            impl SubAssign<Complex<$t>> for Quaternion<$t> {
                fn sub_assign(&mut self,other: Complex<$t>) {
                    self.r -= other.r;
                    self.i -= other.i;
                }
            }

            // quaternion -= quaternion
            impl SubAssign<Quaternion<$t>> for Quaternion<$t> {
                fn sub_assign(&mut self,other: Quaternion<$t>) {
                    self.r -= other.r;
                    self.i -= other.i;
                    self.j -= other.j;
                    self.k -= other.k;
                }
            }

            // scalar * quaternion
            impl Mul<Quaternion<$t>> for $t {
                type Output = Quaternion<$t>;
                fn mul(self,other: Quaternion<$t>) -> Self::Output {
                    Quaternion {
                        r: self * other.r,
                        i: self * other.i,
                        j: self * other.j,
                        k: self * other.k,
                    }
                }
            }

            // complex * quaternion
            impl Mul<Quaternion<$t>> for Complex<$t> {
                type Output = Quaternion<$t>;
                fn mul(self,other: Quaternion<$t>) -> Self::Output {
                    Quaternion {
                        r: self.r * other.r - self.i * other.i,
                        i: self.r * other.i + self.i * other.r,
                        j: self.r * other.j - self.i * other.k,
                        k: self.r * other.k + self.i * other.j,
                    }
                }
            }

            // quaternion * scalar
            impl Mul<$t> for Quaternion<$t> {
                type Output = Quaternion<$t>;
                fn mul(self,other: $t) -> Self::Output {
                    Quaternion {
                        r: self.r * other,
                        i: self.i * other,
                        j: self.j * other,
                        k: self.k * other,
                    }
                }
            }

            // quaternion * complex
            impl Mul<Complex<$t>> for Quaternion<$t> {
                type Output = Quaternion<$t>;
                fn mul(self,other: Complex<$t>) -> Self::Output {
                    Quaternion {
                        r: self.r * other.r - self.i * other.i,
                        i: self.r * other.i + self.i * other.r,
                        j: self.j * other.r + self.k * other.i,
                        k: -self.j * other.i + self.k * other.r,
                    }
                }
            }

            // quaternion * vector
            /*impl Mul<Vec3<$t>> for Quaternion<$t> {
                type Output = Vec3<$t>;
                fn mul(self,other: Vec3<$t>) {
                    let aq = self * Quaternion { r: <$t>::ZERO,i: other.x,j: other.y,k: other.z, };
                    let aqa_inv = aq * aq.inv();
                    Vec3 {
                        x: aqa_inv.i,
                        y: aqa_inv.j,
                        z: aqa_inv.k,
                    }
                }
            }*/

            // quaternion * quaternion
            impl Mul<Quaternion<$t>> for Quaternion<$t> {
                type Output = Quaternion<$t>;
                fn mul(self,other: Quaternion<$t>) -> Self::Output {
                    Quaternion {
                        r: self.r * other.r - self.i * other.i - self.j * other.j - self.k * other.k,
                        i: self.r * other.i + self.i * other.r + self.j * other.k - self.k * other.j,
                        j: self.r * other.j - self.i * other.k + self.j * other.r + self.k * other.i,
                        k: self.r * other.k + self.i * other.j - self.j * other.i + self.k * other.r,
                    }
                }
            }

            // quaternion * vector
            impl Mul<Vec3<$t>> for Quaternion<$t> {
                type Output = Vec3<$t>;
                fn mul(self,other: Vec3<$t>) -> Self::Output {
                    let rr = self.r * self.r;
                    let ri = self.r * self.i;
                    let rj = self.r * self.j;
                    let rk = self.r * self.k;
                    let ii = self.i * self.i;
                    let ij = self.i * self.j;
                    let ik = self.i * self.k;
                    let jj = self.j * self.j;
                    let jk = self.j * self.k;
                    let kk = self.k * self.k;
                    let ijprk = ij + rk;
                    let ijprk2 = ijprk + ijprk;
                    let ijmrk = ij - rk;
                    let ijmrk2 = ijmrk + ijmrk;
                    let jkpri = jk + ri;
                    let jkpri2 = jkpri + jkpri;
                    let jkmri = jk - ri;
                    let jkmri2 = jkmri + jkmri;
                    let ikprj = ik + rj;
                    let ikprj2 = ikprj + ikprj;
                    let ikmrj = ik - rj;
                    let ikmrj2 = ikmrj + ikmrj;
                    Vec3 {
                        x: (rr + ii - jj - kk) * other.x + ijmrk2 * other.y + ikprj2 * other.z,
                        y: (rr - ii + jj - kk) * other.y + jkmri2 * other.z + ijprk2 * other.x,
                        z: (rr - ii - jj + kk) * other.z + ikmrj2 * other.x + jkpri2 * other.y,
                    }            
                }
            }

            // quaternion *= scalar
            impl MulAssign<$t> for Quaternion<$t> {
                fn mul_assign(&mut self,other: $t) {
                    self.r *= other;
                    self.i *= other;
                    self.j *= other;
                    self.k *= other;
                }
            }

            // quaternion *= complex
            impl MulAssign<Complex<$t>> for Quaternion<$t> {
                fn mul_assign(&mut self,other: Complex<$t>) {
                    let r = self.r * other.r - self.i * other.i;
                    let i = self.r * other.i + self.i * other.r;
                    let j = self.j * other.r + self.k * other.i;
                    let k = -self.j * other.i + self.k * other.r;
                    self.r = r;
                    self.i = i;
                    self.j = j;
                    self.k = k;
                }
            }

            // quaternion *= quaternion
            impl MulAssign<Quaternion<$t>> for Quaternion<$t> {
                fn mul_assign(&mut self,other: Quaternion<$t>) {
                    let r = self.r * other.r - self.i * other.i - self.j * other.j - self.k * other.k;
                    let i = self.r * other.i + self.i * other.r + self.j * other.k - self.k * other.j;
                    let j = self.r * other.j - self.i * other.k + self.j * other.r + self.k * other.i;
                    let k = self.r * other.k + self.i * other.j - self.j * other.i + self.k * other.r;
                    self.r = r;
                    self.i = i;
                    self.j = j;
                    self.k = k;            
                }
            }

            // scalar / quaternion
            impl Div<Quaternion<$t>> for $t {
                type Output = Quaternion<$t>;
                fn div(self,other: Quaternion<$t>) -> Self::Output {
                    let f = other.r * other.r + other.i * other.i + other.j * other.j + other.k * other.k;
                    Quaternion {
                        r: (self * other.r) / f,
                        i: (-self * other.i) / f,
                        j: (-self * other.j) / f,
                        k: (-self * other.k) / f,
                    }
                }
            }

            // complex / quaternion
            impl Div<Quaternion<$t>> for Complex<$t> {
                type Output = Quaternion<$t>;
                fn div(self,other: Quaternion<$t>) -> Self::Output {
                    let f = other.r * other.r + other.i * other.i + other.j * other.j + other.k * other.k;
                    Quaternion {
                        r: (self.r * other.r + self.i * other.i) / f,
                        i: (self.i * other.r - self.r * other.i) / f,
                        j: (-self.r * other.j + self.i * other.k) / f,
                        k: (-self.r * other.k - self.i * other.j) / f,
                    }
                }
            }

            // quaternion / scalar
            impl Div<$t> for Quaternion<$t> {
                type Output = Quaternion<$t>;
                fn div(self,other: $t) -> Self::Output {
                    Quaternion {
                        r: self.r / other,
                        i: self.i / other,
                        j: self.j / other,
                        k: self.k / other,
                    }
                }
            }

            // quaternion / complex
            impl Div<Complex<$t>> for Quaternion<$t> {
                type Output = Quaternion<$t>;
                fn div(self,other: Complex<$t>) -> Self::Output {
                    let f = other.r * other.r + other.i * other.i;
                    Quaternion {
                        r: (self.r * other.r + self.i * other.i) / f,
                        i: (self.i * other.r - self.r * other.i) / f,
                        j: (self.j * other.r - self.k * other.i) / f,
                        k: (self.k * other.r + self.j * other.i) / f,
                    }
                }
            }

            // quaternion / quaternion
            impl Div<Quaternion<$t>> for Quaternion<$t> {
                type Output = Quaternion<$t>;
                fn div(self,other: Quaternion<$t>) -> Self::Output {
                    let f = other.r * other.r + other.i * other.i + other.j * other.j + other.k * other.k;
                    Quaternion {
                        r: (self.r * other.r + self.i * other.i + self.j * other.j + self.k * other.k) / f,
                        i: (self.i * other.r - self.j * other.k + self.k * other.j - self.r * other.i) / f,
                        j: (self.j * other.r - self.k * other.i - self.r * other.j + self.i * other.k) / f,
                        k: (self.k * other.r - self.r * other.k - self.i * other.j + self.j * other.i) / f,
                    }
                }
            }

            // quaternion /= scalar
            impl DivAssign<$t> for Quaternion<$t> {
                fn div_assign(&mut self,other: $t) {
                    self.r *= other;
                    self.i *= other;
                    self.j *= other;
                    self.k *= other;
                }
            }

            // quaternion /= complex
            impl DivAssign<Complex<$t>> for Quaternion<$t> {
                fn div_assign(&mut self,other: Complex<$t>) {
                    let r = self.r * other.r - self.i * other.i;
                    let i = self.r * other.i + self.i * other.r;
                    let j = self.j * other.r + self.k * other.i;
                    let k = -self.j * other.i + self.k * other.r;
                    self.r = r;
                    self.i = i;
                    self.j = j;
                    self.k = k;
                }
            }

            // quaternion /= quaternion
            impl DivAssign<Quaternion<$t>> for Quaternion<$t> {
                fn div_assign(&mut self,other: Quaternion<$t>) {
                    let r = self.r * other.r - self.i * other.i - self.j * other.j - self.k * other.k;
                    let i = self.r * other.i + self.i * other.r + self.j * other.k - self.k * other.j;
                    let j = self.r * other.j - self.i * other.k + self.j * other.r + self.k * other.i;
                    let k = self.r * other.k + self.i * other.j - self.j * other.i + self.k * other.r;
                    self.r = r;
                    self.i = i;
                    self.j = j;
                    self.k = k;            
                }
            }

            // -quaternion
            impl Neg for Quaternion<$t> {
                type Output = Self;
                fn neg(self) -> Self::Output {
                    Quaternion {
                        r: -self.r,
                        i: -self.i,
                        j: -self.j,
                        k: -self.k,
                    }
                }
            }
        )+
    }
}

quaternion_impl! { f32 f64 }
