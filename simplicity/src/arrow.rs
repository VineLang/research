use std::fmt::{self, Debug};

use util::graph::Edge;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Arrow(pub u8);

impl Arrow {
  pub fn from_bits(bits: u8) -> Self {
    Self(bits)
  }

  pub fn join(self, other: Self) -> Option<Self> {
    if self.0 == 0 || other.0 == 0 {
      return None;
    }
    let table = [
      (0b00001, 0b00001, 0b00000),
      (0b00010, 0b00001, 0b00011),
      (0b00010, 0b00010, 0b11011),
      (0b00100, 0b00001, 0b11100),
      (0b00100, 0b00010, 0b11100),
      (0b00100, 0b00100, 0b11111),
      (0b01000, 0b00001, 0b01000),
      (0b01000, 0b00010, 0b01000),
      (0b01000, 0b00100, 0b00100),
      (0b01000, 0b01000, 0b11111),
      (0b10000, 0b00001, 0b01000),
      (0b10000, 0b00010, 0b01000),
      (0b10000, 0b00100, 0b00100),
      (0b10000, 0b01000, 0b00111),
      (0b10000, 0b10000, 0b00100),
    ];
    let mut o = 0;
    for (a, b, c) in table {
      let b = Arrow(b).converse().unwrap().0;
      if (self.0 & a) != 0 && (other.0 & b) != 0 {
        o |= c as u8
      }
      let a = Arrow(a).converse().unwrap().0;
      let b = Arrow(b).converse().unwrap().0;
      let (a, b) = (b, a);
      let c = Arrow(c).converse().unwrap().0;
      if (self.0 & a) != 0 && (other.0 & b) != 0 {
        o |= c as u8
      }
    }
    if o == 0b11111 {
      None
    } else {
      Some(Self::from_bits(o))
    }
  }
}

impl Edge for Arrow {
  fn converse(self) -> Option<Self> {
    Some(Arrow::from_bits(self.0.reverse_bits() >> 3))
  }

  fn merge(self, other: Self) -> Self {
    Arrow::from_bits(self.0 & other.0)
  }
}

impl Debug for Arrow {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}{}{}{}{}",
      if self.0 & 0b10000 != 0 { '<' } else { '-' },
      if self.0 & 0b01000 != 0 { '<' } else { '-' },
      if self.0 & 0b00100 != 0 { '*' } else { '-' },
      if self.0 & 0b00010 != 0 { '>' } else { '-' },
      if self.0 & 0b00001 != 0 { '>' } else { '-' },
    )
  }
}

#[test]
fn contrapositive() {
  for a in (0..32).map(Arrow) {
    for b in (0..32).map(Arrow) {
      if let Some(c) = a.join(b) {
        assert!(
          b.join(Arrow(c.0 ^ 31).converse().unwrap())
            .unwrap_or(Arrow(31))
            .converse()
            .unwrap()
            .merge(a)
            == Arrow(0),
          "{a:?} {b:?} {c:?}"
        );
      }
    }
  }
}
