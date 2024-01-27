use std::fmt::{Debug, Display, Formatter};
use either::*;

/**************************************************************************************************/
/*                                              AST                                               */
/**************************************************************************************************/
pub trait AST {
    fn get_type(&self) -> String;
    fn get_start(&self) -> Position;
    fn get_next(&self) -> Position;
    fn get_tokens(&self) -> Vec<Token>;
    fn get_json(&self, with_position: bool) -> String;
    fn is(&self, value: &str) -> bool;
}

/**************************************************************************************************/
/*                                           POSITION                                             */
/**************************************************************************************************/
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Position {
    pub cursor: usize,
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn get_json(&self) -> String {
        format!(r#"{{"cursor":{},"line":{},"column":{}}}"#, self.cursor, self.line, self.column)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}


/**************************************************************************************************/
/*                                             TOKEN                                              */
/**************************************************************************************************/
fn is_one_of(token: &Token, chr: Option<Vec<char>>, str: Option<Vec<&str>>) -> bool {
    if let Some(vec) = chr {
        for c in vec {
            if !is_equal_value(token, Some(c), None) {
                return false;
            }
        }
        true
    } else if let Some(vec) = str {
        for s in vec {
            if !is_equal_value(token, None, Some(s)) {
                return false;
            }
        }
        true
    } else {
        true
    }
}

fn is_equal_value(token: &Token, chr: Option<char>, str: Option<&str>) -> bool {
    if let Some(c) = chr {
        String::from(c).eq(&token.get_value())
    } else if let Some(s) = str {
        String::from(s).eq(&token.get_value())
    } else {
        true
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String, Position, Position),
    Number(String, Position, Position),
    Operator(String, Position, Position),
    Punctuation(char, Position, Position),
    Whitespace(String, Position, Position),
}

impl Token {
    fn get_value(&self) -> String {
        match self {
            Token::Identifier(value, _, _) => value.to_string(),
            Token::Number(value, _, _) => value.to_string(),
            Token::Operator(value, _, _) => value.to_string(),
            Token::Punctuation(value, _, _) => value.to_string(),
            Token::Whitespace(value, _, _) => value.to_string(),
        }
    }

    pub fn is_identifier(&self, value: Option<&str>) -> bool {
        match self {
            Token::Identifier(_, _, _) => is_equal_value(self, None, value),
            _ => false,
        }
    }

    pub fn is_number(&self, value: Option<&str>) -> bool {
        match self {
            Token::Number(_, _, _) => is_equal_value(self, None, value),
            _ => false,
        }
    }

    pub fn is_operator(&self, value: Option<&str>) -> bool {
        match self {
            Token::Operator(_, _, _) => is_equal_value(self, None, value),
            _ => false,
        }
    }

    pub fn is_punctuation(&self, value: Option<char>) -> bool {
        match self {
            Token::Punctuation(_, _, _) => is_equal_value(self, value, None),
            _ => false,
        }
    }
    pub fn is_one_of_punctuation(&self, value: Vec<char>) -> bool {
        match self {
            Token::Punctuation(_, _, _) => is_one_of(self, Some(value), None),
            _ => false,
        }
    }

    pub fn is_whitespace(&self, value: Option<&str>) -> bool {
        match self {
            Token::Whitespace(_, _, _) => is_equal_value(self, None, value),
            _ => false,
        }
    }
}

impl AST for Token {
    fn get_type(&self) -> String {
        match self {
            Token::Identifier(_, _, _) => String::from("Identifier"),
            Token::Number(_, _, _) => String::from("Number"),
            Token::Operator(_, _, _) => String::from("Operator"),
            Token::Punctuation(_, _, _) => String::from("Punctuation"),
            Token::Whitespace(_, _, _) => String::from("Whitespace"),
        }
    }

    fn get_start(&self) -> Position {
        match self {
            Token::Identifier(_, start, _) => start.clone(),
            Token::Number(_, start, _) => start.clone(),
            Token::Operator(_, start, _) => start.clone(),
            Token::Punctuation(_, start, _) => start.clone(),
            Token::Whitespace(_, start, _) => start.clone(),
        }
    }

    fn get_next(&self) -> Position {
        match self {
            Token::Identifier(_, _, next) => next.clone(),
            Token::Number(_, _, next) => next.clone(),
            Token::Operator(_, _, next) => next.clone(),
            Token::Punctuation(_, _, next) => next.clone(),
            Token::Whitespace(_, _, next) => next.clone(),
        }
    }

    fn get_tokens(&self) -> Vec<Token> {
        vec![self.clone()]
    }

    fn get_json(&self, with_position: bool) -> String {
        let value = self.get_value()
            .replace("\r\n", "\\n")
            .replace("\n", "\\n")
            .replace("\r", "\\n")
            .replace("\t", "\\t")
            .replace("\"", "\\\"")
        ;

        if with_position {
            format!(
                r#"{{"type":"{}","value":"{}","start":{},"next":{}}}"#,
                self.get_type(),
                value,
                self.get_start().get_json(),
                self.get_next().get_json(),
            )
        } else {
            format!(
                r#"{{"type":"{}","value":"{}"}}"#,
                self.get_type(),
                value,
            )
        }
    }

    fn is(&self, value: &str) -> bool {
        self.get_type().eq(value)
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.get_type(), self.get_value())
    }
}

/**************************************************************************************************/
/*                                             NODE                                               */
/**************************************************************************************************/
pub type NodeItem = Either<Token, Node>;
pub type NodeValue = Vec<NodeItem>;

#[derive(Debug, Clone)]
pub struct Node {
    kind: Box<String>,
    value: Box<NodeValue>,
}

impl Node {
    pub fn create(kind: &str, value: NodeValue) -> Node {
        Node {
            kind: Box::new(kind.to_string()),
            value: Box::new(value)
        }
    }

    pub fn create_atom(token: Token) -> Node {
        Node::create("atom", vec![Left(token)])
    }

    pub fn get_text(&self) -> String {
        let mut tokens = self.get_tokens();
        let mut str = String::from("");
        while 0 < tokens.len() {
            tokens.rotate_left(1);
            if let Some(token) = tokens.pop() {
                str.push_str(&token.get_value());
            }
        }
        String::from(str)
    }

    pub fn get_token(&self) -> Option<Token> {
        let kind = self.get_type();
        if kind.eq("atom") {
            let value = self.get_value();
            if let Left(token) = value.iter().nth(0).unwrap() {
                Some(token.clone())
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_value(&self) -> NodeValue {
        let mut value: NodeValue = vec![];
        for item in self.value.iter() {
            match item {
                Left(token) => value.push(Left(token.clone())),
                Right(node) => value.push(Right(node.clone())),
            }
        }
        return value;
    }
}

impl AST for Node {
    fn get_type(&self) -> String {
        return *self.kind.clone();
    }

    fn get_start(&self) -> Position {
        let item = self.value.iter().nth(0).unwrap();
        if let Left(token) = item {
            token.get_start()
        } else if let Right(node) = item {
            node.get_start()
        } else {
            panic!("Unknown item type")
        }
    }

    fn get_next(&self) -> Position {
        let item = self.value.iter().nth(0).unwrap();
        if let Left(token) = item {
            token.get_next()
        } else if let Right(node) = item {
            node.get_next()
        } else {
            panic!("Unknown item type")
        }
    }

    fn get_tokens(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        for item in self.get_value().iter() {
            if let Left(token) = item {
                tokens.push(token.clone());
            } else if let Right(node) = item {
                tokens.append(&mut node.get_tokens());
            }
        }
        return tokens;
    }

    fn get_json(&self, with_position: bool) -> String {
        let mut str = String::from("");
        for item in self.get_value() {
            if let Left(token) = item {
                str = format!("{},{}", str, token.get_json(with_position))
            } else if let Right(node) = item {
                str = format!("{},{}", str, node.get_json(with_position))
            }
        }
        let str = str.trim_start_matches(&[',']);

        if with_position {
            format!(
                r#"{{"type":"{}","value":[{}],"start":{},"next":{}}}"#,
                self.get_type(),
                str,
                self.get_start().get_json(),
                self.get_next().get_json(),
            )
        } else {
            format!(
                r#"{{"type":"{}","value":[{}]}}"#,
                self.get_type(),
                str,
            )
        }
    }

    fn is(&self, kind: &str) -> bool {
        if let Some(token) = self.get_token() {
            token.is(kind)
        } else {
            self.get_type().eq(kind)
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.get_type(), self.get_text())
    }
}

/**************************************************************************************************/
/*                                             ERROR                                              */
/**************************************************************************************************/

#[derive(Debug, Clone)]
pub enum Error {
    EOF,
    NoBlockEnd(char, Position),
    UnexpectedCharacter(char, char, Position),
    UnexpectedEOF(Position),
    UnexpectedItem(NodeItem),
    UnexpectedToken(Token),
    UnprocessableCharacter(char, Position),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::EOF
                => write!(f, "End of file"),
            Error::NoBlockEnd(c, pos)
                => write!(f, "Block '{}' is not ended in {}", c, pos),
            Error::UnexpectedCharacter(expected, value, pos)
                => write!(f, "Expected token '{}' got '{}' in {}.", expected, value, pos),
            Error::UnexpectedEOF(pos)
                => write!(f, "Unexpected end of file in {}.", pos),
            Error::UnexpectedItem(item)
                => if let Left(token) = item {
                    write!(f, "Unexpected token {} in {}.", token, token.get_start())
                } else if let Right(node) = item {
                    write!(f, "Unexpected node {} in {}.", node, node.get_start())
                } else {
                    write!(f, "Unexpected error")
                },
            Error::UnexpectedToken(token)
                => write!(f, "Unexpected token {} in {}.", token, token.get_start()),
            Error::UnprocessableCharacter(c, pos)
                => write!(f, "Can't handle character {} in {}.", c, pos),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Exception {
    pub error: Error,
    trace: Vec<String>,
}

impl Exception {
    pub fn create(error: Error, method: &str) -> Self {
        Self { error, trace: vec![String::from(method)] }
    }

    pub fn relay(exception: Self, method: &str) -> Self {
        let error = exception.error.clone();
        let mut trace = exception.trace.clone();
        trace.push(method.to_string());

        Self { error, trace }
    }
}

impl Display for Exception {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut message = String::from("");
        for method in &self.trace {
            message.push_str(&format!("\n{}", &method));
        }
        write!(f, "{}\n{}", self.error, &message)
    }
}
