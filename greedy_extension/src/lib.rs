pub mod dsu;
mod greedy_extension;
mod algorithm;
mod implemented_algorithms;
mod veclin_greedy_extension_entry_points;

use pyo3::prelude::*;

use crate::veclin_greedy_extension_entry_points::greedy_extension_u16;

#[pymodule]
fn veclin_greedy_extension(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<implemented_algorithms::GreedyExtensionU16>()?;
    m.add_function(wrap_pyfunction!(greedy_extension_u16, m)?)?;
    Ok(())
}
