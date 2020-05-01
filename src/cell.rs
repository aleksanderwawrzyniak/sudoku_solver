use super::bit_vec::BitSet;

use std::fmt;

pub type Cell = BitSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimpleCell {
    Const(u8),
    Var,
}

impl SimpleCell {
    pub fn new(val: u8) -> Self {
        if val == 0 {
            Self::Var
        } else {
            Self::Const(val)
        }
    }

    pub fn value(&self) -> &u8 {
        match self {
            Self::Var => &0,
            Self::Const(v) => v,
        }
    }

    pub fn empty(&self) -> bool {
        match self {
            Self::Var => true,
            _ => false,
        }
    }
}

impl fmt::Display for SimpleCell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}
