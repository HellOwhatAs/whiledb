mod lexer;
mod ast;
mod grammar;
mod src_error;

fn main() {
    let input = include_str!("../test.wd");
    let lexer_rules = lexer::lexer_rules();
    let lexemes = match santiago::lexer::lex(&lexer_rules, input) {
        Ok(lexemes) => lexemes,
        Err(err) => {
            eprintln!("{}", src_error::lexer_error(input, &err));
            return;
        },
    };
    println!("{:?}", lexemes.iter().map(|x| &x.raw).collect::<Vec<&String>>());

    let grammar = grammar::grammar();
    let parse_trees = santiago::parser::parse(&grammar, &lexemes).expect("[parse error]");
    let ast = parse_trees.into_iter().map(|e| e.as_abstract_syntax_tree()).collect::<Vec<_>>();
    println!("{}", ast.len());
    println!("{:?}", ast);
}