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
