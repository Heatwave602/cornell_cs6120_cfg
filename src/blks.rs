
use std::error::Error;
use std::mem::take;

use crate::parse::{InstrLbl, Instruction};

pub type Blk = Vec<Instruction>;

pub fn blocks(
  instrs: Vec<InstrLbl>,
) -> Result<Vec<Blk>, Box<dyn Error>> {

  let mut blks: Vec<Blk> = Vec::new();
  let mut curr_blk = Vec::new();

  for instr in instrs {
    match instr {
      InstrLbl::Instr(i) => {
        let is_terminator = i.is_terminator();

        curr_blk.push(i);
        if is_terminator {
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
  if !curr_blk.is_empty() {blks.push(curr_blk);}

  Ok(blks)
}