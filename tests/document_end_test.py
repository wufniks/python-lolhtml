from lolhtml import ContentType, rewrite_str, DocumentContentHandler, TagNameError


def rewrite_on_end(mocker, html: str, handler: callable) -> str:
    mocked_handler = mocker.patch("handler")

    rewrite_str(
        html,
        [],
        [DocumentContentHandler(mocked_handler)],
    )

    mocked_handler.assert_called()


def test_document_content_handler():
    def on_document_end(end):
        end.append("<bar>", ContentType.Html)
        end.append("<baz>", ContentType.Text)

    result = rewrite_str(
        r'<div id="foo"><!-- content --></div><img>',
        [],
        [DocumentContentHandler(end=on_document_end)],
    )

    assert result == r'<div id="foo"><!-- content --></div><img><bar>&lt;baz&gt;'


def test_append_to_empty_document():
    def on_document_end(end):
        end.append("<div></div>", ContentType.Html)

    result = rewrite_str(
        r'',
        [],
        [DocumentContentHandler(end=on_document_end)],
    )

    assert result == r'<div></div>'

def test_append_content():
    def on_document_end(end):
        end.append("<span>", ContentType.Html)
        end.append("world", ContentType.Text)
        end.append("<foo>", ContentType.Text)
        end.append("</span>", ContentType.Html)

    result = rewrite_str(
        r'<div><h1>Hεllo</h1></div>',
        [],
        [DocumentContentHandler(end=on_document_end)],
    )

    assert result == r'<div><h1>Hεllo</h1></div><span>world&lt;foo&gt;</span>'

def test_append_content_regression():
    def on_document_end(end):
        end.append("<foo>", ContentType.Text)

    result = rewrite_str(
        r'',
        [],
        [DocumentContentHandler(end=on_document_end)],
    )

    assert result == r'&lt;foo&gt;'
