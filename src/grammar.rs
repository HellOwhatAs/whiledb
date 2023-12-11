use santiago::grammar::Associativity;
use santiago::grammar::Grammar;
use crate::ast::*;

pub fn grammar() -> Grammar<Cmd> {
    santiago::grammar!(
        "cmd" => rules "cmd" "useless_semicol" => |rules| {
            let mut rules: Vec<Cmd> = rules;
            rules.pop();
            rules.pop().unwrap()
        };
        "cmd" => rules "expr";
        "cmd" => rules "break";
        "cmd" => rules "continue";
        "cmd" => rules "expr" "=" "expr" => |rules| {
            let mut rules: Vec<Cmd> = rules;
            let right: Cmd = rules.pop().unwrap();
            rules.pop();
            let left: Cmd = rules.pop().unwrap();
            if let (Cmd::Expr(expr_left), Cmd::Expr(expr_right)) = (left, right) {
                return Cmd::Asgn(Box::new(expr_left), Box::new(expr_right));
            }
            else {
                unreachable!();
            }
        };
        "cmd" => rules "cmd" ";" "cmd" => |rules| {
            let mut rules: Vec<Cmd> = rules;
            let right: Cmd = rules.pop().unwrap();
            rules.pop();
            let left: Cmd = rules.pop().unwrap();
            Cmd::Seq(Box::new(left), Box::new(right))
        };
        "cmd" => rules "cmd_block";
        "cmd" => rules "cmd_block" "cmd" => |rules| {
            let mut rules: Vec<Cmd> = rules;
            let right: Cmd = rules.pop().unwrap();
            let left: Cmd = rules.pop().unwrap();
            Cmd::Seq(Box::new(left), Box::new(right))
        };
        "cmd_block" => rules "if" "expr" "then" "{" "cmd" "}" "else" "{" "cmd" "}" => |rules| {
            let mut rules: Vec<Cmd> = rules;
            rules.pop();
            let else_branch: Cmd = rules.pop().unwrap();
            rules.pop(); rules.pop(); rules.pop();
            let if_branch: Cmd = rules.pop().unwrap();
            rules.pop(); rules.pop();
            let cond: Cmd = rules.pop().unwrap();
            if let Cmd::Expr(e) = cond {
                return Cmd::If(Box::new(e), Box::new(if_branch), Box::new(else_branch));
            }
            unreachable!();
        };
        "cmd_block" => rules "if" "expr" "{" "cmd" "}" "else" "{" "cmd" "}" => |rules| {
            let mut rules: Vec<Cmd> = rules;
            rules.pop();
            let else_branch: Cmd = rules.pop().unwrap();
            rules.pop(); rules.pop(); rules.pop();
            let if_branch: Cmd = rules.pop().unwrap();
            rules.pop();
            let cond: Cmd = rules.pop().unwrap();
            if let Cmd::Expr(e) = cond {
                return Cmd::If(Box::new(e), Box::new(if_branch), Box::new(else_branch));
            }
            unreachable!();
        };
        "cmd_block" => rules "if" "expr" "then" "{" "cmd" "}" => |rules| {
            let mut rules: Vec<Cmd> = rules;
            rules.pop();
            let if_branch: Cmd = rules.pop().unwrap();
            rules.pop(); rules.pop();
            let cond: Cmd = rules.pop().unwrap();
            if let Cmd::Expr(e) = cond {
                return Cmd::If(Box::new(e), Box::new(if_branch), Box::new(Cmd::Nop));
            }
            unreachable!();
        };
        "cmd_block" => rules "if" "expr" "{" "cmd" "}" => |rules| {
            let mut rules: Vec<Cmd> = rules;
            rules.pop();
            let if_branch: Cmd = rules.pop().unwrap();
            rules.pop();
            let cond: Cmd = rules.pop().unwrap();
            if let Cmd::Expr(e) = cond {
                return Cmd::If(Box::new(e), Box::new(if_branch), Box::new(Cmd::Nop));
            }
            unreachable!();
        };
        "cmd_block" => rules "while" "expr" "do" "{" "cmd" "}" => |rules| {
            let mut rules: Vec<Cmd> = rules;
            rules.pop();
            let body: Cmd = rules.pop().unwrap();
            rules.pop(); rules.pop();
            let cond: Cmd = rules.pop().unwrap();
            if let Cmd::Expr(e) = cond {
                return Cmd::While(Box::new(e), Box::new(body));
            }
            unreachable!();
        };
        "cmd_block" => rules "while" "expr" "{" "cmd" "}" => |rules| {
            let mut rules: Vec<Cmd> = rules;
            rules.pop();
            let body: Cmd = rules.pop().unwrap();
            rules.pop();
            let cond: Cmd = rules.pop().unwrap();
            if let Cmd::Expr(e) = cond {
                return Cmd::While(Box::new(e), Box::new(body));
            }
            unreachable!();
        };

        "expr_list" => rules "expr";
        "expr_list" => rules "expr" "," "expr_list" => |rules| {
            let mut rules: Vec<Cmd> = rules;
            let right: Cmd = rules.pop().unwrap();
            rules.pop();
            let left: Cmd = rules.pop().unwrap();
            if let (Cmd::Expr(expr_left), Cmd::Expr(expr_right)) = (left, right) {
                return Cmd::Expr(Expr::Tuple(Box::new(expr_left), Box::new(expr_right)));
            }
            else {
                unreachable!();
            }
        };
        "expr" => rules "ident" "("  ")" => |rules| {
            let mut rules: Vec<Cmd> = rules;
            rules.pop();
            rules.pop();
            let ident: Cmd = rules.pop().unwrap();
            if let Cmd::Expr(Expr::Var(s)) = ident {
                return Cmd::Expr(Expr::Call(s, None));
            }
            unreachable!();
        };
        "expr" => rules "ident" "(" "expr_list" ")" => |rules| {
            let mut rules: Vec<Cmd> = rules;
            rules.pop();
            let expr_list: Cmd = rules.pop().unwrap();
            rules.pop();
            let ident: Cmd = rules.pop().unwrap();
            if let (Cmd::Expr(Expr::Var(s)), Cmd::Expr(e)) = (ident, expr_list) {
                return Cmd::Expr(Expr::Call(s, Some(Box::new(e))));
            }
            unreachable!();
        };

        "expr" => rules "int";
        "expr" => rules "(" "expr" ")" => |mut rules| {
            rules.pop();
            rules.pop().unwrap()
        };
        "expr" => rules "negate" "expr" => |mut rules| {
            if let Cmd::Expr(e) = rules.pop().unwrap() {
                return Cmd::Expr(Expr::UnOp(UnOp::Negate, Box::new(e)));
            }
            unreachable!();
        };
        "expr" => rules "expr" "+" "expr" => |rules| {
            let mut rules: Vec<Cmd> = rules;
            let right: Cmd = rules.pop().unwrap();
            rules.pop();
            let left: Cmd = rules.pop().unwrap();
            if let (Cmd::Expr(expr_left), Cmd::Expr(expr_right)) = (left, right) {
                return Cmd::Expr(Expr::BinOp(BinOp::Plus, Box::new(expr_left), Box::new(expr_right)));
            }
            else {
                unreachable!();
            }
        };
        "expr" => rules "expr" "-" "expr" => |rules| {
            let mut rules: Vec<Cmd> = rules;
            let right: Cmd = rules.pop().unwrap();
            rules.pop();
            let left: Cmd = rules.pop().unwrap();
            if let (Cmd::Expr(expr_left), Cmd::Expr(expr_right)) = (left, right) {
                return Cmd::Expr(Expr::BinOp(BinOp::Minus, Box::new(expr_left), Box::new(expr_right)));
            }
            else {
                unreachable!();
            }
        };
        "expr" => rules "expr" "*" "expr" => |rules| {
            let mut rules: Vec<Cmd> = rules;
            let right: Cmd = rules.pop().unwrap();
            rules.pop();
            let left: Cmd = rules.pop().unwrap();
            if let (Cmd::Expr(expr_left), Cmd::Expr(expr_right)) = (left, right) {
                return Cmd::Expr(Expr::BinOp(BinOp::Mul, Box::new(expr_left), Box::new(expr_right)));
            }
            else {
                unreachable!();
            }
        };
        "expr" => rules "expr" "/" "expr" => |rules| {
            let mut rules: Vec<Cmd> = rules;
            let right: Cmd = rules.pop().unwrap();
            rules.pop();
            let left: Cmd = rules.pop().unwrap();
            if let (Cmd::Expr(expr_left), Cmd::Expr(expr_right)) = (left, right) {
                return Cmd::Expr(Expr::BinOp(BinOp::Div, Box::new(expr_left), Box::new(expr_right)));
            }
            else {
                unreachable!();
            }
        };
        "expr" => rules "expr" "%" "expr" => |rules| {
            let mut rules: Vec<Cmd> = rules;
            let right: Cmd = rules.pop().unwrap();
            rules.pop();
            let left: Cmd = rules.pop().unwrap();
            if let (Cmd::Expr(expr_left), Cmd::Expr(expr_right)) = (left, right) {
                return Cmd::Expr(Expr::BinOp(BinOp::Mod, Box::new(expr_left), Box::new(expr_right)));
            }
            else {
                unreachable!();
            }
        };
        "expr" => rules "expr" "<" "expr" => |rules| {
            let mut rules: Vec<Cmd> = rules;
            let right: Cmd = rules.pop().unwrap();
            rules.pop();
            let left: Cmd = rules.pop().unwrap();
            if let (Cmd::Expr(expr_left), Cmd::Expr(expr_right)) = (left, right) {
                return Cmd::Expr(Expr::BinOp(BinOp::Lt, Box::new(expr_left), Box::new(expr_right)));
            }
            else {
                unreachable!();
            }
        };
        "expr" => rules "expr" ">" "expr" => |rules| {
            let mut rules: Vec<Cmd> = rules;
            let right: Cmd = rules.pop().unwrap();
            rules.pop();
            let left: Cmd = rules.pop().unwrap();
            if let (Cmd::Expr(expr_left), Cmd::Expr(expr_right)) = (left, right) {
                return Cmd::Expr(Expr::BinOp(BinOp::Gt, Box::new(expr_left), Box::new(expr_right)));
            }
            else {
                unreachable!();
            }
        };
        "expr" => rules "expr" "<=" "expr" => |rules| {
            let mut rules: Vec<Cmd> = rules;
            let right: Cmd = rules.pop().unwrap();
            rules.pop();
            let left: Cmd = rules.pop().unwrap();
            if let (Cmd::Expr(expr_left), Cmd::Expr(expr_right)) = (left, right) {
                return Cmd::Expr(Expr::BinOp(BinOp::Le, Box::new(expr_left), Box::new(expr_right)));
            }
            else {
                unreachable!();
            }
        };
        "expr" => rules "expr" ">=" "expr" => |rules| {
            let mut rules: Vec<Cmd> = rules;
            let right: Cmd = rules.pop().unwrap();
            rules.pop();
            let left: Cmd = rules.pop().unwrap();
            if let (Cmd::Expr(expr_left), Cmd::Expr(expr_right)) = (left, right) {
                return Cmd::Expr(Expr::BinOp(BinOp::Ge, Box::new(expr_left), Box::new(expr_right)));
            }
            else {
                unreachable!();
            }
        };
        "expr" => rules "expr" "==" "expr" => |rules| {
            let mut rules: Vec<Cmd> = rules;
            let right: Cmd = rules.pop().unwrap();
            rules.pop();
            let left: Cmd = rules.pop().unwrap();
            if let (Cmd::Expr(expr_left), Cmd::Expr(expr_right)) = (left, right) {
                return Cmd::Expr(Expr::BinOp(BinOp::Eq, Box::new(expr_left), Box::new(expr_right)));
            }
            else {
                unreachable!();
            }
        };
        "expr" => rules "expr" "!=" "expr" => |rules| {
            let mut rules: Vec<Cmd> = rules;
            let right: Cmd = rules.pop().unwrap();
            rules.pop();
            let left: Cmd = rules.pop().unwrap();
            if let (Cmd::Expr(expr_left), Cmd::Expr(expr_right)) = (left, right) {
                return Cmd::Expr(Expr::BinOp(BinOp::Ne, Box::new(expr_left), Box::new(expr_right)));
            }
            else {
                unreachable!();
            }
        };
        "expr" => rules "expr" "and" "expr" => |rules| {
            let mut rules: Vec<Cmd> = rules;
            let right: Cmd = rules.pop().unwrap();
            rules.pop();
            let left: Cmd = rules.pop().unwrap();
            if let (Cmd::Expr(expr_left), Cmd::Expr(expr_right)) = (left, right) {
                return Cmd::Expr(Expr::BinOp(BinOp::And, Box::new(expr_left), Box::new(expr_right)));
            }
            else {
                unreachable!();
            }
        };
        "expr" => rules "expr" "or" "expr" => |rules| {
            let mut rules: Vec<Cmd> = rules;
            let right: Cmd = rules.pop().unwrap();
            rules.pop();
            let left: Cmd = rules.pop().unwrap();
            if let (Cmd::Expr(expr_left), Cmd::Expr(expr_right)) = (left, right) {
                return Cmd::Expr(Expr::BinOp(BinOp::Or, Box::new(expr_left), Box::new(expr_right)));
            }
            else {
                unreachable!();
            }
        };
        "expr" => rules "not" "expr" => |mut rules| {
            if let Cmd::Expr(e) = rules.pop().unwrap() {
                return Cmd::Expr(Expr::UnOp(UnOp::Not, Box::new(e)));
            }
            unreachable!();
        };
        "expr" => rules "deref" "expr" => |mut rules| {
            if let Cmd::Expr(e) = rules.pop().unwrap() {
                return Cmd::Expr(Expr::UnOp(UnOp::Deref, Box::new(e)));
            }
            unreachable!();
        };
        "expr" => rules "ident";

        "int" => lexemes "INT" => |lexemes| {
            Cmd::Expr(Expr::Const(str::parse(&lexemes[0].raw).unwrap()))
        };
        "if" => lexemes "IF" => |_| Cmd::Nop;
        "then" => lexemes "THEN" => |_| Cmd::Nop;
        "else" => lexemes "ELSE" => |_| Cmd::Nop;
        "while" => lexemes "WHILE" => |_| Cmd::Nop;
        "do" => lexemes "DO" => |_| Cmd::Nop;
        "continue" => lexemes "CONTINUE" => |_| Cmd::Continue;
        "break" => lexemes "BREAK" => |_| Cmd::Break;
        "ident" => lexemes "IDENT" => |lexemes| {
            Cmd::Expr(Expr::Var(lexemes[0].raw.to_string()))
        };
        ";" => lexemes "SEMICOL" => |_| Cmd::Nop;
        "useless_semicol" => lexemes "SEMICOL" => |_| Cmd::Nop;
        "," => lexemes "COMMA" => |_| Cmd::Nop;
        "(" => lexemes "LEFT_PAREN" => |_| Cmd::Nop;
        ")" => lexemes "RIGHT_PAREN" => |_| Cmd::Nop;
        "{" => lexemes "LEFT_BRACE" => |_| Cmd::Nop;
        "}" => lexemes "RIGHT_BRACE" => |_| Cmd::Nop;
        "+" => lexemes "PLUS" => |_| Cmd::Nop;
        "-" => lexemes "MINUS" => |_| Cmd::Nop;
        "negate" => lexemes "MINUS" => |_| Cmd::Nop;
        "*" => lexemes "MUL" => |_| Cmd::Nop;
        "deref" => lexemes "MUL" => |_| Cmd::Nop;
        "/" => lexemes "DIV" => |_| Cmd::Nop;
        "%" => lexemes "MOD" => |_| Cmd::Nop;
        "<" => lexemes "LT" => |_| Cmd::Nop;
        ">" => lexemes "GT" => |_| Cmd::Nop;
        "<=" => lexemes "LE" => |_| Cmd::Nop;
        ">=" => lexemes "GE" => |_| Cmd::Nop;
        "==" => lexemes "EQ" => |_| Cmd::Nop;
        "!=" => lexemes "NE" => |_| Cmd::Nop;
        "=" => lexemes "ASGNOP" => |_| Cmd::Nop;
        "and" => lexemes "AND" => |_| Cmd::Nop;
        "or" => lexemes "OR" => |_| Cmd::Nop;
        "not" => lexemes "NOT" => |_| Cmd::Nop;

        // Associativity::None => rules "=";
        Associativity::Left => rules "or";
        Associativity::Left => rules "and";
        Associativity::Left => rules ">" ">=" "<" "<=" "==" "!=";
        Associativity::Left => rules "+" "-";
        Associativity::Left => rules "*" "/" "%";
        Associativity::None => rules "deref" "negate" "not";
        // Associativity::Left => rules "(" ")";
        Associativity::Right => rules ";";
        Associativity::Left => rules "useless_semicol";
    )
}