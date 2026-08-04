use std::{env, error::Error, fs, process};

fn main() {
    let prog = Program::build(env::args()).
    unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1)
    });

    if let Err(err) = run(prog) {
        eprintln!("Application error: {err}");
        process::exit(1)
    }
}

fn run(prog: Program) -> Result<(), Box<dyn Error>> {
    let instrs = fs::read_to_string(prog.path)?;
    for instr in instrs.lines() {
        println!("{instr}");
    }
    Ok(())
}

struct Program {
    path: String
}

impl Program {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next();
        let path = match args.next() {
            Some(fp) => fp,
            None => return Err("not enough arguments"),
        };
        Ok(Program {path})
    }
}