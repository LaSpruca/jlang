use crate::common::Value;
use log::{debug, error, info};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fmt::{Display, Formatter};

pub type ParsedFile = HashMap<String, Vec<(i32, Token)>>;

#[derive(Debug, Clone)]
pub enum Token {
    Def,
    Symbol(String),
    Value(Value),
    End,
    Var,
    Ret,
    Eq,
    Ne,
    If,
    Else,
    LBrace,
    RBrace,
    Add,
    Minus,
    Times,
    Divide,
    Comma,
    Assign,
    NewLine,
    While,
}

#[derive(Debug)]
pub struct ParseError {
    pub line: i32,
    pub file: String,
    pub error: String,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n {}",
               self.error,
               console::style(format!("{}:{}", self.file, self.line)).dim())
    }
}

impl Error for ParseError {}

pub fn parse(source: &String, file_path: &String) -> Result<ParsedFile, ParseError> {
    debug!("Parsing {}", file_path);
    let mut parsed_files = HashMap::new();
    let match_integer = regex::Regex::new("[0-9]+").unwrap();
    let match_float = regex::Regex::new("[0-9]+.[0-9]+").unwrap();

    let mut collector = String::new();

    let mut elements: Vec<(i32, Token)> = vec![];

    let mut line = 1;

    let mut import_next = false;

    let mut read_string = false;

    for mut a in source.split("") {
        if import_next {
            if a != "\n" {
                collector += a
            } else {
                collector = collector.trim().to_owned();
                let path = std::path::Path::new(file_path.as_str());
                let file = format!("{}/{}.j", path.parent().unwrap().display(), collector);
                let source = match fs::read_to_string(file.clone()) {
                    Ok(a) => a,
                    Err(e) => {
                        return Err(ParseError {
                            line,
                            file: file_path.to_owned(),
                            error: format!("Could not import file {}\n {}", file, e)
                        })
                    }
                };

                info!("Importing file {}", file);
                debug!("Read file {}", source);
                debug!("Parsing");
                let parsed = match parse(&source, &file) {
                    Err(e) => {
                        return Err(e);
                    },
                    Ok(code) => code
                };
                parsed_files.extend(parsed);

                collector = "".to_string();
                import_next = false;
            }
            continue;
        } else if read_string {
            if a != "\"" && !collector.ends_with("\\") {
                collector += a
            } else {
                elements.push((line, Token::Value(Value::String(collector.clone()))));
                debug!("Pushing String \"{}\"", collector);
                collector = "".to_string();

                read_string = false;
            }
            continue;
        }
        let mut after = None;
        match a {
            "(" => {
                after = Some((line, Token::LBrace));
                a = " ";
            }
            ")" => {
                after = Some((line, Token::RBrace));
                a = " ";
            }
            "," => {
                after = Some((line, Token::Comma));
                a = " ";
            }
            "+" => {
                after = Some((line, Token::Add));
                a = " ";
            }
            "-" => {
                after = Some((line, Token::Minus));
                a = " ";
            }
            "*" => {
                after = Some((line, Token::Times));
                a = " ";
            }
            "/" => {
                after = Some((line, Token::Divide));
                a = " ";
            }
            "\"" => {
                read_string = true;
                continue;
            }
            _ => {
                collector += a;
                collector = collector.trim().to_owned();
            }
        }

        match collector.as_str() {
            "def" => {
                collector = "".to_string();
                elements.push((line, Token::Def));
                debug!("Pushing Def")
            }
            "end" => {
                collector = "".to_string();
                elements.push((line, Token::End));
                debug!("Pushing End");
            }
            "var" => {
                collector = "".to_string();
                elements.push((line, Token::Var));
                debug!("Pushing Var");
            }
            "ret" => {
                collector = "".to_string();
                elements.push((line, Token::Ret));
                debug!("Pushing Ret");
            }
            "if" => {
                collector = "".to_string();
                elements.push((line, Token::If));
                debug!("Pushing If");
            }
            "else" => {
                collector = "".to_string();
                elements.push((line, Token::Else));
                debug!("Pushing Else");
            }
            "imp" => {
                collector = "".to_string();
                import_next = true;
            }
            "true" => {
                collector = "".to_string();
                elements.push((line, Token::Value(Value::Boolean(true))));
                debug!("Pushing True");
            }
            "false" => {
                collector = "".to_string();
                elements.push((line, Token::Value(Value::Boolean(true))));
                debug!("Pushing False");
            }
            "while" => {
                collector = "".to_string();
                elements.push((line, Token::While));
                debug!("Pushing False");
            }
            _ => {}
        }

        match a {
            "\n" => {
                if collector != "".to_string() {
                    debug!("Pushing token {}", collector);
                    elements.push((line, Token::Symbol(collector)));
                    collector = "".to_string();
                }
                elements.push((line, Token::NewLine));
                line += 1;
            }
            " " | "\t" => {
                if collector != "".to_string() {
                    if match_float.is_match(&collector) {
                        elements.push((
                            line,
                            Token::Value(Value::FloatingPointInteger(collector.parse().unwrap())),
                        ));
                    } else if match_integer.is_match(&collector) {
                        elements.push((
                            line,
                            Token::Value(Value::Integer(collector.parse().unwrap())),
                        ));
                    }
                    match collector.as_str() {
                        "=" => {
                            collector = "".to_string();
                            elements.push((line, Token::Assign));
                            debug!("Pushing Assign");
                        }
                        "==" => {
                            collector = "".to_string();
                            elements.push((line, Token::Eq));
                            debug!("Pushing Eq");
                        }
                        "!=" => {
                            collector = "".to_string();
                            elements.push((line, Token::Ne));
                            debug!("Pushing Ne");
                        }
                        _ => {
                            debug!("Pushing token {}", collector);
                            elements.push((line, Token::Symbol(collector)));
                            collector = "".to_string();
                        }
                    }
                }
            }
            _ => {}
        }

        if let Some(aft) = after {
            debug!("Pushing {:?}", aft.1);
            elements.push(aft);
        }
    }
    parsed_files.insert(file_path.to_owned(), elements);
    debug!("Finished parsing {}", file_path);
    Ok(parsed_files)
}
