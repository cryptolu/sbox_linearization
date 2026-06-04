use pyo3::prelude::*;
use rand::Rng;
use crate::greedy_extension::greedy_extension_py;
use crate::implemented_algorithms::GreedyExtensionU16;

#[pyfunction]
#[pyo3(signature = (alg, init=None, seed=None))]
pub fn greedy_extension_u16(py: Python, mut alg: PyRefMut<GreedyExtensionU16>, init: Option<Vec<u16>>, seed: Option<u64>) -> PyResult<Vec<u16>> {
    let init = init.unwrap_or_default();
    let seed = seed.unwrap_or_else(|| rand::thread_rng().gen());
    Ok(greedy_extension_py(py, &mut alg.implemented, &init, seed)?)
}
