use std::{process::{Command, self}, io::Write};

use crate::print::user_io::CommandLine;

pub type ExitCode = i32;
pub trait ExecuteCommand {
    fn execute(&self) -> Result<ExitCode, ExitCode>;
}

impl ExecuteCommand for CommandLine {
    fn execute(&self) -> Result<ExitCode, ExitCode> {
        for command in self.commands.iter() {
            match command.execute() {
                Ok(_) => (),
                Err(code) => {
                    return Err(code)
                }
            };
        }
        Ok(0)
    }
}

impl ExecuteCommand for crate::print::user_io::Command {
    fn execute(&self) -> Result<ExitCode, ExitCode> {
        let mut stdout = std::io::stdout();
        let mut stderr = std::io::stderr();

        let child = match Command::new(self.path.as_str())
            .args(&self.arguments)
            .stdin(process::Stdio::piped())
            .stderr(process::Stdio::piped())
            .stdout(process::Stdio::piped())
            .spawn() {
                Ok(child) => child,
                Err(err) => {
                    println!("spawn error {}", err);
                    return Err(-1);
                }
            };

        let output = child.wait_with_output().unwrap();

        stdout.write_all(&output.stdout).unwrap();
        stderr.write_all(&output.stderr).unwrap();
        
        match output.status.code() {
            Some(code) => {
                if code == 0 {
                    Ok(0)
                } else {
                    Err(code)
                }
            },
            None => Err(-1),
        }
    }
}