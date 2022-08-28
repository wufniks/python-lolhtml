use lol_html::html_content::{ContentType, Element, EndTag};
use pyo3::exceptions::PyException;
use pyo3::prelude::*;

use crate::tokens::end_tag::PyEndTag;

pub(crate) fn register(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyElement>()?;
    m.add_class::<PyContentType>()?;
    m.add("TagNameError", py.get_type::<PyTagNameError>())?;
    m.add("EndTagError", py.get_type::<PyEndTagError>())?;
    Ok(())
}

pyo3::create_exception!(module, PyTagNameError, PyException);
pyo3::create_exception!(module, PyEndTagError, PyException);

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

#[pyclass]
pub(crate) struct Attribute {
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    value: String,
}

#[pyclass(unsendable, name = "Element")]
pub(crate) struct PyElement(&'static mut Element<'static, 'static>);

impl PyElement {
    pub fn new(element: &'static mut Element) -> Self {
        Self(element)
    }
}

#[pymethods]
impl PyElement {
    fn tag_name(&self) -> String {
        self.0.tag_name()
    }

    fn set_tag_name(&mut self, name: &str) -> PyResult<()> {
        Ok(self
            .0
            .set_tag_name(name)
            .map_err(|e| PyTagNameError::new_err(e.to_string()))?)
    }

    fn namespace_uri(&self) -> &'static str {
        self.0.namespace_uri()
    }

    fn attributes(&self) -> Vec<Attribute> {
        self.0
            .attributes()
            .iter()
            .map(|attr| Attribute {
                name: attr.name(),
                value: attr.value(),
            })
            .collect()
    }

    fn get_attribute(&self, name: &str) -> Option<String> {
        self.0.get_attribute(name)
    }

    fn has_attribute(&self, name: &str) -> bool {
        self.0.has_attribute(name)
    }

    fn set_attribute(&mut self, name: &str, value: &str) -> PyResult<()> {
        Ok(self
            .0
            .set_attribute(name, value)
            .map_err(|_e| pyo3::exceptions::PyRuntimeError::new_err("something went wrong"))?)
    }

    fn remove_attribute(&mut self, name: &str) {
        self.0.remove_attribute(name)
    }

    fn before(&mut self, content: &str, content_type: PyContentType) {
        self.0.before(content, content_type.into())
    }

    fn after(&mut self, content: &str, content_type: PyContentType) {
        self.0.after(content, content_type.into())
    }

    fn prepend(&mut self, content: &str, content_type: PyContentType) {
        self.0.prepend(content, content_type.into())
    }

    fn append(&mut self, content: &str, content_type: PyContentType) {
        self.0.append(content, content_type.into())
    }

    fn set_inner_content(&mut self, content: &str, content_type: PyContentType) {
        self.0.set_inner_content(content, content_type.into())
    }

    fn replace(&mut self, content: &str, content_type: PyContentType) {
        self.0.replace(content, content_type.into())
    }

    fn remove(&mut self) {
        self.0.remove()
    }

    fn remove_and_keep_content(&mut self) {
        self.0.remove_and_keep_content()
    }

    fn removed(&self) -> bool {
        self.0.removed()
    }

    fn on_end_tag(&mut self, handler: Option<PyObject>) -> PyResult<()> {
        if let Some(callback) = handler {
            let handler = move |end: &mut EndTag| {
                let end: &'static mut EndTag<'static> = unsafe { std::mem::transmute(end) };
                Python::with_gil(|py| {
                    let _result = callback.call(py, (PyEndTag::new(end),), None)?;
                    Ok(())
                })
            };
            self.0
                .on_end_tag(handler)
                .map_err(|e| PyEndTagError::new_err(e.to_string()))
        } else {
            Ok(())
        }
    }
}
