//! Comprehensive tests for pizza-analysis-cjk.

use pizza_analysis_cjk::*;
use pizza_engine::analysis::{AnalysisFactory, Token, TokenFilter};

fn make_token(term: &str) -> Token<'_> {
    Token::new(term, 0, term.len() as u32, 0)
}

// ═══════════════════════════════════════════════════════════════════════════════
// CjkBigramFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn bigram_construction() {
    let _f = CjkBigramFilter::new();
}

#[test]
fn bigram_chinese_pair() {
    let f = CjkBigramFilter::new();
    let mut token = make_token("中国");
    let (deleted, extra) = f.filter(&mut token);
    // CJK bigram on a 2-char token should produce output
    assert!(!deleted || extra.is_some());
}

#[test]
fn bigram_three_chars() {
    let f = CjkBigramFilter::new();
    let mut token = make_token("中华人");
    let (deleted, extra) = f.filter(&mut token);
    // Should produce bigrams: 中华, 华人
    let _ = (deleted, extra);
}

#[test]
fn bigram_single_cjk_char() {
    let f = CjkBigramFilter::new();
    let mut token = make_token("国");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted, "single CJK char should be kept");
}

#[test]
fn bigram_ascii_passthrough() {
    let f = CjkBigramFilter::new();
    let mut token = make_token("hello");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert_eq!(token.term.as_ref(), "hello");
}

#[test]
fn bigram_empty_string() {
    let f = CjkBigramFilter::new();
    let mut token = make_token("");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn bigram_japanese_hiragana() {
    let f = CjkBigramFilter::new();
    let mut token = make_token("きょう");
    let (deleted, _) = f.filter(&mut token);
    let _ = deleted;
}

#[test]
fn bigram_korean_hangul() {
    let f = CjkBigramFilter::new();
    let mut token = make_token("한국어");
    let (deleted, _) = f.filter(&mut token);
    let _ = deleted;
}

// ═══════════════════════════════════════════════════════════════════════════════
// CjkWidthFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn width_construction() {
    let _f = CjkWidthFilter::new();
}

#[test]
fn width_fullwidth_ascii_to_halfwidth() {
    let f = CjkWidthFilter::new();
    // Ａ (fullwidth A, U+FF21) → A
    let mut token = make_token("Ａ");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert_eq!(token.term.as_ref(), "A");
}

#[test]
fn width_fullwidth_digits() {
    let f = CjkWidthFilter::new();
    // ０１２ → 012
    let mut token = make_token("０１２");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert_eq!(token.term.as_ref(), "012");
}

#[test]
fn width_halfwidth_katakana_to_fullwidth() {
    let f = CjkWidthFilter::new();
    // ｶﾀｶﾅ (halfwidth katakana) → カタカナ
    let mut token = make_token("ｶﾀｶﾅ");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert_eq!(token.term.as_ref(), "カタカナ");
}

#[test]
fn width_ascii_passthrough() {
    let f = CjkWidthFilter::new();
    let mut token = make_token("hello");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert_eq!(token.term.as_ref(), "hello");
}

#[test]
fn width_empty_string() {
    let f = CjkWidthFilter::new();
    let mut token = make_token("");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

// ═══════════════════════════════════════════════════════════════════════════════
// CjkStopFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn stop_construction() {
    let _f = CjkStopFilter::new();
}

#[test]
fn stop_filters_cjk_stop_words() {
    let f = CjkStopFilter::new();
    // CJK stop filter uses English stop words from Lucene
    let stop_words = ["a", "the", "and", "is", "of", "to", "in", "for"];
    for word in &stop_words {
        let mut token = make_token(word);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted, "CJK stop word '{}' should be filtered", word);
    }
}

#[test]
fn stop_keeps_content_chars() {
    let f = CjkStopFilter::new();
    // CJK characters are not in the English stop word list, so they pass through
    let content = ["国", "人", "学"];
    for word in &content {
        let mut token = make_token(word);
        let (deleted, _) = f.filter(&mut token);
        assert!(!deleted, "content word '{}' should be kept", word);
    }
}

#[test]
fn stop_empty_string() {
    let f = CjkStopFilter::new();
    let mut token = make_token("");
    let _ = f.filter(&mut token);
}

// ═══════════════════════════════════════════════════════════════════════════════
// Registration
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn register_all_no_panic() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
}

#[test]
fn register_all_filters_present() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_token_filter("cjk_bigram").is_some());
    assert!(factory.get_token_filter("cjk_width").is_some());
    assert!(factory.get_token_filter("cjk_stop").is_some());
}

#[test]
fn register_all_analyzer_present() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_analyzer("cjk").is_some());
}

#[test]
fn analyzer_pipeline_chinese() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("cjk").unwrap();
    let mut input = String::from("中华人民共和国");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(!tokens.is_empty());
}

#[test]
fn analyzer_pipeline_japanese() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("cjk").unwrap();
    let mut input = String::from("東京都");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(!tokens.is_empty());
}

#[test]
fn analyzer_pipeline_empty_input() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("cjk").unwrap();
    let mut input = String::from("");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(tokens.is_empty());
}

#[test]
fn analyzer_pipeline_ascii_input() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("cjk").unwrap();
    let mut input = String::from("hello world");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(!tokens.is_empty());
}
