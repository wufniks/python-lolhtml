use lol_html::html_content::DocumentEnd;
use pyo3::prelude::*;

use crate::rewritable_units::PyContentType;

pub(crate) fn register(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyDocumentEnd>()?;
    Ok(())
}

#[pyclass(unsendable)]
pub(crate) struct PyDocumentEnd(&'static mut DocumentEnd<'static>);

impl PyDocumentEnd {
    pub fn new(end: &'static mut DocumentEnd<'static>) -> Self {
        Self(end)
    }
}

#[pymethods]
impl PyDocumentEnd {
    pub fn append(&mut self, content: &str, content_type: PyContentType) {
        self.0.append(content, content_type.into())
    }
}
