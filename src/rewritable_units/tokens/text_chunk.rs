use lol_html::html_content::{TextChunk, TextType};
use pyo3::prelude::*;

use crate::rewritable_units::PyContentType;

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
    /// Returns the textual content of the chunk.
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Returns the type of the text in the chunk.
    ///
    /// The type of the text depends on the surrounding context of the text. E.g. regular visible
    /// text and text inside a `<script>` element will have different types. Refer to [`TextType`]
    /// for more information about possible text types.
    #[inline]
    pub fn text_type(&self) -> PyTextType {
        PyTextType(self.0.text_type())
    }

    /// Returns `true` if the chunk is last in a HTML text node.
    ///
    /// Note that last chunk can have empty textual content.
    #[inline]
    pub fn last_in_text_node(&self) -> bool {
        self.0.last_in_text_node()
    }

    /// Inserts `content` before the text chunk.
    ///
    /// Consequent calls to the method append `content` to the previously inserted content.
    #[inline]
    pub fn before(&mut self, content: &str, content_type: PyContentType) {
        self.0.before(content, content_type.into())
    }

    /// Inserts `content` after the text chunk.
    ///
    /// Consequent calls to the method prepend `content` to the previously inserted content.
    #[inline]
    pub fn after(&mut self, content: &str, content_type: PyContentType) {
        self.0.after(content, content_type.into())
    }

    /// Replaces the text chunk with the `content`.
    ///
    /// Consequent calls to the method overwrite previous replacement content.
    #[inline]
    pub fn replace(&mut self, content: &str, content_type: PyContentType) {
        self.0.after(content, content_type.into())
    }

    /// Removes the text chunk.
    #[inline]
    pub fn remove(&mut self) {
        self.0.remove()
    }

    /// Returns `true` if the text chunk has been replaced or removed.
    #[inline]
    pub fn removed(&self) -> bool {
        self.0.removed()
    }
}
