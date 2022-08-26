use lol_html::html_content::Element;
use pyo3::prelude::*;

pub(crate) fn register(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyElement>()?;
    m.add_class::<ContentType>()?;
    Ok(())
}

#[pyclass]
pub(crate) struct ContentType(lol_html::html_content::ContentType);

impl Clone for ContentType {
    fn clone(&self) -> Self {
        use lol_html::html_content::ContentType;
        match self.0 {
            ContentType::Html => Self(ContentType::Html),
            ContentType::Text => Self(ContentType::Text),
        }
    }
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
            .map_err(|_e| pyo3::exceptions::PyRuntimeError::new_err("something went wrong"))?)
    }

    fn namespace_uri(&self) -> &'static str {
        self.0.namespace_uri()
    }

    // fn attributes(&self) -> PyList<PyAttribute> {
    //     todo!()
    // }

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

    fn before(&mut self, content: &str, content_type: ContentType) {
        self.0.before(content, content_type.0)
    }

    fn after(&mut self, content: &str, content_type: ContentType) {
        self.0.after(content, content_type.0)
    }

    fn prepend(&mut self, content: &str, content_type: ContentType) {
        self.0.prepend(content, content_type.0)
    }

    fn append(&mut self, content: &str, content_type: ContentType) {
        self.0.append(content, content_type.0)
    }

    fn set_inner_content(&mut self, content: &str, content_type: ContentType) {
        self.0.set_inner_content(content, content_type.0)
    }

    fn replace(&mut self, content: &str, content_type: ContentType) {
        self.0.replace(content, content_type.0)
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

    fn on_end_tag(&mut self, _handler: Option<PyObject>) -> PyResult<()> {
        todo!()
    }
}
