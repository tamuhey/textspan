use pyo3::prelude::*;
use textspan::Span;

#[pymodule]
fn textspan(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", "0.3.0")?;

    /// Converts the spans defined in `text` to those defined in `original_text`.
    ///
    /// This is useful, for example, when you want to get the spans in the
    /// original text of spans obtained in the normalized text.
    ///
    /// Examples:
    ///
    ///     >>> import textspan
    ///     >>> spans = [(0, 3), (3, 6)];
    ///     >>> text = "foobarbaz";
    ///     >>> original_text = "FOo.BåR baZ";
    ///     >>> textspan.align_spans(spans, text, original_text)
    ///     [[(0, 3)], [(4, 7)]]
    #[pyfn(m, "align_spans")]
    #[text_signature = "(spans, text, original_text)"]
    pub fn align_spans(
        _py: Python,
        spans: Vec<Span>,
        text: &str,
        original_text: &str,
    ) -> PyResult<Vec<Vec<Span>>> {
        Ok(textspan::align_spans(&spans, text, original_text))
    }

    /// Converts the spans by the given `mapping`.
    ///
    /// Generally speaking, character correspondence between two texts is not
    /// necessarily surjective, not injective, not even a methematical map -
    /// some character in `textA` may not have a correspondence in `textB`,
    /// or may have multiple correspondences in `textB`. Thus, you should
    /// provide `mapping` as `List[List[Tuple[int,int]]]`.
    ///
    /// Examples:
    ///     >>> import textspan
    ///     >>> spans = [(0, 2), (3, 4)]
    ///     >>> mapping = [[0, 1], [], [2], [4, 5, 6]]
    ///     >>> textspan.align_spans_by_mapping(spans, mapping)
    ///     [[(0, 2)], [(4, 7)]]
    #[pyfn(m, "align_spans_by_mapping")]
    #[text_signature = "(spans, mapping)"]
    pub fn align_spans_by_mapping(
        _py: Python,
        spans: Vec<Span>,
        mapping: Vec<Vec<usize>>,
    ) -> PyResult<Vec<Vec<Span>>> {
        Ok(textspan::align_spans_by_mapping(&spans, &mapping))
    }

    /// Returns the span indices of `original_text` from the tokens based on the shortest edit script (SES).
    ///
    /// This is useful, for example, when you want to get the spans in the
    /// original text of tokens obtained in the normalized text.
    ///
    /// Examples:
    ///     >>> import textspan
    ///     >>> tokens = ["foo", "bar"]
    ///     >>> textspan.get_original_spans(tokens, "FO.o  BåR")
    ///     [[(0, 2), (3, 4)], [(6, 9)]]
    ///
    #[pyfn(m, "get_original_spans")]
    #[text_signature = "(tokens, original_text)"]
    pub fn get_original_spans(
        _py: Python,
        tokens: Vec<&str>,
        original_text: &str,
    ) -> PyResult<Vec<Vec<Span>>> {
        Ok(textspan::get_original_spans(&tokens, original_text))
    }

    /// Remove overlapping spans from given `spans`.
    ///
    /// First, longest spans are remained - if the two spans are overlapped, the
    /// first span will be remained. If the two spans are overlapped and their start
    /// positions are same, the longer span will be remained.
    ///
    /// Examples:
    ///     >>> import textspan
    ///     >>> spans = [(0, 2), (0, 3), (2, 4), (5, 7)]
    ///     >>> textspan.remove_span_overlaps(spans)
    ///     [(0, 3), (5, 7)]
    ///
    ///
    #[pyfn(m, "remove_span_overlaps")]
    #[text_signature = "(spans)"]
    pub fn remove_span_overlaps(_py: Python, spans: Vec<Span>) -> PyResult<Vec<Span>> {
        Ok(textspan::remove_span_overlaps(&spans))
    }

    Ok(())
}
