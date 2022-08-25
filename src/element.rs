use std::rc::Rc;

use lol_html::html_content::Element;
use pyo3::prelude::*;

pub(crate) fn register(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyElement>()?;
    Ok(())
}

#[pyclass(unsendable, name = "Element")]
pub(crate) struct PyElement(&'static mut Element<'static, 'static>);

impl PyElement {
    pub fn new(element: &'static mut Element) -> Self {
        Self(element)
    }
}

#[pymethods]
impl PyElement {
    fn set_attribute(&mut self, name: &str, value: &str) -> PyResult<()> {
        Ok(self
            .0
            .set_attribute(name, value)
            .map_err(|_e| pyo3::exceptions::PyRuntimeError::new_err("something went wrong"))?)
    }
}
