//! Ordered keys that can be inserted between any two of them infinitely.
//! like **fractional indexes**, but using variable-length string with custom Ord implementation.
//!
//! Using a `[0, 64]` charset, total length: 3 + 10 + 26 + 26 = 65
//!
//! ```text
//! +-/0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz
//! ```
//!
//! 65 was picked since it's easier to bisect 0~64 at 32, and 0~32 at 16, etc.
//!
//! Notice that `a` equals `aT`, internally `[39]` equals `[39, 32]`.
//! and `a`(`aT`) is greater than `aS`, this is different from normal order of strings.

use std::cmp::{max, Ordering};
use std::fmt::Display;

const CHARSET: &str = "+-/0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

/// create it like:
/// ```rust
/// let _  = bisection_key::KeyNumbers::new("a");
/// ```
#[derive(Debug)]
pub struct KeyNumbers(Vec<u8>);

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

  pub fn bisect(&self, next: &Self) -> Result<Self, String> {
    let mut mid: Vec<u8> = vec![];

    // println!("bisecting {:?} and {:?}", self, next);

    let mut change: Option<NumberChange> = None;

    for i in 0..max(self.0.len(), next.0.len()) {
      let curr = self.0.get(i).or(Some(&32)).unwrap();
      let edge = next.0.get(i).or(Some(&32)).unwrap();

      let delta = *edge as i8 - *curr as i8;

      // println!(
      //   "i:{} curr:{} edge:{} delta:{} change:{:?} mid:{:?}",
      //   i, *curr, *edge, delta, change, mid
      // );

      match change {
        None => {
          if delta == 0 {
            mid.push(*curr);
          } else if delta >= 2 || delta <= -2 {
            mid.push((edge + curr) >> 1);
            return Self(mid).checked();
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
          // println!("edge:{} reach:{}", edge, reach);
          match edge.cmp(&reach) {
            Ordering::Greater => {
              if *edge == 1 {
                // which means current is 64, edge is 1
                mid.push(0);
                mid = promote_from(mid, i, NumberChange::Increased)?;
              } else {
                mid.push((edge - reach) >> 1);
                mid = promote_from(mid, i, NumberChange::Increased)?;
                return Self(mid).checked();
              }
            }
            Ordering::Equal => {
              mid.push(64);
              if reach == 0 {
                // need to bisect again
              } else {
                return Self(mid).checked();
              }
            }
            Ordering::Less => {
              if reach == 1 {
                mid.push(64);
              } else {
                mid.push((*curr + edge) >> 1);
              }
              return Self(mid).checked();
            }
          }
        }

        Some(NumberChange::Decreased) => {
          let reach = 64 - edge;
          match reach.cmp(curr) {
            Ordering::Less => {
              if *curr == 1 {
                // which means current is 1, edge is 64
                mid.push(0);
              } else {
                mid.push((*curr - reach) >> 1);
                return Self(mid).checked();
              }
            }
            Ordering::Equal => {
              mid.push(0);
              if reach == 0 {
                // need to bisect again
              } else {
                return Self(mid).checked();
              }
            }
            Ordering::Greater => {
              if reach == 1 {
                // which means current is 0, edge is 63
                mid.push(64);
                mid = promote_from(mid, i, NumberChange::Decreased)?;
              } else {
                mid.push((*curr + edge) >> 1);
              }
              return Self(mid).checked();
            }
          }
        }
      }
    }

    match change {
      None => Err(format!(
        "not found property way of generating middle key: {} {}",
        self, next
      )),
      Some(NumberChange::Increased) => {
        mid.push(32 + 2);
        Self(mid).checked()
      }
      Some(NumberChange::Decreased) => {
        mid.push(32 - 2);
        Self(mid).checked()
      }
    }
  }

  pub fn bisect_end(&self) -> Result<Self, String> {
    let mut ys: Vec<u8> = vec![];
    for item in &self.0 {
      if *item > 64 {
        return Err(format!("invalid key: {}", self));
      } else if *item == 64 {
        ys.push(64);
      } else if *item == 63 {
        ys.push(64);
        return Self(ys).checked();
      } else {
        // max 62
        ys.push(*item + 2);
        return Self(ys).checked();
      }
    }
    ys.push(32 + 2);
    Self(ys).checked()
  }

  pub fn bisect_beginning(&self) -> Result<Self, String> {
    let mut ys: Vec<u8> = vec![];
    for item in &self.0 {
      if *item > 64 {
        return Err(format!("invalid key: {}", self));
      } else if *item == 0 {
        ys.push(0);
      } else if *item == 1 {
        ys.push(0);
        return Self(ys).checked();
      } else {
        // min 2
        ys.push(*item - 2);
        return Self(ys).checked();
      }
    }
    ys.push(32 - 2);
    Self(ys).checked()
  }

  pub fn promote_from(&self, idx: usize, change: NumberChange) -> Result<Self, String> {
    Ok(Self(promote_from(self.0.to_owned(), idx, change)?))
  }

  pub fn checked(self) -> Result<Self, String> {
    // println!("checking {:?}", self);
    // check
    for i in 0..self.0.len() {
      if self.0[i] > 64 {
        return Err(format!(
          "invalid character for bisection key: {:?}",
          self.0[i]
        ));
      }
    }
    Ok(self)
  }
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum NumberChange {
  Increased,
  Decreased,
}

fn promote_from(base: Vec<u8>, origin_idx: usize, change: NumberChange) -> Result<Vec<u8>, String> {
  if origin_idx == 0 {
    return Err(format!("cannot promote from 0 of: {:?}", base));
  }
  let idx = origin_idx - 1;
  if idx >= base.len() {
    Err(format!("index out of range: {:?} {}", base, idx))
  } else {
    let mut xs = base.to_owned();
    let mut pos = idx;
    loop {
      if change == NumberChange::Decreased {
        if xs[pos] == 0 {
          xs[pos] = 64;
        } else {
          xs[pos] -= 1;
          break Ok(xs);
        }
      } else if xs[pos] == 64 {
        xs[pos] = 0;
      } else {
        xs[pos] += 1;
        break Ok(xs);
      }

      if pos > 0 {
        pos -= 1;
      } else {
        return Err(format!("not found position to promote: {:?}", base));
      }
    }
  }
}
