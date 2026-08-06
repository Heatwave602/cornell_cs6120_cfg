use std::io::{self, BufRead};
use std::{/*env,*/ fs, process};

fn main() {
    let stdin = io::stdin();
    let mut path = String::new();
    if let Err(err) = stdin.lock().read_line(&mut path) {
        eprintln!("Problem reading from stdin: {err}");
        process::exit(1)
    };
    let path = path.trim().to_string();

    let prog = Program::build(path);

    // let prog = Program::build(env::args()).
    // unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguments: {err}");
    //     process::exit(1)
    // });

    if let Err(err) = run(prog) {
        eprintln!("Application error: {err}");
        process::exit(1)
    }
}

fn run(prog: Program) -> Result<(), Box<dyn std::error::Error>> {
    let instrs = fs::read_to_string(&prog.path)?;
    for instr in instrs.lines() {
        println!("{instr}");
    }
    Ok(())
}

struct Program {
    path: String
}

impl Program {
    fn build(path: String) -> Program {
        Program {path,}
    }
    // fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
    //     args.next();
    //     let path = match args.next() {
    //         Some(fp) => fp,
    //         None => return Err("not enough arguments"),
    //     };
    //     Ok(Program {path})
    // }
}