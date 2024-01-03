// code_generator.rs
use super::lexer::DataType;
use super::parser::AstNode;

pub struct CodeGenerator {
    declared_var: bool,
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            declared_var: false
        }
    }
    pub fn generate_code(&mut self, ast: AstNode) -> String {
        // let mut declared_var = false; // Flag to track if the variable has been declared

        match ast {
            AstNode::SourceCode(statements) => {
                let mut code = String::new();
                for statement in statements {
                    code.push_str(&self.generate_code(statement));
                }
                code
            }
            AstNode::Statement(statement) => self.generate_code(*statement),
            AstNode::AssignmentVar(data_type, name, value) => {
                (if !self.declared_var {
                    self.declared_var = true;
                    format!("scoreboard objectives add var dummy\n")
                } else {
                    String::from("")
                }) + &format!(
                    "scoreboard players set {} var {}\n", 
                    name, 
                    value
                )
            }
        }
    }
}
