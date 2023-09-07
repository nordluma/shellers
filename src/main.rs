use std::{
    env,
    io::{self, stdin, stdout, Write},
    path::Path,
    process::{Child, Command, Stdio},
};

fn main() -> io::Result<()> {
    loop {
        print!("> ");
        stdout().flush()?;

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next() {
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            match command {
                "cd" => {
                    let new_dir = args.peekable().peek().map_or("/", |d| *d);
                    let root = Path::new(new_dir);

                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }

                    previous_command = None;
                }
                "exit" => return Ok(()),
                command => {
                    let stdin = previous_command.map_or(Stdio::inherit(), |output: Child| {
                        Stdio::from(output.stdout.unwrap())
                    });

                    let stdout = if commands.peek().is_some() {
                        // Another command is piped, prepare to send
                        // output to next command
                        Stdio::piped()
                    } else {
                        // No other commands piped, send output to shell sdtout
                        Stdio::inherit()
                    };

                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match output {
                        Ok(output) => {
                            previous_command = Some(output);
                        }
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        }
                    }
                }
            }
        }

        if let Some(mut final_command) = previous_command {
            final_command.wait()?;
        }
    }
}
