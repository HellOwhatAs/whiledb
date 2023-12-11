mod lexer;
mod ast;
mod grammar;

fn main() {
    let input = "
    n = read_int(1, 2, 3);
    i = 2;
    flag = 1;
    while flag && i * i <= n {
        if (n % i == 0)
        then { flag = 0 }
        else { flag = 1 };
        i = i + 1
    };
    if flag {
        write_char(80);
        write_char(82);
        write_char(73);
        write_char(77);
        write_char(69);
        write_char(10)
    }
    else {
        write_char(78);
        write_char(79);
        write_char(78);
        write_char(80);
        write_char(82);
        write_char(73);
        write_char(77);
        write_char(69);
        write_char(10)
    }
    ";
    let lexer_rules = lexer::lexer_rules();
    let lexemes = santiago::lexer::lex(&lexer_rules, &input).expect("e1");
    // println!("{:#?}", lexemes);

    let grammar = grammar::grammar();
    let parse_trees = santiago::parser::parse(&grammar, &lexemes).expect("e2");
    let ast = parse_trees.into_iter().map(|e| e.as_abstract_syntax_tree()).collect::<Vec<_>>();
    println!("{:#?}", ast);
}