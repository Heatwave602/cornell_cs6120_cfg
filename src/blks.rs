
use std::error::Error;
use std::mem::take;

use crate::parse::{InstrLbl, Instruction};

enum BlocksErr {
  InstrsEmpty,
}

pub type Blk<'a> = Vec<&'a Instruction>;

pub fn blocks(
  instrs: &[InstrLbl],
) -> Result<Vec<Blk>, Box<dyn Error>> {
  // if instrs.is_empty() {return Err();}

  let mut blks: Vec<Blk> = Vec::new();
  let mut curr_blk = Vec::new();

  for instr in instrs {
    match instr {
      InstrLbl::Instr(i) => {
        curr_blk.push(i);
        if i.is_terminator() {
          let curr = take(&mut curr_blk);
          blks.push(curr);
        }
      },
      InstrLbl::Label {..} => {
        if curr_blk.is_empty() {
          continue;
       }

        curr_blk.clear();
      },
    }
  }

  Ok(blks)
}