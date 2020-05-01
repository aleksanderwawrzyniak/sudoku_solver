use std::collections::HashSet;
use std::fmt;

pub enum Heuristic {
    Greedy,
    Random,
    MostConstrainingVariable,
    MostConstrainedVariable,
    LeastConstrainingVariable,
}

pub trait DomainOperations {
    type Item;

    fn next(&self) -> Self::Item;
    fn remove(&mut self, v: u32);
}

impl DomainOperations for HashSet<u32> {
    type Item = u32;

    fn next(&self) -> Self::Item {
        *self.iter().next().unwrap()
    }

    fn remove(&mut self, v: u32) {
        self.remove(&v);
    }
}

impl DomainOperations for BitSet {
    type Item = u32;

    fn next(&self) -> Self::Item {
        self.current()
    }

    fn remove(&mut self, v: u32) {
        self.remove(v)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BitSet {
    set: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct BitIter {
    current: BitSet,
}

impl BitSet {
    pub fn new() -> Self {
        Self { set: 0 }
    }

    pub fn insert(&mut self, n: u32) {
        self.set |= 1 << n as u64;
    }

    pub fn remove(&mut self, n: u32) {
        self.set &= !(1 << n as u64);
    }

    pub fn len(self) -> usize {
        self.set.count_ones() as usize
    }

    pub fn current(self) -> u32 {
        self.set.trailing_zeros() as u32
    }

    pub fn iter(self) -> BitIter {
        BitIter { current: self }
    }

    pub fn is_empty(self) -> bool {
        self.len() == 0
    }
}

impl Iterator for BitIter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_empty() {
            return None;
        }

        let next = self.current.next();
        self.current.remove(next);

        Some(next)
    }
}

impl fmt::Debug for BitSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // f.debug_struct("BitSet")
        //     .field("data", &format!("{:66b}", self.set))
        //     .finish()
        write!(f, "{}", self)
    }
}

impl fmt::Display for BitSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        s.push('{');
        let iter = self.iter();
        for x in iter {
            s.push_str(&format!("{},", x));
        }
        s.push('}');

        write!(f, "{}", s)
    }
}
