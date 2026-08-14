use std::error::Error;
use std::collections::HashMap;

mod parse;
mod blks;

use parse::functions;
use blks::Blk;
use blks::blocks;

use crate::parse::InstrLbl;

pub fn build_blks(
  bril_js: &str,
) -> Result<HashMap<String, Vec<Blk>>, Box<dyn Error>> {
  let mut fun_blks: HashMap<String, Vec<Blk>> = HashMap::new();

  for f in functions(bril_js)? {
    let instrs = f.instrs();
    fun_blks.insert(f.name().to_string(), blocks(instrs.to_vec())?);
  }

  Ok(fun_blks)
}

pub fn get_functions(bril_js: &str) -> Result<HashMap<String, Vec<InstrLbl>>, Box<dyn Error>> {
  let mut map = HashMap::new();

  for f in functions(bril_js)? {
    map.insert(f.name().to_string(), f.instrs().to_vec());
  }
  Ok(map)
}