mod utils;
mod greedy_algorithms;
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn benchmark(benchmark_name:String) -> PyResult<i32> {
    if benchmark_name == "lab1" {
        Ok(3)
    } else {
        Ok(2)
    }
}

#[pymodule]
fn evolutionary(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(benchmark, m)?)?;
    Ok(())
}