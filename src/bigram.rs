use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use pizza_engine::analysis::Token;
use pizza_engine::analysis::TokenFilter;

/// Forms bigrams from CJK (Han, Hiragana, Katakana, Hangul) characters.
///
/// Consecutive CJK characters are bigrammed: "中国人" → ["中国", "国人"].
/// Non-CJK tokens pass through unchanged.
/// Lone CJK characters emit as unigrams.
#[derive(Clone, Debug)]
pub struct CjkBigramFilter {
    output_unigrams: bool,
}

impl CjkBigramFilter {
    pub fn new() -> Self {
        Self {
            output_unigrams: false,
        }
    }

    pub fn with_output_unigrams(mut self, output: bool) -> Self {
        self.output_unigrams = output;
        self
    }
}

impl Default for CjkBigramFilter {
    fn default() -> Self {
        Self::new()
    }
}

fn is_cjk(c: char) -> bool {
    matches!(c as u32,
        0x4E00..=0x9FFF       // CJK Unified Ideographs
        | 0x3400..=0x4DBF     // CJK Extension A
        | 0x20000..=0x2A6DF   // CJK Extension B
        | 0x2A700..=0x2B73F   // CJK Extension C
        | 0x2B740..=0x2B81F   // CJK Extension D
        | 0x2B820..=0x2CEAF   // CJK Extension E
        | 0xF900..=0xFAFF     // CJK Compatibility Ideographs
        | 0x2F800..=0x2FA1F   // CJK Compat Supplement
        | 0x3040..=0x309F     // Hiragana
        | 0x30A0..=0x30FF     // Katakana
        | 0x31F0..=0x31FF     // Katakana Extensions
        | 0xAC00..=0xD7AF     // Hangul Syllables
        | 0x1100..=0x11FF     // Hangul Jamo
        | 0x3130..=0x318F     // Hangul Compatibility Jamo
    )
}

impl TokenFilter for CjkBigramFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let text = token.term.as_ref();
        let chars: Vec<char> = text.chars().collect();

        let has_cjk = chars.iter().any(|c| is_cjk(*c));
        if !has_cjk {
            return (false, None);
        }

        let mut results: Vec<String> = Vec::new();
        let mut i = 0;

        while i < chars.len() {
            if is_cjk(chars[i]) {
                let run_start = i;
                while i < chars.len() && is_cjk(chars[i]) {
                    i += 1;
                }
                let run = &chars[run_start..i];

                if run.len() == 1 {
                    results.push(run[0].to_string());
                } else {
                    for j in 0..run.len() - 1 {
                        let bigram: String = run[j..=j + 1].iter().collect();
                        results.push(bigram);
                    }
                    if self.output_unigrams {
                        for c in run {
                            results.push(c.to_string());
                        }
                    }
                }
            } else {
                i += 1;
            }
        }

        if results.is_empty() {
            return (true, None);
        }

        let first = results.remove(0);
        token.term = Cow::Owned(first);

        if results.is_empty() {
            return (false, None);
        }

        let extra: Vec<Token<'a>> = results
            .into_iter()
            .enumerate()
            .map(|(idx, s)| Token {
                term: Cow::Owned(s),
                start_offset: token.start_offset,
                end_offset: token.end_offset,
                position: token.position + (idx as u32) + 1,
            })
            .collect();

        (false, Some(extra))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bigram_basic() {
        let filter = CjkBigramFilter::new();
        let mut token = Token {
            term: Cow::Borrowed("中国人"),
            start_offset: 0,
            end_offset: 9,
            position: 0,
        };
        let (deleted, extra) = filter.filter(&mut token);
        assert!(!deleted);
        assert_eq!(token.term.as_ref(), "中国");
        let extra = extra.unwrap();
        assert_eq!(extra.len(), 1);
        assert_eq!(extra[0].term.as_ref(), "国人");
    }

    #[test]
    fn test_non_cjk_passthrough() {
        let filter = CjkBigramFilter::new();
        let mut token = Token {
            term: Cow::Borrowed("hello"),
            start_offset: 0,
            end_offset: 5,
            position: 0,
        };
        let (deleted, extra) = filter.filter(&mut token);
        assert!(!deleted);
        assert!(extra.is_none());
        assert_eq!(token.term.as_ref(), "hello");
    }

    #[test]
    fn test_single_cjk_unigram() {
        let filter = CjkBigramFilter::new();
        let mut token = Token {
            term: Cow::Borrowed("中"),
            start_offset: 0,
            end_offset: 3,
            position: 0,
        };
        let (deleted, extra) = filter.filter(&mut token);
        assert!(!deleted);
        assert!(extra.is_none());
        assert_eq!(token.term.as_ref(), "中");
    }
}
