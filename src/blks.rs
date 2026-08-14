
use std::error::Error;
use std::mem::take;

use crate::parse::InstrLbl;

pub type Blk = Vec<InstrLbl>;

pub fn blocks(
  instrs: Vec<InstrLbl>,
) -> Result<Vec<Blk>, Box<dyn Error>> {

  let mut blks: Vec<Blk> = Vec::new();
  let mut curr_blk = Vec::new();

  for instr in instrs {
      let InstrLbl::Instr(i) = instr else {
        //label
        let curr = take(&mut curr_blk);
        blks.push(curr);
        curr_blk.push(instr);
        continue;
      };

      let is_terminator = i.is_terminator();
      curr_blk.push(InstrLbl::Instr(i));
      if is_terminator {
        let curr = take(&mut curr_blk);
        blks.push(curr);
      }
  }

  if !curr_blk.is_empty() {blks.push(curr_blk);}

  Ok(blks)
}