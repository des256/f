use {
    crate::*,
    std::ops::{
        Mul,
        MulAssign,
    },
};

#[derive(Copy,Clone,Debug)]
pub struct Pose<T> {
    pub p: Vec3<T>,
    pub o: Quaternion<T>,
}

macro_rules! pose_impl {
    ($($t:ty)+) => {
        $(
            impl Pose<$t> {
                pub fn inv(self) -> Pose<$t> {
                    let o = self.o.inv();
                    Pose {
                        p: o * -self.p,
                        o: o,
                    }
                }
            }

            impl One for Pose<$t> {
                const ONE: Self = Pose { p: Vec3::<$t>::ZERO,o: Quaternion::<$t>::ONE, };
            }

            // pose * vector
            impl Mul<Vec3<$t>> for Pose<$t> {
                type Output = Vec3<$t>;
                fn mul(self,other: Vec3<$t>) -> Vec3<$t> {
                    self.o * other + self.p
                }
            }

            // pose * pose
            impl Mul<Pose<$t>> for Pose<$t> {
                type Output = Pose<$t>;
                fn mul(self,other: Pose<$t>) -> Pose<$t> {
                    Pose {
                        p: self.p + self.o * other.p,
                        o: self.o * other.o,
                    }
                }
            }

            // pose *= pose
            impl MulAssign<Pose<$t>> for Pose<$t> {
                fn mul_assign(&mut self,other: Pose<$t>) {
                    self.p += self.o * other.p;
                    self.o *= other.o;
                }
            }
        )+
    }
}

pose_impl! { f32 f64 }