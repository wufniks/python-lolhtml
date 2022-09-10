# python-lolhtml

Python bindings for [cloudflare/lol-html](https://github.com/cloudflare/lol-html/). Bindings are built with [PyO3/pyo3](https://github.com/PyO3/pyo3). This is *experimental*.

## Installation

``` shell
maturin develop
```

## Example

``` python
def test_http_to_https():
    def modify_scheme(elem):
        href = elem.get_attribute("href").replace("http:", "https:")
        elem.set_attribute("href", href)
    result = rewrite_str(
        r'<div><a href="http://example.com"></a></div>',
        [ElementContentHandler("a", element=modify_scheme)],
    )

    assert result == r'<div><a href="https://example.com"></a></div>'
```
