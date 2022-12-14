#!/usr/bin/env python3

from lolhtml import ContentType, rewrite_str, ElementContentHandler, TagNameError
import pytest


def rewrite_element(html: str, selector: str, handler) -> str:
    handler_called = False

    def check_if_called(el):
        nonlocal handler_called
        handler_called = True
        handler(el)

    def edit(el):
        el.before("[before: should be removed]", ContentType.Text)
        el.after("[after: should be removed]", ContentType.Text)
        el.append("[append: should be removed]", ContentType.Text)
        el.before("[before: should be removed]", ContentType.Text)

    rewrite_str(
        html,
        element_content_handlers=[
            ElementContentHandler(selector, element=check_if_called),
            ElementContentHandler("inner-remove-me", element=edit),
        ],
    )

    assert handler_called, "Handler not called."


def test_empty_tag_name():
    def handler(el):
        with pytest.raises(TagNameError):
            el.set_tag_name("")

    rewrite_element(r"<div>", "div", handler)


def test_forbidden_characters_in_tag_name():
    def handler(el):
        for ch in [" ", "\n", "\r", "\t", "\x0C", "/", ">"]:
            with pytest.raises(TagNameError):
                el.set_tag_name(ch)

    rewrite_element(r"<div>", "div", handler)


# def test_encoding_ummappable_chars_in_tag_name():
#     raise NotImplementedError


def test_invalid_first_char_of_tag_name():
    def handler(el):
        with pytest.raises(TagNameError):
            el.set_tag_name("1foo")

    rewrite_element(r"<div>", "div", handler)


############################
# Tests from doc test
############################


def test_before():
    def element_handler(elem):
        elem.before("<bar>", ContentType.Html)
        elem.before("<qux>", ContentType.Html)
        elem.before("<quz>", ContentType.Text)

    result = rewrite_str(
        r'<div id="foo"></div>',
        element_content_handlers=[
            ElementContentHandler("#foo", element=element_handler)
        ],
    )

    assert result == r'<bar><qux>&lt;quz&gt;<div id="foo"></div>'


def test_after():
    def element_handler(elem):
        elem.after("<bar>", ContentType.Html)
        elem.after("<qux>", ContentType.Html)
        elem.after("<quz>", ContentType.Text)

    result = rewrite_str(
        r'<div id="foo"></div>',
        element_content_handlers=[
            ElementContentHandler("#foo", element=element_handler)
        ],
    )

    assert result == r'<div id="foo"></div>&lt;quz&gt;<qux><bar>'


def test_prepend():
    def handler(elem):
        elem.prepend("<bar>", ContentType.Html)
        elem.prepend("<qux>", ContentType.Html)
        elem.prepend("<quz>", ContentType.Text)

    result = rewrite_str(
        r'<div id="foo"><!-- content --></div><img>',
        element_content_handlers=[
            ElementContentHandler("#foo", element=handler),
            ElementContentHandler("img", element=handler),
        ],
    )

    assert result == r'<div id="foo">&lt;quz&gt;<qux><bar><!-- content --></div><img>'


def test_append():
    def handler(elem):
        elem.append("<bar>", ContentType.Html)
        elem.append("<qux>", ContentType.Html)
        elem.append("<quz>", ContentType.Text)

    result = rewrite_str(
        r'<div id="foo"><!-- content --></div><img>',
        element_content_handlers=[
            ElementContentHandler("#foo", element=handler),
            ElementContentHandler("img", element=handler),
        ],
    )

    assert result == r'<div id="foo"><!-- content --><bar><qux>&lt;quz&gt;</div><img>'


def test_set_inner_content():
    def handler(elem):
        elem.append("<!-- only one -->", ContentType.Html)
        elem.set_inner_content("<!-- will -->", ContentType.Html)
        elem.set_inner_content("<!-- survive -->", ContentType.Html)

    result = rewrite_str(
        r'<div id="foo"><!-- content --></div><img>',
        element_content_handlers=[
            ElementContentHandler("#foo", element=handler),
            ElementContentHandler("img", element=handler),
        ],
    )

    assert result == r'<div id="foo"><!-- survive --></div><img>'


def test_replace():
    def handler(elem):
        elem.replace("<span></span>", ContentType.Html)
        elem.replace("Hello", ContentType.Text)

    result = rewrite_str(
        r'<div id="foo"></div>',
        element_content_handlers=[
            ElementContentHandler("#foo", element=handler),
        ],
    )

    assert result == r"Hello"


def test_remove_and_keep_content():

    result = rewrite_str(
        r"<div><span><!-- 42 --></span></div>",
        element_content_handlers=[
            ElementContentHandler(
                "div", element=lambda elem: elem.remove_and_keep_content()
            ),
        ],
    )

    assert result == r"<span><!-- 42 --></span>"


def test_on_end_tag():
    buffer = ""

    def element_content_handler(el):
        nonlocal buffer
        buffer = ""

        print("element content handler")

        def end_tag_handler(end):
            nonlocal buffer
            print("name: %s" % end.name())
            if len(buffer) == 13:
                end.before("!", ContentType.Text)
            else:
                end.remove()
                name = end.name().upper()
                end.after(f"</{name}>", ContentType.Html)

        el.on_end_tag(end_tag_handler)

    def text_chunk_handler(text):
        nonlocal buffer

        print("text chunk handler")
        buffer += text.as_str()

    result = rewrite_str(
        r"<span>Short</span><span><b>13</b> characters</span>",
        element_content_handlers=[
            ElementContentHandler(
                "span", element=element_content_handler, text=text_chunk_handler
            )
        ],
    )

    # print(result)
    assert result == r"<span>Short</SPAN><span><b>13</b> characters!</span>"
