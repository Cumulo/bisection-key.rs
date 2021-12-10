extern crate bisection_key;

#[cfg(test)]
mod tests {
  #[test]
  fn equality_of_keys() {
    assert_eq!(
      bisection_key::KeyNumbers::new("a"),
      bisection_key::KeyNumbers::new("a")
    );

    assert_ne!(
      bisection_key::KeyNumbers::new("a"),
      bisection_key::KeyNumbers::new("b")
    );

    assert_eq!(
      bisection_key::KeyNumbers::new("a"),
      bisection_key::KeyNumbers::new("aT")
    );

    assert_eq!(
      bisection_key::KeyNumbers::new("aT"),
      bisection_key::KeyNumbers::new("aT")
    );
  }

  #[test]
  fn order_of_keys() {
    assert!(bisection_key::KeyNumbers::new("a") < bisection_key::KeyNumbers::new("b"));
    assert!(bisection_key::KeyNumbers::new("a") == bisection_key::KeyNumbers::new("aT"));
    assert!(bisection_key::KeyNumbers::new("b") > bisection_key::KeyNumbers::new("a"));
    assert!(bisection_key::KeyNumbers::new("aT") < bisection_key::KeyNumbers::new("b"));
    assert!(bisection_key::KeyNumbers::new("azzzzzz") < bisection_key::KeyNumbers::new("b"));
  }
}
