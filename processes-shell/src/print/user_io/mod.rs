use std::{io::{self, BufWriter, stdout, Write}};
use regex::Regex;

#[derive(Debug, Clone)]
pub enum CommandOperatorKind {
    /** && */
    AND,
    /** || */
    OR,
    /** >> */
    APPEND_REDIRECT,
    /** > */
    REDIRECT,
    /** | */
    PIPE,
}

#[derive(Debug, Clone)]
pub struct Command {
    pub path: String,
    pub arguments: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct CommandLine {
    pub commands: Vec<Command>,
    pub operators: Vec<CommandOperatorKind>,
}

impl CommandLine {
    pub fn from(commands: Vec<Command>, operators: Vec<CommandOperatorKind>) -> Self {
        CommandLine { commands: commands, operators: operators }
    }
}


pub fn request_command() -> Result<CommandLine, String> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut str = String::from("");

    stdout.flush().unwrap();
    print!("wish> ");
    stdout.flush().unwrap();

    loop {
        match stdin.read_line(&mut str) {
            Ok(_) => {
                break
            },
            Err(_) => ()
        }
    }

    let operators = regex::Regex::new(
        r"(?<OR>\|\|)|(?<AND>&&)|(?<APPEND_REDIRECT>>>)|(?<REDIRECT>>)|(?<PIPE>\|)",
    ).unwrap();
    
    let mut ranges: Vec<(usize, usize)> = vec![(0, str.len())];
    let mut new_commands: Vec<String> = vec![];
    let mut new_operators: Vec<CommandOperatorKind> = vec![];
    let before_split = str.clone();

    for captured in operators.captures_iter(&before_split) {
        match captured.get(0) {
            Some(matched) => {
                if let Some(last) = ranges.last_mut() {
                    (*last).1 = matched.start();
                }
                ranges.push((matched.end(), str.len()));

                match matched.as_str() {
                    "&&" => new_operators.push(CommandOperatorKind::AND),
                    "||" => new_operators.push(CommandOperatorKind::OR),
                    ">>" => new_operators.push(CommandOperatorKind::APPEND_REDIRECT),
                    ">" => new_operators.push(CommandOperatorKind::REDIRECT),
                    "|" => new_operators.push(CommandOperatorKind::PIPE),
                    _ => (),
                }
            },
            _ => (),
        }
    }

    for range in ranges {
        new_commands.push(str.as_str()[range.0..range.1].chars().as_str().trim().to_string());
    }

    if new_commands.len() != new_operators.len() + 1 {
        Err(String::from("Invalid Command Line Input"))
    } else {
        Ok(CommandLine::from(
            new_commands.iter().filter(|c| c.trim().len() > 0).map(|c| {
                let splitted: Vec<String> = c
                    .split(' ')
                    .map(|token| token.trim().to_string())
                    .filter(|token| token.len() > 0)
                    .collect();
                let path = splitted.get(0).unwrap().to_string();
                let arguments = splitted[1..].to_vec();

                Command {
                    path: path,
                    arguments: arguments,
                }
            }).collect(),
            new_operators,
        ))
    }
}