extern crate bisection_key;

use bisection_key::LexiconKey;

fn main() -> Result<(), String> {
  // let a = LexiconKey::new("aV")?;
  // let b = LexiconKey::new("b")?;
  // println!("{}", a.bisect(&b)?);

  // let mut base = LexiconKey::new("a")?;
  // let next = LexiconKey::new("b")?;
  // let mut ret: Vec<String> = vec![];
  // for _ in 0..100 {
  //   base = base.bisect(&next)?;
  //   println!("{}", base);
  //   ret.push(base.to_string());
  // }

  // println!("{:?}", ret);

  // let mut base = LexiconKey::new("a")?;
  // let mut ret: Vec<String> = vec![];
  // for _ in 0..100 {
  //   base = base.bisect_end()?;
  //   println!("{}", base);
  //   ret.push(base.to_string());
  // }

  // println!("{:?}", ret);

  let mut base = LexiconKey::new("B")?;
  let edge = LexiconKey::new("A")?;

  for _ in 0..100 {
    base = base.bisect(&edge)?;
    println!("{}", base);
  }

  Ok(())
}
