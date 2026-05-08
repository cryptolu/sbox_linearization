pub mod dsu;
mod greedy_extension;
mod algorithm;
mod implemented_algorithms;

use pyo3::prelude::*;

use crate::greedy_extension::{greedy_extension_py};
use crate::implemented_algorithms::AlgorithmU16;

#[pyfunction]
pub fn greedy_extension_u16(py: Python, mut alg: PyRefMut<AlgorithmU16>, init: Vec<u16>, seed: u64) -> PyResult<Vec<u16>>  {
    Ok(greedy_extension_py(py, &mut alg.implemented, &init, seed)?)
}

#[pymodule]
fn greedy_extention(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<implemented_algorithms::AlgorithmU16>()?;
    m.add_function(wrap_pyfunction!(greedy_extension_u16, m)?)?;
    Ok(())
}
