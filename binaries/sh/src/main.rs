use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
    process::Command,
};

enum ControlFlow {
    Wait,
    Exit,
}

fn prompt(stdin: &io::Stdin, stdout: &mut io::Stdout, control_flow: &mut ControlFlow) {
    match stdout.write(b"> ").and(stdout.flush()).err() {
        Some(err) => match err.kind() {
            _ => {
                println!("Failed to write to stdout!");
                *control_flow = ControlFlow::Exit;
                return;
            }
        },
        None => {}
    };

    let mut input = String::new();
    match stdin.read_line(&mut input).err() {
        Some(err) => match err.kind() {
            _ => {
                println!("Failed to read from stdout!");
                *control_flow = ControlFlow::Exit;
                return;
            }
        },
        None => {}
    };

    let input_split: Vec<String> = input.trim().split(" ").map(String::from).collect();

    if let Some(cmd) = input_split.get(0) {
        match input.as_str().trim() {
            "exit" | "quit" => *control_flow = ControlFlow::Exit,
            other => {
                let path: PathBuf = other.into();

                if path.try_exists().unwrap_or(false) {
                    if let Ok(mut handle) = Command::new(path).args(&input_split[1..]).spawn() {
                        match handle.wait() {
                            Ok(code) => println!("{cmd} exited with code {code}."),
                            Err(_) => println!("{cmd} failed to exit smoothly!"),
                        }
                    }
                } else {
                    println!("Unknown command {}!", other);
                }

                *control_flow = ControlFlow::Wait;
            }
        }
    }
}

fn main() {
    let mut control_flow = ControlFlow::Wait;

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        match control_flow {
            ControlFlow::Exit => break,
            ControlFlow::Wait => prompt(&stdin, &mut stdout, &mut control_flow),
        }
    }
}
