// parser.rs
use super::lexer::{DataType, Token, TokenType,Lexer};

#[derive(Debug)]
pub enum AstNode {
    SourceCode(Vec<AstNode>),
    Statement(Box<AstNode>),
    AssignmentVar(DataType, String, String),
}


pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Parser { lexer, current_token: None };
        parser
    }

    fn consume_token(&mut self) -> &Option<Token> {
        self.current_token = self.lexer.next_token();
        &self.current_token
    }

    fn expect_token(&mut self, expected_type: TokenType) {
        self.consume_token();
        if let Some(ref token) = self.current_token {
            if token.token_type != expected_type {
                panic!("Expected {:?}, found {:?}", expected_type, token.token_type);
            }
        } else {
            panic!("Unexpected end of input");
        }
    }

    pub fn parse(&mut self) -> AstNode {
        self.parse_source_code()
    }

    fn parse_source_code(&mut self) -> AstNode {
        let mut statements = Vec::new();
        while let Some(ref token) = self.consume_token() {
            statements.push(self.parse_statement());
            //self.expect_token(TokenType::Semicolon);
        }
        AstNode::SourceCode(statements)
    }

    fn parse_statement(&mut self) -> AstNode {
        let statement= if let Some(token) = &self.current_token  {
            match &token.token_type {
                TokenType::DataKeyword(data_type) => self.parse_assignment_var(),
                _ => panic!("Unexpected token")
            }
        }else {
            panic!("None token")
        };
        self.expect_token(TokenType::Semicolon);
        AstNode::Statement(Box::new(statement))
    }

    fn parse_assignment_var(&mut self) -> AstNode {
        //self.expect_token(TokenType::DataKeyword(DataType::Num)); // Change DataType::Num to the desired default data type
        let data_type = DataType::Num; // Set the default data type
        self.expect_token(TokenType::Name);
        let name = self.current_token.as_ref().unwrap().lexeme.clone();
        self.expect_token(TokenType::Equal);
        self.expect_token(TokenType::NumValue);
        let value = self.current_token.as_ref().unwrap().lexeme.clone();
        println!("var name: {},var value: {}",&name,&value);
        AstNode::AssignmentVar(data_type, name, value)
    }
}
