from typing import List, Tuple

def align_spans(
    spans: List[Tuple[int, int]], text: str, original_text: str,
) -> List[List[Tuple[int, int]]]:
    """Converts the spans defined in `text` to those defined in `original_text`.

    It is useful, for example, when you get the spans on normalized text but you
    want the spans in original, unnormalized text.

    Examples:
        >>> spans = [(0, 2), (3, 4)]
        >>> mapping = [[0, 1], [], [2], [4, 5, 6]]
        >>> align_spans_by_mapping(&spans, &mapping)
        [[(0, 2)], [(4, 7)]]
    """
    pass

def align_spans_by_mapping(
    spans: List[Tuple[int, int]], mapping: List[List[int]],
) -> List[List[Tuple[int, int]]]:
    """Converts the spans by the given `mapping`.

    Generally speaking, character correspondence between two texts is not
    necessarily surjective, not injective, not even a methematical map - some
    character in `textA` may not have a correspondence in `textB`, or may have
    multiple correspondences in `textB`. Thus, you should provide `mapping` as
    `List[List[Tuple[int,int]]]`.

    Examples:
        >>> spans = [(0, 2), (3, 4)]
        >>> mapping = [vec![0, 1], vec![], vec![2], vec![4, 5, 6]]
        >>> align_spans_by_mapping(spans, mapping)
        [[(0, 2)], [(4, 7)]]
    """
    pass

def get_original_spans(
    tokens: List[str], original_text: str,
) -> List[List[Tuple[int, int]]]:
    """Returns the span indices of `original_text` from the tokens based on the shortest edit script (SES).

    Examples:
        >>> import textspan
        >>> tokens = ["foo", "bar"]
        >>> textspan.get_original_spans(tokens, "FO.o  BåR")
        [[(0, 2), (3, 4)], [(6, 9)]]
    """
    pass

