use santiago::lexer::LexerRules;

pub fn lexer_rules() -> LexerRules {
    santiago::lexer_rules!(
        "DEFAULT" | "INT" = pattern r"0|[1-9][0-9]*";
        "DEFAULT" | "FLOAT" = pattern r"\d+\.\d+";
        "DEFAULT" | "IF" = string "if";
        "DEFAULT" | "THEN" = string "then";
        "DEFAULT" | "ELSE" = string "else";
        "DEFAULT" | "WHILE" = string "while";
        "DEFAULT" | "DO" = string "do";
        "DEFAULT" | "CONTINUE" = string "continue";
        "DEFAULT" | "BREAK" = string "break";
        "DEFAULT" | "FUNC" = string "fn";
        "DEFAULT" | "CLASS" = string "class";
        "DEFAULT" | "RETURN" = string "return";
        "DEFAULT" | "DOT" = string ".";
        "DEFAULT" | "IDENT" = pattern r"[_A-Za-z\u4e00-\u9fa5][_A-Za-z0-9\u4e00-\u9fa5]*";
        "DEFAULT" | "SEMICOL" = string ";";
        "DEFAULT" | "COMMA" = string ",";
        "DEFAULT" | "LEFT_PAREN" = string "(";
        "DEFAULT" | "RIGHT_PAREN" = string ")";
        "DEFAULT" | "LEFT_BRACE" = string "{";
        "DEFAULT" | "RIGHT_BRACE" = string "}";
        "DEFAULT" | "LEFT_BRACKET" = string "[";
        "DEFAULT" | "RIGHT_BRACKET" = string "]";
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
        "DEFAULT" | "COMMENT" = pattern r"//[^\r\n]*" => |lexer| lexer.skip();
        "DEFAULT" | "COMMENT" = pattern r"/\*([^*]|\*+[^*/])*\*+/" => |lexer| lexer.skip();
        "DEFAULT" | "WS" = pattern r"\s" => |lexer| lexer.skip();
    )
}