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
pub fn lex(text: &str, with_position: bool) -> Result<String, JsError> {
  match Lexer::from(String::from(text)).lex() {
    Err(error) => Err(JsError::new(format!(r#"{}"#, error).as_str())),
    Ok(tokens) => {
      let mut str = String::from("");
      for token in tokens {
        str = format!("{},{}", str, token.get_json(with_position))
      }
      Ok(format!(r#"[{}]"#, str.trim_start_matches(&[','])))
    }
  }
}

#[wasm_bindgen]
pub fn parse(text: &str, with_position: bool) -> Result<String, JsError> {
  match Parser::from(String::from(text)).parse() {
    Err(error) => Err(JsError::new(format!(r#"{}"#, error).as_str())),
    Ok(node) => Ok(format!(r#"{}"#, node.get_json(with_position)))
  }
}
