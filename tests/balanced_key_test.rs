extern crate bisection_key;

use bisection_key::BalancedKey;

#[test]
fn equality_of_keys() {
  assert_eq!(BalancedKey::new("a"), BalancedKey::new("a"));
  assert_ne!(BalancedKey::new("a"), BalancedKey::new("b"));
  assert_eq!(BalancedKey::new("a"), BalancedKey::new("aT"));
  assert_eq!(BalancedKey::new("aT"), BalancedKey::new("aT"));
}

#[test]
fn order_of_keys() {
  assert!(BalancedKey::new("a") < BalancedKey::new("b"));
  assert!(BalancedKey::new("a") == BalancedKey::new("aT"));
  assert!(BalancedKey::new("b") > BalancedKey::new("a"));
  assert!(BalancedKey::new("aT") < BalancedKey::new("b"));
  assert!(BalancedKey::new("azzzzzz") < BalancedKey::new("b"));
}

#[test]
fn test_insertion_beginning() -> Result<(), String> {
  let mut k = BalancedKey::default();
  for _ in 0..1000 {
    let next = k.bisect_beginning()?;
    assert!(next < k);
    k = next;
  }

  Ok(())
}

#[test]
fn test_insertion_end() -> Result<(), String> {
  let mut k = BalancedKey::default();
  for _ in 0..1000 {
    let next = k.bisect_end()?;
    assert!(k < next);
    k = next;
  }

  Ok(())
}

#[test]
fn test_insertion_middle() -> Result<(), String> {
  let mut left = BalancedKey::new("a")?;
  let right = BalancedKey::new("b")?;
  for _ in 0..1000 {
    let next = left.bisect(&right)?;
    assert!(left < next);
    assert!(next < right);
    left = next;
  }

  let left = BalancedKey::new("a")?;
  let mut right = BalancedKey::new("b")?;
  for _ in 0..1000 {
    let next = left.bisect(&right)?;
    assert!(left < next);
    assert!(next < right);
    right = next;
  }

  let mut left = BalancedKey::new("a")?;
  let mut right = BalancedKey::new("b")?;
  let mut prefer_left = true;
  for _ in 0..1000 {
    let next = left.bisect(&right)?;
    // println!("{} {} {}", left, next, right);
    assert!(left < next);
    assert!(next < right);

    if prefer_left {
      left = next;
      prefer_left = false;
    } else {
      right = next;
      prefer_left = true
    }
  }

  Ok(())
}
