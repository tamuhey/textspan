# Text span utilities for Rust and Python

[![creates.io](https://img.shields.io/crates/v/textspan.svg)](https://crates.io/crates/textspan)
[![pypi](https://img.shields.io/pypi/v/pytextspan.svg)](https://pypi.org/project/pytextspan/)
[![Action Status](https://github.com/tamuhey/textspan/workflows/Test%20and%20Deploy/badge.svg)](https://github.com/tamuhey/textspan/actions)

- Rust doc: https://docs.rs/textspan


## Usage (Python)

Install: `pip install pytextspan`

### `align_spans`

```python
def align_spans(spans: List[Tuple[int, int]], text: str, original_text: str) -> List[List[Tuple[int, int]]]: ...
```

Converts the spans defined in `text` to those defined in `original_text`.

This is useful, for example, when you want to get the spans in the
original text of spans obtained in the normalized text.

```python
>>> import textspan
>>> spans = [(0, 3), (3, 6)];
>>> text = "foobarbaz";
>>> original_text = "FOo.BåR baZ";
>>> textspan.align_spans(spans, text, original_text)
[[(0, 3)], [(4, 7)]]
```

### `align_spans_by_mapping`

```python
def align_spans_by_mapping(spans: List[Tuple[int, int]], mapping: List[List[int]]) -> List[List[Tuple[int, int]]]: ...
```

Converts the spans by the given `mapping`.

Generally speaking, the character correspondence between two texts is not
necessarily surjective, not injective, not even a methematical map -
some character in `textA` may not have a correspondence in `textB`,
or may have multiple correspondences in `textB`. Thus, you should
provide `mapping` as `List[List[Tuple[int,int]]]`.

```python
>>> import textspan
>>> spans = [(0, 2), (3, 4)]
>>> mapping = [[0, 1], [], [2], [4, 5, 6]]
>>> textspan.align_spans_by_mapping(spans, mapping)
[[(0, 2)], [(4, 7)]]
```

### `get_original_spans`

```python
def get_original_spans(tokens: List[str], original_text: str) -> List[List[Tuple[int, int]]]: ...
```

Returns the span indices of `original_text` from the tokens based on the shortest edit script (SES).

This is useful, for example, when you want to get the spans in the
original text of tokens obtained in the normalized text.

```python
>>> import textspan
>>> tokens = ["foo", "bar"]
>>> textspan.get_original_spans(tokens, "FO.o  BåR")
[[(0, 2), (3, 4)], [(6, 9)]]
```

### `remove_span_overlaps`

```python
def remove_span_overlaps(tokens: List[Tuple[int, int]]) -> List[Tuple[int, int]]: ...
```

Remove overlapping spans from given `spans`.

First, longest spans are remained - if the two spans are overlapped, the
first span will be remained. If the two spans are overlapped and their start
positions are same, the longer span will be remained.

```python
>>> import textspan
>>> spans = [(0, 2), (0, 3), (2, 4), (5, 7)]
>>> textspan.remove_span_overlaps(spans)
[(0, 3), (5, 7)]
```


