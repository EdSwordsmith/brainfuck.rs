#[derive(Debug, Clone, Copy)]
pub enum Token {
    IncValue(i16),
    IncPtr(isize),
    BeginLoop,
    EndLoop,
    Print,
    Read
}

pub fn tokenize(input: impl AsRef<str>) -> Vec<Token> {
    let mut tokens = Vec::new();
    let input: Vec<char> = input.as_ref().chars().collect();
    let mut index = 0;

    while index < input.len() {
        let c = input[index];
        index += 1;
        
        if c == '+' || c == '-' {
            let mut value = if c == '+' { 1 } else { -1 };

            while index < input.len() && (input[index] == '+' || input[index] == '-') {
                value = if input[index] == '+' { value + 1 } else { value - 1 };
                index += 1;
            }

            tokens.push(Token::IncValue(value));
        } else if c == '>' || c == '<' {
            let mut value = if c == '>' { 1 } else { -1 };

            while index < input.len() && (input[index] == '>' || input[index] == '<') {
                value = if input[index] == '>' { value + 1 } else { value - 1 };
                index += 1;
            }

            tokens.push(Token::IncPtr(value));
        } else if c == '.' {
            tokens.push(Token::Print);
        } else if c == ',' {
            tokens.push(Token::Read);
        } else if c == '[' {
            tokens.push(Token::BeginLoop);
        } else if c == ']' {
            tokens.push(Token::EndLoop);
        }
    }

    tokens
}
