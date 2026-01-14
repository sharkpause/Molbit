use crate::parser::{ TopLevel };

enum SemanticError {
    NoEntryFunction,
    MainIsReserved,
}

pub struct SemanticAnalyzer {
    functions: Vec<TopLevel>
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        return Self {
            functions: Vec::new()
        };
    }
}

pub fn validate(program: &Vec<TopLevel>) -> Result<(), SemanticError> {
    
    // TODO: Make minimal semantic check for no entry function and main is reserved
}