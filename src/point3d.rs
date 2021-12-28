use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Point3D {
        Point3D { x, y, z }
    }

    pub fn zeroes() -> Point3D {
        Point3D::new(0.0, 0.0, 0.0)
    }

    pub fn ones() -> Point3D {
        Point3D::new(1.0, 1.0, 1.0)
    }
}

impl Point3D {
    pub fn x(self) -> f64 {
        self.x
    }

    pub fn y(self) -> f64 {
        self.y
    }

    pub fn z(self) -> f64 {
        self.z
    }
}

impl Add<Point3D> for Point3D {
    type Output = Point3D;

    fn add(self, rhs: Self) -> Point3D {
        Point3D::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign<Point3D> for Point3D {
    fn add_assign(&mut self, rhs: Point3D) {
        *self = Point3D::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z);
    }
}

impl Sub<Point3D> for Point3D {
    type Output = Point3D;

    fn sub(self, rhs: Point3D) -> Point3D {
        Point3D::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl SubAssign<Point3D> for Point3D {
    fn sub_assign(&mut self, rhs: Point3D) {
        *self = Point3D::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z);
    }
}

impl Mul<f64> for Point3D {
    type Output = Point3D;

    fn mul(self, rhs: f64) -> Point3D {
        Point3D::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl MulAssign<f64> for Point3D {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Point3D::new(self.x * rhs, self.y * rhs, self.z * rhs);
    }
}

impl Div<f64> for Point3D {
    type Output = Point3D;
    fn div(self, rhs: f64) -> Point3D {
        Point3D::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl DivAssign<f64> for Point3D {
    fn div_assign(&mut self, rhs: f64) {
        *self = Point3D::new(self.x / rhs, self.y / rhs, self.z / rhs);
    }
}

impl Neg for Point3D {
    type Output = Point3D;

    fn neg(self) -> Point3D {
        Point3D::new(-self.x, -self.y, -self.z)
    }
}

#[cfg(test)]
mod creation_test {
    use super::*;

    #[test]
    fn test_new() {
        let v = Point3D::new(1.0, 2.0, 3.0);

        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_zeroes() {
        let v = Point3D::zeroes();

        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
        assert_eq!(v.z, 0.0);
    }

    #[test]
    fn test_ones() {
        let v = Point3D::ones();

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
        let lhs = Point3D::new(1.0, 2.0, 3.0);

        let res = lhs + Point3D::new(4.0, 5.0, 6.0);

        assert_eq!(res.x, 5.0);
        assert_eq!(res.y, 7.0);
        assert_eq!(res.z, 9.0);
    }

    #[test]
    fn test_ops_add_assign() {
        let mut lhs = Point3D::new(1.0, 2.0, 3.0);
        lhs += Point3D::new(4.0, 5.0, 6.0);

        assert_eq!(lhs.x, 5.0);
        assert_eq!(lhs.y, 7.0);
        assert_eq!(lhs.z, 9.0);
    }

    #[test]
    fn test_ops_sub() {
        let lhs = Point3D::new(9.0, 8.0, 7.0);

        let res = lhs - Point3D::new(1.0, 2.0, 3.0);

        assert_eq!(res.x, 8.0);
        assert_eq!(res.y, 6.0);
        assert_eq!(res.z, 4.0);
    }

    #[test]
    fn test_ops_sub_assign() {
        let mut lhs = Point3D::new(1.0, 2.0, 3.0);

        lhs -= Point3D::new(9.0, 8.0, 7.0);

        assert_eq!(lhs.x, -8.0);
        assert_eq!(lhs.y, -6.0);
        assert_eq!(lhs.z, -4.0);
    }

    #[test]
    fn test_ops_mul() {
        let lhs = Point3D::new(1.0, 2.0, 3.0);

        let res = lhs * 2.0;
        assert_eq!(res.x, 2.0);
        assert_eq!(res.y, 4.0);
        assert_eq!(res.z, 6.0);
    }

    #[test]
    fn test_ops_mul_assign() {
        let mut lhs = Point3D::new(1.0, 2.0, 3.0);

        lhs *= 2.0;

        assert_eq!(lhs.x, 2.0);
        assert_eq!(lhs.y, 4.0);
        assert_eq!(lhs.z, 6.0);
    }

    #[test]
    fn test_ops_div() {
        let lhs = Point3D::new(2.0, 4.0, 6.0);

        let res = lhs / 2.0;

        assert_eq!(res.x, 1.0);
        assert_eq!(res.y, 2.0);
        assert_eq!(res.z, 3.0);
    }

    #[test]
    fn test_ops_div_assign() {
        let mut lhs = Point3D::new(2.0, 4.0, 6.0);

        lhs /= 2.0;

        assert_eq!(lhs.x, 1.0);
        assert_eq!(lhs.y, 2.0);
        assert_eq!(lhs.z, 3.0);
    }

    #[test]
    fn test_ops_neg() {
        let lhs = Point3D::new(1.0, 2.0, 3.0);

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
        let lhs = Point3D::new(1.0, 2.0, 3.0);

        assert_eq!(lhs.x(), 1.0);
        assert_eq!(lhs.y(), 2.0);
        assert_eq!(lhs.z(), 3.0);
    }
}