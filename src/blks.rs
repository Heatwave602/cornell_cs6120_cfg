/*
  a block is a set of instructions, 
  that end either on a label or a terminator instruction
*/

// use std::collections::HashMap;
use std::error::Error;
use serde::Deserialize;

fn cfg(bril_js: &str) {
  for func in functions(bril_js) {
    println!("function: {func:?}");
  }
}

fn functions(bril_js: &str) -> Result<Vec<BrilFunc>, Box<dyn Error>> {
  unimplemented!("blks::functions()")
}

#[derive(Debug, Deserialize)]
enum BrilFunction {
  RetFun{
    name:     String,
    args:     Vec<Arg>,
    #[serde(rename="type")]
    ret_type: String,
    instrs:   Vec<Instr>,
  },
  Fun{
    name:   String,
    args:   Vec<Arg>,
    instrs: Vec<Instr>,
  },
}

enum Type {
  Prim(String),
  Param,
}

#[derive(Debug, Deserialize)]
struct Arg {
  name:     String,
  #[serde(rename="type")]
  arg_type: String,
}

#[derive(Debug, Deserialize)]
enum Instr {
  Label {label: String},
  Instr(Instruction),
}

#[derive(Debug, Deserialize)]
enum Instruction {
  Const {
    dest: ,
    #[serde(rename="type")]
    const_type: ,
      
  },
  Val {},
  Effect {},
}

#[cfg(test)]
mod tests {
    use crate::blks::BrilFunc;

  #[test]
  fn functions() {
    let bril_js = r#"functions": [
      {
        "name": "add5",
        "args": [{"name": "n", "type": "int"}],
        "type": "int",
        "instrs": [
          { "op": "const", "type": "int", "dest": "five", "value": 5 },
        ]
      }
    ]"#;

    let expected = BrilFunc {
      name: String::from("add5"),
      args:
      #[serde(rename="type")]
      fun_type: String::from("int"),
      instrs: ,
    };
    assert_eq!(vec![],)
  }

}