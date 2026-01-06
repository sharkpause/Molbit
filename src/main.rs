mod token;
mod lexer;
mod parser;
mod codegen;

use std::{env, fs, process::Command};

use crate::token::print_token;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::parser::TopLevel;
use crate::parser::Statement;
use crate::parser::Expression;
use crate::codegen::{ CodeGenerator };

fn read_file(path: &String) -> String {
    let source_code =
        fs::read_to_string(path).expect("Failed to read source code");

    return source_code;
}

fn write_file(path: String, contents: &String) {
    fs::write(path, contents).expect("Failed to write assembly");
}

fn assemble_and_link(asm_path: &str, output_exe: &str) {
    let nasm_status = Command::new("nasm")
        .args(&["-f", "elf64", asm_path, "-o", "out.o"])
        .status()
        .expect("Failed to run NASM");

    if !nasm_status.success() {
        panic!("Assembling failed");
    }

    let ld_status = Command::new("ld")
        .args(&["out.o", "-o", output_exe])
        .status()
        .expect("Failed to run LD");

    if !ld_status.success() {
        panic!("Linking failed");
    }

    println!("Executable '{}' produced", output_exe);
}

fn print_statement(stmt: &Statement, indent: usize) {
    let padding = "  ".repeat(indent);

    match stmt {
        Statement::Return(expr) => {
            println!("{}Return:", padding);
            print_expression(expr, indent + 1);
        }
        Statement::VariableDeclare(ty, name, expr) => {
            println!("{}Declare {:?} {}", padding, ty, name);
            print_expression(expr, indent + 1);
        }
        Statement::VariableAssignment(name, expr) => {
            println!("{}Assign {}", padding, name);
            print_expression(expr, indent + 1);
        }
        Statement::Block(stmts) => {
            println!("{}Block:", padding);
            for s in stmts {
                print_statement(s, indent + 1);
            }
        }
        Statement::Expression(expr) => {
            println!("{}Expression:", padding);
            print_expression(expr, indent + 1);
        }
        Statement::If(cond, then_body, else_body) => {
            println!("{}If:", padding);
            print_expression(cond, indent + 1);
            println!("{}Then:", padding);
            print_statement(then_body, indent + 1);
            if let Some(else_stmt) = else_body {
                println!("{}Else:", padding);
                print_statement(else_stmt, indent + 1);
            }
        }
        Statement::Else(body) => {
            println!("{}Else:", padding);
            print_statement(body, indent + 1);
        }
    }
}

fn print_expression(expr: &Expression, indent: usize) {
    let padding = "  ".repeat(indent);

    match expr {
        Expression::Variable(name) => println!("{}Variable {}", padding, name),
        Expression::IntLiteral(value) => println!("{}Int {}", padding, value),
        Expression::UnaryOperation(op, inner) => {
            println!("{}Unary {:?}", padding, op);
            print_expression(inner, indent + 1);
        }
        Expression::BinaryOperation(lhs, op, rhs) => {
            println!("{}Binary {:?}", padding, op);
            print_expression(lhs, indent + 1);
            print_expression(rhs, indent + 1);
        }
        Expression::FunctionCall(func, args) => {
            println!("{}Call:", padding);
            print_expression(func, indent + 1);
            for arg in args {
                print_expression(arg, indent + 1);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Incorrect usage");
        return;
    }

    let input = read_file(&args[1]);

    let mut lexer = Lexer {
        input,
        index: 0,
    };

    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(e) => {
            eprintln!("lexer error: {:?}", e);
            return;
        }
    };

    println!("Tokens:");
    for token in &tokens {
        print_token(token);
    }

    let mut parser = Parser {
        tokens,
        index: 0,
    };

    let program = match parser.parse_program() {
        Ok(program) => program,
        Err(e) => {
            eprintln!("parser error: {:?}", e);
            return;
        }
    };

    for toplevel in &program {
        match toplevel {
            TopLevel::Function(f) => {
                println!("Function: {}", f.name);
                print_statement(&f.body, 1);
            }
            TopLevel::Statement(s) => print_statement(&s, 1),
        }
    }

    let mut codegen = CodeGenerator::default();

    let output = match codegen.generate(program) {
        Ok(output) => output,
        Err(e) => {
            eprintln!("codegen error: {:?}", e);
            return;
        }
    };

    println!("{}", output);

    write_file(String::from("out.asm"), &output);
    assemble_and_link("out.asm", "out");
}