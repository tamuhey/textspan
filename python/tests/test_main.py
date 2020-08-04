import pytest
from hypothesis import strategies as st, given
import textspan


@pytest.mark.parametrize(
    "spans,text,original_text,expected",
    [([(0, 1), (3, 7)], "foobarbaz", "foo bar baz", [[(0, 1)], [(4, 7), (8, 9)]])],
)
def test_align_spans(spans, text, original_text, expected):
    assert textspan.align_spans(spans, text, original_text) == expected


@pytest.mark.parametrize(
    "spans,mapping,expected",
    [
        (
            [(0, 2), (2, 5)],
            [[0], [1], [2, 3], [], [5, 7]],
            [[(0, 2)], [(2, 4), (5, 6), (7, 8)]],
        )
    ],
)
def test_align_spans_by_mapping(spans, mapping, expected):
    assert textspan.align_spans_by_mapping(spans, mapping) == expected


@given(st.lists(st.text()), st.text())
def test_random_get_original_spans(tokens, text):
    textspan.get_original_spans(tokens, text)
    ret = textspan.get_original_spans(tokens, "".join(tokens))
    assert all(x is not None for x in ret)


@pytest.mark.parametrize(
    "tokens,text,expected",
    [
        (["Hello", "world"], "Hello world", [[(0, 5)], [(6, 11)]]),
        (["hello", "``world``"], 'Hello "world"', [[(0, 5)], [(7, 12)]]),
    ],
)
def test_random_get_original_spans(tokens, text, expected):
    ret = textspan.get_original_spans(tokens, text)
    assert ret == expected, (tokens, text)
