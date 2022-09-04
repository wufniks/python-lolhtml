mod rewritable_units;
mod settings;

use std::rc::Rc;

use pyo3::create_exception;
use pyo3::exceptions::{PyException, PyRuntimeError};
use pyo3::prelude::*;

use self::settings::{PyDocumentContentHandler, PyElementContentHandler};

create_exception!(module, PyRewritingError, PyException);

/// Rewrites given html string with the provided settings.
#[pyfunction(
    html,
    "*",
    element_content_handlers = "Vec::new()",
    document_content_handlers = "Vec::new()"
)]
fn rewrite_str(
    py: Python<'_>,
    html: &str,
    element_content_handlers: Vec<PyRefMut<'_, PyElementContentHandler>>,
    document_content_handlers: Vec<PyRefMut<'_, PyDocumentContentHandler>>,
) -> PyResult<String> {
    let element_content_handlers = element_content_handlers
        .into_iter()
        .map(|handler| handler.as_element_content_handlers())
        .collect();
    let document_content_handlers = document_content_handlers
        .into_iter()
        .map(|handler| handler.as_document_content_handlers())
        .collect();
    lol_html::rewrite_str(
        html,
        lol_html::RewriteStrSettings {
            element_content_handlers,
            document_content_handlers,
            ..Default::default()
        },
    )
    .map_err(|e| {
        if let lol_html::errors::RewritingError::ContentHandlerError(mut inner) = e {
            if let Some(pyerr) = inner.downcast_mut::<PyErr>() {
                return pyerr.clone_ref(py);
            } else {
                PyRuntimeError::new_err(inner.to_string())
            }
        } else {
            PyRuntimeError::new_err(e.to_string())
        }
    })
}

#[pyclass(unsendable)]
#[derive(Clone)]
struct RewriteStrSettings(Rc<lol_html::RewriteStrSettings<'static, 'static>>);

/// Python bindings of lol-html.
#[pymodule]
fn lolhtml(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rewrite_str, m)?)?;
    m.add_class::<RewriteStrSettings>()?;
    m.add("RewritingError", py.get_type::<PyRewritingError>())?;
    rewritable_units::register(py, m)?;
    settings::register(py, m)?;
    Ok(())
}
