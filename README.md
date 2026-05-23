<div align="center">

# 🀄 pizza-analysis-cjk

**CJK (Chinese/Japanese/Korean) bigram analysis for [INFINI Pizza](https://pizza.rs)**

[![Crate](https://img.shields.io/badge/crate-pizza--analysis--cjk-blue)](https://github.com/pizza-rs/analysis-cjk)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)

</div>

---

## Overview

CJK bigram tokenization and width normalization for Chinese, Japanese, and Korean text.
Provides dictionary-free CJK analysis by generating overlapping character bigrams —
a simple but effective approach when morphological analysis is not needed.

## Components

| Type | Name | Description |
|:-----|:-----|:------------|
| TokenFilter | `cjk_bigram` | Generate overlapping bigrams from CJK character runs |
| TokenFilter | `cjk_width` | Normalize fullwidth ↔ halfwidth characters (Ａ→A, ｶ→カ) |
| TokenFilter | `cjk_stop` | Common CJK stop characters (35 entries) |
| Analyzer | `cjk` | Full pipeline: cjk_width → cjk_bigram → stop |

### How CJK Bigram Works

For input `中华人民`, generates overlapping pairs:
- `中华`, `华人`, `人民`

This enables substring matching without dictionary segmentation. For higher-quality
Chinese segmentation, use `analysis-jieba` or `analysis-smartcn` instead.

### Width Normalization

| Input | Output | Description |
|:------|:-------|:------------|
| Ａ Ｂ Ｃ | A B C | Fullwidth ASCII → ASCII |
| ｶ ﾀ ｶ ﾅ | カタカナ | Halfwidth katakana → fullwidth |

## Example

```rust
use pizza_engine::analysis::AnalysisFactory;

let mut factory = AnalysisFactory::new();
pizza_analysis_cjk::register_all(&mut factory);

let analyzer = factory.get_analyzer("cjk").unwrap();
// "東京都" → bigrams: ["東京", "京都"]
```

## Installation

```toml
[dependencies]
pizza-analysis-cjk = "0.1"
```

Or via `pizza-analysis-all`:

```toml
[dependencies]
pizza-analysis-all = { version = "0.1", features = ["cjk"] }
```

## License

MIT

---

<div align="center">
<sub>Part of the <a href="https://pizza.rs">INFINI Pizza</a> ecosystem</sub>
</div>
