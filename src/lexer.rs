use santiago::lexer::LexerRules;

pub fn lexer_rules() -> LexerRules {
    santiago::lexer_rules!(
        "DEFAULT" | "INT" = pattern r"0|[1-9][0-9]*";
        "DEFAULT" | "IF" = string "if";
        "DEFAULT" | "THEN" = string "then";
        "DEFAULT" | "ELSE" = string "else";
        "DEFAULT" | "WHILE" = string "while";
        "DEFAULT" | "DO" = string "do";
        "DEFAULT" | "CONTINUE" = string "continue";
        "DEFAULT" | "BREAK" = string "break";
        "DEFAULT" | "IDENT" = pattern r"[_A-Za-z][_A-Za-z0-9]*";
        "DEFAULT" | "SEMICOL" = string ";";
        "DEFAULT" | "COMMA" = string ",";
        "DEFAULT" | "LEFT_PAREN" = string "(";
        "DEFAULT" | "RIGHT_PAREN" = string ")";
        "DEFAULT" | "LEFT_BRACE" = string "{";
        "DEFAULT" | "RIGHT_BRACE" = string "}";
        "DEFAULT" | "PLUS" = string "+";
        "DEFAULT" | "MINUS" = string "-";
        "DEFAULT" | "MUL" = string "*";
        "DEFAULT" | "DIV" = string "/";
        "DEFAULT" | "MOD" = string "%";
        "DEFAULT" | "LT" = string "<";
        "DEFAULT" | "GT" = string ">";
        "DEFAULT" | "LE" = string "<=";
        "DEFAULT" | "GE" = string ">=";
        "DEFAULT" | "EQ" = string "==";
        "DEFAULT" | "NE" = string "!=";
        "DEFAULT" | "ASGNOP" = string "=";
        "DEFAULT" | "AND" = string "&&";
        "DEFAULT" | "OR" = string "||";
        "DEFAULT" | "NOT" = string "!";
        "DEFAULT" | "WS" = pattern r"\s" => |lexer| lexer.skip();
    )
}