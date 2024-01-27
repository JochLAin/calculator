mod model;
mod lexer;
mod parser;

use wasm_bindgen::prelude::*;
use model::*;
use lexer::*;
use parser::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn lex(text: &str, with_position: bool) -> String {
  let mut str = String::from("");
  for token in Lexer::from(String::from(text)).read() {
    str = format!("{},{}", str, token.get_json(with_position))
  }
  format!(r#"[{}]"#, str.trim_start_matches(&[',']))
}

#[wasm_bindgen]
pub fn parse(text: &str, with_position: bool) -> String {
  format!(r#"{}"#, Parser::from(String::from(text)).read().get_json(with_position))
}
