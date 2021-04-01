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

/// Returns the span indices of `original_text` from the tokens based on the
/// shortest edit script (SES).
///
/// This is useful, for example, when you want to get the spans in the original
/// text of tokens obtained in the normalized text.
///
/// # Examples
///
/// ```
/// let tokens = vec!["a", "la", "gorge"];
/// let original_text = "à  LA    gorge";
/// let spans = textspan::get_original_spans(&tokens, original_text);
/// assert_eq!(spans, vec![vec![(0, 1)], vec![(3, 5)], vec![(9, 14)]]);
/// ```
pub fn get_original_spans<S: Borrow<str>>(tokens: &[S], original_text: &str) -> Vec<Vec<Span>> {
    let spans = get_span_indices(tokens);
    let text = tokens.join("");
    align_spans(&spans, &text, original_text)
}

/// Converts the spans defined in `text` to those defined in `original_text`.
///
/// This is useful, for example, when you want to get the spans in the original
/// text of spans obtained in the normalized text.
///
/// # Examples
///
/// ```
/// let spans = [(0, 3), (3, 6)];
/// let text = "foobarbaz";
/// let original_text = "FOo.BåR baZ";
/// assert_eq!(
///     textspan::align_spans(&spans, text, original_text),
///     [[(0, 3)], [(4, 7)]]
/// )
/// ```
pub fn align_spans(spans: &[Span], text: &str, original_text: &str) -> Vec<Vec<Span>> {
    let (mapping, _) = tokenizations::get_charmap(text, original_text);
    align_spans_by_mapping(spans, &mapping)
}

/// Converts the spans by the given `mapping`.
/// Generally speaking, the character correspondence between two texts is not
/// necessarily surjective, not injective, not even a methematical map - some
/// character in `textA` may not have a correspondence in `textB`, or may have
/// multiple correspondences in `textB`. Thus, `mapping` should be provided as
/// `Vec<Vec<Span>>`.
///
/// # Examples
///
/// ```
/// let spans = [(0, 2), (3, 4)];
/// let mapping = [vec![0, 1], vec![], vec![2], vec![4, 5, 6]];
/// assert_eq!(
///     textspan::align_spans_by_mapping(&spans, &mapping),
///     [[(0, 2)], [(4, 7)]]
/// )
/// ```
pub fn align_spans_by_mapping<T: AsRef<[usize]>>(spans: &[Span], mapping: &[T]) -> Vec<Vec<Span>> {
    let mut ret = vec![];
    for &(start, end) in spans {
        let mut l = None;
        let mut r = None;
        let mut prevy: Option<usize> = None;
        let mut pret = vec![];
        for item in mapping.iter().take(end).skip(start) {
            for &y in item.as_ref() {
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

/// Remove overlapping spans from given `spans`.
/// First, longest spans are remained - if the two spans are overlapped, the
/// first span will be remained. If the two spans are overlapped and their start
/// positions are same, the longer span will be remained.
///
/// # Example
///
/// ```
/// use textspan::remove_span_overlaps;
/// let spans = [(0, 2), (0, 3), (2, 4), (5, 7)];
/// let ret = remove_span_overlaps(&spans);
/// assert_eq!(ret, [(0, 3), (5, 7)]);
/// ```
pub fn remove_span_overlaps(spans: &[Span]) -> Vec<Span> {
    let mut spans = spans.to_vec();
    spans.sort_by_key(|x| (x.0, !0 - x.1)); // to take first longest spans
    let mut ret = vec![];
    let mut cur = 0;
    for &(l, r) in &spans {
        if l < cur {
            continue;
        }
        ret.push((l, r));
        cur = r;
    }
    ret
}

/// Remove overlapping spans from given `spans`, and returns remained span indices.
/// First, longest spans are remained - if the two spans are overlapped, the
/// first span will be remained. If the two spans are overlapped and their start
/// positions are same, the longer span will be remained.
///
/// # Example
///
/// ```
/// use textspan::remove_span_overlaps_idx;
/// let spans = [(0, 2), (0, 3), (2, 4), (5, 7)];
/// let ret = remove_span_overlaps_idx(&spans);
/// assert_eq!(ret, [1, 3]);
/// ```
pub fn remove_span_overlaps_idx(spans: &[Span]) -> Vec<usize> {
    let mut indices: Vec<_> = (0..spans.len()).collect();
    indices.sort_by_key(|&i| {
        let (l, r) = spans[i];
        (l, !0 - r)
    });
    let mut ret = vec![];
    let mut cur = 0;
    for i in indices {
        let (l, r) = spans[i];
        if l < cur {
            continue;
        }
        ret.push(i);
        cur = r;
    }
    ret
}

/// Convert `span` indices to `target_spans` based indices.
/// Expects `target_spans` is sorted and not overlapping.
///
/// # Example
///
/// ```
/// use textspan::lift_span_index;
/// let target_spans = [(0, 3), (3, 4), (4, 9), (9, 12)];
/// assert_eq!(lift_span_index((0, 3), &target_spans), (Ok(0), Ok(1)));
/// assert_eq!(lift_span_index((0, 4), &target_spans), (Ok(0), Ok(2)));
/// assert_eq!(lift_span_index((1, 4), &target_spans), (Err(0), Ok(2)));
/// assert_eq!(lift_span_index((1, 5), &target_spans), (Err(0), Err(3)));
/// assert_eq!(lift_span_index((1, 9), &target_spans), (Err(0), Ok(3)));
/// assert_eq!(lift_span_index((0, 9), &target_spans), (Ok(0), Ok(3)));
/// assert_eq!(lift_span_index((1, 13), &target_spans), (Err(0), Err(4)));
///
/// let target_spans = [(3, 4), (4, 9), (9, 12)];
/// assert_eq!(lift_span_index((0, 9), &target_spans), (Err(0), Ok(2)));
///
/// assert_eq!(lift_span_index((0, 0), &[(0, 0)]), (Ok(0), Ok(1)));
/// assert_eq!(lift_span_index((0, 0), &[]), (Err(0), Err(0)));
pub fn lift_span_index(
    span: Span,
    target_spans: &[Span],
) -> (Result<usize, usize>, Result<usize, usize>) {
    if target_spans.is_empty() {
        return (Err(0), Err(0));
    }
    let (l, r) = span;
    // i = max i where l >= ri-1
    // if li == l Ok, else Err
    let li = {
        if target_spans[0].0 == l {
            Ok(0)
        } else if target_spans[0].1 > l {
            Err(0)
        } else {
            let mut ok = target_spans.len();
            let mut ng = 0;
            while ok - ng > 1 {
                let m = (ok + ng) / 2;
                if target_spans[m].1 > l {
                    ok = m;
                } else {
                    ng = m;
                }
            }
            if ok < target_spans.len() && target_spans[ok].0 == l {
                Ok(ok)
            } else {
                Err(ok)
            }
        }
    };
    // i = min(r <= l_i)
    // if ri-1 == r Ok, else Err
    let ri = {
        if target_spans[0].1 == r {
            Ok(1)
        } else if target_spans[0].0 >= r {
            Err(0)
        } else {
            let mut ok = target_spans.len();
            let mut ng = 0;
            while ok - ng > 1 {
                let m = (ok + ng) / 2;
                if target_spans[m].0 >= r {
                    ok = m;
                } else {
                    ng = m;
                }
            }
            if ok > 0 && target_spans[ok - 1].1 == r {
                Ok(ok)
            } else {
                Err(ok)
            }
        }
    };
    (li, ri)
}

/// Convert `spans` indices on `target_spans`
///
/// # Example
///
/// ```
/// use textspan::lift_spans_index;
/// let target_spans = [(3, 5), (5, 9), (11, 15)];
///
/// assert_eq!(lift_spans_index(&[(3, 9)], &target_spans), &[(Ok(0), Ok(2))]);
/// ```
pub fn lift_spans_index(
    spans: &[Span],
    target_spans: &[Span],
) -> Vec<(Result<usize, usize>, Result<usize, usize>)> {
    let mut ret = vec![];
    let mut cur = 0usize;
    for &(l, r) in spans {
        // i = argmin(l < ri)
        while cur < target_spans.len() && target_spans[cur].1 <= l {
            cur += 1;
        }
        let li = if cur < target_spans.len() && target_spans[cur].0 == l {
            Ok(cur)
        } else {
            Err(cur)
        };
        // i = argmin(r <= l_i)
        let mut cur = cur;
        while cur < target_spans.len() && target_spans[cur].0 < r {
            cur += 1;
        }
        let ri = if cur > 0 && target_spans[cur - 1].1 == r {
            Ok(cur)
        } else {
            Err(cur)
        };
        ret.push((li, ri));
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::collection as pc;
    use proptest::prelude::*;
    use proptest::strategy::Strategy;
    use rstest::*;
    #[quickcheck]
    fn test_lift_spans_index(spans: Vec<Span>, target_spans: Vec<Span>) {
        let sanitize = |spans: Vec<Span>| {
            let mut v = vec![];
            for (l, r) in spans {
                if l == r {
                    continue;
                }
                if l > r {
                    v.push((r, l));
                } else {
                    v.push((l, r));
                }
            }
            let mut spans = remove_span_overlaps(&v);
            spans.sort_unstable();
            spans
        };
        let spans = sanitize(spans);
        let target_spans = sanitize(target_spans);
        assert_eq!(
            lift_spans_index(&spans, &target_spans),
            spans
                .iter()
                .cloned()
                .map(|span| lift_span_index(span, &target_spans))
                .collect::<Vec<_>>(),
            "\nspans: {:?}\ntarget_spans: {:?}\n",
            spans,
            target_spans
        );
    }
    #[quickcheck]
    fn remove_span_overlaps_quick(spans: Vec<Span>) {
        let new_spans = remove_span_overlaps(&spans);
        let mut cur = 0;
        for &(l, r) in &new_spans {
            assert!(l >= cur);
            cur = r;
        }
        let indices = remove_span_overlaps_idx(&spans);
        let new_spans2: Vec<_> = indices.into_iter().map(|i| spans[i]).collect();
        assert_eq!(new_spans, new_spans2);
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
            if ret[0].is_empty() {
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
                mapping[start].is_empty() || mapping[l].iter().any(|x| mapping[start].contains(&x)),
                "compare start.
                ret: {:?}
                l  : {}
                ",
                ret,
                l
            );

            let r = rev(ret[ret.len() - 1][ret[0].len() - 1].1 - 1);
            assert!(
                mapping[end - 1].is_empty()
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
          fn align_spans_by_mapping_proptest((span, mapping) in cases_align_spans_by_mapping(1000)) {
              let ret = align_spans_by_mapping(&[span], &mapping);
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

    #[rstest(input, expected,
            case(
                (vec!["fあo①が", "bar"], "fあo1かbar"),
                vec![vec![(0, 5)], vec![(5, 8)]],
            ),
            case((vec!["New York"], "NewYork"), vec![vec![(0, 7)]]),
            case(
                (vec!["A'B", "", ""], "A B"),
                vec![vec![(0, 1), (2, 3)], vec![], vec![]],
            ),
            case(
                (vec!["A'b", ""], "a b"),
                vec![vec![(0, 1), (2, 3)], vec![]],
            ),
            case((vec!["", "", ""], ""), vec![vec![], vec![], vec![]]),
            case(
                (vec!["hello", "``world``"], "Hello \"world\""),
                vec![vec![(0, 5)], vec![(7, 12)]],
            ),
            case(
                (vec!["à", " ", "", "la", "gorge", ""], "a     lagorge"),
                vec![
                    vec![(0, 1)],
                    vec![(5, 6)],
                    vec![],
                    vec![(6, 8)],
                    vec![(8, 13)],
                    vec![],
                ],
            ),
             )]
    fn hm_get_original_spans(input: (Vec<&str>, &str), expected: Vec<Vec<(usize, usize)>>) {
        assert_eq!(
            get_original_spans(&input.0, input.1),
            expected,
            "{:?}",
            input
        );
    }
}
