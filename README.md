# Text span utility

## Usage (Python)

### `get_original_spans`

```python
>>> import textspan
>>> tokens = ["foo", "bar"]
>>> textspan.get_original_spans(tokens, "FO.o  BåR")
[[(0, 2), (3, 4)], [(6, 9)]]
```

