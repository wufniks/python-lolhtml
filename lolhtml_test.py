from lolhtml import *

def test_rewrite_str():
    result = rewrite_str(
        r'<div><a href="http://example.com"></a></div>',
        ElementContentHandler("a", lambda elem: elem.set_attribute("class", "dummy")),
    )
    assert result == r'<div><a href="http://example.com"></a></div>'
