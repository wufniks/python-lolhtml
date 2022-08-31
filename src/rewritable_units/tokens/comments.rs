use lol_html::html_content::Comment;
use pyo3::prelude::*;

pub(crate) fn register(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyComment>()?;
    Ok(())
}

#[pyclass(unsendable)]
pub(crate) struct PyComment(&'static mut Comment<'static>);

impl PyComment {
    pub fn new(end: &'static mut Comment<'static>) -> Self {
        Self(end)
    }
}

#[pymethods]
impl PyComment {
    #[inline]
    pub fn text(&self) -> String {
        self.0.text()
    }
}
