use std::error::Error;
use std::collections::HashMap;

mod parse;
mod blks;

use parse::functions;
use blks::Blk;
use blks::blocks;

pub fn build_blks(
  bril_js: &str,
) -> Result<HashMap<String, Vec<Blk>>, Box<dyn Error>> {
  let mut fun_blks: HashMap<String, Vec<Blk>> = HashMap::new();

  for f in functions(bril_js)? {
    fun_blks.insert(f.name(), blocks(f.instrs())?);
  }

  Ok(fun_blks)
}