use pyo3::prelude::*;

/// Python glue for mokaccino library.
/// 
#[pymodule]
mod mokaccino_py {
    use mokaccino::prelude::CNFQueryable;
    use pyo3::{prelude::*, types::{PyIterator, PyType}};

    #[derive(Clone)]
    #[pyclass]
    pub struct Query(mokaccino::prelude::Query);

    #[pymethods]
    impl Query {
        #[classmethod]
        fn from_kv(_cls: &Bound<'_, PyType>, k: String, v: String) -> PyResult<Self> {
            Ok(Self(k.has_value(v)))
        }

        #[classmethod]
        fn from_kprefix(_cls: &Bound<'_, PyType>, k: String, v: String) -> PyResult<Self> {
            Ok(Self(k.has_prefix(v)))
        }

        #[classmethod]
        fn from_klt(_cls: &Bound<'_, PyType>, k: String, v: i64) -> PyResult<Self> {
            Ok(Self(k.i64_lt(v)))
        }
        
        #[classmethod]
        fn from_kle(_cls: &Bound<'_, PyType>, k: String, v: i64) -> PyResult<Self> {
            Ok(Self(k.i64_le(v)))
        }

        #[classmethod]
        fn from_keq(_cls: &Bound<'_, PyType>, k: String, v: i64) -> PyResult<Self> {
            Ok(Self(k.i64_eq(v)))
        }

        #[classmethod]
        fn from_kge(_cls: &Bound<'_, PyType>, k: String, v: i64) -> PyResult<Self> {
            Ok(Self(k.i64_ge(v)))
        }

        #[classmethod]
        fn from_kgt(_cls: &Bound<'_, PyType>, k: String, v: i64) -> PyResult<Self> {
            Ok(Self(k.i64_gt(v)))
        }

        #[classmethod]
        fn from_not(_cls: &Bound<'_, PyType>, q: &Self) -> PyResult<Self> {
            Ok(Self(! q.0.clone()))
        }

        #[classmethod]
        fn from_and(_cls: &Bound<'_, PyType>, iterable: &Bound<'_,PyAny>) -> PyResult<Self> {
            let mut items: Vec<mokaccino::prelude::Query> = vec![];
            for item in  PyIterator::from_object(iterable)? {
                let q: Self = item?.extract::<Query>()?;  
                items.push(q.0);
            }
            Ok(Self(mokaccino::prelude::Query::from_and(items)))
        }

        #[classmethod]
        fn from_or(_cls: &Bound<'_, PyType>, iterable: &Bound<'_,PyAny>) -> PyResult<Self> {
            let mut items: Vec<mokaccino::prelude::Query> = vec![];
            for item in  PyIterator::from_object(iterable)? {
                let q: Self = item?.extract::<Query>()?;  
                items.push(q.0);
            }
            Ok(Self(mokaccino::prelude::Query::from_or(items)))
        }
    }


    #[derive(Clone)]
    #[pyclass]
    pub struct Document(mokaccino::prelude::Document);

    #[pymethods]
    impl Document{

        #[new]
        fn new() -> Self{
            Self(mokaccino::prelude::Document::new())
        }

        pub fn with_value(&self, field: String, value: String) -> PyResult<Self> {
            Ok(Self(self.0.clone().with_value(field, value)))
        }

        pub fn field_values(&self) -> PyResult<Vec<(String,String)>>{
            Ok(self.0.field_values()
            .map(|(f,v)| (f.to_string(), v.to_string()))
            .collect())
        }
    }
}
