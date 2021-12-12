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
//! Generated key matches lexiongraphic order.

use std::cmp::{max, Ordering};
use std::fmt::Display;

const CHARSET: &str = "+-/0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

/// create it like:
/// ```rust
/// let _  = bisection_key::LexiconKey::new("a");
/// ```
#[derive(Debug)]
pub struct LexiconKey(Vec<u8>);

impl Eq for LexiconKey {}

/// missing length are filled with `32`s, then compare like a vector
impl PartialEq for LexiconKey {
  fn eq(&self, other: &LexiconKey) -> bool {
    let xs = &self.0;
    let ys = &other.0;
    let size = max(xs.len(), ys.len());
    for idx in 0..size {
      let x = u8_or_neg(xs.get(idx));
      let y = u8_or_neg(ys.get(idx));
      if x != y {
        return false;
      }
    }
    true
  }
}

/// missing length are filled with `32`s, then compare like a vector
impl Ord for LexiconKey {
  fn cmp(&self, other: &LexiconKey) -> Ordering {
    let xs = &self.0;
    let ys = &other.0;
    let size = max(xs.len(), ys.len());
    for idx in 0..size {
      let x = u8_or_neg(xs.get(idx));
      let y = u8_or_neg(ys.get(idx));
      match x.cmp(&y) {
        Ordering::Equal => continue,
        x => return x,
      }
    }
    Ordering::Equal
  }
}

impl PartialOrd for LexiconKey {
  fn partial_cmp(&self, other: &LexiconKey) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Display for LexiconKey {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let mut buf: String = String::new();
    for i in &self.0 {
      buf.push(CHARSET.chars().nth(*i as usize).unwrap());
    }
    write!(f, "{}", buf)
  }
}

impl LexiconKey {
  pub fn new(s: &str) -> Result<Self, String> {
    let mut buf: Vec<u8> = vec![];
    for c in s.chars() {
      match CHARSET.find(c) {
        Some(i) => buf.push(i as u8),
        None => return Err(format!("invalid character for bisection key: {:?}", c)),
      }
    }
    Ok(LexiconKey(buf))
  }

  pub fn bisect(&self, next: &Self) -> Result<Self, String> {
    let mut mid: Vec<u8> = vec![];

    // println!("bisecting {:?} and {:?}", self, next);

    let mut change: Option<NumberChange> = None;

    for i in 0..max(self.0.len(), next.0.len()) {
      let curr = self.0.get(i).or(Some(&0)).unwrap();
      let edge = next.0.get(i).or(Some(&0)).unwrap();

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
        // leave some spaces: 0 1 2 3
        mid.push(4);
        Self(mid).checked()
      }
      Some(NumberChange::Decreased) => {
        // leave some spaces: 61 62 63 64
        mid.push(60);
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
        ys.push(63);
        // add some space here: "0 1 2 3"
        ys.push(4);
        return Self(ys).checked();
      } else if *item == 62 {
        ys.push(63);
        return Self(ys).checked();
      } else {
        // max 61
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
        // leave some space here: "61 62 63 64"
        ys.push(60);
        return Self(ys).checked();
      } else if *item == 2 {
        ys.push(1);
        return Self(ys).checked();
      } else {
        // min 2
        ys.push(*item - 2);
        return Self(ys).checked();
      }
    }
    Err(format!(
      "trailing 0 is invalid during bisect_beggining: {}",
      self
    ))
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

/// -1 for None
fn u8_or_neg(x: Option<&u8>) -> i64 {
  match x {
    Some(x) => *x as i64,
    None => -1,
  }
}
