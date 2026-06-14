use std::{
    io::{self, Write},
    process::{ChildStdout, Command, Stdio},
};

fn main() {
    loop {
        print!("blazing_shell> ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();

        let mut user_input = String::new();
        stdin
            .read_line(&mut user_input)
            .expect("failed to read line");

        let trimmed_input = user_input.trim();

        if trimmed_input == "exit" {
            break;
        }

        let commands: Vec<&str> = trimmed_input.splitn(2, "|").collect();
        let mut input = Stdio::inherit();

        for (i, command) in commands.iter().enumerate() {
            let output = if i == commands.len() - 1 {
                Stdio::inherit()
            } else {
                Stdio::piped()
            };

            let res = process_command(command, input, output);
            match res {
                Some(out) => {
                    input = Stdio::from(out);
                }
                None => break,
            }
        }
    }
}

fn process_command(trimmed_input: &str, input: Stdio, out: Stdio) -> Option<ChildStdout> {
    let mut split = trimmed_input.split_ascii_whitespace();
    let command = split.next();
    let args: Vec<&str> = split.map(|arg| arg.trim()).collect();
    if command.is_none() {
        return None;
    }
    let child_process = Command::new(command.unwrap().trim())
        .args(args)
        .stdin(input)
        .stdout(out)
        .spawn();

    match child_process {
        Ok(mut process) => {
            let out = process.stdout.take();
            let ecode = process.wait().expect("failed to wait on child");
            if ecode.success() {
                return out;
            } else {
                println!("{}", ecode);
                None
            }
        }
        Err(e) => {
            println!("Ошибка запуска '{}': {}", command.unwrap(), e);
            None
        }
    }
}
