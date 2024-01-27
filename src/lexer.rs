use crate::model::*;

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
/*                                             INPUT                                              */
/**************************************************************************************************/

type InputResult = Result<String, Exception>;

struct InputStream {
    buffer: String,
    pub position: Position
}

impl From<String> for InputStream {
    fn from(buffer: String) -> Self {
        let position = Position { cursor: 0, line: 0, column: 0 };
        InputStream { buffer, position }
    }
}

impl InputStream {
    pub fn eof(&self, idx: isize) -> bool {
        self.peek(idx) == None
    }

    pub fn peek(&self, idx: isize) -> Option<char> {
        let pos = self.position.cursor;
        let pos = if idx < 0 { pos - idx.abs() as usize } else { pos + idx.abs() as usize };
        self.buffer.chars().nth(pos)
    }

    pub fn next(&mut self) -> Result<char, Exception> {
        if self.eof(0) {
            return Err(Exception::create(Error::UnexpectedEOF(self.position), current_method!()));
        }

        match self.peek(0) {
            None => Err(Exception::create(Error::UnexpectedEOF(self.position), current_method!())),
            Some(c) => {
                self.position = Position {
                    cursor: self.position.cursor + 1,
                    line: self.position.line + (if '\n' == c { 1 } else { 0 }),
                    column: if '\n' == c { 0 } else { self.position.column + 1 },
                };
                Ok(c)
            }
        }
    }

    pub fn read_escaped<F>(&mut self, mut predicate: F) -> InputResult where F: FnMut(char, &str) -> bool {
        let mut escaped = false;
        self.read_while(|c, str| {
            if escaped {
                escaped = false;
                return true;
            }
            if c == '\\' {
                escaped = true;
                return true;
            }
            predicate(c, str)
        })
    }

    pub fn read_identifier(&mut self, with_escaped: bool) -> InputResult {
        let predicate = |c: char, _: &str| { c.is_alphanumeric() || c == '-' || c == '_' };
        if !with_escaped { self.read_while(predicate) }
        else { self.read_escaped(predicate) }
    }

    pub fn read_number(&mut self) -> InputResult {
        self.read_while(|c, str| {
            if c == '.' { !str.contains('.') }
            else if c == 'e' { !str.contains('e') }
            else if c == '+' || c == '-' { str.len() == 0 || str.ends_with('e') }
            else { c.is_digit(10) }
        })
    }

    pub fn read_while<F>(&mut self, mut predicate: F) -> InputResult where F: FnMut(char, &str) -> bool {
        let mut str: String = String::from("");
        while !self.eof(0) {
            let next = self.peek(0).unwrap();
            if !predicate(next, &str.clone()) {
                break;
            }
            match self.next() {
                Err(error) => return Err(Exception::relay(error, current_method!())),
                Ok(c) => str.push(c),
            }
        }
        Ok(str)
    }
}

/**************************************************************************************************/
/*                                             LEXER                                              */
/**************************************************************************************************/

type LexerResult = Result<Token, Exception>;

pub struct Lexer {
    input: InputStream,
}

impl From<String> for Lexer {
    fn from(content: &mut String) -> Self {
        Lexer { input: InputStream::from(content.retain(|c| !c.is_whitespace())) }
    }
}

impl Lexer {
    fn is_operator(c: char) -> bool { None != "+-*/%=&|!><^~".find(c) }
    fn is_punctuation(c: char) -> bool { None != ",;(){}[]:.#".find(c) }
    fn is_whitespace(c: char) -> bool { None != "\t\r\n ".find(c) }

    pub fn eof(&self) -> bool {
        self.input.eof(0)
    }

    pub fn read(&mut self) -> Vec<Token> {
        let mut buffer: Vec<Token> = vec![];
        while !self.input.eof(0) {
            match self.next() {
                Err(error) => panic!("{}", error),
                Ok(token) => buffer.push(token),
            }
        }
        buffer
    }

    pub fn next(&mut self) -> LexerResult {
        if self.input.eof(0) {
            return Err(Exception::create(Error::UnexpectedEOF(self.input.position), current_method!()));
        }

        let c = self.input.peek(0).unwrap();
        if Lexer::is_whitespace(c) { self.read_whitespace() }
        else if self.is_number_start() { self.read_number() }
        else if Lexer::is_punctuation(c) { self.read_punctuation() }
        else if self.is_ident_start(c) { self.read_identifier() }
        else if Lexer::is_operator(c) { self.read_operator() }
        else { Err(Exception::create(Error::UnprocessableCharacter(c, self.input.position), current_method!())) }
    }

    fn is_ident_start(&self, c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }

    fn is_number_start(&self) -> bool {
        if self.input.eof(0) { return false; }
        let current = self.input.peek(0).unwrap();
        if current.is_digit(10) { return true; } // 1

        if self.input.eof(1) { return false; }
        let next = self.input.peek(1).unwrap();
        if current == '.' && next.is_digit(10) { return true; } // .1

        if '+' != current && '-' != current { return false; }
        if next.is_digit(10) { return true; } // +1 | -1

        if self.input.eof(2) { return false; }
        next == '.' && self.input.peek(2).unwrap().is_digit(10) // +.1 | -.1
    }

    fn read_identifier(&mut self) -> LexerResult {
        let start = self.input.position;
        match self.input.read_identifier(true) {
            Err(error) => Err(Exception::relay(error, current_method!())),
            Ok(value) => Ok(Token::Identifier(value, start, self.input.position)),
        }
    }

    fn read_number(&mut self) -> LexerResult {
        let start = self.input.position;
        match self.input.read_number() {
            Err(error) => Err(Exception::relay(error, current_method!())),
            Ok(value) => Ok(Token::Number(value, start, self.input.position)),
        }
    }

    fn read_operator(&mut self) -> LexerResult {
        let start = self.input.position;
        match self.input.next() {
            Err(error) => Err(Exception::relay(error, current_method!())),
            Ok(value) => {
                if "&|=".contains(value) {
                    if let Some(next) = self.input.peek(0) {
                        if next == value {
                            let value = format!("{}{}", value, self.input.next().unwrap());
                            return Ok(Token::Operator(value,start,self.input.position));
                        }
                    }
                }
                Ok(Token::Operator(String::from(value), start, self.input.position))
            },
        }
    }

    fn read_punctuation(&mut self) -> LexerResult {
        let start = self.input.position;
        match self.input.next() {
            Err(error) => Err(Exception::relay(error, current_method!())),
            Ok(value) => Ok(Token::Punctuation(value, start, self.input.position)),
        }
    }

    fn read_whitespace(&mut self) -> LexerResult {
        let start = self.input.position;
        match self.input.read_while(|c, _| { Lexer::is_whitespace(c) }) {
            Err(error) => Err(Exception::relay(error, current_method!())),
            Ok(value) => Ok(Token::Whitespace(value, start, self.input.position)),
        }
    }
}
