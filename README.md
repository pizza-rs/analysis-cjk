# pizza-analysis-cjk

CJK (Chinese, Japanese, Korean) analysis with bigram tokenization, width normalization, and stop words.

Part of the [Pizza](https://pizza.rs) search engine.

## Components

| Name | Type | Description |
|------|------|-------------|
| `cjk_bigram` | Token Filter | Produces overlapping bigrams for CJK ideographs |
| `cjk_width` | Token Filter | Normalizes fullwidth/halfwidth character variants |
| `cjk_stop` | Token Filter | CJK stop words filter (35 common particles) |
| `cjk` | Analyzer | Full pipeline: cjk_width → cjk_bigram → stop |

## Usage

### Built-in Analyzer

```json
{
  "analyzer": {
    "type": "cjk"
  }
}
```

### Custom Pipeline

```json
{
  "analyzer": {
    "type": "custom",
    "tokenizer": "standard",
    "filter": ["cjk_bigram", "cjk_width", "cjk_stop"]
  }
}
```

## License

MIT — see [LICENSE](LICENSE).

## Related Crates

- [analysis-core](https://github.com/pizza-rs/analysis-core) — Core analysis components and pipeline
- [analysis-icu](https://github.com/pizza-rs/analysis-icu) — ICU Unicode normalization and tokenization
- [analysis-english](https://github.com/pizza-rs/analysis-english) — English analysis
- [analysis-all](https://github.com/pizza-rs/analysis-all) — Meta-crate registering all analyzers
