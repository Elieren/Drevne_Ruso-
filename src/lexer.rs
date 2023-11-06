use crate::token::Token;


pub fn process_variable(variable: &str, result: &mut Vec<Token>) {
    if !variable.is_empty() {
        if variable.parse::<i32>().is_ok() {
            let value = variable.parse::<i32>().unwrap();
            result.push(Token::Number(value));
        } else if variable == "ВОДА_БАЙКАЛА" {
            result.push(Token::Print);
        } else if variable == "ЦАРЬ_БАТЮШКА" {

        } else {
            result.push(Token::Variable(variable.to_string()));
        }
    }
}

pub fn lexer(input: &str) -> Vec<Token> {
    let mut result = Vec::new();
    let mut variable = String::new();
    let mut value = String::new();
    let mut recording = false;

    for character in input.chars() {
        match character {
            '(' => {
                process_variable(&variable, &mut result);
                if variable == "ЦАРЬ_БАТЮШКА" {
                    recording = true;
                }
                variable.clear();
                result.push(Token::OpenParenthesis);
            },
            ')' => {
                if recording {
                    result.push(Token::PrintText(value.clone()));
                    value.clear();
                    recording = false;
                }
                result.push(Token::CloseParenthesis);
            },
            '+' | '-' | '=' | ';' | '*' | '/' | '^' | '√' => {
                process_variable(&variable, &mut result);
                variable.clear();
                match character {
                    '+' => result.push(Token::Plus),
                    '-' => result.push(Token::Minus),
                    '=' => result.push(Token::Equal),
                    ';' => result.push(Token::EndOfLine),
                    '*' => result.push(Token::Multiply),
                    '/' => result.push(Token::Divide),
                    '^' => result.push(Token::Exponentiation),
                    '√' => result.push(Token::Sqrt),
                    _ => {},
                }
            },
            _ if character.is_whitespace() => {
                process_variable(&variable, &mut result);
                variable.clear();
            },
            _ => {
                if recording {
                    value.push(character);
                } else {
                    variable.push(character);
                }
            },
        }
    }

    process_variable(&variable, &mut result);

    result
}
