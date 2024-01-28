use crate::lexer::*;
use crate::model::*;
use either::*;

macro_rules! current_method {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        &name[..name.len() - 3]
    }}
}

/**************************************************************************************************/
/*                                         LEXER STREAM                                           */
/**************************************************************************************************/

type LexerResult = Result<Token, Exception>;

struct LexerStream {
    buffer: Vec<Token>,
    cursor: usize,
}

impl From<Lexer> for LexerStream {
    fn from(mut lexer: Lexer) -> LexerStream {
        LexerStream { buffer: lexer.read(), cursor: 0 }
    }
}

impl From<String> for LexerStream {
    fn from(content: String) -> Self {
        LexerStream::from(Lexer::from(content))
    }
}

impl From<Vec<Token>> for LexerStream {
    fn from(buffer: Vec<Token>) -> Self {
        LexerStream { buffer, cursor: 0 }
    }
}

impl LexerStream {
    pub fn eof(&self) -> bool {
        self.cursor == self.buffer.len()
    }

    pub fn peek(&self, idx: isize) -> Option<Token> {
        let value = idx.abs() as usize;
        let pos = if idx < 0 { self.cursor - value } else { self.cursor + value };
        let token = self.buffer.iter().nth(pos);

        if let Some(token) = token { Some(token.clone()) }
        else { None }
    }

    pub fn is<F>(&self, predicate: F) -> bool where F: Fn(Token) -> bool {
        match self.peek(0) {
            None => false,
            Some(token) => predicate(token),
        }
    }

    pub fn is_equal<F>(&self, value: &str, callback: F) -> bool where F: Fn(Token, char) -> bool {
        for (idx, c) in value.chars().enumerate() {
            if false == match self.peek(idx as isize) {
                None => false,
                Some(token) => callback(token, c),
            } {
                return false;
            }
        }
        return true;
    }

    pub fn is_type_or_equal<F>(&self, value: Option<&str>, callback: F) -> bool where F: Fn(Token, Option<char>) -> bool {
        if let Some(str) = value {
            self.is_equal(str, |token, char| callback(token, Some(char)))
        } else {
            self.is(|token| callback(token, None))
        }
    }

    pub fn is_identifier(&self, value: Option<&str>) -> bool {
        self.is(|token| token.is_identifier(value))
    }

    pub fn is_muted(&self) -> bool {
        self.is(|token| token.is_whitespace(None))
    }

    pub fn is_operator(&self, value: Option<&str>) -> bool {
        self.is_type_or_equal(None, |token, _| token.is_operator(value))
    }

    pub fn is_one_of_punctuation(&self, value: Vec<char>) -> bool {
        self.is(|token| token.is_one_of_punctuation(value.clone()))
    }

    pub fn is_punctuation(&self, value: Option<&str>) -> bool {
        self.is_type_or_equal(value, |token, value| token.is_punctuation(value))
    }

    pub fn next(&mut self) -> LexerResult {
        match self.peek(0) {
            None => Err(Exception::create(Error::EOF, current_method!())),
            Some(token) => {
                self.cursor += 1;
                Ok(token)
            }
        }
    }

    pub fn read<F>(&mut self, mut callback: F) -> LexerResult where F: FnMut(Token) -> bool {
        if let Some(token) = self.peek(0) {
            if !callback(token.clone()) {
                Err(Exception::create(Error::UnexpectedToken(token), current_method!()))
            } else {
                Ok(self.next().unwrap())
            }
        } else {
            Err(Exception::create(Error::EOF, current_method!()))
        }
    }

    pub fn read_punctuation(&mut self, value: Option<&str>) -> LexerResult {
        if self.is_punctuation(value) {
            self.read(|_| true)
        } else {
            if let Some(token) = self.peek(0) {
                Err(Exception::create(Error::UnexpectedToken(token), current_method!()))
            } else {
                Err(Exception::create(Error::EOF, current_method!()))
            }
        }
    }

    pub fn read_while<F>(&mut self, mut predicate: F) -> Vec<Token> where F: FnMut(Token) -> bool {
        let mut values: Vec<Token> = vec![];
        while !self.eof() {
            let token = self.peek(0).unwrap();
            if !predicate(token) { break; }
            values.push(self.next().unwrap());
        }

        values
    }
}

/**************************************************************************************************/
/*                                            PARSER                                              */
/**************************************************************************************************/

type ParserResult = Result<Node, Exception>;

pub struct Parser {
    stream: LexerStream,
}

impl From<String> for Parser {
    fn from(content: String) -> Self {
        Parser { stream: LexerStream::from(content) }
    }
}

impl From<Vec<Token>> for Parser {
    fn from(buffer: Vec<Token>) -> Self {
        Parser { stream: LexerStream::from(buffer) }
    }
}

impl Parser {
    pub fn eof(&self) -> bool {
        self.stream.eof()
    }

    pub fn parse(&mut self) -> ParserResult {
        match self.parse_additive() {
            Err(error) => Err(Exception::relay(error, current_method!())),
            Ok(value) => Ok(Node::create("Calcul", vec![value]))
        }
    }

    fn parse_additive(&mut self) -> Result<NodeItem, Exception> {
        match self.parse_multiplicative() {
            Err(error) => Err(Exception::relay(error, current_method!())),
            Ok(mut left) => {
                while !self.stream.eof() {
                    if !self.stream.is_operator(Some("+")) && !self.stream.is_operator(Some("-")) { break; }
                    match self.stream.next() {
                        Err(error) => return Err(Exception::relay(error, current_method!())),
                        Ok(token) => {
                            match self.parse_multiplicative() {
                                Err(error) => return Err(Exception::relay(error, current_method!())),
                                Ok(right) => {
                                    if token.is_operator(Some("+")) {
                                        left = Right(Node::create("Add", vec![left, right]))
                                    } else if token.is_operator(Some("-")) {
                                        left = Right(Node::create("Subtract", vec![left, right]))
                                    } else {
                                        return Err(Exception::create(Error::UnexpectedToken(token), current_method!()));
                                    }
                                }
                            }
                        }
                    }
                }
                Ok(left)
            }
        }
    }

    fn parse_multiplicative(&mut self) -> Result<NodeItem, Exception> {
        match self.parse_primary() {
            Err(error) => Err(Exception::relay(error, current_method!())),
            Ok(mut left) => {
                while !self.stream.eof() {
                    if !self.stream.is_operator(Some("*")) && !self.stream.is_operator(Some("/")) { break; }
                    match self.stream.next() {
                        Err(error) => return Err(Exception::relay(error, current_method!())),
                        Ok(token) => {
                            match self.parse_primary() {
                                Err(error) => return Err(Exception::relay(error, current_method!())),
                                Ok(right) => {
                                    if token.is_operator(Some("*")) {
                                        left = Right(Node::create("Multiply", vec![left, right]));
                                    } else if token.is_operator(Some("/")) {
                                        left = Right(Node::create("Divide", vec![left, right]));
                                    } else {
                                        return Err(Exception::create(Error::UnexpectedToken(token.clone()), current_method!()));
                                    }
                                }
                            }
                        }
                    }
                }

                Ok(left)
            }
        }
    }

    fn parse_primary(&mut self) -> Result<NodeItem, Exception> {
        match self.stream.next() {
            Err(error) => Err(Exception::relay(error, current_method!())),
            Ok(token) => {
                if token.is_number(None) {
                    Ok(Left(token))
                } else if token.is_punctuation(Some('(')) {
                    match self.parse_additive() {
                        Err(error) => Err(Exception::relay(error, current_method!())),
                        Ok(node) => {
                            match self.stream.read_punctuation(Some(")")) {
                                Err(error) => Err(Exception::relay(error, current_method!())),
                                Ok(_) => Ok(node)
                            }
                        }
                    }
                } else {
                    Err(Exception::create(Error::UnexpectedToken(token.clone()), current_method!()))
                }
            }
        }
    }
}
