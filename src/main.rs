use std::fs;
use std::io::Write;
use std::process::exit;

use log::{debug, error, info};

use crate::lexer::lex;
use crate::parser::parse;

pub mod common;
pub mod lexer;
pub mod parser;

fn setup_logger() {
    if let Err(_) = std::env::var("RUST_LOG") {
        std::env::set_var("RUST_LOG", "info");
    }

    env_logger::builder()
        .format(|buf, record| {
            writeln!(
                buf,
                "{}: {}",
                match record.level() {
                    log::Level::Debug => {
                        console::style("Debug").green().bold()
                    }
                    log::Level::Info => {
                        console::style("Info").blue().bold()
                    }
                    log::Level::Warn => {
                        console::style("Warn").yellow().bold()
                    }
                    log::Level::Trace => {
                        console::style("Trace").yellow().bold()
                    }
                    log::Level::Error => {
                        console::style("\nError").red().bold()
                    }
                },
                record.args()
            )
        })
        .init();
}

fn main() {
    setup_logger();
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() <= 1 {
        error!("Please specify an input file");
        exit(-1);
    }

    let file = args[1].to_owned();

    info!("Parsing {}", file);

    let source = match fs::read_to_string(file.clone()) {
        Ok(a) => a,
        Err(e) => {
            error!("Error, could not read file\n{}", e);
            exit(-1);
        }
    };

    debug!("Read file {}", source);
    let parsed = match parse(&source, &file) {
        Err(error) => {
            error!("{}", error);
            exit(-1);
        }
        Ok(a) => a
    };

    debug!("{:#?}", parsed);

    let lexed = match lex(parsed) {
        Err(error) => {
            error!("{}", error);
            exit(-1);
        }
        Ok(a) => a
    };
}
