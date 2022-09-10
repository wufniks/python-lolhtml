#!/usr/bin/env python3

from lolhtml import ContentType, rewrite_str, ElementContentHandler, TagNameError
import pytest

############################
# Tests from doc test
############################


def test_before():
    def comment_handler(comment):
        comment.before("<!-- 42 -->", ContentType.Html)
        comment.before("bar", ContentType.Text)

    result = rewrite_str(
        r"<div><!-- foo --></div>",
        element_content_handlers=[
            ElementContentHandler("div", comments=comment_handler)
        ],
    )

    assert result == r"<div><!-- 42 -->bar<!-- foo --></div>"


def test_after():
    def handler(comment):
        comment.after("Bar", ContentType.Html)
        comment.after("Qux", ContentType.Html)

    result = rewrite_str(
        r"<div><!-- foo --></div>",
        element_content_handlers=[ElementContentHandler("div", comments=handler)],
    )

    assert result == r"<div><!-- foo -->QuxBar</div>"


def test_replace():
    def handler(comment):
        comment.replace("Bar", ContentType.Text)
        comment.replace("Qux", ContentType.Text)

    result = rewrite_str(
        r"<div><!-- foo --></div>",
        element_content_handlers=[
            ElementContentHandler("div", comments=handler),
        ],
    )

    assert result == r"<div>Qux</div>"
