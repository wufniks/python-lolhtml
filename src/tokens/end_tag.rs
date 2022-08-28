use lol_html::html_content::EndTag;
use pyo3::prelude::*;

use crate::element::PyContentType;

pub(crate) fn register(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyEndTag>()?;
    Ok(())
}

#[pyclass]
pub(crate) struct PyEndTag(&'static mut EndTag<'static>);

impl PyEndTag {
    pub fn new(end: &'static mut EndTag<'static>) -> Self {
        Self(end)
    }
}

#[pymethods]
impl PyEndTag {
    #[inline]
    pub fn name(&self) -> String {
        self.0.name()
    }

    #[inline]
    pub fn set_name(&mut self, _name: &[u8]) {
        todo!()
    }

    #[inline]
    pub fn set_name_str(&mut self, name: String) {
        self.0.set_name_str(name)
    }

    #[inline]
    pub fn before(&mut self, content: &str, content_type: PyContentType) {
        self.0.before(content, content_type.into())
    }

    #[inline]
    pub fn after(&mut self, content: &str, content_type: PyContentType) {
        self.0.after(content, content_type.into())
    }

    /// Removes the end tag.
    #[inline]
    pub fn remove(&mut self) {
        self.0.remove()
    }
}
