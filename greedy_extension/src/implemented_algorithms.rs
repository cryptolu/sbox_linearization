use pyo3::prelude::*;
use std::ops::{Deref, DerefMut};
use std::time::{Duration, Instant};
use rand::Rng;
use crate::algorithm::Algorithm;
use crate::greedy_extension::greedy_extension_py;

#[pyclass]
pub struct GreedyExtensionU16 {
    pub implemented: Algorithm
}

#[pymethods]
impl GreedyExtensionU16 {
    #[new]
    #[pyo3(signature = (s_box, n=None, m=None))]
    pub fn new(s_box: &Bound<'_, PyAny>, n: Option<usize>, m: Option<usize>) -> PyResult<Self> {
        let values: Vec<u16> = s_box.try_iter()?
            .map(|item| item.and_then(|v| v.extract::<u16>()))
            .collect::<PyResult<Vec<u16>>>()?;

        // try to get n from argument, then from s_box.input_size(), then deduce from len
        let n = if let Some(n) = n {
            n
        } else if let Ok(val) = s_box.call_method0("input_size") {
            val.extract::<usize>()?
        } else {
            let len = values.len();
            // must be a power of 2
            if len == 0 || (len & (len - 1)) != 0 {
                return Err(pyo3::exceptions::PyValueError::new_err(
                    format!("S-box length {} is not a power of 2; please specify n explicitly", len)
                ));
            }
            len.trailing_zeros() as usize
        };

        // try to get m from argumen, then from s_box.output_size(), then deduce from max value
        let m = if let Some(m) = m {
            m
        } else if let Ok(val) = s_box.call_method0("output_size") {
            val.extract::<usize>()?
        } else {
            let max_val = values.iter().copied().max().unwrap_or(0) as usize;
            if max_val == 0 {
                1
            } else {
                (usize::BITS - max_val.leading_zeros()) as usize
            }
        };

        Ok(Self {
            implemented: Algorithm::new(values, n, m)
        })
    }

    #[pyo3(signature = (init=None, seed=None))]
    pub fn run(&mut self, py: Python, init: Option<Vec<u16>>, seed: Option<u64>) -> PyResult<Vec<u16>> {
        let init = init.unwrap_or_default();
        let seed = seed.unwrap_or_else(|| rand::thread_rng().gen());
        let xs = greedy_extension_py(py, &mut self.implemented, &init, seed)?;
        Ok(xs)
    }

    #[pyo3(signature = (init=None, seed=None))]
    pub fn run_ext(&mut self, py: Python, init: Option<Vec<u16>>, seed: Option<u64>) -> PyResult<(Vec<u16>, Vec<usize>, Vec<(usize, usize)>)> {
        let init = init.unwrap_or_default();
        let seed = seed.unwrap_or_else(|| rand::thread_rng().gen());
        let xs = greedy_extension_py(py, &mut self.implemented, &init, seed)?;
        let profile = self.implemented.profile.clone();
        let last_merges = self.implemented.last_merges.clone();
        Ok((xs, profile, last_merges))
    }

    fn best_of_inner(&mut self, py: Python, num: usize, time: Option<f64>) -> PyResult<(Vec<u16>, Vec<usize>, Vec<(usize, usize)>, u64)> {
        let deadline = time.map(|t| Instant::now() + Duration::from_secs_f64(t));
        let mut rng = rand::thread_rng();

        let mut best_xs: Vec<u16> = vec![];
        let mut best_profile: Vec<usize> = vec![];
        let mut best_merges: Vec<(usize, usize)> = vec![];
        let mut best_seed: u64 = 0;

        let init: Vec<u16> = vec![];
        for _i in 0..num {
            py.check_signals()?;

            let seed: u64 = rng.gen();
            let xs = greedy_extension_py(py, &mut self.implemented, &init, seed)?;
            if xs.len() > best_xs.len() {
                best_xs = xs;
                best_profile = self.implemented.profile.clone();
                best_merges = self.implemented.last_merges.clone();
                best_seed = seed;
            }

            if let Some(dl) = deadline {
                if Instant::now() >= dl {
                    break;
                }
            }
        }

        Ok((best_xs, best_profile, best_merges, best_seed))
    }

    #[pyo3(signature = (num=1000, time=None))]
    pub fn best_of(&mut self, py: Python, num: usize, time: Option<f64>) -> PyResult<Vec<u16>> {
        Ok(self.best_of_inner(py, num, time)?.0)
    }

    #[pyo3(signature = (num=1000, time=None))]
    pub fn best_of_ext(&mut self, py: Python, num: usize, time: Option<f64>) -> PyResult<(Vec<u16>, Vec<usize>, Vec<(usize, usize)>, u64)> {
        self.best_of_inner(py, num, time)
    }

    #[getter]
    pub fn last_seed(&self) -> u64 {
        self.implemented.last_seed
    }

    #[getter]
    pub fn profile(&self) -> Vec<usize> {
        self.implemented.profile.clone()
    }

    #[getter]
    pub fn last_merges(&self) -> Vec<(usize, usize)> {
        self.implemented.last_merges.clone()
    }
}

impl Deref for GreedyExtensionU16 {
    type Target = Algorithm;
    fn deref(&self) -> &Algorithm {
        &self.implemented
    }
}
impl DerefMut for GreedyExtensionU16 {
    fn deref_mut(&mut self) -> &mut Algorithm {
        &mut self.implemented
    }
}
