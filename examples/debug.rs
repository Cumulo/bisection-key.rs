use bisection_key::BalancedKey;

fn main() -> Result<(), String> {
  // let a = BalancedKey::new("aV")?;
  // let b = BalancedKey::new("b")?;
  // println!("{}", a.bisect(&b)?);

  let mut base = BalancedKey::new("a")?;
  let next = BalancedKey::new("b")?;
  let mut ret: Vec<String> = vec![];
  for _ in 0..100 {
    base = base.bisect(&next)?;
    println!("{}", base);
    ret.push(base.to_string());
  }

  println!("{:?}", ret);

  let mut base = BalancedKey::new("a")?;
  let mut ret: Vec<String> = vec![];
  for _ in 0..100 {
    base = base.bisect_end()?;
    println!("{}", base);
    ret.push(base.to_string());
  }

  println!("{:?}", ret);

  Ok(())
}
