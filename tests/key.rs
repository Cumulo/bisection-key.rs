extern crate bisection_key;

#[cfg(test)]
mod tests {
  #[test]
  fn equality_of_keys() {
    assert_eq!(
      bisection_key::BalancedKey::new("a"),
      bisection_key::BalancedKey::new("a")
    );

    assert_ne!(
      bisection_key::BalancedKey::new("a"),
      bisection_key::BalancedKey::new("b")
    );

    assert_eq!(
      bisection_key::BalancedKey::new("a"),
      bisection_key::BalancedKey::new("aT")
    );

    assert_eq!(
      bisection_key::BalancedKey::new("aT"),
      bisection_key::BalancedKey::new("aT")
    );
  }

  #[test]
  fn order_of_keys() {
    assert!(bisection_key::BalancedKey::new("a") < bisection_key::BalancedKey::new("b"));
    assert!(bisection_key::BalancedKey::new("a") == bisection_key::BalancedKey::new("aT"));
    assert!(bisection_key::BalancedKey::new("b") > bisection_key::BalancedKey::new("a"));
    assert!(bisection_key::BalancedKey::new("aT") < bisection_key::BalancedKey::new("b"));
    assert!(bisection_key::BalancedKey::new("azzzzzz") < bisection_key::BalancedKey::new("b"));
  }
}
