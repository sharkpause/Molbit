use crate::parser::{Expression, Statement,TopLevel};

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
        Statement::Return(Expression::IntLiteral(value)) => {
            output.push_str(&format!("    mov rax, {}\n", value));
            output.push_str("    ret\n");
        
            return Ok(output);
        },
        _ => {
            return Err(CodegenError::GenericError);
        }
    }
}