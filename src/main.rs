use std::fs::File;
use std::io::Read;
use std::io::stdin;

use crate::out_xhtml::Out_xhtml;
use crate::parser::Parser;

pub mod out_xhtml;
pub mod parser;

enum TokenType {
    INVALID,
    IDENTIFIER,
    KEYWORD,
    OPERATOR,
    INTCONSTANT,
    FLOATCONSTANT
}

impl TokenType {
    fn as_str(&self) -> &'static str {
        match &self {
            TokenType::INVALID => "invalid",
            TokenType::IDENTIFIER => "identifier",
            TokenType::KEYWORD => "keyword",
            TokenType::OPERATOR => "operator",
            TokenType::INTCONSTANT => "intconstant",
            TokenType::FLOATCONSTANT => "floatconstant"
        }
    }
}

pub struct Token {
    text: String,
    token_type: String,
    line_num: i32,
    char_pos: i32
}

impl Token {
    pub fn new(input: String, tkn: &str, ln_num: i32, ch_pos: i32) -> Token {
        Token {
            text: input,
            token_type: tkn.to_string(),
            line_num: ln_num,
            char_pos: ch_pos
        }
    }
}

struct Scanner {
    stream: Cstream,
    filename: String
}

impl Scanner {
    fn new(fname: String) -> Scanner {
        Scanner {
            stream: Cstream::new(fname.to_string()),
            filename: fname
        }
    }

    fn tokenize(&mut self) -> Vec<Token> {          
        let input = self.stream.read(); 
        //println!("Here {}", input);  
        let mut flag = 0;
        let mut digit = 0;
        let mut flt = 0;
        let mut inv = false;
        let mut obj: Token = Token::new("".to_string(), "", 0, 0);
        let mut token_objs = Vec::<Token>::new();
        let mut ch_pos = -1;
        let mut ln_num = 0;
        let mut alpha = false;

        for (i, elem) in input.chars().enumerate() {
            ch_pos = ch_pos + 1;
            //println!("{}", elem);
            if (elem == '=' || elem == '<' || elem == '>' || elem == '!') && input.chars().nth(i as usize + 1).unwrap() == '=' {
                obj = Token::new(input[i..i as usize + 2].to_string(), TokenType::OPERATOR.as_str(), ln_num, ch_pos);
                flag = 1;
            }
            else if elem == ' ' || elem == '\n' || elem == '\t' || flag != 0 {
                if elem == '\n' {
                    ln_num += 1;
                    ch_pos = -1;
                }
                if flag != 0 {
                    flag = flag - 1;
                }
                if elem == '\t' {
                    ch_pos = ch_pos + 4;
                }
                continue;
            }
            else if elem == '=' || elem == '<' || elem == '>' || elem == '(' || elem == ')' || elem == '{' || elem == '}' || elem == ',' || elem == '+' || (elem == '-' && !(input.chars().nth(i as usize + 1).unwrap().is_digit(10))) || elem == '*' || elem == '/' || elem == ';'{
                obj = Token::new(elem.to_string(), TokenType::OPERATOR.as_str(), ln_num, ch_pos);
            }
            else if elem == 'u' && alpha == false && input[i..i as usize + 8].to_string() == "unsigned".to_string() && input.chars().nth(i as usize + 8).unwrap() == ' ' {
                obj = Token::new(input[i..i as usize + 8].to_string(), TokenType::KEYWORD.as_str(), ln_num, ch_pos);
                flag = 7;
            }
            else if elem == 'c' && alpha == false && input[i..i as usize + 4].to_string() == "char".to_string() && input.chars().nth(i as usize + 4).unwrap() == ' ' {
                obj = Token::new(input[i..i as usize + 4].to_string(), TokenType::KEYWORD.as_str(), ln_num, ch_pos);
                flag = 3;
            }
            else if elem == 's' && alpha == false && input[i..i as usize + 5].to_string() == "short".to_string() && input.chars().nth(i as usize + 5).unwrap() == ' ' {
                obj = Token::new(input[i..i as usize + 5].to_string(), TokenType::KEYWORD.as_str(), ln_num, ch_pos);
                flag = 4;
            }
            else if elem == 'i' && alpha == false && input[i..i as usize + 3].to_string() == "int".to_string() && input.chars().nth(i as usize + 3).unwrap() == ' ' {
                obj = Token::new(input[i..i as usize + 3].to_string(), TokenType::KEYWORD.as_str(), ln_num, ch_pos);
                flag = 2;
            }
            else if elem == 'l' && alpha == false && input[i..i as usize + 4].to_string() == "long".to_string() && input.chars().nth(i as usize + 4).unwrap() == ' ' {
                obj = Token::new(input[i..i as usize + 4].to_string(), TokenType::KEYWORD.as_str(), ln_num, ch_pos);
                flag = 3;
            }
            else if elem == 'f' && alpha == false && input[i..i as usize + 5].to_string() == "float".to_string() && input.chars().nth(i as usize + 5).unwrap() == ' ' {
                obj = Token::new(input[i..i as usize + 5].to_string(), TokenType::KEYWORD.as_str(), ln_num, ch_pos);
                flag = 4;
            }
            else if elem == 'd' && alpha == false && input[i..i as usize + 6].to_string() == "double".to_string() && input.chars().nth(i as usize + 6).unwrap() == ' ' {
                obj = Token::new(input[i..i as usize + 6].to_string(), TokenType::KEYWORD.as_str(), ln_num, ch_pos);
                flag = 5;
            }
            else if elem == 'w' && alpha == false && input[i..i as usize + 5].to_string() == "while".to_string() && (input.chars().nth(i as usize + 5).unwrap() == ' ' || input.chars().nth(i as usize + 5).unwrap() == '(') {
                obj = Token::new(input[i..i as usize + 5].to_string(), TokenType::KEYWORD.as_str(), ln_num, ch_pos);
                flag = 4;
            }
            else if elem == 'i' && alpha == false && input[i..i as usize + 2].to_string() == "if".to_string() && (input.chars().nth(i as usize + 2).unwrap() == ' ' || input.chars().nth(i as usize + 2).unwrap() == '(') {
                obj = Token::new(input[i..i as usize + 2].to_string(), TokenType::KEYWORD.as_str(), ln_num, ch_pos);
                flag = 1;
            }
            else if elem == 'r' && alpha == false && input[i..i as usize + 6].to_string() == "return".to_string() && (input.chars().nth(i as usize + 6).unwrap() == ' ' || input.chars().nth(i as usize + 6).unwrap() == ';') {
                obj = Token::new(input[i..i as usize + 6].to_string(), TokenType::KEYWORD.as_str(), ln_num, ch_pos);
                flag = 5;
            }
            else if elem == 'v' && alpha == false && input[i..i as usize + 4] == "void".to_string() && (input.chars().nth(i as usize + 4).unwrap() == ' ')  {
                //println!("here");
                obj = Token::new(input[i..i as usize + 4].to_string(), TokenType::KEYWORD.as_str(), ln_num, ch_pos);
                flag = 3;
            }
            else if elem == 'm' && alpha == false && input[i..i as usize + 4].to_string() == "main".to_string() && (input.chars().nth(i as usize + 4).unwrap() == ' ' || input.chars().nth(i as usize + 4).unwrap() == '(') {
                obj = Token::new(input[i..i as usize + 4].to_string(), TokenType::KEYWORD.as_str(), ln_num, ch_pos);
                flag = 3;
            }
            else if (elem.is_digit(10) || elem == '.' || inv == true || elem == '-') && alpha == false {
                if digit == 0 {
                    if elem == '.' {
                        inv = true;
                    }
                    alpha = false;
                    digit = i;
                }
                if elem == '.' {
                    flt = flt + 1;
                }
                if (input.chars().nth(i as usize + 1).unwrap()).is_alphabetic() {
                    inv = true;
                    continue;
                }
                if (input.chars().nth(i as usize + 1).unwrap()).is_digit(10) || (input.chars().nth(i as usize + 1).unwrap()) == '.' {
                    continue;
                }
                else {
                    if flt == 1 && inv == false {
                        obj = Token::new(input[digit..i as usize + 1].to_string(), TokenType::FLOATCONSTANT.as_str(), ln_num, ch_pos - (i - digit) as i32);
                    }
                    else if flt == 0 && inv == false {
                        obj = Token::new(input[digit..i as usize + 1].to_string(), TokenType::INTCONSTANT.as_str(), ln_num, ch_pos - (i - digit) as i32);
                    }
                    else {
                        obj = Token::new(input[digit..i as usize + 1].to_string(), TokenType::INVALID.as_str(), ln_num, ch_pos - (i - digit) as i32);
                    }
                    digit = 0;
                    inv = false;
                    flt = 0;
                }
            }
            else if elem.is_alphabetic() || elem == '_' || alpha == true {
                if digit == 0 {
                    digit = i;
                    alpha = true;
                }
                //println!("here {}", alpha);
                if (input.chars().nth(i as usize + 1).unwrap()).is_alphabetic() || (input.chars().nth(i as usize + 1).unwrap()).is_digit(10) || elem == '_' {
                    continue;
                }
                else {
                    if inv == false {
                        obj = Token::new(input[digit..i as usize + 1].to_string(), TokenType::IDENTIFIER.as_str(), ln_num, ch_pos - (i - digit) as i32);
                    }
                    else {
                        obj = Token::new(input[digit..i as usize + 1].to_string(), TokenType::INVALID.as_str(), ln_num, ch_pos - (i - digit) as i32);
                    }
                    inv = false;
                    digit = 0;
                    alpha = false;
                }
            }
            else {
                obj = Token::new(elem.to_string(), TokenType::INVALID.as_str(), ln_num, ch_pos);
            }
            //println!("{}:{}", obj.text, obj.token_type);
            token_objs.push(obj);
        }
        return token_objs;
    }
}

struct Cstream {
    filename: String
}

impl Cstream {
    fn new(fname: String) -> Cstream {
        Cstream {
            filename: fname.to_string()
        }
    }

    fn read(&mut self) -> String {
        let mut data = String::new();
        let mut file = File::open(&self.filename).expect("Unable");
        file.read_to_string(&mut data).expect("Empty");
        return data;
    }
}



fn main() {
    let mut f_name = String::new();
    /* Prompting user input */
    println!("Enter file name (include extention):");
    stdin().read_line(&mut f_name).unwrap();
    /* Creating Cstream object and reading the file */
    let mut f = Cstream::new(f_name[0..f_name.len()-1].to_string());
    println!("\n{} \n",f.read());
    let mut scan = Scanner::new(f.filename);
    /* All tokens vector containting all the tokens with token types, line numbers, and character positions */
    let all_tokens = scan.tokenize();
    let mut here = Parser::new(all_tokens);
    here.fun_program();
    
    let html_tt = scan.tokenize();

    let mut out = Out_xhtml::new(f_name[0..f_name.len()-3].to_string(), html_tt);
    out.prt_out();
}
