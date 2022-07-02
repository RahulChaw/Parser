use crate::Token;
use std::process;
extern crate custom_error;
use custom_error::custom_error;

custom_error! { pub MyError
    Valid = "Input program is syntactacilly correct.",
    Invalid{l: i32, c: i32, syn: String} = "Error at Line {l} Character {c}. The syntax should be: {syn}."
}

pub struct Parser {
    lst: Vec<Token>,
    curr_index: usize
}

impl Parser {
    pub fn new(tkn_vec: Vec<Token>) -> Parser {
        Parser {
           lst: tkn_vec,
           curr_index: 0
        }
    }
    
    pub fn get_next_tkn(&mut self) -> &Token {
        self.curr_index += 1;
        return &self.lst[self.curr_index];
    }

    pub fn peek_next_tkn(&mut self) -> &Token {
        return &self.lst[self.curr_index + 1];
    }


    // Program := {Declaration} MainDeclaration {FunctionDefintion}
    pub fn fun_program(&mut self) -> Result<(), String> {
        let mut fl = false;
        let curr_tkn = &self.lst[self.curr_index];
        if curr_tkn.text == "unsigned" || curr_tkn.text == "char" || curr_tkn.text == "int" || curr_tkn.text == "short" || curr_tkn.text == "long" || curr_tkn.text == "float" || curr_tkn.text == "double" {
            fl = true;
            while &self.lst[self.curr_index].text == "unsigned" || &self.lst[self.curr_index].text == "char" || &self.lst[self.curr_index].text == "int" || &self.lst[self.curr_index].text == "short" || &self.lst[self.curr_index].text == "long" || &self.lst[self.curr_index].text == "float" || &self.lst[self.curr_index].text == "double" {
                
                match self.fun_dec() {
                    Ok(()) => { 
                        continue;
                    }
                    Err(_) => {
                        
                        println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Program := {Declaration} MainDeclaration {Function}".to_string()});
                        process::exit(0);
                    }
                }
            }

        }

        if &self.peek_next_tkn().text == "unsigned" || &self.peek_next_tkn().text == "char" || &self.peek_next_tkn().text == "int" || &self.peek_next_tkn().text == "short" || &self.peek_next_tkn().text == "long" || &self.peek_next_tkn().text == "float" || &self.peek_next_tkn().text == "double" {
            while &self.peek_next_tkn().text == "unsigned" || &self.peek_next_tkn().text == "char" || &self.peek_next_tkn().text == "int" || &self.peek_next_tkn().text == "short" || &self.peek_next_tkn().text == "long" || &self.peek_next_tkn().text == "float" || &self.peek_next_tkn().text == "double" {
                let _ = &self.get_next_tkn();
                match self.fun_dec() {
                    Ok(()) => { 
                        continue;
                    }
                    Err(_) => {
                        
                        println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Program := {Declaration} MainDeclaration {Function}".to_string()});
                        process::exit(0);
                    }
                }
            }
            
        }
        
        if self.curr_index != 0 {
            let _ = &self.get_next_tkn();
        }
        match self.fun_main_dec() {
            Ok(()) => {
                if self.curr_index != self.lst.len() - 1 && (fl == true || &self.peek_next_tkn().text == "unsigned" || &self.peek_next_tkn().text == "char" || &self.peek_next_tkn().text == "int" || &self.peek_next_tkn().text == "short" || &self.peek_next_tkn().text == "long" || &self.peek_next_tkn().text == "float" || &self.peek_next_tkn().text == "double") {
                    while self.curr_index != self.lst.len() - 1 && (fl == true || &self.peek_next_tkn().text == "unsigned" || &self.peek_next_tkn().text == "char" || &self.peek_next_tkn().text == "int" || &self.peek_next_tkn().text == "short" || &self.peek_next_tkn().text == "long" || &self.peek_next_tkn().text == "float" || &self.peek_next_tkn().text == "double") {
                        let _ = &self.get_next_tkn();
                        match self.fun_fun_def() {
                            Ok(()) => {
                                continue;
                            }
                            Err(_) => {
                                // return error
                                println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Program := {Declaration} MainDeclaration {Function}".to_string()});
                                process::exit(0);
                            }
                        }
                    }
                }
            }
            Err(_) => {
                
                println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Program := {Declaration} MainDeclaration {Function}".to_string()});
                process::exit(0);
            }
        }
        println!("Input program is syntactacilly correct.");
        return Ok(());
    }

    // Declaration := DeclarationType (VariableDeclaration | FunctionDeclaration)
    pub fn fun_dec(&mut self) -> Result<(), String> {
        match self.fun_dec_type() {
            Ok(()) => {
                let next_tkn = self.get_next_tkn();
                if next_tkn.text == "=" || next_tkn.text == ";" {
                    match self.fun_var_dec() {
                        Ok(()) => {
                            return Ok(());
                        }
                        Err(_) => {
                            
                            println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Declaration := DeclarationType (VariableDeclaration | FunctionDeclaration)".to_string()});
                            process::exit(0);
                        }
                    }
                }
                else if next_tkn.text == "(" {
                    match self.fun_fun_dec() {
                        Ok(()) => {
                            return Ok(());
                        }
                        Err(_) => {
                            
                            println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Declaration := DeclarationType (VariableDeclaration | FunctionDeclaration)".to_string()});
                            process::exit(0);
                        }
                    }
                }
                else {
                    
                    println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Declaration := DeclarationType (VariableDeclaration | FunctionDeclaration)".to_string()});
                    process::exit(0);
                }
            }
            Err(_) => {
                println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Declaration := DeclarationType (VariableDeclaration | FunctionDeclaration)".to_string()});
                process::exit(0);
            }
        }
    }

    // FunctionDefinition := DeclarationType ParameterBlock Block
    pub fn fun_fun_def(&mut self) -> Result<(), String> {
        match self.fun_dec_type() {
            Ok(()) => {
                let _ = &self.get_next_tkn();
                match self.fun_para_blk() {
                    Ok(()) => {
                        match self.fun_blk() {  
                            Ok(()) => {
                                return Ok(());
                            }
                            Err(_) => {
                                println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "FunctionDefinition := DeclarationType ParameterBlock Block".to_string()});
                                process::exit(0);
                            }
                        }
                    }
                    Err(_) => {
                        println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "FunctionDefinition := DeclarationType ParameterBlock Block".to_string()});
                        process::exit(0);
                    }
                }
            }
            Err(_) => {
                // return error
                println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "FunctionDefinition := DeclarationType ParameterBlock Block".to_string()});
                process::exit(0);
            }
        }
    }

    // DeclarationType := DataType Identifier
    pub fn fun_dec_type(&mut self) -> Result<(), String> {
        match self.fun_data_type() {
            Ok(()) => {
                let next_tkn = &self.get_next_tkn();
                if next_tkn.token_type == "identifier" {
                    return Ok(());
                }
                println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "DeclarationType := DataType Identifier".to_string()});
                process::exit(0);
            }
            Err(_) => {
                println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "DeclarationType := DataType Identifier".to_string()});
                process::exit(0);
            }
        }
    }

    // VariableDeclaration := [= Constant] ;
    pub fn fun_var_dec(&mut self) -> Result<(), String> {
        let curr_tkn = &self.lst[self.curr_index];
        if curr_tkn.text == ";" {
            return Ok(());
        }
        else {
            match self.fun_constant() {
                Ok(()) => {
                    let next_tkn1 = &self.get_next_tkn();
                    if next_tkn1.text == ";" {
                        return Ok(());
                    }
                    else {
                        println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "VariableDeclaration := [= Constant] ;".to_string()});
                        process::exit(0);
                    }
                }
                Err(_) => {
                    println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "VariableDeclaration := [= Constant] ;".to_string()});
                    process::exit(0);
                }
            }
        }
    }

    // FunctionDeclaration := ParameterBlock ;
    pub fn fun_fun_dec(&mut self) -> Result<(), String> {
        match self.fun_para_blk() {
            Ok(()) => {
                let next_tkn = &self.get_next_tkn();
                if next_tkn.text == ";" {
                    return Ok(());
                }
                else {
                    println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "FunctionDeclaration := ParameterBlock ;".to_string()});
                    process::exit(0);
                }
            }
            Err(_) => {
                println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "FunctionDeclaration := ParameterBlock ;".to_string()});
                process::exit(0);
            }
        }
    }

    // ParameterBlock := ( [Parameter {, Parameter}] )
    pub fn fun_para_blk(&mut self) -> Result<(), String> {
       
        let curr_tkn = &self.lst[self.curr_index];
        if curr_tkn.text == "(" {
            if &self.peek_next_tkn().text == ")" {
                let _ = &self.get_next_tkn();
                return Ok(());
            }
            else {
                match self.fun_para() {
                    Ok(()) => {
                        if &self.peek_next_tkn().text == "," {
                            while &self.get_next_tkn().text == "," {
                                match self.fun_para() {
                                    Ok(()) => {
                                        continue;
                                    }
                                    Err(_) => {
                                        // return error
                                        println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "ParameterBlock := ( [Parameter {, Parameter}] )".to_string()});
                                        process::exit(0);
                                    }
                                }
                            }
                        }
                        else if &self.peek_next_tkn().text == ")" {
                            let _ = &self.get_next_tkn();
                        }
                        else {
                            // return error
                            println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "ParameterBlock := ( [Parameter {, Parameter}] )".to_string()});
                            process::exit(0);
                        }
                    }
                    Err(_) => {
                        // return error
                        println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "ParameterBlock := ( [Parameter {, Parameter}] )".to_string()});
                        process::exit(0);
                    }
                }
            }
        }
        else {
            // error
            println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "ParameterBlock := ( [Parameter {, Parameter}] )".to_string()});
            process::exit(0);
        }
        return Ok(());
    }

    // DataType := IntegerType | FloatType
    pub fn fun_data_type(&mut self) -> Result<(), String> {
        match self.fun_float_type() {
            Ok(()) => {
                return Ok(());
            }
            Err(_) => {
                match self.fun_int_type() {
                    Ok(()) => {
                        return Ok(());
                    }
                    Err(_) => {
                        // return error
                        println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "DataType := IntegerType | FloatType".to_string()});
                        process::exit(0);
                    }
                }
                // return error
                println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "DataType := IntegerType | FloatType".to_string()});
                process::exit(0);
            }
        }
    }

    // Constant := IntConstant | FloatConstant
    pub fn fun_constant(&mut self) -> Result<(), String> {
        let next_tkn = &self.get_next_tkn();
        if next_tkn.token_type != "intconstant" && next_tkn.token_type != "floatconstant" {
            // error
            println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Constant := IntConstant | FloatConstant".to_string()});
            process::exit(0);
        }
        return Ok(());
    }

    // Parameter := DataType Identifier
    pub fn fun_para(&mut self) -> Result<(), String> {
        let _ = &self.get_next_tkn();
        match self.fun_data_type() {
            Ok(()) => {
                let next_tkn = &self.get_next_tkn();
                if next_tkn.token_type == "identifier" {
                    return Ok(());
                }
                println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Parameter := DataType Identifier".to_string()});
                process::exit(0);
            }
            Err(_) => {
                // return error
                println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Parameter := DataType Identifier".to_string()});
                process::exit(0);
            }
        }
    }

    // IntegerType := [unsigned] (char | short | int | long)
    pub fn fun_int_type(&mut self) -> Result<(), String> {
        let next_tkn = &self.lst[self.curr_index];
        if next_tkn.text == "unsigned" {
            let next_tkn1 = &self.get_next_tkn();
            if next_tkn1.text != "char" && next_tkn1.text != "short" && next_tkn1.text != "int" && next_tkn1.text != "long" {
                // return error
                return Err("Error".to_string());
            }
            return Ok(());
        }
        else if next_tkn.text != "char" && next_tkn.text != "short" && next_tkn.text != "int" && next_tkn.text != "long" {
            // return error
            return Err("Error".to_string());
        }
        return Ok(());
    }

    // FloatType := float | double
    pub fn fun_float_type(&mut self) -> Result<(), String> {
        let next_tkn = &self.lst[self.curr_index];
        if next_tkn.text != "float" && next_tkn.text != "double" {
            // return error
            return Err("Error".to_string());
        }
        return Ok(());
    }

    // WhileLoop := while ( Expression ) Block
    pub fn fun_while_lp(&mut self) -> Result<(), String> {
        let next_tkn = &self.get_next_tkn();
        if next_tkn.text == "while" {
            let next_tkn1 = &self.get_next_tkn();
            if next_tkn1.text == "(" {
                match self.fun_exp() { 
                    Ok(()) => {
                        let next_tkn2 = &self.get_next_tkn();
                        if next_tkn2.text == ")" {
                            match self.fun_blk() {  
                                Ok(()) => {
                                    return Ok(());
                                }
                                Err(_) => {
                                    // return error
                                    println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "WhileLoop := while ( Expression ) Block".to_string()});
                                    process::exit(0);
                                }
                            }
                        }
                        else {
                            println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "WhileLoop := while ( Expression ) Block".to_string()});
                            process::exit(0);
                        }
                    }
                    Err(_) => {
                        println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "WhileLoop := while ( Expression ) Block".to_string()});
                        process::exit(0);
                    }
                }
            }
            else {
                println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "WhileLoop := while ( Expression ) Block".to_string()});
                process::exit(0);
            }
        }
        else {
            println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "WhileLoop := while ( Expression ) Block".to_string()});
            process::exit(0);
        }
    }

    // IfStatement := if ( Expression ) Block
    pub fn fun_if_state(&mut self) -> Result<(), String> {
        let next_tkn = &self.get_next_tkn();
        if next_tkn.text == "if" {
            let next_tkn1 = &self.get_next_tkn();
            if next_tkn1.text == "(" {
                match self.fun_exp() {      
                    Ok(()) => {
                        let next_tkn2 = &self.get_next_tkn();
                        if next_tkn2.text == ")" {
                            match self.fun_blk() {  
                                Ok(()) => {
                                    return Ok(());
                                }
                                Err(_) => {
                                    // return error
                                    println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "IfStatement := if ( Expression ) Block".to_string()});
                                    process::exit(0);
                                }
                            }
                        }
                        else {
                            println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "IfStatement := if ( Expression ) Block".to_string()});
                            process::exit(0);
                        }
                    }
                    Err(_) => {
                        // return error
                        println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "IfStatement := if ( Expression ) Block".to_string()});
                        process::exit(0);
                    }
                }
            }
            else {
                println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "IfStatement := if ( Expression ) Block".to_string()});
                process::exit(0);
            }
        }
        else {
            println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "IfStatement := if ( Expression ) Block".to_string()});
            process::exit(0);
        }
    }

    // ReturnStatement := return Expression ;
    pub fn fun_ret_state(&mut self) -> Result<(), String> {
        let next_tkn = &self.get_next_tkn();
        if next_tkn.text == "return" {
            match self.fun_exp() {      
                Ok(()) => {
                    let next_tkn1 = &self.get_next_tkn();
                    if next_tkn1.text == ";" {
                        return Ok(());
                    }
                    else {
                        println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "ReturnStatement := return Expression ;".to_string()});
                        process::exit(0);
                    }
                }
                Err(_) => {
                    // return error
                    println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "ReturnStatement := return Expression ;".to_string()});
                    process::exit(0);
                }
            }
        }
        else {
            println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "ReturnStatement := return Expression ;".to_string()});
            process::exit(0);
        }
    }

    // MainDeclaration := void main () Block
    pub fn fun_main_dec(&mut self) -> Result<(), String> {
        let curr_tkn = &self.lst[self.curr_index];
        if curr_tkn.text == "void" {
            let next_tkn = &self.get_next_tkn();
            if next_tkn.text == "main" {
                let next_tkn1 = &self.get_next_tkn();
                if next_tkn1.text == "(" {
                    let next_tkn2 = &self.get_next_tkn();
                    if next_tkn2.text == ")" {
                        match self.fun_blk() {
                            Ok(()) => {
                                return Ok(());
                            }
                            Err(_) => {
                                // return error
                                println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "MainDeclaration := void main () Block".to_string()});
                                process::exit(0);
                            }
                        }
                    }
                    else {
                        // return error
                        println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "MainDeclaration := void main () Block".to_string()});
                        process::exit(0);
                    }
                }
                else {
                    // return error
                    println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "MainDeclaration := void main () Block".to_string()});
                    process::exit(0);
                }
            }
            else {
                // return error
                println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "MainDeclaration := void main () Block".to_string()});
                process::exit(0);
            }
        }
        else {
            // return error
            println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "MainDeclaration := void main () Block".to_string()});
            process::exit(0);
        }
    }

    //Block := { {Declaration} {Statement} {FunctionDefinition} }
    pub fn fun_blk(&mut self) -> Result<(), String> {
        let next_tkn = &self.get_next_tkn();
        if next_tkn.text != "{" {
            // return error
            println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Block := { {Declaration} {Statement} {FunctionDefinition} }".to_string()});
            process::exit(0);
        }
        if &self.peek_next_tkn().text == "unsigned" || &self.peek_next_tkn().text == "char" || &self.peek_next_tkn().text == "int" || &self.peek_next_tkn().text == "short" || &self.peek_next_tkn().text == "long" || &self.peek_next_tkn().text == "float" || &self.peek_next_tkn().text == "double" {
            while &self.peek_next_tkn().text == "unsigned" || &self.peek_next_tkn().text == "char" || &self.peek_next_tkn().text == "int" || &self.peek_next_tkn().text == "short" || &self.peek_next_tkn().text == "long" || &self.peek_next_tkn().text == "float" || &self.peek_next_tkn().text == "double" {
                let _ = &self.get_next_tkn();
                match self.fun_dec() {
                    Ok(()) => {
                        continue;
                    }
                    Err(_) => {
                        // return error
                        println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Block := { {Declaration} {Statement} {FunctionDefinition} }".to_string()});
                        process::exit(0);
                    }
                }
            }
        }

        if &self.peek_next_tkn().token_type == "identifier" || &self.peek_next_tkn().text == "while" || &self.peek_next_tkn().text == "if" || &self.peek_next_tkn().text == "return" || &self.peek_next_tkn().text == "(" || &self.peek_next_tkn().token_type == "intconstant" || &self.peek_next_tkn().token_type == "floatconstant" {
            while &self.peek_next_tkn().token_type == "identifier" || &self.peek_next_tkn().text == "while" || &self.peek_next_tkn().text == "if" || &self.peek_next_tkn().text == "return" || &self.peek_next_tkn().text == "(" || &self.peek_next_tkn().token_type == "intconstant" || &self.peek_next_tkn().token_type == "floatconstant" {
                match self.fun_stat() {
                    Ok(()) => {
                        continue;
                    }
                    Err(_) => {
                        // return error
                        println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Block := { {Declaration} {Statement} {FunctionDefinition} }".to_string()});
                        process::exit(0);
                    }
                }
            }
        }
        
        if &self.peek_next_tkn().text == "unsigned" || &self.peek_next_tkn().text == "char" || &self.peek_next_tkn().text == "int" || &self.peek_next_tkn().text == "short" || &self.peek_next_tkn().text == "long" || &self.peek_next_tkn().text == "float" || &self.peek_next_tkn().text == "double" {
            while &self.peek_next_tkn().text == "unsigned" || &self.peek_next_tkn().text == "char" || &self.peek_next_tkn().text == "int" || &self.peek_next_tkn().text == "short" || &self.peek_next_tkn().text == "long" || &self.peek_next_tkn().text == "float" || &self.peek_next_tkn().text == "double" {
                let _ = &self.get_next_tkn();
                match self.fun_fun_def() {
                    Ok(()) => {
                        continue;
                    }
                    Err(_) => {
                        // return error
                        println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Block := { {Declaration} {Statement} {FunctionDefinition} }".to_string()});
                        process::exit(0);
                    }
                }
            }
        }

        if self.get_next_tkn().text != "}" {
            // return error
            println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Block := { {Declaration} {Statement} {FunctionDefinition} }".to_string()});
            process::exit(0);
        }
        return Ok(());
    }

    //Statement := Assignment | WhileLoop | IfStatement | ReturnStatement | (Expression ;)
    pub fn fun_stat(&mut self) -> Result<(), String> {
        if &self.peek_next_tkn().token_type == "identifier" {
            match self.fun_assgn() {
                Ok(()) => {
                    return Ok(());
                }
                Err(_) => {
                    // return error
                    println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Statement := Assignment | WhileLoop | IfStatement | ReturnStatement | (Expression ;)".to_string()});
                    process::exit(0);
                }
            }
            match self.fun_exp() {
                Ok(()) => {
                    let next_tkn = &self.get_next_tkn();
                    if next_tkn.text == ";" {
                        return Ok(());
                    }
                    else {
                        // return error
                        println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Statement := Assignment | WhileLoop | IfStatement | ReturnStatement | (Expression ;)".to_string()});
                        process::exit(0);
                    }
                }
                Err(_) => {
                    // return error
                    println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Statement := Assignment | WhileLoop | IfStatement | ReturnStatement | (Expression ;)".to_string()});
                    process::exit(0);
                }
            }
        }
        else if &self.peek_next_tkn().text == "while" {
            match self.fun_while_lp() {
                Ok(()) => {
                    return Ok(());
                }
                Err(_) => {
                    // return error
                    println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Statement := Assignment | WhileLoop | IfStatement | ReturnStatement | (Expression ;)".to_string()});
                    process::exit(0);
                }
            }
        }
        else if &self.peek_next_tkn().text == "if" {
            match self.fun_if_state() {
                Ok(()) => {
                    return Ok(());
                }
                Err(_) => {
                    // return error
                    println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Statement := Assignment | WhileLoop | IfStatement | ReturnStatement | (Expression ;)".to_string()});
                    process::exit(0);
                }
            }
        }
        else if &self.peek_next_tkn().text == "return" {
            match self.fun_ret_state() {
                Ok(()) => {
                    return Ok(());
                }
                Err(_) => {
                    // return error
                    println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Statement := Assignment | WhileLoop | IfStatement | ReturnStatement | (Expression ;)".to_string()});
                    process::exit(0);
                }
            }
        }
        else if &self.peek_next_tkn().text == "(" || &self.peek_next_tkn().token_type == "identifier" || &self.peek_next_tkn().token_type == "intconstant" || &self.peek_next_tkn().token_type == "floatconstant" {
            match self.fun_exp() {
                Ok(()) => {
                    let next_tkn = &self.get_next_tkn();
                    if next_tkn.text == ";" {
                        return Ok(());
                    }
                    else {
                        // return error
                        println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Statement := Assignment | WhileLoop | IfStatement | ReturnStatement | (Expression ;)".to_string()});
                        process::exit(0);
                    }
                }
                Err(_) => {
                    // return error
                    println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Statement := Assignment | WhileLoop | IfStatement | ReturnStatement | (Expression ;)".to_string()});
                    process::exit(0);
                }
            }
        }
        else {
            // return error
            println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Statement := Assignment | WhileLoop | IfStatement | ReturnStatement | (Expression ;)".to_string()});
            process::exit(0);
        }
    }

    //Assignment := Identifier = {Identifier =} Expression ;
    pub fn fun_assgn(&mut self) -> Result<(), String> {
        if &self.peek_next_tkn().token_type == "identifier" {
            match self.fun_iden() {
                Ok(()) => {
                    let next_tkn = &self.get_next_tkn();
                    if next_tkn.text != "=" {
                        // return error
                        println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Assignment := Identifier = {Identifier =} Expression ;".to_string()});
                        process::exit(0);
                    }
                    
                    if &self.peek_next_tkn().token_type == "identifier" && &self.lst[self.curr_index + 2].text == "=" {
                        match self.fun_iden() {
                            Ok(()) => {
                                let next_tkn1 = &self.get_next_tkn();
                                if next_tkn1.text != "=" {
                                    // return error
                                    println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Assignment := Identifier = {Identifier =} Expression ;".to_string()});
                                    process::exit(0);
                                }
                                while &self.peek_next_tkn().token_type == "identifier" {
                                    match self.fun_iden() {
                                        Ok(()) => {
                                            let next_tkn2 = &self.get_next_tkn();
                                            if next_tkn2.text != "=" {
                                                // return error
                                                println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Assignment := Identifier = {Identifier =} Expression ;".to_string()});
                                                process::exit(0);
                                            }
                                        }
                                        Err(_) => {
                                            // return error
                                            println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Assignment := Identifier = {Identifier =} Expression ;".to_string()});
                                            process::exit(0);
                                        }
                                    }
                                }
                            }
                            Err(_) => {
                                // return error
                                println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Assignment := Identifier = {Identifier =} Expression ;".to_string()});
                                process::exit(0);
                            }
                        }
                    }
                    if &self.peek_next_tkn().text == "(" || &self.peek_next_tkn().token_type == "identifier" || &self.peek_next_tkn().token_type == "intconstant" || &self.peek_next_tkn().token_type == "floatconstant" {
                        match self.fun_exp() {      
                            Ok(()) => {
                                let next_tkn3 = &self.get_next_tkn();
                                if next_tkn3.text == ";" {
                                    return Ok(());
                                }
                                else {
                                    println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Assignment := Identifier = {Identifier =} Expression ;".to_string()});
                                    process::exit(0);
                                }
                            }
                            Err(_) => {
                                // return error
                                println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Assignment := Identifier = {Identifier =} Expression ;".to_string()});
                                process::exit(0);
                            }
                        }
                    }
                    else {
                        // return error
                        println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Assignment := Identifier = {Identifier =} Expression ;".to_string()});
                        process::exit(0);
                    }
                }
                Err(_) => {
                    // return error
                    println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Assignment := Identifier = {Identifier =} Expression ;".to_string()});
                    process::exit(0);
                }
            }
        }
        else {
            // return error
            println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Assignment := Identifier = {Identifier =} Expression ;".to_string()});
            process::exit(0);
        }
    }

    //Identifier
    pub fn fun_iden(&mut self) -> Result<(), String> {
        let next_tkn = &self.get_next_tkn();
        if next_tkn.token_type == "identifier" {
            return Ok(());
        }
        else {
            // return error
            println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Identifier".to_string()});
            process::exit(0);
        }
    }

    //Expression := SimpleExpression [ RelationOperator SimpleExpression ]
    pub fn fun_exp(&mut self) -> Result<(), String> {
        match self.fun_simple() {
            Ok(()) => {
                if &self.peek_next_tkn().text == "==" || &self.peek_next_tkn().text == "<=" || &self.peek_next_tkn().text == "!=" || &self.peek_next_tkn().text == ">=" || &self.peek_next_tkn().text == ">" || &self.peek_next_tkn().text == "<" {
                    match self.fun_rel_oprtr() {
                        Ok(()) => {
                            match self.fun_simple() {
                                Ok(()) => {
                                    return Ok(());
                                }
                                Err(_) => {
                                    // return error
                                    println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Expression := SimpleExpression [ RelationOperator SimpleExpression ]".to_string()});
                                    process::exit(0);
                                }
                            }
                        }
                        Err(_) => {
                            // return error
                            println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Expression := SimpleExpression [ RelationOperator SimpleExpression ]".to_string()});
                            process::exit(0);
                        }
                    }
                }
                return Ok(());
            }
            Err(_) => {
                // return error
                println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Expression := SimpleExpression [ RelationOperator SimpleExpression ]".to_string()});
                process::exit(0);
            }
        }
    }

    //SimpleExpression := Term { AddOperator Term }
    pub fn fun_simple(&mut self) -> Result<(), String> {
        match self.fun_term() {
            Ok(()) => {
                if &self.peek_next_tkn().text == "+" || &self.peek_next_tkn().text == "-" {
                    while &self.peek_next_tkn().text == "+" || &self.peek_next_tkn().text == "-" {
                        match self.fun_add_oprtr() {
                            Ok(()) => {
                                match self.fun_term() {
                                    Ok(()) => {
                                        continue;
                                    }
                                    Err(_) => {
                                        // return error
                                        println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "SimpleExpression := Term { AddOperator Term }".to_string()});
                                        process::exit(0);
                                    }
                                }
                            }
                            Err(_) => {
                                // return error
                                println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "SimpleExpression := Term { AddOperator Term }".to_string()});
                                process::exit(0);
                            }
                        }
                    }
                }
            }
            Err(_) => {
                // return error
                println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "SimpleExpression := Term { AddOperator Term }".to_string()});
                process::exit(0);
            }
        }
        return Ok(());
    }

    //Term := Factor { MultOperator Factor }
    pub fn fun_term(&mut self) -> Result<(), String> {
        match self.fun_fctr() {
            Ok(()) => {
                if &self.peek_next_tkn().text == "*" || &self.peek_next_tkn().text == "/" {
                    while &self.peek_next_tkn().text == "*" || &self.peek_next_tkn().text == "/" {
                        match self.fun_mul_oprtr() {
                            Ok(()) => {
                                match self.fun_fctr() {
                                    Ok(()) => {
                                        continue;
                                    }
                                    Err(_) => {
                                        // return error
                                        println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Term := Factor { MultOperator Factor }".to_string()});
                                        process::exit(0);
                                    }
                                }
                            }
                            Err(_) => {
                                // return error
                                println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Term := Factor { MultOperator Factor }".to_string()});
                                process::exit(0);
                            }
                        }
                    }
                }
            }
            Err(_) => {
                // return error
                println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Term := Factor { MultOperator Factor }".to_string()});
                process::exit(0);
            }
        }
        return Ok(());
    }

    //Factor := ( ( Expression ) ) | Constant | ( Identifier [ ( [ Expression {, Expression } ] ) ] )
    pub fn fun_fctr(&mut self) -> Result<(), String> {
        if &self.peek_next_tkn().text == "("  {
            let _ = &self.get_next_tkn();
            match self.fun_exp() {
                Ok(()) => {
                    let next_tkn1 = &self.get_next_tkn();
                    if next_tkn1.text != ")" {
                        // return error
                        println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Factor := ( ( Expression ) ) | Constant | ( Identifier [ ( [ Expression {, Expression } ] ) ] )".to_string()});
                        process::exit(0);
                    }
                    return Ok(());
                }
                Err(_) => {
                    // return error
                    println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Factor := ( ( Expression ) ) | Constant | ( Identifier [ ( [ Expression {, Expression } ] ) ] )".to_string()});
                    process::exit(0);
                }
            }
        }
        else if &self.peek_next_tkn().token_type == "intconstant" || &self.peek_next_tkn().token_type == "floatconstant" {
            match self.fun_constant() {
                Ok(()) => {
                    return Ok(());
                }
                Err(_) => {
                    // return error
                    println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Factor := ( ( Expression ) ) | Constant | ( Identifier [ ( [ Expression {, Expression } ] ) ] )".to_string()});
                    process::exit(0);
                }
            }
        }
        else if &self.peek_next_tkn().token_type == "identifier" {
            match self.fun_iden() {
                Ok(()) => {
                    if &self.peek_next_tkn().text == "(" {
                        let _ = &self.get_next_tkn();
                        if &self.peek_next_tkn().text == "(" || &self.peek_next_tkn().token_type == "identifier" || &self.peek_next_tkn().token_type == "intconstant" || &self.peek_next_tkn().token_type == "floatconstant" {
                            match self.fun_exp() {
                                Ok(()) => {
                                    if &self.peek_next_tkn().text == "," {
                                        while &self.get_next_tkn().text == "," {
                                            match self.fun_exp() {
                                                Ok(()) => {
                                                    continue;
                                                }
                                                Err(_) => {
                                                    // return error
                                                    println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Factor := ( ( Expression ) ) | Constant | ( Identifier [ ( [ Expression {, Expression } ] ) ] )".to_string()});
                                                    process::exit(0);
                                                }
                                            }
                                        }
                                    }
                                    if &self.peek_next_tkn().text == ")" {
                                        // return error
                                        let _ = &self.get_next_tkn();
                                        return Ok(());
                                    }
                                }
                                Err(_) => {
                                    // return error
                                    println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Factor := ( ( Expression ) ) | Constant | ( Identifier [ ( [ Expression {, Expression } ] ) ] )".to_string()});
                                    process::exit(0);
                                }
                            }
                        }
                        else {
                            if &self.peek_next_tkn().text != ")" {
                                // return error
                                println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Factor := ( ( Expression ) ) | Constant | ( Identifier [ ( [ Expression {, Expression } ] ) ] )".to_string()});
                                process::exit(0);
                            }
                            return Ok(());
                        }
                    }
                }
                Err(_) => {
                    // return error
                    println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Factor := ( ( Expression ) ) | Constant | ( Identifier [ ( [ Expression {, Expression } ] ) ] )".to_string()});
                    process::exit(0);
                }
            }
        }
        else {
            // return error
            println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "Factor := ( ( Expression ) ) | Constant | ( Identifier [ ( [ Expression {, Expression } ] ) ] )".to_string()});
            process::exit(0);
        }
        return Ok(());
    }

    //RelationOperator := (==) | < | > | (<=) | (>=) | (!=)
    pub fn fun_rel_oprtr(&mut self) -> Result<(), String> {
        let next_tkn = &self.get_next_tkn();
        if next_tkn.text == "==" || next_tkn.text == "<=" || next_tkn.text == ">=" || next_tkn.text == "!=" || next_tkn.text == "<" || next_tkn.text == ">" {
            return Ok(());
        }
        else {
            // return error
            println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "RelationOperator := (==) | < | > | (<=) | (>=) | (!=)".to_string()});
            process::exit(0);
        }
    }

    //MultOperator := * | /
    pub fn fun_mul_oprtr(&mut self) -> Result<(), String> {
        let next_tkn = &self.get_next_tkn();
        if next_tkn.text == "*" || next_tkn.text == "/" {
            return Ok(());
        }
        else {
            // return error
            println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "MultOperator := * | /".to_string()});
            process::exit(0);
        }
    }

    //AddOperator := + | -
    pub fn fun_add_oprtr(&mut self) -> Result<(), String> {
        let next_tkn = &self.get_next_tkn();
        if next_tkn.text == "+" || next_tkn.text == "-" {
            return Ok(());
        }
        else {
            // return error
            println!("{}", MyError::Invalid{l: self.lst[self.curr_index].line_num, c: self.lst[self.curr_index].char_pos, syn: "AddOperator := + | -".to_string()});
            process::exit(0);
        }
    }
}
    
