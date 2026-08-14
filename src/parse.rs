/*
  a block is a set of instructions, 
  that end either on a label or a terminator instruction
*/
use std::error::Error;
use serde::Deserialize;

const TERMINATORS: &str = "jmp|br|ret";

pub fn functions(bril_js: &str) -> Result<Vec<BrilFunction>, Box<dyn Error>> {
  #[derive(Debug, Deserialize)]
  struct Functions{functions: Vec<BrilFunction>}

  let fs: Functions = serde_json::from_str(bril_js)?;

  Ok(fs.functions)
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum BrilFunction {
  RetFun{
    name:     String,
    /*optional field*/
    #[serde(default)]
    args:     Vec<Arg>,
    #[serde(rename="type")]
    ret_type: Type,
    instrs:   Vec<InstrLbl>,
  },
  Fun{
    name:   String,
    /*optional field*/
    #[serde(default)]
    args:   Vec<Arg>,
    instrs: Vec<InstrLbl>,
  },
}

impl BrilFunction {
  pub fn name(&self) -> &str {
    match self {
      BrilFunction::RetFun { name, .. } => name,
      BrilFunction::Fun { name, .. }    => name,
    }
  }

  pub fn instrs(&self) -> &[InstrLbl] {
    match self {
      BrilFunction::RetFun {name:_, args:_, ret_type:_, 
        instrs} => instrs,
      BrilFunction::Fun {name:_, args:_, 
        instrs} => instrs,
    }
  }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum Type {
  Primitive     (String),
  Parameterized {ptr: Box<Type>},
}

#[derive(Debug, Deserialize)]
pub struct Arg {
  name:     String,
  #[serde(rename="type")]
  arg_type: Type,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum InstrLbl {
  Instr(Instruction),
  Label(Lbl),
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum Instruction {
  Const {
    op:         String,
    dest:       String,
    #[serde(rename="type")]
    instr_type: Type,
    value:      Literal,
  },
  Val {
    op:         String,
    dest:       String,
    #[serde(rename="type")]
    instr_type: Type,

    /*optional fields*/
    #[serde(default)]
    args:   Vec<String>,
    #[serde(default)]
    funcs:  Vec<String>,
    #[serde(default)]
    labels: Vec<String>,
  },
  Effect {
    op: String,

    /*optional fields*/
    #[serde(default)]
    args:   Vec<String>,
    #[serde(default)]
    funcs:  Vec<String>,
    #[serde(default)]
    labels: Vec<String>,
  },
}

#[derive(Debug, Deserialize, Clone)]
pub struct Lbl {
  label_name: String,
}

impl Instruction {
  pub fn is_terminator(&self) -> bool {
    match self {
      Self::Const {op , ..} |
      Self::Val {op, ..} |
      Self::Effect {op, ..} => TERMINATORS.contains(op),
    }
  }
}

#[derive(Debug,Deserialize, Clone)]
#[serde(untagged)]
enum Literal {
  Num(serde_json::Number),
  Bool(bool),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_function_without_return_type() {
        let json = r#"
        {
          "functions": [
            {
              "name": "main",
              "instrs": [
                {"op": "const", "dest": "a", "type": "int", "value": 4},
                {"op": "print", "args": ["a"]}
              ]
            }
          ]
        }"#;

        let funcs = functions(json).expect("should parse");
        assert_eq!(funcs.len(), 1);

        match &funcs[0] {
            BrilFunction::Fun { name, args, instrs } => {
                assert_eq!(name, "main");
                assert!(args.is_empty(), "missing args should default to empty");
                assert_eq!(instrs.len(), 2);
            }
            other => panic!("expected Fun variant, got {other:?}"),
        }
    }

    #[test]
    fn parses_function_with_return_type() {
        let json = r#"
        {
          "functions": [
            {
              "name": "add",
              "args": [{"name": "x", "type": "int"}, {"name": "y", "type": "int"}],
              "type": "int",
              "instrs": [
                {"op": "add", "dest": "sum", "type": "int", "args": ["x", "y"]},
                {"op": "ret", "args": ["sum"]}
              ]
            }
          ]
        }"#;

        let funcs = functions(json).expect("should parse");

        match &funcs[0] {
            BrilFunction::RetFun { name, args, ret_type, instrs } => {
                assert_eq!(name, "add");
                assert_eq!(args.len(), 2);
                assert!(matches!(ret_type, Type::Primitive(t) if t == "int"));
                assert_eq!(instrs.len(), 2);
            }
            other => panic!("expected RetFun variant, got {other:?}"),
        }
    }

    #[test]
    fn parses_labels_as_labels_not_instructions() {
        let json = r#"
        {
          "functions": [
            {
              "name": "loop",
              "instrs": [
                {"label": "start"},
                {"op": "print", "args": ["x"]},
                {"label": "end"}
              ]
            }
          ]
        }"#;

        let funcs = functions(json).expect("should parse");

        let BrilFunction::Fun { instrs, .. } = &funcs[0] else {
            panic!("expected Fun variant");
        };

        assert!(matches!(&instrs[0], InstrLbl::Label(Lbl{label_name}) if label_name == "start"));
        assert!(matches!(&instrs[1], InstrLbl::Instr(_)));
        assert!(matches!(&instrs[2], InstrLbl::Label(Lbl{label_name}) if label_name == "end"));
    }

    #[test]
    fn parses_effect_instruction_with_no_dest() {
        let json = r#"
        {
          "functions": [
            {
              "name": "f",
              "instrs": [
                {"op": "jmp", "labels": ["start"]}
              ]
            }
          ]
        }"#;

        let funcs = functions(json).expect("should parse");
        let BrilFunction::Fun { instrs, .. } = &funcs[0] else {
            panic!("expected Fun variant");
        };

        match &instrs[0] {
            InstrLbl::Instr(Instruction::Effect { op, labels, .. }) => {
                assert_eq!(op, "jmp");
                assert_eq!(labels, &vec!["start".to_string()]);
            }
            other => panic!("expected Effect instruction, got {other:?}"),
        }
    }

    #[test]
    fn const_before_val_in_untagged_ordering() {
        // Regression guard: Const has an extra `value` field that Val lacks.
        // If Val were listed before Const in the enum, this could still parse
        // as a Val followed by an unknown-field error, or worse, silently
        // ignore `value`. This test pins Const's actual shape.
        let json = r#"
        {
          "functions": [
            {
              "name": "f",
              "instrs": [
                {"op": "const", "dest": "x", "type": "bool", "value": true}
              ]
            }
          ]
        }"#;

        let funcs = functions(json).expect("should parse");
        let BrilFunction::Fun { instrs, .. } = &funcs[0] else {
            panic!("expected Fun variant");
        };

        match &instrs[0] {
            InstrLbl::Instr(Instruction::Const { dest, value, .. }) => {
                assert_eq!(dest, "x");
                assert!(matches!(value, Literal::Bool(true)));
            }
            other => panic!("expected Const instruction, got {other:?}"),
        }
    }

    #[test]
    fn parses_pointer_type() {
        let json = r#"
        {
          "functions": [
            {
              "name": "f",
              "args": [{"name": "p", "type": {"ptr": "int"}}],
              "instrs": []
            }
          ]
        }"#;

        let funcs = functions(json).expect("should parse");
        let BrilFunction::Fun { args, .. } = &funcs[0] else {
            panic!("expected Fun variant");
        };

        match &args[0].arg_type {
            Type::Parameterized { ptr } => {
                assert!(matches!(ptr.as_ref(), Type::Primitive(t) if t == "int"));
            }
            other => panic!("expected Parameterized type, got {other:?}"),
        }
    }

    #[test]
    fn rejects_malformed_json() {
        let json = r#"{ "functions": [ { "name": "f" "instrs": [] } ] }"#; // missing comma
        assert!(functions(json).is_err());
    }

    #[test]
    fn empty_functions_list_is_ok() {
        let json = r#"{"functions": []}"#;
        let funcs = functions(json).expect("should parse");
        assert!(funcs.is_empty());
    }
}