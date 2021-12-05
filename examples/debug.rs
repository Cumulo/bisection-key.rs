use bisection_key::KeyNumbers;

fn main() -> Result<(), String> {
  // let a = KeyNumbers::new("aV")?;
  // let b = KeyNumbers::new("b")?;
  // println!("{}", a.bisect(&b)?);

  let mut base = KeyNumbers::new("a")?;
  let next = KeyNumbers::new("b")?;
  let mut ret: Vec<String> = vec![];
  for _ in 0..100 {
    base = base.bisect(&next)?;
    println!("{}", base);
    ret.push(base.to_string());
  }

  println!("{:?}", ret);

  let mut base = KeyNumbers::new("a")?;
  let mut ret: Vec<String> = vec![];
  for _ in 0..100 {
    base = base.bisect_end()?;
    println!("{}", base);
    ret.push(base.to_string());
  }

  println!("{:?}", ret);

  Ok(())
}
