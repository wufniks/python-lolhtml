pub(crate) mod document_end;
pub(crate) mod element;
pub(crate) mod tokens;

use lol_html::html_content::ContentType;
use pyo3::prelude::*;

pub(crate) fn register(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    element::register(py, m)?;
    document_end::register(py, m)?;
    tokens::register(py, m)?;
    m.add_class::<PyContentType>()?;
    Ok(())
}

#[derive(Debug, Clone, Copy)]
#[pyclass(name = "ContentType")]
pub(crate) enum PyContentType {
    Text,
    Html,
}

impl From<PyContentType> for ContentType {
    fn from(this: PyContentType) -> Self {
        match this {
            PyContentType::Text => ContentType::Text,
            PyContentType::Html => ContentType::Html,
        }
    }
}
