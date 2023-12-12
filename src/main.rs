mod lexer;
mod ast;
mod grammar;
mod src_error;

use colored::Colorize;

fn main() {
    let input = include_str!("../test.wd");
    let lexer_rules = lexer::lexer_rules();
    let lexemes = match santiago::lexer::lex(&lexer_rules, input) {
        Ok(lexemes) => lexemes,
        Err(err) => {
            eprintln!("{}", src_error::lexer_error_msg(input, &err));
            return;
        },
    };

    let grammar = grammar::grammar();
    let parse_trees = match santiago::parser::parse(&grammar, &lexemes) {
        Ok(parse_trees) => parse_trees,
        Err(err) => {
            eprintln!("{}", src_error::parse_error_msg(input, &err));
            return;
        },
    };
    let asts = parse_trees.into_iter().map(|e| e.as_abstract_syntax_tree()).collect::<Vec<_>>();
    let tree = match asts.len() {
        0 => {
            eprintln!("{}", "[ERROR]: No any possible Parse Tree. Incorrect parse rules.".red());
            return;
        },
        1 => &asts[0],
        _ => {
            eprintln!("{}", format!("[WARNING]: Exists {} possible Parse Trees. Incorrect parse rules.", asts.len()).bright_yellow());
            &asts[0]
        }
    };
    println!("{:?}", tree);
}