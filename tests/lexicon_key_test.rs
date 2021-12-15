extern crate bisection_key;

use bisection_key::LexiconKey;

#[test]
fn equality_of_keys() {
  assert_eq!(LexiconKey::new("a"), LexiconKey::new("a"));
  assert_ne!(LexiconKey::new("a"), LexiconKey::new("b"));
  assert_ne!(LexiconKey::new("a"), LexiconKey::new("aT"));
  assert_eq!(LexiconKey::new("aT"), LexiconKey::new("aT"));
}

#[test]
fn order_of_keys() {
  assert!(LexiconKey::new("a") < LexiconKey::new("b"));
  assert!(LexiconKey::new("a") < LexiconKey::new("aT"));
  assert!(LexiconKey::new("b") > LexiconKey::new("a"));
  assert!(LexiconKey::new("aT") < LexiconKey::new("b"));
  assert!(LexiconKey::new("azzzzzz") < LexiconKey::new("b"));
}

#[test]
fn test_insertion_beginning() -> Result<(), String> {
  let mut k = LexiconKey::default();
  for _ in 0..1000 {
    let next = k.bisect_beginning()?;
    assert!(next < k);
    k = next;
  }

  Ok(())
}

#[test]
fn test_insertion_end() -> Result<(), String> {
  let mut k = LexiconKey::default();
  for _ in 0..1000 {
    let next = k.bisect_end()?;
    assert!(k < next);
    k = next;
  }

  Ok(())
}

#[test]
fn test_insertion_middle() -> Result<(), String> {
  let mut left = LexiconKey::new("a")?;
  let right = LexiconKey::new("b")?;
  for _ in 0..1000 {
    let next = left.bisect(&right)?;
    assert!(left < next);
    assert!(next < right);
    left = next;
  }

  let left = LexiconKey::new("a")?;
  let mut right = LexiconKey::new("b")?;
  for _ in 0..1000 {
    let next = left.bisect(&right)?;
    assert!(left < next);
    assert!(next < right);
    right = next;
  }

  let mut left = LexiconKey::new("a")?;
  let mut right = LexiconKey::new("b")?;
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
