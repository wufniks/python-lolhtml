mod element;

use std::borrow::Cow;
use std::rc::Rc;
use std::str::FromStr;
use std::sync::Arc;

use element::PyElement;
use lol_html::html_content::Element;
use lol_html::ElementContentHandlers;
use lol_html::Selector;
// use lol_html::RewriteStrSettings;
use pyo3::create_exception;
use pyo3::exceptions::{PyException, PyRuntimeError, PyTypeError};
use pyo3::prelude::*;
use pyo3::types::{PyList, PyTuple};

create_exception!(module, RewritingError, PyException);

/// Rewrites given html string with the provided settings.
// #[pyfunction(element_content_handlers = "Vec::new()")]
#[pyfunction]
fn rewrite_str(
    py: Python<'_>,
    html: &str,
    element_content_handlers: Vec<PyRefMut<'_, PyElementContentHandler>>,
) -> PyResult<String> {
    let element_content_handlers = element_content_handlers
        .into_iter()
        .map(|handler| handler.as_element_content_handlers())
        .collect();
    lol_html::rewrite_str(
        html,
        lol_html::RewriteStrSettings {
            element_content_handlers,
            ..Default::default()
        },
    )
    .map_err(|_e| PyTypeError::new_err("Error message"))
}

// /// Rewrites given html string with the provided settings.
// #[pyfunction(element_content_handlers = "Vec::new()")]
// fn rewrite_str(html: &str) -> PyResult<String> {
//     lol_html::rewrite_str(
//         html,
//         lol_html::RewriteStrSettings {
//             ..Default::default()
//         },
//     )
//     .map_err(|e| RewritingError::new_err(e.to_string()))
// }

// #[derive(Clone)]
// pub(crate) struct CallbackHandler {
//     /// The python callback itself.
//     cb: Rc<PyObject>,
// }

// impl CallbackHandler {
//     /// Creates a new instance of this struct wrapping the PyObject in a
//     /// arc to make for cheap clones.
//     pub(crate) fn new(cb: PyObject) -> Self {
//         Self { cb: Rc::new(cb) }
//     }

//     /// Invokes the callback by acquiring the gil internally.
//     pub(crate) fn invoke(&self, args: impl IntoPy<Py<PyTuple>>) -> PyResult<()> {
//         Python::with_gil(|py| -> PyResult<()> {
//             let _ = self.cb.call1(py, args)?;
//             Ok(())
//         })
//     }
// }

#[pyclass(name = "ElementContentHandler")]
struct PyElementContentHandler {
    pub(crate) selector: String,
    pub(crate) element: Option<Arc<PyObject>>,
    // pub(crate) comments: Option<Py<PyAny>>,
    // pub(crate) text: Option<Py<PyAny>>,
}

#[pymethods]
impl PyElementContentHandler {
    #[new]
    fn __new__(selector: &str, element: Option<PyObject>) -> Self {
        Self {
            selector: selector.to_owned(),
            element: element.map(Arc::new),
        }
    }
}

impl PyElementContentHandler {
    pub fn as_element_content_handlers<'h>(
        &self,
    ) -> (Cow<'h, Selector>, ElementContentHandlers<'h>) {
        let mut handlers = ElementContentHandlers::default();

        if let Some(handler) = self.element.clone() {
            handlers = handlers.element(move |elem: &mut _| {
                let elem: &'static mut Element = unsafe { std::mem::transmute(elem) };
                Python::with_gil(|py| {
                    let _result = handler
                        .call(py, (PyElement::new(elem),), None)
                        .map_err(|_e| PyRuntimeError::new_err("failed to invoke callback"))?;
                    Ok(())
                })
            })
        }

        (Cow::Owned(self.selector.parse().unwrap()), handlers)
    }
}

#[pyclass(unsendable)]
#[derive(Clone)]
struct RewriteStrSettings(Rc<lol_html::RewriteStrSettings<'static, 'static>>);

/// Python bindings of lol-html.
#[pymodule]
fn lolhtml(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rewrite_str, m)?)?;
    m.add_class::<RewriteStrSettings>()?;
    m.add_class::<PyElementContentHandler>()?;
    m.add("RewritingError", py.get_type::<RewritingError>())?;
    element::register(py, m)?;
    Ok(())
}
