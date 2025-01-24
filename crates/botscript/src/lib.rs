enum Token {
    Drive(i32),
    Turn(i32),
    Delay(i32),
    Comment(String),
}

#[derive(Debug)]
enum Error<'a> {
    UnknownCommand(&'a str),
}

pub fn compile_file(file: String) -> Vec<String> {
    let mut tokens = Vec::new();
    for line in file.lines() {
        tokens.push(compile_line(line).unwrap());
    }
    generate_java_from_tokens(tokens)
}

fn compile_line(line: &str) -> Result<Token, Error> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    match parts.as_slice() {
        ["DRIVE", num] => Ok(Token::Drive(num.parse().unwrap())),
        ["TURN", deg] => Ok(Token::Turn(deg.parse().unwrap())),
        ["DELAY", num] => Ok(Token::Delay(num.parse().unwrap())),
        ["//", text] => Ok(Token::Comment(text.to_string())),
        _ => Err(Error::UnknownCommand(line)),
    }
}

fn generate_java_from_tokens(tokens: Vec<Token>) -> Vec<String> {
    let mut java_lines = Vec::new();
    for t in tokens {
        match t {
            Token::Drive(i) => java_lines.push(format!("        encoderDrive(DRIVE_SPEED, {}, {}, 5.0);", i, i)),
            Token::Turn(angle) => java_lines.push(format!(
                "        encoderDrive(TURN_SPEED, (Math.PI * TRACK_WIDTH * {} / 180), -(Math.PI * TRACK_WIDTH * {} / 180), 5.0);",
                angle, angle
            )),
            Token::Delay(i) => java_lines.push(format!("        sleep({});", i)),
            Token::Comment(_) => {}
        }
    }
    java_lines
}
