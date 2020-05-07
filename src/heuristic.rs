use crate::bitset::BitSet;

#[derive(Debug, Clone)]
pub enum Heuristic {
    Greedy,
    Random,
    Reverse,
    MostConstrainedVariable,
    LeastConstrainedVariable,
}

pub trait HeuristicDomainOperations {
    type Item;

    fn next(&self, heuristic: &Heuristic) -> Self::Item;
    fn remove(&mut self, v: u32);
}

impl HeuristicDomainOperations for BitSet {
    type Item = u32;

    fn next(&self, heuristic: &Heuristic) -> Self::Item {
        use Heuristic::*;

        match heuristic {
            Random => self.random(),
            Reverse => self.last(),
            _ => self.current(),
        }
    }

    fn remove(&mut self, v: u32) {
        self.remove(v);
    }
}
