use crate::instruction::Instruction;
use crate::parser::Expr;
use crate::pretty_printer::PrettyPrinter;
use crate::token::{Token, TokenType};

pub struct Compiler {
    should_log: bool,
    next_unique_number: u32,
}

impl Compiler {
    pub fn new(should_log: bool) -> Self {
        Self {
            should_log,
            next_unique_number: 1,
        }
    }

    pub fn compile(&mut self, expr: &Expr) -> Vec<Instruction> {
        self.log("Top level compile function called");
        let mut instructions = Vec::new();
        self.emit(expr, &mut instructions, 0);
        instructions
    }

    fn log(&self, stmt: &str) {
        if self.should_log {
            println!("{stmt}");
        }
    }

    fn emit(&mut self, expr: &Expr, instructions: &mut Vec<Instruction>, depth: usize) {
        let log_id: u32 = self.next_unique_number;
        self.next_unique_number += 1;

        let indent = "  ".repeat(depth);

        match expr {
            Expr::NumberLiteral(value) => {
                self.log(&format!("{indent}[{log_id}] Emit with number literal {value}"));
                instructions.push(Instruction::PushInt(*value));
            }
            Expr::Binary { left, operator, right } => {
                let display = PrettyPrinter::print(expr);
                self.log(&format!("{indent}[{log_id}] On binary expression: {display}"));
                self.log(&format!("{indent}[{log_id}]   left side ->"));
                self.emit(left, instructions, depth + 1);

                self.log(&format!("{indent}[{log_id}]   right side ->"));
                self.emit(right, instructions, depth + 1);

                let binary_instruction = instruction_for_operator(operator);
                self.log(&format!(
                    "{indent}[{log_id}]   push operator {binary_instruction:?}"
                ));
                instructions.push(binary_instruction);
            }
        }
    }
}

fn instruction_for_operator(operator: &Token) -> Instruction {
    match operator.token_type {
        TokenType::Plus => Instruction::Add,
        TokenType::Minus => Instruction::Sub,
        TokenType::Star => Instruction::Mul,
        TokenType::Slash => Instruction::Div,
        _ => panic!("Invalid operator {:?}", operator),
    }
}