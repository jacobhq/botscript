use std::fmt;
use wasm_bindgen::prelude::*;

enum Token {
    Drive(i32),
    Turn(i32),
    Delay(i32),
    Comment,
}

#[derive(Debug)]
pub enum Error {
    UnknownCommandError(String),
    ArgumentParseError(String)
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[macro_export]
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => ($crate::log(&format_args!($($t)*).to_string()))
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::UnknownCommandError(cmd) => write!(f, "UnknownCommandError at: {}", cmd),
            Error::ArgumentParseError(arg) => write!(f, "ArgumentParseError at: {}", arg),
        }
    }
}

impl std::error::Error for Error {}

pub fn compile_file(file: String) -> Result<Vec<String>, Error> {
    let mut tokens = Vec::new();
    for line in file.lines() {
        tokens.push(compile_line(line)?);
    }
    Ok(generate_java_from_tokens(tokens))
}

fn compile_line(line: &str) -> Result<Token, Error> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    match parts.as_slice() {
        ["DRIVE", num] => num.parse().map(Token::Drive).map_err(|_| Error::ArgumentParseError(line.parse().unwrap())),
        ["TURN", deg] => deg.parse().map(Token::Turn).map_err(|_| Error::ArgumentParseError(line.parse().unwrap())),
        ["DELAY", num] => num.parse().map(Token::Delay).map_err(|_| Error::ArgumentParseError(line.parse().unwrap())),
        ["//"] => Ok(Token::Comment),
        _ => Err(Error::UnknownCommandError(line.parse().unwrap())),
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
            Token::Comment => {}
        }
    }
    java_lines
}
