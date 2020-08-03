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

    // fn cases_align_spans_by_mapping() -> impl Strategy<Value = (Vec<(usize, usize)>, Vec<usize>)> {
    //     let mapping = pc::vec(0..4usize, 0..10000usize).prop_map(|v| {
    //         v.iter()
    //             .scan(0, |s, x| {
    //                 *s += x;
    //                 Some(*s)
    //             })
    //             .collect()
    //     });
    //     let span = prop::strategy::
    // }
    // prop_compose! {
    //     // Generate arbitrary integers up to half the maximum desired value,
    //     // then multiply them by 2, thus producing only even integers in the
    //     // desired range.
    //     fn even_integer(max: i32)(base in 0..max/2) -> i32 { base * 2 }
    // }
    // fn foo() -> impl Strategy<Value = usize> {
    //     let v = even_integer(10);
    //     (0..10).into()
    // }
    // proptest! {
    //       #[test]
    //       fn align_spans_by_mapping_proptest(v in pc::vec(0..3usize, 1..4usize)) {
    //           sum(&v);
    //       }
    // }
}
