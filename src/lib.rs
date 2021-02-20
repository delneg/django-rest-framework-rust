use pyo3::prelude::*;
use pyo3::{wrap_pyfunction, PyObjectProtocol};

/// Formats the sum of two numbers as string.
// #[pyfunction]
// fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
//     Ok((a + b).to_string())
// }

#[pyfunction]
fn is_html_input(dictionary: &PyAny) -> PyResult<bool> {
    dictionary.hasattr("getlist")
}

/// A Python module implemented in Rust.
#[pymodule]
fn rest_framework(py: Python, m: &PyModule) -> PyResult<()> {
    // m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(is_html_input, m)?)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
