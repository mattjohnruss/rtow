use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};
use rand_distr::{Distribution, Uniform, UnitBall, UnitSphere};

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { e: [x, y, z] }
    }

    pub fn zero() -> Self {
        Vec3 { e: [0.0; 3] }
    }

    pub fn random() -> Self {
        Vec3::random_range(0.0, 1.0)
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        let uniform_dist = Uniform::new(min, max);
        let mut rng = rand::thread_rng();
        let x = uniform_dist.sample(&mut rng);
        let y = uniform_dist.sample(&mut rng);
        let z = uniform_dist.sample(&mut rng);
        Vec3 { e: [x, y, z] }
    }

    pub fn random_in_unit_ball() -> Self {
        Vec3 { e: UnitBall.sample(&mut rand::thread_rng()) }
    }

    pub fn random_in_unit_half_ball(normal: Vec3) -> Self {
        let in_unit_ball = Vec3::random_in_unit_ball();
        if in_unit_ball.dot(normal) > 0.0 {
            in_unit_ball
        } else {
            -in_unit_ball
        }
    }

    pub fn random_on_unit_sphere() -> Self {
        Vec3 { e: UnitSphere.sample(&mut rand::thread_rng()) }
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

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn dot(&self, rhs: Self) -> f64 {
        self.e[0] * rhs.e[0] + self.e[1] * rhs.e[1] + self.e[2] * rhs.e[2]
    }

    pub fn cross(&self, rhs: Self) -> Self {
        Vec3 {
            e: [
                self.e[1] * rhs.e[2] - self.e[2] * rhs.e[1],
                self.e[2] * rhs.e[0] - self.e[0] * rhs.e[2],
                self.e[0] * rhs.e[1] - self.e[1] * rhs.e[0],
            ],
        }
    }

    pub fn unit(&self) -> Self {
        *self / self.length()
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2],
            ],
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] - rhs.e[0],
                self.e[1] - rhs.e[1],
                self.e[2] - rhs.e[2],
            ],
        }
    }
}

impl Mul<Self> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] * rhs.e[0],
                self.e[1] * rhs.e[1],
                self.e[2] * rhs.e[2],
            ],
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            e: [rhs * self.e[0], rhs * self.e[1], rhs * self.e[2]],
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            e: [self * rhs.e[0], self * rhs.e[1], self * rhs.e[2]],
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        (1.0 / rhs) * self
    }
}

pub type Point3 = Vec3;
pub type Colour = Vec3;
