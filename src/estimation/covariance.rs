use crate::enums::CovarianceType;
use nalgebra::{DMatrix, Matrix6};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Covariance {
    elements: Matrix6<f64>,
    covariance_type: CovarianceType,
}

impl Covariance {
    pub fn get_element(&self, i: usize, j: usize) -> f64 {
        self.elements[(i, j)]
    }

    pub fn set_element(&mut self, i: usize, j: usize, value: f64) {
        self.elements[(i, j)] = value;
    }

    pub fn get_covariance_type(&self) -> CovarianceType {
        self.covariance_type
    }

    pub fn get_sigmas(&self) -> Vec<f64> {
        let mut sigmas = vec![0.0; 6];
        for (i, sigma) in sigmas.iter_mut().enumerate() {
            *sigma = self.elements[(i, i)].sqrt();
        }
        sigmas
    }
}

impl From<Covariance> for [[f64; 6]; 6] {
    fn from(cov: Covariance) -> Self {
        let mut elements = [[0.0; 6]; 6];
        for (i, row) in elements.iter_mut().enumerate() {
            for (j, element) in row.iter_mut().enumerate() {
                *element = cov.elements[(i, j)];
            }
        }
        elements
    }
}

impl From<([[f64; 6]; 6], CovarianceType)> for Covariance {
    fn from(input: ([[f64; 6]; 6], CovarianceType)) -> Self {
        let (elements, covariance_type) = input;
        Covariance {
            elements: Matrix6::from_row_slice(&elements.concat()),
            covariance_type,
        }
    }
}

impl From<(DMatrix<f64>, CovarianceType)> for Covariance {
    fn from(input: (DMatrix<f64>, CovarianceType)) -> Self {
        let (input_cov, covariance_type) = input;
        let mut elements: Matrix6<f64> = Matrix6::zeros();
        elements.copy_from(&input_cov.fixed_rows::<6>(0).fixed_columns::<6>(0));
        Covariance {
            elements,
            covariance_type,
        }
    }
}
