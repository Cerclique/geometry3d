use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::point3d::Point3D;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3D {
        Vector3D { x, y, z }
    }

    pub fn from_point3d(p: Point3D) -> Vector3D {
        Vector3D::new(p.x(), p.y(), p.z())
    }

    pub fn zeroes() -> Vector3D {
        Vector3D::new(0.0, 0.0, 0.0)
    }

    pub fn ones() -> Vector3D {
        Vector3D::new(1.0, 1.0, 1.0)
    }
}

impl Vector3D {
    pub fn x(self) -> f64 {
        self.x
    }

    pub fn y(self) -> f64 {
        self.y
    }
    pub fn z(self) -> f64 {
        self.z
    }
    
    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn unit(self) -> Vector3D {
        self / self.length()
    }

    pub fn dot(self, rhs: Vector3D) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(self, rhs: Vector3D) -> Vector3D {
        Vector3D::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}

impl Add<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn add(self, rhs: Self) -> Vector3D {
        Vector3D::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign<Vector3D> for Vector3D {
    fn add_assign(&mut self, rhs: Vector3D) {
        *self = Vector3D::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z);
    }
}

impl Sub<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn sub(self, rhs: Vector3D) -> Vector3D {
        Vector3D::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl SubAssign<Vector3D> for Vector3D {
    fn sub_assign(&mut self, rhs: Vector3D) {
        *self = Vector3D::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z);
    }
}

impl Mul<f64> for Vector3D {
    type Output = Vector3D;

    fn mul(self, rhs: f64) -> Vector3D {
        Vector3D::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl MulAssign<f64> for Vector3D {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Vector3D::new(self.x * rhs, self.y * rhs, self.z * rhs);
    }
}

impl Div<f64> for Vector3D {
    type Output = Vector3D;
    fn div(self, rhs: f64) -> Vector3D {
        Vector3D::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl DivAssign<f64> for Vector3D {
    fn div_assign(&mut self, rhs: f64) {
        *self = Vector3D::new(self.x / rhs, self.y / rhs, self.z / rhs);
    }
}

impl Neg for Vector3D {
    type Output = Vector3D;

    fn neg(self) -> Vector3D {
        Vector3D::new(-self.x, -self.y, -self.z)
    }
}

#[cfg(test)]
mod creation_test {
    use super::*;

    #[test]
    fn test_new() {
        let v = Vector3D::new(1.0, 2.0, 3.0);

        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_from_point3d() {
        let lhs = Vector3D::from_point3d(Point3D::new(1.0, 2.0, 3.0));
        
        assert_eq!(lhs.x, 1.0);
        assert_eq!(lhs.y, 2.0);
        assert_eq!(lhs.z, 3.0);
    }

    #[test]
    fn test_zeroes() {
        let v = Vector3D::zeroes();

        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
        assert_eq!(v.z, 0.0);
    }

    #[test]
    fn test_ones() {
        let v = Vector3D::ones();

        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 1.0);
        assert_eq!(v.z, 1.0);
    }
}

#[cfg(test)]
mod ops_test {
    use super::*;

    #[test]
    fn test_ops_add() {
        let lhs = Vector3D::new(1.0, 2.0, 3.0);

        let res = lhs + Vector3D::new(4.0, 5.0, 6.0);

        assert_eq!(res.x, 5.0);
        assert_eq!(res.y, 7.0);
        assert_eq!(res.z, 9.0);
    }

    #[test]
    fn test_ops_add_assign() {
        let mut lhs = Vector3D::new(1.0, 2.0, 3.0);
        lhs += Vector3D::new(4.0, 5.0, 6.0);

        assert_eq!(lhs.x, 5.0);
        assert_eq!(lhs.y, 7.0);
        assert_eq!(lhs.z, 9.0);
    }

    #[test]
    fn test_ops_sub() {
        let lhs = Vector3D::new(9.0, 8.0, 7.0);

        let res = lhs - Vector3D::new(1.0, 2.0, 3.0);

        assert_eq!(res.x, 8.0);
        assert_eq!(res.y, 6.0);
        assert_eq!(res.z, 4.0);
    }

    #[test]
    fn test_ops_sub_assign() {
        let mut lhs = Vector3D::new(1.0, 2.0, 3.0);

        lhs -= Vector3D::new(9.0, 8.0, 7.0);

        assert_eq!(lhs.x, -8.0);
        assert_eq!(lhs.y, -6.0);
        assert_eq!(lhs.z, -4.0);
    }

    #[test]
    fn test_ops_mul() {
        let lhs = Vector3D::new(1.0, 2.0, 3.0);

        let res = lhs * 2.0;
        assert_eq!(res.x, 2.0);
        assert_eq!(res.y, 4.0);
        assert_eq!(res.z, 6.0);
    }

    #[test]
    fn test_ops_mul_assign() {
        let mut lhs = Vector3D::new(1.0, 2.0, 3.0);

        lhs *= 2.0;

        assert_eq!(lhs.x, 2.0);
        assert_eq!(lhs.y, 4.0);
        assert_eq!(lhs.z, 6.0);
    }

    #[test]
    fn test_ops_div() {
        let lhs = Vector3D::new(2.0, 4.0, 6.0);

        let res = lhs / 2.0;

        assert_eq!(res.x, 1.0);
        assert_eq!(res.y, 2.0);
        assert_eq!(res.z, 3.0);
    }

    #[test]
    fn test_ops_div_assign() {
        let mut lhs = Vector3D::new(2.0, 4.0, 6.0);

        lhs /= 2.0;

        assert_eq!(lhs.x, 1.0);
        assert_eq!(lhs.y, 2.0);
        assert_eq!(lhs.z, 3.0);
    }

    #[test]
    fn test_ops_neg() {
        let lhs = Vector3D::new(1.0, 2.0, 3.0);

        let res = -lhs;

        assert_eq!(res.x, -1.0);
        assert_eq!(res.y, -2.0);
        assert_eq!(res.z, -3.0);
    }
}

#[cfg(test)]
mod utility_test {
    use super::*;

    #[test]
    fn test_getters() {
        let lhs = Vector3D::new(1.0, 2.0, 3.0);

        assert_eq!(lhs.x(), 1.0);
        assert_eq!(lhs.y(), 2.0);
        assert_eq!(lhs.z(), 3.0);
    }

    #[test]
    fn test_length() {
        let lhs = Vector3D::new(4.0, 4.0, 2.0);
        assert_eq!(lhs.length(), 6.0);
    }

    #[test]
    fn test_length_squared() {
        let lhs = Vector3D::new(4.0, 4.0, 2.0);
        assert_eq!(lhs.length_squared(), 36.0);    
    }

    #[test]
    fn test_unit() {
        let lhs = Vector3D::new(1.0, 2.0, 3.0);
        assert_eq!(lhs.unit().length(), 1.0)
    }

    #[test]
    fn test_dot() {
        let lhs = Vector3D::new(1.0, 2.0, 3.0);
        let rhs = Vector3D::new(4.0, 5.0, 6.0);

        let res = lhs.dot(rhs);

        assert_eq!(res, 32.0)
    }

    #[test]
    fn test_cross() {
        let lhs = Vector3D::new(1.0, 2.0, 3.0);
        let rhs = Vector3D::new(4.0, 5.0, 6.0);

        let res = lhs.cross(rhs);

        assert_eq!(res, Vector3D::new(-3.0, 6.0, -3.0));
    }
}
