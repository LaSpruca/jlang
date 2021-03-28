use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::process::exit;

use log::{debug, error, info};

use crate::parser::ParsedFile;
use crate::parser::Token;

#[derive(Debug)]
pub struct Function {
    code: Vec<Token>,
    file: String,
    args: Vec<String>,
}

#[derive(Debug)]
pub struct SyntaxError {
    pub line: i32,
    pub file: String,
    pub error: String,
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Syntax Error: {}\n {}",
               self.error,
               console::style(format!("{}:{}", self.file, self.line)).dim())
    }
}

impl Error for SyntaxError {}

pub fn lex(sources: ParsedFile) -> Result<(), SyntaxError> {
    let mut functions = HashMap::new();
    for (file, code) in sources.iter() {
        for mut i in (0..code.len()) {
            let token = (&code[i].1).to_owned();
            let mut function_name = String::new();
            let mut args = vec!();
            match token {
                Token::Def => {
                    i += 1;
                    let token = (&code[i].1).to_owned();
                    match token {
                        Token::Symbol(name) => {
                            function_name = name;
                            i += 1;

                            let token = (&code[i].1).to_owned();

                            match token {
                                Token::LBrace => {
                                    i += 1;
                                    'args_loop: loop {
                                        let token = (&code[i].1).to_owned();
                                        match token.clone() {
                                            Token::Symbol(a) => {
                                                args.push(a);
                                                i += 1;
                                                let token = (&code[i].1).to_owned();
                                                match token {
                                                    Token::Comma => {
                                                        i += 1;
                                                    }
                                                    Token::RBrace => {
                                                        break 'args_loop;
                                                    }
                                                    _ => {
                                                        return Err(SyntaxError {
                                                            file: file.to_string(),
                                                            line: (&code[i].0).to_owned(),
                                                            error: format!("Expected , ), got {:?}", token)
                                                        });
                                                    }
                                                }
                                            }
                                            Token::RBrace => {
                                                break 'args_loop;
                                            }
                                            _ => {
                                                return Err(SyntaxError {
                                                    file: file.to_string(),
                                                    line: (&code[i].0).to_owned(),
                                                    error: format!("Expected symbol ), got {:?}", token)
                                                });
                                            }
                                        }
                                    }

                                },
                                _ => {
                                    return Err(SyntaxError {
                                        error: format!("Expected (, found {:?}", token),
                                        file: file.to_owned(),
                                        line: code[i].0
                                    });
                                }
                            }
                        }
                        _ => {
                            return Err(SyntaxError {
                                error: format!("Expected name, found {:?}", token),
                                file: file.to_owned(),
                                line: code[i].0
                            });
                        }
                    }
                    functions.insert(function_name, Function {
                        args,
                        file: file.to_owned(),
                        code: vec!()
                    });
                }
                _ => {}
            }
        }
    }

    debug!("{:#?}", functions);

    Ok(())
}
