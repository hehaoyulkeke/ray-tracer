use std::ops::{Index, IndexMut, Neg, Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use std::cmp::PartialEq;
use std::fmt::Display;


#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub e: [f64; 3]
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Vec3{e: [e0, e1, e2]}
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }
    pub fn r(&self) -> f64 {
        self.e[0]
    }
    pub fn g(&self) -> f64 {
        self.e[1]
    }
    pub fn b(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2])
    }
    pub fn squared_length(&self) -> f64 {
        self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2]
    }

    pub fn make_unit_vec(v: Vec3) -> Vec3 {
        v / v.length()
    }

    pub fn dot(v1: Vec3, v2: Vec3) -> f64 {
        v1.e[0]*v2.e[0] + v1.e[1]*v2.e[1] + v1.e[2]*v2.e[2]
    }

    pub fn cross(v1 : Vec3, v2 : Vec3) -> Vec3 {
        Vec3::new(v1.e[1] * v2.e[2] - v1.e[2]*v2.e[1],
            -(v1.e[0]*v2.e[2] - v1.e[2]*v2.e[0]),
            v1.e[0]*v2.e[1] - v1.e[1] * v2.e[0])
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
        f64::abs(self.e[0]-other.e[0])<core::f64::EPSILON &&
        f64::abs(self.e[1]-other.e[1])<core::f64::EPSILON &&
        f64::abs(self.e[2]-other.e[2])<core::f64::EPSILON
    }

    fn ne(&self, other: &Vec3) -> bool {
        !self.eq(other)
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.e[0], self.e[1], self.e[2])
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        &mut self.e[index]
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.e[0]+other.e[0], self.e[1]+other.e[1], self.e[2]+other.e[2])
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.e[0]-other.e[0], self.e[1]-other.e[1], self.e[2]-other.e[2])
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        Vec3::new(self.e[0]*other, self.e[1]*other, self.e[2]*other)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self.e[0]*other.e[0], self.e[1]*other.e[1], self.e[2]*other.e[2])
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self * other.e[0],
            self * other.e[1],
            self * other.e[2])
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        Vec3::new(self.e[0]/other, self.e[1]/other, self.e[2]/other)
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3::new(self.e[0]/other.e[0], self.e[1]/other.e[1], self.e[2]/other.e[2])
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

impl AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, other: f64) {
        self.e[0] += other;
        self.e[1] += other;
        self.e[2] += other;
    }
}

impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        self.e[0] -= other.e[0];
        self.e[1] -= other.e[1];
        self.e[2] -= other.e[2];
    }
}

impl SubAssign<f64> for Vec3 {
    fn sub_assign(&mut self, other: f64) {
        self.e[0] -= other;
        self.e[1] -= other;
        self.e[2] -= other;
    }
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        self.e[0] *= other.e[0];
        self.e[1] *= other.e[1];
        self.e[2] *= other.e[2];
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        self.e[0] *= other;
        self.e[1] *= other;
        self.e[2] *= other;
    }
}

impl DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, other: Vec3) {
        self.e[0] /= other.e[0];
        self.e[1] /= other.e[1];
        self.e[2] /= other.e[2];
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        self.e[0] /= other;
        self.e[1] /= other;
        self.e[2] /= other;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq() {
        let a = Vec3::new(1.0,2.0, 3.0000001);
        let b = Vec3::new(1.0,2.0, 3.0);
        assert_eq!(a, b);
    }

    #[test]
    fn test_ne() {
        let a = Vec3::new(1.0,2.0, 3.1);
        let b = Vec3::new(1.0,2.0, 3.0);
        assert_ne!(a, b);
    }

    #[test]
    fn test_index() {
        let a = Vec3::new(1.0,2.0, 3.1);
        assert_eq!(a[0], 1.0);
        assert_eq!(a[1], 2.0);
        assert_eq!(a[2], 3.1);
    }

    #[test]
    fn test_index_assign() {
        let mut a = Vec3::new(1.0,2.0, 3.1);
        a[0] = 2.0;
        assert_eq!(a[0], 2.0);
        a[1] = 3.0;
        assert_eq!(a[1], 3.0);
        a[2] = 4.0;
        assert_eq!(a[2], 4.0);
    }

    #[test]
    fn test_neg() {
        let a = Vec3::new(1.0,2.0, 3.1);
        let answer = Vec3::new(-1.0,-2.0, -3.1);
        assert_eq!(-a, answer);
    }

    #[test]
    fn test_add() {
        let a = Vec3::new(1.0,2.0, 3.1);
        let b = Vec3::new(1.0,2.0, 3.1);
        let answer = Vec3::new(2.0,4.0, 6.2);
        assert_eq!(a+b, answer);
    }

    #[test]
    fn test_sub() {
        let a = Vec3::new(1.0,2.0, 3.1);
        let b = Vec3::new(1.0,2.0, 3.1);
        let answer = Vec3::new(0.0,0.0, 0.0);
        assert_eq!(a-b, answer);
    }

    #[test]
    fn test_mul() {
        let a = Vec3::new(1.0,2.0, 3.1);
        let b = Vec3::new(1.0,2.0, 3.1);
        let answer = Vec3::new(1.0,4.0, 9.61);
        assert_eq!(a*b, answer);
    }

    #[test]
    fn test_div() {
        let a = Vec3::new(4.0,9.0, 12.0);
        let b = Vec3::new(2.0,3.0, 4.0);
        let answer = Vec3::new(2.0,3.0, 3.0);
        assert_eq!(a/b, answer);
    }

    #[test]
    fn test_add_assign() {
        let mut a = Vec3::new(1.0,2.0, 3.1);
        let b = Vec3::new(1.0,2.0, 3.1);
        let answer = Vec3::new(2.0,4.0, 6.2);
        a += b;
        assert_eq!(a, answer);
    }

    #[test]
    fn test_sub_assign() {
        let mut a = Vec3::new(1.0,2.0, 3.1);
        let b = Vec3::new(1.0,2.0, 3.1);
        let answer = Vec3::new(0.0,0.0, 0.0);
        a -= b;
        assert_eq!(a, answer);
    }

    #[test]
    fn test_make_unit_vector() {
        let a = Vec3::new(1.0,2.0, 3.0);
        let u = Vec3::make_unit_vec(a);
        assert!(f64::abs(1.0 - u.length()) < core::f64::EPSILON);
    }
}