use std::collections::VecDeque;
use santiago::grammar::Associativity;
use santiago::grammar::Grammar;
use crate::ast::*;

pub fn grammar() -> Grammar<Cmd> {
    santiago::grammar!(
        "cmd" => empty => |_| Cmd::Seq(VecDeque::new());
        "cmd" => rules "cmd_block" "cmd" => |rules| {
            let mut rules: Vec<Cmd> = rules;
            let right: Cmd = rules.pop().unwrap();
            let left: Cmd = rules.pop().unwrap();
            match right {
                Cmd::Seq(mut right) => {
                    right.push_front(left);
                    Cmd::Seq(right)
                },
                _ => unreachable!()
            }
        };
        "cmd" => rules "cmd_simple" ";" "cmd" => |rules| {
            let mut rules: Vec<Cmd> = rules;
            let right: Cmd = rules.pop().unwrap();
            rules.pop();
            let left: Cmd = rules.pop().unwrap();
            match right {
                Cmd::Seq(mut right) => {
                    right.push_front(left);
                    Cmd::Seq(right)
                },
                _ => unreachable!()
            }
        };
        "cmd_simple" => rules "expr";
        "cmd_simple" => rules "break";
        "cmd_simple" => rules "continue";
        "cmd_simple" => rules "return" "expr" => |rules| {
            let mut rules: Vec<Cmd> = rules;
            let val: Cmd = rules.pop().unwrap();
            if let Cmd::Expr(e) = val {
                return Cmd::Return(e);
            }
            else {
                unreachable!();
            }
        };
        "cmd_simple" => rules "expr" "=" "expr" => |rules| {
            let mut rules: Vec<Cmd> = rules;
            let right: Cmd = rules.pop().unwrap();
            rules.pop();
            let left: Cmd = rules.pop().unwrap();
            if let (Cmd::Expr(expr_left), Cmd::Expr(expr_right)) = (left, right) {
                return Cmd::Asgn(expr_left, expr_right);
            }
            else {
                unreachable!();
            }
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
                return Cmd::If(e, Box::new(if_branch), Box::new(else_branch));
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
                return Cmd::If(e, Box::new(if_branch), Box::new(else_branch));
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
                return Cmd::If(e, Box::new(if_branch), Box::new(Cmd::Nop));
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
                return Cmd::If(e, Box::new(if_branch), Box::new(Cmd::Nop));
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
                return Cmd::While(e, Box::new(body));
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
                return Cmd::While(e, Box::new(body));
            }
            unreachable!();
        };
        "cmd_block" => rules "fn_block";
        "cmd_block" => rules "class_block";
        "fn_block" => rules "fn" "ident" "(" "ident_list" ")" "{" "cmd" "}" => |rules| {
            let mut rules: Vec<Cmd> = rules;
            rules.pop();
            let body: Cmd = rules.pop().unwrap();
            rules.pop(); rules.pop();
            let args: Cmd = rules.pop().unwrap();
            rules.pop();
            let func_name: Cmd = rules.pop().unwrap();
            if let Cmd::Expr(func_name) = func_name {
                let args = match args {
                    Cmd::Expr(e) => e,
                    _ => unreachable!()
                };
                let func_name = match *func_name {
                    Expr::Var(func_name) => func_name,
                    _ => unreachable!()
                };
                Cmd::Func(func_name, args, Box::new(body))
            }
            else {
                unreachable!()
            }
        };

        "class_block" => rules "class" "ident" "{" "fn_list" "}" => |rules| {
            let mut rules: Vec<Cmd> = rules;
            rules.pop();
            let methods: Cmd = rules.pop().unwrap();
            rules.pop();
            let class_name: Cmd = rules.pop().unwrap();
            match class_name {
                Cmd::Expr(class_name) => {
                    match *class_name {
                        Expr::Var(class_name) => Cmd::Class(class_name, Box::new(methods)),
                        _ => unreachable!()
                    }
                },
                _ => unreachable!()
            }
        };
        "fn_list" => empty => |_| Cmd::Seq(VecDeque::new());
        "fn_list" => rules "fn_block" "fn_list" => |rules| {
            let mut rules: Vec<Cmd> = rules;
            let right: Cmd = rules.pop().unwrap();
            let left: Cmd = rules.pop().unwrap();
            match right {
                Cmd::Seq(mut right) => {
                    right.push_front(left);
                    Cmd::Seq(right)
                },
                _ => unreachable!()
            }
        };

        "ident_list" => empty => |_| Cmd::Expr(Box::new(Expr::Tuple(VecDeque::new())));
        "ident_list" => rules "ident" => |rules| {
            let mut rules: Vec<Cmd> = rules;
            let expr: Cmd = rules.pop().unwrap();
            match expr {
                Cmd::Expr(expr) => Cmd::Expr(Box::new(Expr::Tuple(VecDeque::from_iter([*expr])))),
                _ => unreachable!()
            }
        };
        "ident_list" => rules "ident" "," "ident_list" => |rules| {
            let mut rules: Vec<Cmd> = rules;
            let right = rules.pop().unwrap();
            rules.pop();
            let left = rules.pop().unwrap();
            match right {
                Cmd::Expr(ident_right) => {
                    if let (Cmd::Expr(expr_left), Expr::Tuple(mut expr_right)) = (left, *ident_right) {
                        expr_right.push_front(*expr_left);
                        Cmd::Expr(Box::new(Expr::Tuple(expr_right)))
                    }
                    else { unreachable!() }
                }
                _ => unreachable!()
            }
        };
        
        "expr_list" => empty => |_| Cmd::Expr(Box::new(Expr::Tuple(VecDeque::new())));
        "expr_list" => rules "expr" => |rules| {
            let mut rules: Vec<Cmd> = rules;
            let expr: Cmd = rules.pop().unwrap();
            match expr {
                Cmd::Expr(expr) => Cmd::Expr(Box::new(Expr::Tuple(VecDeque::from_iter([*expr])))),
                _ => unreachable!()
            }
        };
        "expr_list" => rules "expr" "," "expr_list" => |rules| {
            let mut rules: Vec<Cmd> = rules;
            let right: Cmd = rules.pop().unwrap();
            rules.pop();
            let left: Cmd = rules.pop().unwrap();
            match right {
                Cmd::Expr(expr_right) => {
                    if let (Cmd::Expr(expr_left), Expr::Tuple(mut expr_right)) = (left, *expr_right) {
                        expr_right.push_front(*expr_left);
                        Cmd::Expr(Box::new(Expr::Tuple(expr_right)))
                    }
                    else { unreachable!() }
                },
                _ => unreachable!()
            }
        };
        "expr" => rules "expr" "call(" "expr_list" ")call" => |rules| {
            let mut rules: Vec<Cmd> = rules;
            rules.pop();
            let expr_list: Cmd = rules.pop().unwrap();
            rules.pop();
            let func: Cmd = rules.pop().unwrap();
            match (func, expr_list) {
                (Cmd::Expr(func), Cmd::Expr(expr_list)) => Cmd::Expr(Box::new(Expr::Call(func, expr_list))),
                _ => unreachable!()
            }
        };
        "expr" => rules "[" "expr_list" "]" => |rules| {
            let mut rules: Vec<Cmd> = rules;
            rules.pop();
            rules.pop().unwrap()
        };

        "expr" => rules "int";
        "expr" => rules "float";
        "expr" => rules "(" "expr" ")" => |mut rules| {
            rules.pop();
            rules.pop().unwrap()
        };
        "expr" => rules "negate" "expr" => |mut rules| {
            if let Cmd::Expr(e) = rules.pop().unwrap() {
                return Cmd::Expr(Box::new(Expr::UnOp(UnOp::Negate, e)));
            }
            unreachable!();
        };
        "expr" => rules "expr" "+" "expr" => |rules| {
            let mut rules: Vec<Cmd> = rules;
            let right: Cmd = rules.pop().unwrap();
            rules.pop();
            let left: Cmd = rules.pop().unwrap();
            if let (Cmd::Expr(expr_left), Cmd::Expr(expr_right)) = (left, right) {
                return Cmd::Expr(Box::new(Expr::BinOp(BinOp::Plus, expr_left, expr_right)));
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
                return Cmd::Expr(Box::new(Expr::BinOp(BinOp::Minus, expr_left, expr_right)));
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
                return Cmd::Expr(Box::new(Expr::BinOp(BinOp::Mul, expr_left, expr_right)));
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
                return Cmd::Expr(Box::new(Expr::BinOp(BinOp::Div, expr_left, expr_right)));
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
                return Cmd::Expr(Box::new(Expr::BinOp(BinOp::Mod, expr_left, expr_right)));
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
                return Cmd::Expr(Box::new(Expr::BinOp(BinOp::Lt, expr_left, expr_right)));
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
                return Cmd::Expr(Box::new(Expr::BinOp(BinOp::Gt, expr_left, expr_right)));
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
                return Cmd::Expr(Box::new(Expr::BinOp(BinOp::Le, expr_left, expr_right)));
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
                return Cmd::Expr(Box::new(Expr::BinOp(BinOp::Ge, expr_left, expr_right)));
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
                return Cmd::Expr(Box::new(Expr::BinOp(BinOp::Eq, expr_left, expr_right)));
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
                return Cmd::Expr(Box::new(Expr::BinOp(BinOp::Ne, expr_left, expr_right)));
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
                return Cmd::Expr(Box::new(Expr::BinOp(BinOp::And, expr_left, expr_right)));
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
                return Cmd::Expr(Box::new(Expr::BinOp(BinOp::Or, expr_left, expr_right)));
            }
            else {
                unreachable!();
            }
        };
        "expr" => rules "expr" "." "ident" => |rules| {
            let mut rules: Vec<Cmd> = rules;
            let right: Cmd = rules.pop().unwrap();
            rules.pop();
            let left: Cmd = rules.pop().unwrap();
            match (left, right) {
                (Cmd::Expr(expr_left), Cmd::Expr(expr_right)) => {
                    match *expr_right {
                        Expr::Var(attr) => {
                            Cmd::Expr(Box::new(Expr::GetAttr(expr_left, attr)))
                        },
                        _ => unreachable!()
                    }
                },
                _ => unreachable!()
            }
        };
        "expr" => rules "not" "expr" => |mut rules| {
            if let Cmd::Expr(e) = rules.pop().unwrap() {
                return Cmd::Expr(Box::new(Expr::UnOp(UnOp::Not, e)));
            }
            unreachable!();
        };
        "expr" => rules "deref" "expr" => |mut rules| {
            if let Cmd::Expr(e) = rules.pop().unwrap() {
                return Cmd::Expr(Box::new(Expr::UnOp(UnOp::Deref, e)));
            }
            unreachable!();
        };
        "expr" => rules "ident";

        "int" => lexemes "INT" => |lexemes| {
            Cmd::Expr(Box::new(Expr::ConstInt(lexemes[0].raw.clone())))
        };
        "float" => lexemes "FLOAT" => |lexemes| {
            Cmd::Expr(Box::new(Expr::ConstFloat(lexemes[0].raw.clone())))
        };
        "if" => lexemes "IF" => |_| Cmd::Nop;
        "then" => lexemes "THEN" => |_| Cmd::Nop;
        "else" => lexemes "ELSE" => |_| Cmd::Nop;
        "while" => lexemes "WHILE" => |_| Cmd::Nop;
        "do" => lexemes "DO" => |_| Cmd::Nop;
        "continue" => lexemes "CONTINUE" => |_| Cmd::Continue;
        "break" => lexemes "BREAK" => |_| Cmd::Break;
        "fn" => lexemes "FUNC" => |_| Cmd::Nop;
        "class" => lexemes "CLASS" => |_| Cmd::Nop;
        "return" => lexemes "RETURN" => |_| Cmd::Nop;
        "." => lexemes "DOT" => |_| Cmd::Nop;
        "ident" => lexemes "IDENT" => |lexemes| {
            Cmd::Expr(Box::new(Expr::Var(lexemes[0].raw.to_string())))
        };
        ";" => lexemes "SEMICOL" => |_| Cmd::Nop;
        "," => lexemes "COMMA" => |_| Cmd::Nop;
        "(" => lexemes "LEFT_PAREN" => |_| Cmd::Nop;
        ")" => lexemes "RIGHT_PAREN" => |_| Cmd::Nop;
        "call(" => lexemes "LEFT_PAREN" => |_| Cmd::Nop;
        ")call" => lexemes "RIGHT_PAREN" => |_| Cmd::Nop;
        "{" => lexemes "LEFT_BRACE" => |_| Cmd::Nop;
        "}" => lexemes "RIGHT_BRACE" => |_| Cmd::Nop;
        "[" => lexemes "LEFT_BRACKET" => |_| Cmd::Nop;
        "]" => lexemes "RIGHT_BRACKET" => |_| Cmd::Nop;
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

        Associativity::Left => rules "or";
        Associativity::Left => rules "and";
        Associativity::Left => rules ">" ">=" "<" "<=" "==" "!=";
        Associativity::Left => rules "+" "-";
        Associativity::Left => rules "*" "/" "%";
        Associativity::None => rules "deref" "negate" "not";
        Associativity::None => rules "call(" ")call" ".";
    )
}