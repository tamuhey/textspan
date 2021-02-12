use pyo3::prelude::*;
use textspan::Span;

#[pymodule]
fn textspan(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", "0.5.1")?;

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
    /// Generally speaking, the character correspondence between two texts is not
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
    ///     >>> assert textspan.remove_span_overlaps(spans) == [(0, 3), (5, 7)]
    ///
    ///
    #[pyfn(m, "remove_span_overlaps")]
    #[text_signature = "(spans)"]
    pub fn remove_span_overlaps(_py: Python, spans: Vec<Span>) -> PyResult<Vec<Span>> {
        Ok(textspan::remove_span_overlaps(&spans))
    }

    /// Remove overlapping spans from given `spans`, and returns remained span indices.
    ///
    /// First, longest spans are remained - if the two spans are overlapped, the
    /// first span will be remained. If the two spans are overlapped and their start
    /// positions are same, the longer span will be remained.
    ///
    /// Examples:
    ///     >>> import textspan
    ///     >>> spans = [(0, 2), (0, 3), (2, 4), (5, 7)]
    ///     >>> assert textspan.remove_span_overlaps_idx(spans) == [1, 3]
    ///
    ///
    #[pyfn(m, "remove_span_overlaps_idx")]
    #[text_signature = "(spans)"]
    pub fn remove_span_overlaps_idx(_py: Python, spans: Vec<Span>) -> PyResult<Vec<usize>> {
        Ok(textspan::remove_span_overlaps_idx(&spans))
    }

    fn to_tuple<T>(x: Result<T, T>) -> (T, bool) {
        match x {
            Ok(x) => (x, true),
            Err(x) => (x, false),
        }
    }

    /// Examples:
    ///     >>> import textspan
    ///     >>> spans = [(0, 3), (3, 4), (4, 9), (9, 12)]
    ///     >>> assert textspan.lift_spans_index((2, 10), spans) == (0, 4)
    #[pyfn(m, "lift_span_index")]
    #[text_signature = "(span, target_spans)"]
    pub fn lift_span_index(
        _py: Python,
        span: Span,
        target_spans: Vec<Span>,
    ) -> PyResult<((usize, bool), (usize, bool))> {
        let (l, r) = textspan::lift_span_index(span, &target_spans);
        Ok((to_tuple(l), to_tuple(r)))
    }

    #[pyfn(m, "lift_spans_index")]
    #[text_signature = "(spans, target_spans)"]
    pub fn lift_spans_index(
        _py: Python,
        spans: Vec<Span>,
        target_spans: Vec<Span>,
    ) -> PyResult<Vec<((usize, bool), (usize, bool))>> {
        Ok(textspan::lift_spans_index(&spans, &target_spans)
            .into_iter()
            .map(|(l, r)| (to_tuple(l), to_tuple(r)))
            .collect())
    }

    Ok(())
}
