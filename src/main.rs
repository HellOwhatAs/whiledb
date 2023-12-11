mod lexer;
mod ast;
mod grammar;

fn main() {
    let input = "
    a;b;c
    ";
    let lexer_rules = lexer::lexer_rules();
    let lexemes = santiago::lexer::lex(&lexer_rules, &input).expect("e1");
    // println!("{:#?}", lexemes);

    let grammar = grammar::grammar();
    let parse_trees = santiago::parser::parse(&grammar, &lexemes).expect("e2");
    let ast = parse_trees.into_iter().map(|e| e.as_abstract_syntax_tree()).collect::<Vec<_>>();
    println!("{:?}", ast);
}