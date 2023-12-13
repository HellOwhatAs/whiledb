mod lexer;
pub mod ast;
mod grammar;
pub mod src_error;

use ast::Cmd;
use src_error::{SrcError, lexer_error_msg, parse_error_msg};

pub fn parse(input: &str) -> Result<Cmd, SrcError<Cmd>> {
    let lexer_rules = lexer::lexer_rules();
    let lexemes = match santiago::lexer::lex(&lexer_rules, input) {
        Ok(lexemes) => lexemes,
        Err(err) => {
            let msg = lexer_error_msg(input, &err);
            return Err(SrcError::LexerError(err, msg));
        },
    };

    let grammar = grammar::grammar();
    let parse_trees = match santiago::parser::parse(&grammar, &lexemes) {
        Ok(parse_trees) => parse_trees,
        Err(err) => {
            let msg = parse_error_msg(input, &err);
            return Err(SrcError::ParseError(err, msg));
        },
    };
    let mut asts = parse_trees.into_iter().map(|e| e.as_abstract_syntax_tree()).collect::<Vec<_>>();
    match asts.len() {
        0 => {
            Err(SrcError::SelfError("No any possible Parse Tree. Incorrect parse rules.".to_string()))
        },
        1 => Ok(asts.pop().unwrap()),
        _ => {
            Err(SrcError::SelfWarning(asts.pop().unwrap(), "No any possible Parse Tree. Incorrect parse rules.".to_string()))
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::parse;
    #[test]
    fn sample_src() {
        match parse(include_str!("../tests/sample_src.wd")) {
            Ok(res) => println!("{:?}", res),
            Err(err) => panic!("{}", err)
        }
    }
    #[test]
    fn test() {
        match parse(include_str!("../tests/test.wd")) {
            Ok(res) => println!("{:?}", res),
            Err(err) => panic!("{}", err)
        }
    }
    #[test]
    fn test2() {
        match parse(include_str!("../tests/test2.wd")) {
            Ok(res) => println!("{:?}", res),
            Err(err) => println!("{}", err)
        }
    }
}