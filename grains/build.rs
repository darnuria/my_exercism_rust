use std::io::{Result, Write};
use std::env;
use std::fs;
use std::path::Path;

fn main() -> Result<()> {
  let out_dir = env::var("OUT_DIR").unwrap();
  let dest_path = Path::new(&out_dir).join("grains.rs");
  let mut f = fs::File::create(&dest_path).unwrap();

  write!(f, "const GRAINS: [u64; 64] = [\n")?;
  for i in 1..64 {
    write!(f, "  {},\n", 2u64.pow(i) / 2)?;
  }
  write!(f, "  9_223_372_036_854_775_808u64,\n")?;
  write!(f, "];\n")?;

  Ok(())
}
