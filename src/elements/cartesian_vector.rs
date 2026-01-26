use super::SphericalVector;
use nalgebra::Vector3;
use std::hash::{Hash, Hasher};
use std::ops::{Add, Index, IndexMut, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CartesianVector {
    n_vector: Vector3<f64>,
}

impl Hash for CartesianVector {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.n_vector[0].to_bits().hash(state);
        self.n_vector[1].to_bits().hash(state);
        self.n_vector[2].to_bits().hash(state);
    }
}

impl Eq for CartesianVector {}

impl Mul<f64> for CartesianVector {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Self {
            n_vector: self.n_vector * scalar,
        }
    }
}

impl Add for CartesianVector {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            n_vector: self.n_vector + other.n_vector,
        }
    }
}

impl Sub for CartesianVector {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            n_vector: self.n_vector - other.n_vector,
        }
    }
}

impl CartesianVector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            n_vector: Vector3::new(x, y, z),
        }
    }

    pub fn get_x(&self) -> f64 {
        self.n_vector.x
    }

    pub fn get_y(&self) -> f64 {
        self.n_vector.y
    }

    pub fn get_z(&self) -> f64 {
        self.n_vector.z
    }

    pub fn get_magnitude(&self) -> f64 {
        self.n_vector.norm()
    }

    pub fn distance(&self, other: &Self) -> f64 {
        (self.n_vector - other.n_vector).norm()
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.n_vector.dot(&other.n_vector)
    }

    pub fn angle(&self, other: &Self) -> f64 {
        let dot_product = self.dot(other);
        let magnitude_product = self.get_magnitude() * other.get_magnitude();
        if magnitude_product == 0.0 {
            return 0.0;
        }
        (dot_product / magnitude_product).acos()
    }
}

impl From<CartesianVector> for SphericalVector {
    fn from(cart: CartesianVector) -> Self {
        let r = cart.get_magnitude();
        let ra = cart.n_vector.y.atan2(cart.n_vector.x);
        let dec = cart
            .n_vector
            .z
            .atan2((cart.n_vector.x * cart.n_vector.x + cart.n_vector.y * cart.n_vector.y).sqrt());
        SphericalVector::new(r, ra.to_degrees(), dec.to_degrees())
    }
}

impl From<[f64; 3]> for CartesianVector {
    fn from(vec: [f64; 3]) -> Self {
        Self::new(vec[0], vec[1], vec[2])
    }
}

impl From<CartesianVector> for [f64; 3] {
    fn from(cartesian_vector: CartesianVector) -> Self {
        [
            cartesian_vector.get_x(),
            cartesian_vector.get_y(),
            cartesian_vector.get_z(),
        ]
    }
}

impl Index<usize> for CartesianVector {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.n_vector[0],
            1 => &self.n_vector[1],
            2 => &self.n_vector[2],
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for CartesianVector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.n_vector[0],
            1 => &mut self.n_vector[1],
            2 => &mut self.n_vector[2],
            _ => panic!("Index out of bounds"),
        }
    }
}
