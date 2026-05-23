use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use pizza_engine::analysis::Token;
use pizza_engine::analysis::TokenFilter;

/// Normalizes CJK width differences:
/// - Fullwidth ASCII (U+FF01–U+FF5E) → Basic Latin (U+0021–U+007E)
/// - Halfwidth Katakana (U+FF65–U+FF9F) → Fullwidth Katakana
///
/// Essential for CJK search where users mix fullwidth/halfwidth characters.
#[derive(Clone, Debug, Default)]
pub struct CjkWidthFilter;

impl CjkWidthFilter {
    pub fn new() -> Self {
        Self
    }
}

impl TokenFilter for CjkWidthFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let text = token.term.as_ref();

        let needs_folding = text.chars().any(|c| {
            let cp = c as u32;
            (0xFF01..=0xFF5E).contains(&cp) || (0xFF65..=0xFF9F).contains(&cp)
        });

        if !needs_folding {
            return (false, None);
        }

        let mut result = String::with_capacity(text.len());

        for c in text.chars() {
            let cp = c as u32;
            if (0xFF01..=0xFF5E).contains(&cp) {
                // Fullwidth ASCII → Basic Latin
                let folded = char::from_u32(cp - 0xFEE0).unwrap_or(c);
                result.push(folded);
            } else if (0xFF65..=0xFF9F).contains(&cp) {
                // Halfwidth Katakana → Fullwidth Katakana
                result.push(halfwidth_kana_to_full(cp));
            } else {
                result.push(c);
            }
        }

        token.term = Cow::Owned(result);
        (false, None)
    }
}

/// Convert halfwidth Katakana code point to fullwidth equivalent.
fn halfwidth_kana_to_full(cp: u32) -> char {
    let mapping: &[(u32, char)] = &[
        (0xFF65, '\u{30FB}'), // ・
        (0xFF66, '\u{30F2}'), // ヲ
        (0xFF67, '\u{30A1}'), // ァ
        (0xFF68, '\u{30A3}'), // ィ
        (0xFF69, '\u{30A5}'), // ゥ
        (0xFF6A, '\u{30A7}'), // ェ
        (0xFF6B, '\u{30A9}'), // ォ
        (0xFF6C, '\u{30E3}'), // ャ
        (0xFF6D, '\u{30E5}'), // ュ
        (0xFF6E, '\u{30E7}'), // ョ
        (0xFF6F, '\u{30C3}'), // ッ
        (0xFF70, '\u{30FC}'), // ー
        (0xFF71, '\u{30A2}'), // ア
        (0xFF72, '\u{30A4}'), // イ
        (0xFF73, '\u{30A6}'), // ウ
        (0xFF74, '\u{30A8}'), // エ
        (0xFF75, '\u{30AA}'), // オ
        (0xFF76, '\u{30AB}'), // カ
        (0xFF77, '\u{30AD}'), // キ
        (0xFF78, '\u{30AF}'), // ク
        (0xFF79, '\u{30B1}'), // ケ
        (0xFF7A, '\u{30B3}'), // コ
        (0xFF7B, '\u{30B5}'), // サ
        (0xFF7C, '\u{30B7}'), // シ
        (0xFF7D, '\u{30B9}'), // ス
        (0xFF7E, '\u{30BB}'), // セ
        (0xFF7F, '\u{30BD}'), // ソ
        (0xFF80, '\u{30BF}'), // タ
        (0xFF81, '\u{30C1}'), // チ
        (0xFF82, '\u{30C4}'), // ツ
        (0xFF83, '\u{30C6}'), // テ
        (0xFF84, '\u{30C8}'), // ト
        (0xFF85, '\u{30CA}'), // ナ
        (0xFF86, '\u{30CB}'), // ニ
        (0xFF87, '\u{30CC}'), // ヌ
        (0xFF88, '\u{30CD}'), // ネ
        (0xFF89, '\u{30CE}'), // ノ
        (0xFF8A, '\u{30CF}'), // ハ
        (0xFF8B, '\u{30D2}'), // ヒ
        (0xFF8C, '\u{30D5}'), // フ
        (0xFF8D, '\u{30D8}'), // ヘ
        (0xFF8E, '\u{30DB}'), // ホ
        (0xFF8F, '\u{30DE}'), // マ
        (0xFF90, '\u{30DF}'), // ミ
        (0xFF91, '\u{30E0}'), // ム
        (0xFF92, '\u{30E1}'), // メ
        (0xFF93, '\u{30E2}'), // モ
        (0xFF94, '\u{30E4}'), // ヤ
        (0xFF95, '\u{30E6}'), // ユ
        (0xFF96, '\u{30E8}'), // ヨ
        (0xFF97, '\u{30E9}'), // ラ
        (0xFF98, '\u{30EA}'), // リ
        (0xFF99, '\u{30EB}'), // ル
        (0xFF9A, '\u{30EC}'), // レ
        (0xFF9B, '\u{30ED}'), // ロ
        (0xFF9C, '\u{30EF}'), // ワ
        (0xFF9D, '\u{30F3}'), // ン
        (0xFF9E, '\u{3099}'), // ゙ (voiced mark)
        (0xFF9F, '\u{309A}'), // ゚ (semi-voiced mark)
    ];

    for &(code, full) in mapping {
        if cp == code {
            return full;
        }
    }
    char::from_u32(cp).unwrap_or('?')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fullwidth_ascii_to_basic_latin() {
        let filter = CjkWidthFilter::new();
        let mut token = Token {
            term: Cow::Borrowed("\u{FF21}\u{FF22}\u{FF23}"), // ＡＢＣ
            start_offset: 0,
            end_offset: 9,
            position: 0,
        };
        let (deleted, _) = filter.filter(&mut token);
        assert!(!deleted);
        assert_eq!(token.term.as_ref(), "ABC");
    }

    #[test]
    fn test_no_change_needed() {
        let filter = CjkWidthFilter::new();
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
}
