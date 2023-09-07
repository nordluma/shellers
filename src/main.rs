use std::{
    io::{self, stdin, stdout, Write},
    process::Command,
};

fn main() -> io::Result<()> {
    loop {
        print!("> ");
        stdout().flush()?;

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let command = input.trim();
        let mut child = Command::new(command).spawn().unwrap();

        child.wait()?;
    }
}
