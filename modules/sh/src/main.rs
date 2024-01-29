use std::{env, io::{self, Write}};

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

    match input.as_str().trim() {
        "exit" | "quit" => *control_flow = ControlFlow::Exit,
        other => {
            println!("Unknown command {}!", other);
            *control_flow = ControlFlow::Wait;
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
