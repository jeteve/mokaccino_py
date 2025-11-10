use pyo3::prelude::*;

/// Python glue for mokaccino library.
/// 
#[pymodule]
mod mokaccino_py {
    use pyo3::prelude::*;

    #[pyclass]
    struct Query(mokaccino::prelude::Query);


    /// Formats the sum of two numbers as string.
    #[pyfunction]
    fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
        Ok((a + b).to_string())
    }
}
