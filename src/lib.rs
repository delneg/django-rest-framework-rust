use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn time_since_midnight(hour: usize, minute: Option<usize>) -> PyResult<usize> {
    let min = minute.unwrap_or(0);
    Ok(hour * 60 + min)
}

/// A Python module implemented in Rust.
#[pymodule]
fn rest_framework(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(time_since_midnight, m)?)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
