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
