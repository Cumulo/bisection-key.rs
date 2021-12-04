use std::cmp::{max, Ordering};
use std::fmt::Display;

const CHARSET: &str = "+-/0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

#[derive(Debug)]
pub struct KeyNumbers(Vec<u8>);

/// generate bisected new key
pub fn bisect_key(low: &str, high: &str) -> String {
  "".to_string()
}

impl Eq for KeyNumbers {}

impl PartialEq for KeyNumbers {
  fn eq(&self, other: &KeyNumbers) -> bool {
    self.0 == other.0
  }
}

impl Ord for KeyNumbers {
  fn cmp(&self, other: &KeyNumbers) -> Ordering {
    self.0.cmp(&other.0)
  }
}

impl PartialOrd for KeyNumbers {
  fn partial_cmp(&self, other: &KeyNumbers) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Display for KeyNumbers {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let mut buf: String = String::new();
    for i in &self.0 {
      buf.push(CHARSET.chars().nth(*i as usize).unwrap());
    }
    write!(f, "{}", buf)
  }
}

impl KeyNumbers {
  pub fn new(s: &str) -> Result<Self, String> {
    let mut buf: Vec<u8> = vec![];
    for c in s.chars() {
      match CHARSET.find(c) {
        Some(i) => buf.push(i as u8),
        None => return Err(format!("invalid character for bisection key: {:?}", c)),
      }
    }
    Ok(KeyNumbers(buf))
  }

  pub fn strip_last_mut(&mut self) {
    while !self.0.is_empty() && self.0[self.0.len() - 1] == 32 {
      self.0.pop();
    }
  }

  // if last element is `T`, they can safely be removed to shorten the key
  pub fn strip_last(&self) -> Self {
    let mut xs = self.0.to_owned();
    while !xs.is_empty() && xs[xs.len() - 1] == 32 {
      xs.pop();
    }
    KeyNumbers(xs.to_owned())
  }

  pub fn bisect(&self, next: Self) -> Result<Vec<u8>, String> {
    let mut mid: Vec<u8> = vec![];

    let mut change: Option<NumberChange> = None;

    for i in 0..max(self.0.len(), next.0.len()) {
      let curr = self.0.get(i).or(Some(&32)).unwrap();
      let edge = next.0.get(i).or(Some(&32)).unwrap();

      let delta = *edge as i8 - *curr as i8;

      match change {
        None => {
          if delta == 0 {
            mid.push(*curr);
          } else if delta >= 2 || delta <= -2 {
            mid.push((edge + curr) >> 1);
            return Ok(mid);
          } else if delta == 1 {
            change = Some(NumberChange::Increased);
            mid.push(curr.to_owned());
          } else if delta == -1 {
            change = Some(NumberChange::Decreased);
            mid.push(curr.to_owned());
          } else {
            unreachable!("call cases are exhaustive");
          }
        }
        Some(NumberChange::Increased) => {
          let reach = 64 - *curr;
          match reach.cmp(edge) {
            Ordering::Less => {
              mid.insert(mid.len() - 1, mid[mid.len() - 1] + 1);
              mid.push((edge - reach) >> 1);
              return Ok(mid);
            }
            Ordering::Equal => {
              mid.push(64);
              return Ok(mid);
            }
            Ordering::Greater => {
              mid.push((*curr + edge) >> 1);
            }
          }
        }
        Some(NumberChange::Decreased) => {}
      }
    }

    Err(format!(
      "not found property way of generating middle key: {} {}",
      self, next
    ))
  }
}

enum NumberChange {
  Increased,
  Decreased,
}
