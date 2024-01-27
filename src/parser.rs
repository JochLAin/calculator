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

    pub fn is_operator(&self) -> bool {
        self.is_type_or_equal(None, |token, _| token.is_operator(None))
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

    pub fn read(&mut self) -> Node {
        let mut items: NodeValue = vec![];
        while !self.stream.eof() {
            match self.next() {
                Err(error) => panic!("{}", error),
                Ok(node) => {
                    if let Some(token) = node.get_token() {
                        items.push(Left(token));
                    } else if node.is("Expression") {
                        items.append(&mut node.get_value());
                    } else {
                        items.push(Right(node));
                    }
                },
            }
        }
        Node::create("Calcul", items)
    }

    pub fn next(&mut self) -> ParserResult {
        if self.stream.is_muted() {
            let token = self.stream.next().unwrap();
            return Ok(Node::create_atom(token))
        }

        let mut nodes: NodeValue = vec![];
        while !self.stream.eof() {
            match self.parse_unary() {
                Err(error) => return Err(Exception::relay(error, current_method!())),
                Ok(atom) => {
                    nodes.push(atom);
                    if self.stream.is_operator() {
                        let operator = self.stream.next().unwrap();
                        match self.parse_unary() {
                            Err(error) => return Err(Exception::relay(error, current_method!())),
                            Ok(atom) => {
                                return Ok(Node::create("Unary", vec![Right(Node::create("Expression", nodes)), Left(operator), atom]));
                            },
                        }
                    }
                }
            }
        }
        Ok(Node::create("Expression", nodes))
    }

    // fn parse_unary(&mut self) -> Result<NodeItem, Exception> {
    //     let left_term = self.parse_value();
    //     if !self.stream.is_operator() {
    //         Err(Exception::create(Error::UnexpectedToken(self.stream.next().unwrap()), current_method!()))
    //     }
    //
    //     let operator = self.stream.next().unwrap();
    //     let right_term = self.parse_value();
    // }

    fn parse_arguments(&mut self) -> Result<Node, Exception> {
        match self.stream.read_punctuation(Some("(")) {
            Err(error) => Err(Exception::relay(error, current_method!())),
            Ok(open) => {
                let mut value: NodeValue = vec![Left(open)];
                while !self.stream.eof() {
                    let token = self.stream.peek(0).unwrap();
                    if token.is_punctuation(Some(')')) { break; }
                    let expr = self.parse_expression(|token| token.is_one_of_punctuation(vec![',', ')']));
                    match expr {
                        Err(error) => return Err(Exception::relay(error, current_method!())),
                        Ok(mut nodes) => {
                            value.append(&mut nodes);
                            if self.stream.is_punctuation(Some(",")) {
                                value.push(Left(self.stream.next().unwrap()));
                            }
                        }
                    }
                }
                match self.stream.read_punctuation(Some(")")) {
                    Err(error) => Err(Exception::relay(error, current_method!())),
                    Ok(close) => {
                        value.push(Left(close));
                        Ok(Node::create("Argument", value))
                    },
                }
            }
        }
    }

    fn parse_expression<F>(&mut self, mut predicate: F) -> Result<NodeValue, Exception> where F: FnMut(Token) -> bool {
        let mut value: NodeValue = vec![];
        let mut decl: NodeValue = vec![];
        while !self.stream.eof() {
            let next = self.stream.peek(0).unwrap();
            if !predicate(next) { break; }

            match self.read_atom() {
                Err(error) => return Err(Exception::relay(error, current_method!())),
                Ok(atom) => {
                    if let Left(token) = atom {
                        if token.is_punctuation(None) || token.is_whitespace(None) {
                            value.append(&mut decl);
                            value.push(Left(token));
                            break;
                        } else {
                            decl.push(Left(token));
                        }
                    } else if let Right(node) = atom {
                        decl.push(Right(node));
                    }
                }
            }
        }

        value.append(&mut decl);
        Ok(value)
    }

    fn parse_function(&mut self, item: NodeItem) -> Result<Node, Exception> {
        match self.parse_arguments() {
            Err(error) => Err(Exception::relay(error, current_method!())),
            Ok(args) => Ok(Node::create("Function", vec![item, Right(args)])),
        }
    }

    fn parse_unary(&mut self) -> Result<NodeItem, Exception> {
        let result: Option<ParserResult> = if self.stream.is_punctuation(Some("(")) {
            Some(self.parse_wrapped("Parenthesis", '(', ')'))
        } else { None };

        if let Some(value) = result {
            match (value) {
                Err(error) => Err(Exception::relay(error, current_method!())),
                Ok(node) => Ok(Right(node)),
            }
        }

        if self.stream.is_operator() {
            Err(Exception::create(Error::UnexpectedToken(self.stream.next().unwrap()), current_method!()))
        }

        match self.stream.next() {
            Err(error) => Err(Exception::relay(error, current_method!())),
            Ok(token) => {
                if token.is("Identifier") {
                    match self.parse_function(Left(token)) {
                        Err(error) => Err(Exception::relay(error, current_method!())),
                        Ok(node) => Ok(Right(node)),
                    }
                } else {
                    Ok(Left(token))
                }
            }
        }
    }

    fn parse_wrapped(&mut self, kind: &str, open: char, close: char) -> Result<Node, Exception> {
        match self.stream.read_punctuation(Some(&open.to_string())) {
            Err(error) => Err(Exception::relay(error, current_method!())),
            Ok(open) => {
                match self.parse_expression(|token| !token.is_punctuation(Some(close))) {
                    Err(error) => Err(Exception::relay(error, current_method!())),
                    Ok(mut expr) => {
                        match self.stream.read_punctuation(Some(&close.to_string())) {
                            Err(error) => Err(Exception::relay(error, current_method!())),
                            Ok(close) => {
                                let mut value: NodeValue = vec![];
                                value.push(Left(open));
                                value.append(&mut expr);
                                value.push(Left(close));
                                Ok(Node::create(kind, value))
                            },
                        }
                    }
                }
            }
        }
    }
}
