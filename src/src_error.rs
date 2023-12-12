use colored::Colorize;
use santiago::lexer::LexerError;


fn line_start(num: usize, num_width: usize) -> String {
    let num = num.to_string();
    return " ".repeat(num_width - num.len()) + &num + " ";
}

pub fn lexer_error(src: &str, err: &LexerError) -> String {
    let mut res = "[LexerError]".bright_yellow().to_string() + ": ";
    res.push_str(&err.to_string());
    res.push('\n');
    let lines_src: Vec<&str> = src.lines().collect();

    let start = std::cmp::max(1, err.position.line as i32 - 5) as usize;
    let end = std::cmp::min(lines_src.len(), err.position.line + 5);
    let num_width = std::cmp::max(start.to_string().len(), end.to_string().len());
    
    for line_num in start..=end {
        if line_num == err.position.line {
            res.push_str(&line_start(line_num, num_width).on_bright_red().to_string());
            {
                let tmp: Vec<char> = lines_src[line_num - 1].chars().collect();
                for idx in 0..err.position.column - 1 {
                    res.push(tmp[idx]);
                }
                res.push_str(&tmp[err.position.column - 1].to_string().bright_red().to_string());
                for idx in err.position.column..tmp.len() {
                    res.push(tmp[idx]);
                }
            }
        }
        else {
            res.push_str(&line_start(line_num, num_width).on_green().to_string());
            res.push_str(lines_src[line_num - 1]);
        }
        res.push('\n');
    }
    res
}