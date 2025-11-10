use pyo3::prelude::*;

/// Python glue for mokaccino library.
/// 
#[pymodule]
mod mokaccino_py {
    use pyo3::{prelude::*, types::PyType};

    #[pyclass]
    pub struct Query(mokaccino::prelude::Query);

    #[pymethods]
    impl Query {
        #[classmethod]
        fn from_kv(cls: &Bound<'_, PyType>, k: String, v: String) -> PyResult<Self> {
            let query = mokaccino::prelude::Query::term(k, v);
            Ok(Self(query))
        }
    }

    /// Formats the sum of two numbers as string.
    #[pyfunction]
    fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
        Ok((a + b).to_string())
    }
}
