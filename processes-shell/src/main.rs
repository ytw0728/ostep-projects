mod execute;
mod print;

use execute::ExecuteCommand;
use print::user_io;

fn main() {
    loop {
        match user_io::request_command() {
            Ok(command) => {
                match command.execute() {
                    Ok(_) => (),
                    Err(code) => {
                        println!("Error (exit code: {})", code);
                    }
                }
            },
            Err(err) => println!("{}", err),
        };
    }
}
