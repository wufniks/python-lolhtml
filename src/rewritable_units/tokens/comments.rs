use lol_html::html_content::Comment;
use pyo3::prelude::*;

use crate::rewritable_units::PyContentType;

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
    /// Returns the text of the comment.
    #[inline]
    pub fn text(&self) -> String {
        self.0.text()
    }

    /// Sets the text of the comment.
    // #[inline]
    // pub fn set_text(&mut self, text: &str) -> Result<(), CommentTextError> {
    // }

    /// Inserts `content` before the comment.
    ///
    /// Consequent calls to the method append `content` to the previously inserted content.
    #[inline]
    pub fn before(&mut self, content: &str, content_type: PyContentType) {
        self.0.before(content, content_type.into())
    }

    /// Inserts `content` after the comment.
    ///
    /// Consequent calls to the method prepend `content` to the previously inserted content.
    #[inline]
    pub fn after(&mut self, content: &str, content_type: PyContentType) {
        self.0.after(content, content_type.into())
    }

    /// Replaces the comment with the `content`.
    ///
    /// Consequent calls to the method overwrite previous replacement content.
    #[inline]
    pub fn replace(&mut self, content: &str, content_type: PyContentType) {
        self.0.replace(content, content_type.into())
    }

    /// Removes the comment.
    #[inline]
    pub fn remove(&mut self) {
        self.0.remove()
    }

    /// Returns `true` if the comment has been replaced or removed.
    #[inline]
    pub fn removed(&self) -> bool {
        self.0.removed()
    }
}
