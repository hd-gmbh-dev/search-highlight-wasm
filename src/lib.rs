use std::collections::BTreeMap;

use tantivy::{schema::Field, snippet::SnippetGenerator, tokenizer::{LowerCaser, NgramTokenizer, RemoveLongFilter, TextAnalyzer}};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn highlight_ngram3(line: String, search: String) -> Result<String, JsValue> {
    let tokenizer = TextAnalyzer::builder(NgramTokenizer::all_ngrams(3, 50).unwrap())
        .filter(LowerCaser)
        .filter(RemoveLongFilter::limit(150))
        .build();
    let snippet_generator = SnippetGenerator::new(
        BTreeMap::from_iter([(search, 1.0)])
    , tokenizer, Field::from_field_id(0), 150);
    let html = snippet_generator.snippet(&line).to_html();
    if html.is_empty() {
        return Ok(line);
    }
    Ok(html)
}
