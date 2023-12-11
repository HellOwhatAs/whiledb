mod lexer;
mod ast;
mod grammar;

fn main() {
    let input = "
    while true {
        a = a + 1;
        if a == 10000 {
            c = 1 * 2;
            break;
        }
    }
    if 1 {
        if 2 {
            0;
        }
        if 3 {
            0;
        }
    }
    ";
    let lexer_rules = lexer::lexer_rules();
    let lexemes = santiago::lexer::lex(&lexer_rules, &input).expect("[lexer error]");
    // println!("{:#?}", lexemes);

    let grammar = grammar::grammar();
    let parse_trees = santiago::parser::parse(&grammar, &lexemes).expect("[parse error]");
    let ast = parse_trees.into_iter().map(|e| e.as_abstract_syntax_tree()).collect::<Vec<_>>();
    println!("{}", ast.len());
    println!("{:?}", ast);
}