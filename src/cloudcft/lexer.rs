use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum DataType {
    Num,
    String,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    DataKeyword(DataType),
    Name,
    NumValue,
    Equal,
    Semicolon,
    Ignored,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
}

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer { input, position: 0 }
    }

    fn consume_ignored(&mut self) {
        while self.position < self.input.len()
            && (self.input.chars().nth(self.position).unwrap().is_whitespace()
                || self.input.chars().nth(self.position).unwrap() == '\n')
        {
            self.position += 1;
        }
    }

    fn consume_identifier(&mut self) -> String {
        let start = self.position;
        let identifier_regex = Regex::new(r"[_a-zA-Z][_A-Z0-9a-z]*").unwrap();

        while self.position < self.input.len() {
            let current_char = self.input.chars().nth(self.position).unwrap();
            if identifier_regex.is_match(&current_char.to_string()) {
                self.position += 1;
            } else {
                break;
            }
        }

        self.input[start..self.position].to_string()
    }
    fn consume_num_value(&mut self) -> String {
        let start = self.position;
        let value_regex = Regex::new(r"[1-9][0-9]*").unwrap();

        while self.position < self.input.len() {
            let current_char = self.input.chars().nth(self.position).unwrap();
            if value_regex.is_match(&current_char.to_string()) {
                self.position += 1;
            } else {
                break;
            }
        }

        self.input[start..self.position].to_string()
    }


    pub fn next_token(&mut self) -> Option<Token> {
        self.consume_ignored();

        if self.position >= self.input.len() {
            return None;
        }

        let current_char = self.input.chars().nth(self.position).unwrap();
        println!("{}",&current_char);
        let token = match current_char {
            '1'..='9' => {
                let lexeme=self.consume_num_value();
                Token {
                    token_type: TokenType::NumValue,
                    lexeme: lexeme,
                }
            }
            '=' => {
                self.position += 1;
                Token {
                    token_type: TokenType::Equal,
                    lexeme: "=".to_string(),
                }
            }
            ';' => {
                self.position += 1;
                Token {
                    token_type: TokenType::Semicolon,
                    lexeme: ";".to_string(),
                }
            }
            'a'..='z' | 'A'..='Z' => {
                let lexeme = self.consume_identifier();
                if lexeme.starts_with('n') && lexeme.len() == 3 && &lexeme[1..] == "um" {
                    Token {
                        token_type: TokenType::DataKeyword(DataType::Num),
                        lexeme,
                    }
                } else {
                    Token {
                        token_type: TokenType::Name,
                        lexeme,
                    }
                }
            }
            _ => {
                panic!("unknown token");
            }
        };

        Some(token)
    }
}
