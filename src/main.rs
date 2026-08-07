use std::io::{self, Read,};
use std::process;
use std::error::Error;

fn main() {
    let program = Program::build()
        .unwrap_or_else(|err| {
            eprintln!("error reading from stdin: {err}");
            process::exit(1)
        });

    if let Err(err) = run(&program) {
        eprintln!("error: {err}");
        process::exit(1)
    };
}

fn run(prog: &Program) -> Result<(), Box<dyn Error>> {
    println!("{}", prog.bril);

    Ok(())
}

struct Program {
    bril: String,
}

impl Program {
    fn build() -> Result<Self, &'static str> {
        let stdin = io::stdin();
        
        let mut bril = String::new();
        let Ok(_) = stdin.lock().read_to_string(&mut bril) else {
            return Err("io::stdin()::read_to_string()");
        };
        Ok(Program {bril,})
    }
}