use lol_html::html_content::{Element, EndTag};
use pyo3::exceptions::PyException;
use pyo3::prelude::*;

use crate::rewritable_units::{tokens::end_tag::PyEndTag, PyContentType};

pub(crate) fn register(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyElement>()?;
    m.add("TagNameError", py.get_type::<PyTagNameError>())?;
    m.add("EndTagError", py.get_type::<PyEndTagError>())?;
    Ok(())
}

pyo3::create_exception!(module, PyTagNameError, PyException);
pyo3::create_exception!(module, PyEndTagError, PyException);

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
    /// Returns the tag name of the element.
    #[inline]
    fn tag_name(&self) -> String {
        self.0.tag_name()
    }

    /// Sets the tag name of the element.
    #[inline]
    fn set_tag_name(&mut self, name: &str) -> PyResult<()> {
        Ok(self
            .0
            .set_tag_name(name)
            .map_err(|e| PyTagNameError::new_err(e.to_string()))?)
    }

    /// Returns the [namespace URI] of the element.
    ///
    /// [namespace URI]: https://developer.mozilla.org/en-US/docs/Web/API/Element/namespaceURI
    #[inline]
    fn namespace_uri(&self) -> &'static str {
        self.0.namespace_uri()
    }

    /// Returns an immutable collection of element's attributes.
    #[inline]
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

    /// Returns the value of an attribute with the `name`.
    ///
    /// Returns `None` if the element doesn't have an attribute with the `name`.
    #[inline]
    fn get_attribute(&self, name: &str) -> Option<String> {
        self.0.get_attribute(name)
    }

    /// Returns `true` if the element has an attribute with `name`.
    #[inline]
    fn has_attribute(&self, name: &str) -> bool {
        self.0.has_attribute(name)
    }

    /// Sets `value` of element's attribute with `name`.
    ///
    /// If element doesn't have an attribute with the `name`, method adds new attribute
    /// to the element with `name` and `value`.
    #[inline]
    fn set_attribute(&mut self, name: &str, value: &str) -> PyResult<()> {
        Ok(self
            .0
            .set_attribute(name, value)
            .map_err(|_e| pyo3::exceptions::PyRuntimeError::new_err("something went wrong"))?)
    }

    /// Removes an attribute with the `name` if it is present.
    #[inline]
    fn remove_attribute(&mut self, name: &str) {
        self.0.remove_attribute(name)
    }

    /// Inserts `content` before the element.
    ///
    /// Consequent calls to the method append `content` to the previously inserted content.
    #[inline]
    fn before(&mut self, content: &str, content_type: PyContentType) {
        self.0.before(content, content_type.into())
    }

    /// Inserts `content` after the element.
    ///
    /// Consequent calls to the method prepend `content` to the previously inserted content.
    #[inline]
    fn after(&mut self, content: &str, content_type: PyContentType) {
        self.0.after(content, content_type.into())
    }

    /// Prepends `content` to the element's inner content, i.e. inserts content right after
    /// the element's start tag.
    ///
    /// Consequent calls to the method prepend `content` to the previously inserted content.
    /// A call to the method doesn't make any effect if the element is an [empty element].
    ///
    /// [empty element]: https://developer.mozilla.org/en-US/docs/Glossary/Empty_element
    #[inline]
    fn prepend(&mut self, content: &str, content_type: PyContentType) {
        self.0.prepend(content, content_type.into())
    }

    /// Appends `content` to the element's inner content, i.e. inserts content right before
    /// the element's end tag.
    ///
    /// Consequent calls to the method append `content` to the previously inserted content.
    /// A call to the method doesn't make any effect if the element is an [empty element].
    ///
    /// [empty element]: https://developer.mozilla.org/en-US/docs/Glossary/Empty_element
    #[inline]
    fn append(&mut self, content: &str, content_type: PyContentType) {
        self.0.append(content, content_type.into())
    }

    /// Replaces inner content of the element with `content`.
    ///
    /// Consequent calls to the method overwrite previously inserted content.
    /// A call to the method doesn't make any effect if the element is an [empty element].
    ///
    /// [empty element]: https://developer.mozilla.org/en-US/docs/Glossary/Empty_element
    #[inline]
    fn set_inner_content(&mut self, content: &str, content_type: PyContentType) {
        self.0.set_inner_content(content, content_type.into())
    }

    /// Replaces the element and its inner content with `content`.
    ///
    /// Consequent calls to the method overwrite previously inserted content.
    #[inline]
    fn replace(&mut self, content: &str, content_type: PyContentType) {
        self.0.replace(content, content_type.into())
    }

    /// Removes the element and its inner content.
    #[inline]
    fn remove(&mut self) {
        self.0.remove()
    }

    /// Removes the element, but keeps its content. I.e. remove start and end tags of the element.
    #[inline]
    fn remove_and_keep_content(&mut self) {
        self.0.remove_and_keep_content()
    }

    /// Returns `true` if the element has been removed or replaced with some content.
    #[inline]
    fn removed(&self) -> bool {
        self.0.removed()
    }

    /// Sets a handler to run when the end tag is reached.
    ///
    /// Subsequent calls to the method on the same element replace the previous handler.
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
