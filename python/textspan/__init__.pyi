from typing import List, Tuple

Span = Tuple[int, int]

def align_spans(
    spans: List[Span], text: str, original_text: str,
) -> List[List[Span]]: ...
def align_spans_by_mapping(
    spans: List[Span], mapping: List[List[int]],
) -> List[List[Span]]: ...
def get_original_spans(tokens: List[str], original_text: str,) -> List[List[Span]]: ...

