# Text span utility

[![creates.io](https://img.shields.io/crates/v/textspan.svg)](https://crates.io/crates/textspan)
[![pypi](https://img.shields.io/pypi/v/pytextspan.svg)](https://pypi.org/project/pytextspan/)
[![Action Status](https://github.com/tamuhey/textspan/workflows/Test%20and%20Deploy/badge.svg)](https://github.com/tamuhey/textspan/actions)

- Rust doc: https://docs.rs/textspan


## Usage (Python)

Install: `pip install pytextspan`

### `get_original_spans`

```python
def get_original_spans(
    tokens: List[str], original_text: str,
) -> List[List[Tuple[int, int]]]: ...
```

Returns the span indices of `original_text` from the tokens based on the shortest edit script (SES).

This is useful, for example, when modifying the spans from normalized text to its original text position.

```python
>>> import textspan
>>> tokens = ["foo", "bar"]
>>> textspan.get_original_spans(tokens, "FO.o  BåR")
[[(0, 2), (3, 4)], [(6, 9)]]
```

### `align_spans`

```python
def align_spans(
    spans: List[Tuple[int, int]], text: str, original_text: str,
) -> List[List[Tuple[int, int]]]: ...
```

Converts the spans defined in `text` to those defined in `original_text`.
It is useful, for example, when you get the spans on normalized text but you
want the spans in original, unnormalized text.

```python
>>> spans = [(0, 2), (3, 4)]
>>> mapping = [[0, 1], [], [2], [4, 5, 6]]
>>> align_spans_by_mapping(&spans, &mapping)
[[(0, 2)], [(4, 7)]]
```

### `align_spans_by_mapping`

```Python
def align_spans_by_mapping(
    spans: List[Tuple[int, int]], mapping: List[List[int]],
) -> List[List[Tuple[int, int]]]: ...
```

Converts the spans by the given `mapping`.

```python
>>> spans = [(0, 2), (3, 4)]
>>> mapping = [vec![0, 1], vec![], vec![2], vec![4, 5, 6]]
>>> align_spans_by_mapping(spans, mapping)
[[(0, 2)], [(4, 7)]]
```
