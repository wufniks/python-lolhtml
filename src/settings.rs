use std::{borrow::Cow, sync::Arc};

use lol_html::{
    html_content::{Comment, DocumentEnd, Element, TextChunk},
    DocumentContentHandlers, ElementContentHandlers, Selector,
};
use pyo3::prelude::*;

use crate::rewritable_units::{
    document_end::PyDocumentEnd,
    element::PyElement,
    tokens::{comments::PyComment, text_chunk::PyTextChunk},
};

pub(crate) fn register(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyElementContentHandler>()?;
    m.add_class::<PyDocumentContentHandler>()?;
    Ok(())
}

#[pyclass(name = "ElementContentHandler")]
pub(crate) struct PyElementContentHandler {
    pub(crate) selector: String,
    pub(crate) element: Option<Arc<PyObject>>,
    // pub(crate) comments: Option<Py<PyAny>>,
    pub(crate) text: Option<Arc<Py<PyAny>>>,
}

#[pymethods]
impl PyElementContentHandler {
    #[new]
    fn __new__(selector: &str, element: Option<PyObject>, text: Option<PyObject>) -> Self {
        Self {
            selector: selector.to_owned(),
            element: element.map(Arc::new),
            text: text.map(Arc::new),
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
                    let _result = handler.call(py, (PyElement::new(elem),), None)?;
                    Ok(())
                })
            })
        }

        if let Some(handler) = self.text.clone() {
            handlers = handlers.text(move |text: &mut _| {
                let elem: &'static mut TextChunk = unsafe { std::mem::transmute(text) };
                Python::with_gil(|py| {
                    let _result = handler.call(py, (PyTextChunk::new(elem),), None)?;
                    Ok(())
                })
            })
        }

        (Cow::Owned(self.selector.parse().unwrap()), handlers)
    }
}

#[pyclass(name = "DocumentContentHandler")]
pub(crate) struct PyDocumentContentHandler {
    pub(crate) doctype: Option<Arc<PyObject>>,
    pub(crate) comments: Option<Arc<PyObject>>,
    pub(crate) text: Option<Arc<PyObject>>,
    pub(crate) end: Option<Arc<PyObject>>,
}

#[pymethods]
impl PyDocumentContentHandler {
    #[new]
    fn __new__(
        doctype: Option<PyObject>,
        comments: Option<PyObject>,
        text: Option<PyObject>,
        end: Option<PyObject>,
    ) -> Self {
        Self {
            doctype: doctype.map(Arc::new),
            comments: comments.map(Arc::new),
            text: text.map(Arc::new),
            end: end.map(Arc::new),
        }
    }
}

impl PyDocumentContentHandler {
    pub fn as_document_content_handlers<'h>(&self) -> DocumentContentHandlers<'h> {
        let mut handlers = DocumentContentHandlers::default();

        if let Some(handler) = self.doctype.clone() {
            handlers = handlers.doctype(move |doctype: &mut _| {
                let doctype: &'static mut Element = unsafe { std::mem::transmute(doctype) };
                Python::with_gil(|py| {
                    let _result = handler.call1(py, (PyElement::new(doctype),))?;
                    Ok(())
                })
            })
        }

        if let Some(handler) = self.comments.clone() {
            handlers = handlers.comments(move |comments: &mut _| {
                let comments: &'static mut Comment = unsafe { std::mem::transmute(comments) };
                Python::with_gil(|py| {
                    let _result = handler.call1(py, (PyComment::new(comments),))?;
                    Ok(())
                })
            })
        }

        if let Some(handler) = self.text.clone() {
            handlers = handlers.text(move |text: &mut _| {
                let text: &'static mut TextChunk = unsafe { std::mem::transmute(text) };
                Python::with_gil(|py| {
                    let _result = handler.call1(py, (PyTextChunk::new(text),))?;
                    Ok(())
                })
            })
        }

        if let Some(handler) = self.end.clone() {
            handlers = handlers.end(move |end: &mut _| {
                let end: &'static mut DocumentEnd = unsafe { std::mem::transmute(end) };
                Python::with_gil(|py| {
                    let _result = handler.call1(py, (PyDocumentEnd::new(end),))?;
                    Ok(())
                })
            })
        }

        handlers
    }
}
