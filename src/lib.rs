#![feature(specialization)]
#![feature(custom_attribute)]
#[macro_use]
extern crate pyo3;

use pyo3::prelude::*;

#[pyclass]
struct TestObject {
    testing: String,
}

#[pymethods]
impl TestObject {
    #[new]
    fn __new__(obj: &PyRawObject, testing: String) -> PyResult<()> {
        obj.init(||   {
            TestObject {
                testing,
            }
        })
    }

    fn testing(&self, s: &str) -> PyResult<String> {
        Ok(s.to_owned())
    }

    fn testing2(&self) -> PyResult<String> {
        Ok(self.testing.clone())
    }

}

#[pyfunction]
fn return_string() -> PyResult<String> {
    Ok("test".to_string())
}

#[pymodule]
fn string_return(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_function!(return_string))?;
    m.add_class::<TestObject>()?;
    Ok(())
}