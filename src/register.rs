use alloc::boxed::Box;
use alloc::vec;
use pizza_engine::analysis::AnalysisFactory;
use pizza_engine::analysis::Analyzer;
use pizza_engine::analysis::StandardTokenizer;
use pizza_engine::analysis::TokenFilter;

use crate::bigram::CjkBigramFilter;
use crate::stop::CjkStopFilter;
use crate::width::CjkWidthFilter;

/// Register all CJK analysis components into the factory.
///
/// Registers:
/// - `"cjk"` analyzer (width → lowercase → bigram → stop)
/// - `"cjk_bigram"` token filter
/// - `"cjk_width"` token filter
/// - `"cjk_stop"` token filter
pub fn register_all(factory: &mut AnalysisFactory) {
    // Token filters
    factory.register_token_filter_with("cjk_bigram", || Box::new(CjkBigramFilter::new()));
    factory.register_token_filter_with("cjk_width", || Box::new(CjkWidthFilter::new()));
    factory.register_token_filter_with("cjk_stop", || Box::new(CjkStopFilter::new()));

    // Analyzer: standard CJK pipeline
    factory.register_analyzer_with("cjk", || {
        let filters: Vec<Box<dyn TokenFilter>> = vec![
            Box::new(CjkWidthFilter::new()),
            Box::new(CjkBigramFilter::new()),
            Box::new(CjkStopFilter::new()),
        ];
        Analyzer::new(vec![], Box::new(StandardTokenizer::new()), filters)
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_all_no_panic() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
    }

    #[test]
    fn test_filters_registered() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        assert!(factory.get_token_filter("cjk_bigram").is_some());
        assert!(factory.get_token_filter("cjk_width").is_some());
        assert!(factory.get_token_filter("cjk_stop").is_some());
    }

    #[test]
    fn test_analyzer_registered() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        assert!(factory.get_analyzer("cjk").is_some());
    }
}
