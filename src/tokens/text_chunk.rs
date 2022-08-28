use lol_html::html_content::{TextChunk, TextType};
use pyo3::prelude::*;

use crate::element::PyContentType;

pub(super) fn register(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyTextChunk>()?;
    Ok(())
}

#[pyclass]
pub(crate) struct PyTextType(TextType);

#[pyclass(unsendable)]
pub(crate) struct PyTextChunk(&'static mut TextChunk<'static>);

impl PyTextChunk {
    pub fn new(end: &'static mut TextChunk<'static>) -> Self {
        Self(end)
    }
}

#[pymethods]
impl PyTextChunk {
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    #[inline]
    pub fn text_type(&self) -> PyTextType {
        PyTextType(self.0.text_type())
    }

    #[inline]
    pub fn last_in_text_node(&self) -> bool {
        self.0.last_in_text_node()
    }

    #[inline]
    pub fn before(&mut self, content: &str, content_type: PyContentType) {
        self.0.before(content, content_type.into())
    }

    #[inline]
    pub fn after(&mut self, content: &str, content_type: PyContentType) {
        self.0.after(content, content_type.into())
    }

    #[inline]
    pub fn replace(&mut self, content: &str, content_type: PyContentType) {
        self.0.after(content, content_type.into())
    }

    /// Removes the end tag.
    #[inline]
    pub fn remove(&mut self) {
        self.0.remove()
    }

    #[inline]
    pub fn removed(&self) -> bool {
        self.0.removed()
    }
}
