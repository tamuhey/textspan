use pyo3::prelude::*;
use textspanrs::Span;

#[pymodule]
fn textspan(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", "0.2.0")?;

    #[pyfn(m, "align_spans")]
    pub fn align_spans(
        _py: Python,
        spans: Vec<Span>,
        text: &str,
        original_text: &str,
    ) -> PyResult<Vec<Vec<Span>>> {
        Ok(textspanrs::align_spans(&spans, text, original_text))
    }

    #[pyfn(m, "align_spans_by_mapping")]
    pub fn align_spans_by_mapping(
        _py: Python,
        spans: Vec<Span>,
        mapping: Vec<Vec<usize>>,
    ) -> PyResult<Vec<Vec<Span>>> {
        Ok(textspanrs::align_spans_by_mapping(&spans, &mapping))
    }

    #[pyfn(m, "get_original_spans")]
    pub fn get_original_spans(
        _py: Python,
        tokens: Vec<&str>,
        original_text: &str,
    ) -> PyResult<Vec<Vec<Span>>> {
        Ok(textspanrs::get_original_spans(&tokens, original_text))
    }

    Ok(())
}
