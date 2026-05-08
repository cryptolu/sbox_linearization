use pyo3::prelude::*;
use std::ops::{Deref, DerefMut};
use crate::{algorithm::Algorithm};
use crate::greedy_extension::greedy_extension_py;

#[pyclass]
pub struct AlgorithmU16 {
    pub implemented: Algorithm
}

#[pymethods]
impl AlgorithmU16 {
    #[new]
    pub fn new(s_box: Vec<u16>, n: usize, m: usize) -> Self {
        Self {
            implemented: Algorithm::new(s_box, n, m)
        }
    }

    pub fn greedy_extension(&mut self, py: Python, init: Vec<u16>, seed: u64) ->  PyResult<Vec<u16>>{
        Ok(greedy_extension_py(py, &mut self.implemented, &init, seed)?)
    }
}

impl Deref for AlgorithmU16 {
    type Target = Algorithm;
    fn deref(&self) -> &Algorithm {
        &self.implemented
    }
}
impl DerefMut for AlgorithmU16 {
    fn deref_mut(&mut self) -> &mut Algorithm {
        &mut self.implemented
    }
}
