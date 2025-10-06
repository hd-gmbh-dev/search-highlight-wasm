use std::collections::BTreeMap;

use tantivy::{
    Score,
    schema::Field,
    snippet::SnippetGenerator,
    tokenizer::{LowerCaser, NgramTokenizer, RemoveLongFilter, TextAnalyzer},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn highlight_ngram3(line: String, search: String) -> Result<String, JsValue> {
    let tokenizer = TextAnalyzer::builder(NgramTokenizer::all_ngrams(2, 3).unwrap())
        .filter(LowerCaser)
        .filter(RemoveLongFilter::limit(150))
        .build();
    let mut terms_text = BTreeMap::default();
    let mut text_analyzer = tokenizer.clone();
    let mut token_stream = text_analyzer.token_stream(&search);
    let full_length = search.len();
    token_stream.process(&mut |token| {
        let score: Score = (token.offset_to - token.offset_from) as f32 / full_length as f32;
        terms_text.insert(token.text.trim().to_string(), score);
    });
    let snippet_generator =
        SnippetGenerator::new(terms_text, tokenizer, Field::from_field_id(0), 150);
    let html = snippet_generator.snippet(&line).to_html();
    if html.is_empty() {
        return Ok(line);
    }
    Ok(html)
}
