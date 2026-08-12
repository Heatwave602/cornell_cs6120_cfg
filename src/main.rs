
use std::io::{self, Read,};
use std::process;
use std::error::Error;

use mycfg::build_blks;

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
    for (function, blks) in build_blks(&prog.bril_js)? {
        println!("function: {function}\nblock: {blks:#?}");
    }

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