enum Operator {
    Plus,
    Minus,
    Times,
    Div
}

enum Token {
    EOF,
    TokOp(Operator),
    TokIdent(String),
    TokNum(i64),
}

struct Tokenizer {
    source: String,
    start: u64,
    position: u64,
}

impl Tokenizer {
    fn next() -> Token {
        match source::get(0..4) {
            Some(t) => Token::TokIdent(t),
            None => Token::EOF
        }
    }
}

fn op_to_str(op: Operator) -> &'static str {
    match op {
        Operator::Plus => "+",
        Operator::Minus => "-",
        Operator::Times => "*",
        Operator::Div => "/",
    }
}

fn show_content(token: Token) -> String {
    match token {
        Token::TokOp(op) => String::from(op_to_str(op)),
        Token::TokIdent(s) => s,
        Token::TokNum(i) => i.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_op_to_str() {
        assert_eq!(op_to_str(Operator::Plus), "+");
        assert_eq!(op_to_str(Operator::Minus), "-");
        assert_eq!(op_to_str(Operator::Times), "*");
        assert_eq!(op_to_str(Operator::Div), "/");
    }

    #[test]
    fn test_show_content() {
        assert_eq!(show_content(Token::TokOp(Operator::Plus)), "+");
        assert_eq!(show_content(Token::TokIdent(String::from("hehe"))), "hehe");
        assert_eq!(show_content(Token::TokNum(32)), "32");
    }

    #[test]
    fn test_tokenizer_next() {
        let test = Tokenizer{
            source: String::from("JE 53"),
            start: 0,
            position: 0,
        };
        assert_eq!(Token::TokIndent(String::from("J")), test::next());
    }
}

