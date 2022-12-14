from lolhtml import rewrite_str, ElementContentHandler


def test_http_to_https():
    def modify_scheme(elem):
        href = elem.get_attribute("href").replace("http:", "https:")
        elem.set_attribute("href", href)

    result = rewrite_str(
        r'<div><a href="http://example.com"></a></div>',
        element_content_handlers=[ElementContentHandler("a", element=modify_scheme)],
    )
    assert result == r'<div><a href="https://example.com"></a></div>'


def test_lambda():
    result = rewrite_str(
        r'<div><a href="http://example.com"></a></div>',
        element_content_handlers=[
            ElementContentHandler(
                "a", element=lambda elem: elem.set_attribute("class", "test")
            )
        ],
    )
    assert result == r'<div><a href="http://example.com" class="test"></a></div>'
