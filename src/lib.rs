use pyo3::prelude::*;


/// Python glue for mokaccino library.
///
#[pymodule]
mod mokaccino {
    use mokaccino_lib::prelude::{CNFQueryable, Qid};
    use pyo3::{
        exceptions::PyRuntimeError, prelude::*, types::{PyIterator, PyType}
    };
    use pyo3_stub_gen::derive::*;

    #[derive(Clone)]
    #[gen_stub_pyclass]
    #[pyclass]
    pub struct Query(mokaccino_lib::prelude::Query);

    #[gen_stub_pymethods]
    #[pymethods]
    impl Query {

        #[classmethod]
        fn parse(_cls: &Bound<'_, PyType>, s: &str) -> PyResult<Self> {
            s.parse::<mokaccino_lib::prelude::Query>()
                .map(|q| Self(q))
                .map_err(|e| PyRuntimeError::new_err(format!("Parse error: {}", e)))
        }

        /// Create a Query that matches documents where field `k` has value `v`.
        #[classmethod]
        fn from_kv(_cls: &Bound<'_, PyType>, k: &str, v: &str) -> PyResult<Self> {
            Ok(Self(k.has_value(v)))
        }

        #[classmethod]
        fn from_kprefix(_cls: &Bound<'_, PyType>, k: &str, p: &str) -> PyResult<Self> {
            Ok(Self(k.has_prefix(p)))
        }

        #[classmethod]
        fn from_klt(_cls: &Bound<'_, PyType>, k: &str, v: i64) -> PyResult<Self> {
            Ok(Self(k.i64_lt(v)))
        }

        #[classmethod]
        fn from_kle(_cls: &Bound<'_, PyType>, k: &str, v: i64) -> PyResult<Self> {
            Ok(Self(k.i64_le(v)))
        }

        #[classmethod]
        fn from_keq(_cls: &Bound<'_, PyType>, k: &str, v: i64) -> PyResult<Self> {
            Ok(Self(k.i64_eq(v)))
        }

        #[classmethod]
        fn from_kge(_cls: &Bound<'_, PyType>, k: &str, v: i64) -> PyResult<Self> {
            Ok(Self(k.i64_ge(v)))
        }

        #[classmethod]
        fn from_kgt(_cls: &Bound<'_, PyType>, k: &str, v: i64) -> PyResult<Self> {
            Ok(Self(k.i64_gt(v)))
        }

        #[classmethod]
        fn from_not(_cls: &Bound<'_, PyType>, q: &Self) -> PyResult<Self> {
            Ok(Self(!q.0.clone()))
        }

        #[classmethod]
        fn from_and(_cls: &Bound<'_, PyType>, iterable: &Bound<'_, PyAny>) -> PyResult<Self> {
            let mut items: Vec<mokaccino_lib::prelude::Query> = vec![];
            for item in PyIterator::from_object(iterable)? {
                let q: Self = item?.extract::<Query>()?;
                items.push(q.0);
            }
            Ok(Self(mokaccino_lib::prelude::Query::from_and(items)))
        }

        #[classmethod]
        fn from_or(_cls: &Bound<'_, PyType>, iterable: &Bound<'_, PyAny>) -> PyResult<Self> {
            let mut items: Vec<mokaccino_lib::prelude::Query> = vec![];
            for item in PyIterator::from_object(iterable)? {
                let q: Self = item?.extract::<Query>()?;
                items.push(q.0);
            }
            Ok(Self(mokaccino_lib::prelude::Query::from_or(items)))
        }

        fn __str__(&self) -> String{
            self.0.to_string()
        }

        fn __and__(&self, other: Self) -> PyResult<Self> {
            Ok(Self(self.0.clone() & other.0))
        }
 
        fn __or__(&self, other: Self) -> PyResult<Self> {
            Ok(Self(self.0.clone() | other.0))
        }

        fn __invert__(&self) -> PyResult<Self> {
            Ok(Self(! self.0.clone()))
        }


    }

    #[derive(Clone)]
    #[gen_stub_pyclass]
    #[pyclass]
    pub struct Document(mokaccino_lib::prelude::Document);

    #[gen_stub_pymethods]
    #[pymethods]
    impl Document {
        #[new]
        fn new() -> Self {
            Self(mokaccino_lib::prelude::Document::new())
        }

        fn __str__(&self) -> String{
            format!("{:?}" , self.0 )
        }

        pub fn with_value(&self, field: &str, value: &str) -> PyResult<Self> {
            Ok(Self(self.0.clone().with_value(field, value)))
        }

        pub fn field_values(&self) -> PyResult<Vec<(String, String)>> {
            Ok(self
                .0
                .field_values()
                .map(|(f, v)| (f.to_string(), v.to_string()))
                .collect())
        }

        pub fn merge_with(&self, other: &Document) -> PyResult<Self> {
            Ok(Self(self.0.merge_with(&other.0)))
        }
    }

    #[gen_stub_pyclass]
    #[pyclass]
    pub struct Percolator(mokaccino_lib::prelude::Percolator);

    #[gen_stub_pymethods]
    #[pymethods]
    impl Percolator {
        #[new]
        fn new() -> Self {
            Self(mokaccino_lib::prelude::Percolator::default())
        }

        fn add_query(&mut self, query: &Query) -> PyResult<Qid> {
            Ok(self.0.add_query(query.0.clone()))
        }

        fn percolate_list(&self, document: &Document) -> PyResult<Vec<Qid>> {
            Ok(self.0.percolate(&document.0).collect())
        }

        fn to_json(&self) -> PyResult<String> {
            serde_json::to_string(&self.0).map_err(|e|
                PyRuntimeError::new_err(format!("Serialization error: {}", e))
            )
        }

        #[classmethod]
        fn from_json(_cls: &Bound<'_, PyType>, json_str: &str) -> PyResult<Self> {
            let p: mokaccino_lib::prelude::Percolator = serde_json::from_str(json_str).map_err(|e|
                PyRuntimeError::new_err(format!("Deserialization error: {}", e))
            )?;
            Ok(Self(p))
        }
    }

}

use pyo3_stub_gen::define_stub_info_gatherer;
define_stub_info_gatherer!(stub_info);
