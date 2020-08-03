#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;
use tokenizations;

pub type Span = (usize, usize);
fn substr(text: &str, start: usize, end: usize) -> &str {
    if start >= end {
        return "";
    }
    let mut it = text.char_indices();
    let l = it.nth(start).map(|(x, _)| x);
    let r = it.nth(end - start - 1).map(|(x, _)| x);
    match (l, r) {
        (Some(l), Some(r)) => &text[l..r],
        (Some(l), None) => &text[l..],
        (None, _) => "",
    }
}

pub fn align_spans(spans: &[Span], text: &str, original_text: &str) -> Vec<Option<Span>> {
    let span_texts: Vec<_> = spans
        .iter()
        .map(|&(start, end)| substr(text, start, end))
        .collect();
    tokenizations::get_original_spans(&span_texts, original_text)
}

pub fn align_spans_by_mapping(spans: &[Span], mapping: &[usize]) -> Vec<Span> {
    let mut ret = vec![];
    for &(start, end) in spans {
        let mut l = None;
        let mut r = None;
        let mut prevy = None;
        for i in start..end {
            let y = mapping[i];
            if prevy != None && prevy.unwrap() + 1 < y {
                ret.push((l.unwrap(), r.unwrap()));
                l = None;
            } else {
                r = Some(y + 1);
            }
            if l == None {
                l = Some(y);
                r = Some(y + 1);
            }
            prevy = Some(y);
        }
        if let Some(l) = l {
            ret.push((l, r.unwrap()));
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::collection as pc;
    use proptest::prelude::*;
    use proptest::strategy::Strategy;
    fn slow_substr(text: &str, start: usize, end: usize) -> String {
        if start >= end {
            "".to_owned()
        } else {
            text.chars()
                .skip(start)
                .take(end - start)
                .collect::<String>()
        }
    }
    #[test]
    fn substr_handmade() {
        for &(case, expected) in vec![
            (("今日はいい天気だ", 1, 4), "日はい"),
            (("今日", 0, 10), "今日"),
            (("明日はaaaどうだろうか", 2, 8), "はaaaどう"),
            (("\u{0}", 1, 2), ""),
        ]
        .iter()
        {
            let (text, start, end) = case;
            assert_eq!(substr(text, start, end), expected);
            assert_eq!(substr(text, start, end), slow_substr(text, start, end));
        }
    }

    #[quickcheck]
    fn substr_quickcheck(text: String, start: usize, end: usize) {
        let ret = substr(&text, start, end);
        if start >= end {
            assert_eq!(ret, "");
        } else {
            let expected = slow_substr(&text, start, end);
            assert_eq!(ret, &expected);
        }
    }

    #[test]
    fn align_spans_by_mapping_handmade() {
        for (case, expected) in vec![
            ((vec![], vec![]), vec![]),
            (
                (vec![(1, 3), (4, 6)], vec![0, 1, 2, 5, 6, 9, 10]),
                vec![(1, 3), (6, 7), (9, 10)],
            ),
        ]
        .iter()
        {
            let (spans, mapping) = case;
            assert_eq!(align_spans_by_mapping(spans, mapping), *expected);
        }
    }

    fn cases_align_spans_by_mapping(
        max_length: usize,
    ) -> impl Strategy<Value = ((usize, usize), Vec<usize>)> {
        pc::vec(0..4usize, 0..max_length)
            .prop_map(|v| {
                v.iter()
                    .scan(0, |s, x| {
                        *s += x;
                        Some(*s)
                    })
                    .collect()
            })
            .prop_flat_map(|v: Vec<usize>| {
                let l = v.len();
                ((0..=l, 0..=l), Just(v))
            })
    }
    fn check_align(span: Span, mapping: &[usize], ret: &[Span]) {
        let (start, end) = span;
        if start >= end {
            assert_eq!(ret.len(), 0)
        } else {
            let rev = |x: usize| mapping.iter().position(|y| *y == x).unwrap();
            let l = rev(ret[0].0);
            assert_eq!(mapping[l], mapping[start]);
            let mut r = rev(ret[0].1 - 1);
            for i in 1..ret.len() {
                assert_eq!(
                    mapping[r],
                    mapping[rev(ret[i].0) - 1],
                    "connectivity\n\tret: {:?}\n",
                    ret
                );
                r = rev(ret[i].1 - 1);
            }
            assert_eq!(mapping[end - 1], mapping[r]);
        }
    }
    proptest! {
          #[test]
          fn align_spans_by_mapping_proptest((span, mapping) in cases_align_spans_by_mapping(1000)) {
              let ret = align_spans_by_mapping(&vec![span], &mapping);
              check_align(span, &mapping, &ret);
          }
    }
}
