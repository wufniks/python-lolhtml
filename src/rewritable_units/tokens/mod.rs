pub(crate) mod comments;
pub(crate) mod end_tag;
pub(crate) mod text_chunk;

use pyo3::prelude::*;

pub(crate) fn register(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    end_tag::register(py, m)?;
    text_chunk::register(py, m)?;
    comments::register(py, m)?;
    Ok(())
}
