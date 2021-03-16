use crate::interpreter::token::Token;
use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer {
        Lexer {
            input: input.chars().peekable(),
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.input.next() {
            Some('=') => match self.input.peek() {
                Some('=') => {
                    self.input.next();
                    Token::Equal
                }
                _ => Token::Assign,
            },
            Some('!') => match self.input.peek() {
                Some('=') => {
                    self.input.next();
                    Token::NotEqual
                }
                _ => Token::Bang,
            },
            Some(';') => Token::Semicolon,
            Some('(') => Token::LeftParen,
            Some(')') => Token::RightParen,
            Some(',') => Token::Comma,
            Some('+') => Token::Plus,
            Some('-') => Token::Minus,
            Some('{') => Token::LeftBrackets,
            Some('}') => Token::RightBrackets,
            Some('/') => Token::Slash,
            Some('*') => Token::Asterisk,
            Some('<') => Token::LessThan,
            Some('>') => Token::GreaterThan,
            Some(char) => {
                // If the first character is a alphabetic letter, we parse it as
                // an identitier. If it's a digit we parse it as a number.
                if self.is_letter(char) {
                    let identitier = self.read_identifier(char);
                    // Match against "reserved keywords"
                    match identitier.as_str() {
                        "func" => Token::Function,
                        "let" => Token::Let,
                        "true" => Token::True,
                        "false" => Token::False,
                        "if" => Token::If,
                        "else" => Token::Else,
                        "return" => Token::Return,
                        _ => Token::Identifier(identitier),
                    }
                } else if char.is_digit(10) {
                    let integer = self.read_integer(char);
                    Token::Int(integer)
                } else {
                    Token::Illegal
                }
            }
            None => Token::Eof,
        }
    }

    fn is_letter(&self, char: char) -> bool {
        char.is_alphabetic() || char == '_'
    }

    fn read_identifier(&mut self, char: char) -> String {
        let mut identitier = String::new();
        identitier.push(char);

        while let Some(&char) = self.input.peek() {
            if self.is_letter(char) {
                match self.input.next() {
                    Some(char) => identitier.push(char),
                    None => break,
                }
            } else {
                break;
            }
        }

        identitier
    }

    fn read_integer(&mut self, char: char) -> String {
        let mut integer = String::new();
        integer.push(char);

        while let Some(&char) = self.input.peek() {
            if char.is_digit(10) {
                match self.input.next() {
                    Some(char) => integer.push(char),
                    None => break,
                }
            } else {
                break;
            }
        }

        integer
    }

    fn skip_whitespace(&mut self) {
        while let Some(&char) = self.input.peek() {
            if char.is_whitespace() {
                self.input.next();
            } else {
                break;
            }
        }
    }
}

#[test]
fn test_next_token() {
    let input = "
    
    
    let five = 5;
    let ten = 10;
    
    let add = func(x, y) {
        x + y;
    };
    
    let result = add(five, ten);
    !-/*5;
    5 < 10 > 5;
    
    if (5 < 10) {
        return true;
    } else {
        return false;
    }
    
    10 == 10;
    10 != 9;
    ";

    let expected_tokens = vec![
        Token::Let,
        Token::Identifier(String::from("five")),
        Token::Assign,
        Token::Int(String::from("5")),
        Token::Semicolon,
        Token::Let,
        Token::Identifier(String::from("ten")),
        Token::Assign,
        Token::Int(String::from("10")),
        Token::Semicolon,
        Token::Let,
        Token::Identifier(String::from("add")),
        Token::Assign,
        Token::Function,
        Token::LeftParen,
        Token::Identifier(String::from("x")),
        Token::Comma,
        Token::Identifier(String::from("y")),
        Token::RightParen,
        Token::LeftBrackets,
        Token::Identifier(String::from("x")),
        Token::Plus,
        Token::Identifier(String::from("y")),
        Token::Semicolon,
        Token::RightBrackets,
        Token::Semicolon,
        Token::Let,
        Token::Identifier(String::from("result")),
        Token::Assign,
        Token::Identifier(String::from("add")),
        Token::LeftParen,
        Token::Identifier(String::from("five")),
        Token::Comma,
        Token::Identifier(String::from("ten")),
        Token::RightParen,
        Token::Semicolon,
        Token::Bang,
        Token::Minus,
        Token::Slash,
        Token::Asterisk,
        Token::Int(String::from("5")),
        Token::Semicolon,
        Token::Int(String::from("5")),
        Token::LessThan,
        Token::Int(String::from("10")),
        Token::GreaterThan,
        Token::Int(String::from("5")),
        Token::Semicolon,
        Token::If,
        Token::LeftParen,
        Token::Int(String::from("5")),
        Token::LessThan,
        Token::Int(String::from("10")),
        Token::RightParen,
        Token::LeftBrackets,
        Token::Return,
        Token::True,
        Token::Semicolon,
        Token::RightBrackets,
        Token::Else,
        Token::LeftBrackets,
        Token::Return,
        Token::False,
        Token::Semicolon,
        Token::RightBrackets,
        Token::Int(String::from("10")),
        Token::Equal,
        Token::Int(String::from("10")),
        Token::Semicolon,
        Token::Int(String::from("10")),
        Token::NotEqual,
        Token::Int(String::from("9")),
        Token::Semicolon,
        Token::Eof,
    ];

    let mut lexer = Lexer::new(input);

    for expected_token in expected_tokens {
        let token = lexer.next_token();
        assert_eq!(token, expected_token);
    }
}
