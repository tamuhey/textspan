#![deny(clippy::all)]
#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;
use std::borrow::Borrow;
use std::convert::AsRef;

pub type Span = (usize, usize);

fn get_span_indices<S: Borrow<str>>(tokens: &[S]) -> Vec<Span> {
    tokens
        .iter()
        .scan(0, |state, token| {
            let l = *state;
            let r = l + token.borrow().chars().count();
            *state = r;
            Some((l, r))
        })
        .collect()
}

pub fn get_original_spans<S: Borrow<str>>(tokens: &[S], original_text: &str) -> Vec<Vec<Span>> {
    let spans = get_span_indices(tokens);
    let text = tokens.join("");
    align_spans(&spans, &text, original_text)
}

pub fn align_spans(spans: &[Span], text: &str, original_text: &str) -> Vec<Vec<Span>> {
    let (mapping, _) = tokenizations::get_charmap(text, original_text);
    align_spans_by_mapping(spans, &mapping)
}

pub fn align_spans_by_mapping<T: AsRef<[usize]>>(spans: &[Span], mapping: &[T]) -> Vec<Vec<Span>> {
    let mut ret = vec![];
    for &(start, end) in spans {
        let mut l = None;
        let mut r = None;
        let mut prevy: Option<usize> = None;
        let mut pret = vec![];
        for i in start..end {
            for &y in mapping[i].as_ref() {
                if prevy != None && prevy.unwrap() + 1 < y {
                    pret.push((l.unwrap(), r.unwrap()));
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
        }
        if let Some(l) = l {
            pret.push((l, r.unwrap()));
        }
        ret.push(pret)
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::collection as pc;
    use proptest::prelude::*;
    use proptest::strategy::Strategy;
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
    fn align_spans_handmade() {
        for (case, expected) in vec![
            ((vec![], "", ""), vec![]),
            (
                (vec![(1, 4)], "foobar", "foo.bar"),
                vec![vec![(1, 3), (4, 5)]],
            ),
            ((vec![(0, 1)], "foo", "oo"), vec![vec![]]),
            ((vec![(0, 3)], "foo", "fo0o"), vec![vec![(0, 2), (3, 4)]]),
        ]
        .iter()
        {
            let (spans, text, original_text) = case;
            assert_eq!(align_spans(spans, text, original_text), *expected);
        }
    }

    #[test]
    fn align_spans_by_mapping_handmade() {
        for (case, expected) in vec![
            ((vec![], vec![]), vec![]),
            (
                (
                    vec![(1, 3), (4, 6)],
                    vec![
                        vec![0],
                        vec![1],
                        vec![2],
                        vec![5],
                        vec![6],
                        vec![9],
                        vec![10],
                    ],
                ),
                vec![vec![(1, 3)], vec![(6, 7), (9, 10)]],
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
    ) -> impl Strategy<Value = ((usize, usize), Vec<Vec<usize>>)> {
        pc::vec((0..4usize, 0..5usize), 0..max_length)
            .prop_map(|v| {
                v.iter()
                    .scan(0, |s, (d, n)| {
                        *s += d;
                        let v: Vec<_> = (*s..(*s + n)).collect();
                        if *n > 0 {
                            *s += n - 1;
                        }
                        Some(v)
                    })
                    .collect()
            })
            .prop_flat_map(|v: Vec<Vec<usize>>| {
                let l = v.len();
                ((0..=l, 0..=l), Just(v))
            })
    }

    fn check_align(span: Span, mapping: &[Vec<usize>], ret: &[Vec<Span>]) {
        let (start, end) = span;
        if start >= end {
            assert_eq!(ret[0], vec![])
        } else {
            if ret[0].len() == 0 {
                assert_eq!(mapping[start], vec![]);
                assert_eq!(mapping[end - 1], vec![]);
                return;
            }
            let mut cur = None;
            for spans in ret {
                for (start, end) in spans {
                    if let Some(_cur) = cur {
                        assert!(start - _cur > 0);
                    }
                    cur = Some(end);
                }
            }
            let rev = |x: usize| mapping.iter().position(|y| y.contains(&x)).unwrap();
            let l = rev(ret[0][0].0);
            assert!(
                mapping[start].len() == 0 || mapping[l].iter().any(|x| mapping[start].contains(&x)),
                "compare start.
                ret: {:?}
                l  : {}
                ",
                ret,
                l
            );

            let mut r = rev(ret[0][0].1 - 1);
            for pret in ret {
                for i in 1..pret.len() {
                    assert!(
                        mapping[r]
                            .iter()
                            .any(|x| mapping[rev(pret[i].0) - 1].contains(&x)),
                        "continuity
                    ret: {:?}",
                        ret
                    );
                    r = rev(pret[i].1 - 1);
                }
            }
            assert!(
                mapping[end - 1].len() == 0
                    || mapping[end - 1].iter().any(|x| mapping[r].contains(&x)),
                "compare end
            ret: {:?}
            r  : {}
            ",
                ret,
                r
            );
        }
    }
    proptest! {
          #[test]
          fn align_spans_by_mapping_proptest((span, mapping) in cases_align_spans_by_mapping(3)) {
              let ret = align_spans_by_mapping(&vec![span], &mapping);
              check_align(span, &mapping, &ret);
          }
    }

    #[quickcheck]
    fn get_original_spans_for_clean_text_quickcheck(tokens: Vec<String>) -> bool {
        let spans = get_span_indices(&tokens);
        let output = get_original_spans(&tokens, &tokens.join(""))
            .iter()
            .scan(0, |s, x| {
                if let Some(&p) = x.first() {
                    *s = p.1;
                    Some(p)
                } else {
                    Some((*s, *s))
                }
            })
            .collect::<Vec<_>>();
        spans == output
    }

    #[test]
    fn get_original_spans_handmade() {
        let testcases = vec![
            (
                (vec!["fあo①が", "bar"], "fあo1かbar"),
                (vec![vec![(0, 5)], vec![(5, 8)]]),
            ),
            ((vec!["New York"], "NewYork"), (vec![vec![(0, 7)]])),
            (
                (vec!["A'B", "", ""], "A B"),
                (vec![vec![(0, 1), (2, 3)], vec![], vec![]]),
            ),
            (
                (vec!["A'b", ""], "a b"),
                (vec![vec![(0, 1), (2, 3)], vec![]]),
            ),
            ((vec!["", "", ""], ""), (vec![vec![], vec![], vec![]])),
            (
                (vec!["hello", "``world``"], "Hello \"world\""),
                vec![vec![(0, 5)], vec![(7, 12)]],
            ),
            (
                (vec!["à", " ", "", "la", "gorge", ""], "a     lagorge"),
                (vec![
                    vec![(0, 1)],
                    vec![(1, 2)],
                    vec![],
                    vec![(6, 8)],
                    vec![(8, 13)],
                    vec![],
                ]),
            ),
        ];
        for (input, expected) in testcases.into_iter() {
            assert_eq!(
                get_original_spans(&input.0, input.1),
                expected,
                "{:?}",
                input
            );
        }
    }
}
