
use std::io::{self, Read,};
use std::process;
use std::error::Error;

mod blks;

use blks::cfg;

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
    cfg(&prog.bril_js)?;

    Ok(())
}

struct Program {
    bril_js: String,
}

impl Program {
    fn build() -> Result<Self, &'static str> {
        let stdin = io::stdin();
        
        let mut bril_js = String::new();
        let Ok(_) = stdin.lock().read_to_string(&mut bril_js) else {
            return Err("io::stdin()::read_to_string()");
        };
        Ok(Program {bril_js,})
    }
}