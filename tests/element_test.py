#!/usr/bin/env python3

from lolhtml import ContentType, rewrite_str, ElementContentHandler
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

    rewrite_str(html, [ElementContentHandler(
        selector, check_if_called), ElementContentHandler("inner-remove-me", edit)])

    assert handler_called, "Handler not called."


def test_empty_tag_name():
    def handler(el):
        with pytest.raises(RuntimeError):
            err = el.set_tag_name("")

    rewrite_element(r"<div>", "div", handler)
