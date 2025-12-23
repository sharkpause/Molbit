use crate::parser::{Expression, Operator, Statement, TopLevel};

#[derive(Debug)]
pub enum CodegenError {
    GenericError
}

pub fn codegen(program: Vec<TopLevel>) -> Result<String, CodegenError> {
    let mut output = String::from(
        "global _start\n\
        _start:\n\
            \tcall entry\n\
            \tmov rdi, rax\n\
            \tmov rax, 60\n\
            \tsyscall\n\n"
    );

    for toplevel in program {
        match toplevel {
            TopLevel::Function(function) => {
                output.push_str(&format!("{}:\n", function.name));
                
                for statement in function.body {
                    let statement_code = codegen_statement(statement)?;

                    output.push_str(&statement_code);
                }
            },
            TopLevel::Statement(statement) => {
                let statement_code = codegen_statement(statement)?;
                
                output.push_str(&statement_code);
            }
        }
    }

    return Ok(output);
}

fn codegen_statement(statement: Statement) -> Result<String, CodegenError> {
    let mut output = String::new();

    match statement {
        Statement::Return(expression) => {
            output.push_str(&codegen_expression(expression)?);
            output.push_str("    ret\n");
        
            return Ok(output);
        },
        _ => {
            return Err(CodegenError::GenericError);
        }
    }
}

fn codegen_expression(expression: Expression) -> Result<String, CodegenError> {
    let mut output = String::new();

    match expression {
        Expression::IntLiteral(value) => {
            output.push_str(&format!("    mov rax, {}\n", value));
        },
        Expression::UnaryOp(operator, inner) => {
            output.push_str(&codegen_expression(*inner)?);
            output.push_str("    neg rax\n");
        },
        Expression::BinaryOp(lhs,operator ,rhs ) => {
            let left = codegen_expression(*lhs)?;
            output.push_str(&left);
            output.push_str("    push rax\n");

            let right = codegen_expression(*rhs)?;
            output.push_str(&right);
            output.push_str("    pop rbx\n");

            match operator {
                Operator::Add => {
                    output.push_str("    add rbx, rax\n");
                    output.push_str("    mov rax, rbx\n");
                },
                Operator::Subtract => {
                    output.push_str("    sub rbx, rax\n");
                    output.push_str("    mov rax, rbx\n");
                },
                Operator::Multiply => {
                    output.push_str("    imul rbx, rax\n");
                    output.push_str("    mov rax, rbx\n");
                },
                Operator::Divide => {
                    output.push_str("    xchg rax, rbx\n");
                    output.push_str("    xor rdx, rdx\n");
                    output.push_str("    idiv rbx\n");
                }
            }
        }
    }

    return Ok(output);
}