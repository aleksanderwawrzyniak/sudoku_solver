use crate::bitset::BitSet;
use crate::heuristic::Heuristic;

use std::slice::Iter;

use rand::prelude::*;

pub struct EmptySlots {
    slots: Vec<usize>,
    /// A vector working as a stack on which are pushed the taken slots
    taken_slots: Vec<(usize, usize)>,
    heuristic: Heuristic,
}

impl EmptySlots {
    pub fn set_heuristic(&mut self, heuristic: &Heuristic) {
        self.heuristic = heuristic.clone();
    }

    pub fn next(&mut self, domains: &[BitSet]) -> Option<usize> {
        use Heuristic::*;

        if self.slots.is_empty() {
            return None;
        }

        // dbg!(&domains);

        let slot = match self.heuristic {
            Greedy => {
                let slot = self.slots.remove(0);
                self.taken_slots.push((0, slot));
                slot
            }
            Reverse => {
                let len = self.slots.len();
                let slot = self.slots.remove(len - 1);
                self.taken_slots.push((len - 1, slot));
                slot
            }
            Random => {
                let mut rand = thread_rng();
                let pos = rand.gen_range(0, self.slots.len());
                let slot = self.slots.remove(pos);
                self.taken_slots.push((pos, slot));
                slot
            }
            MostConstrainedVariable => {
                // take the position with the least possible domain values
                let mut vec = self
                    .slots
                    .iter()
                    .enumerate()
                    .map(|(idx, &slot)| (idx, domains[slot].len()))
                    .filter(|&(_, len)| len != 0)
                    .collect::<Vec<_>>();
                vec.sort_unstable_by(|&(_, a), (_, b)| a.cmp(b));
                let (idx, _) = *vec.first()?;
                let slot = self.slots.remove(idx);
                self.taken_slots.push((idx, slot));

                // dbg!(slot);
                // dbg!(&self.taken_slots);
                // dbg!(domains[slot]);
                slot
            }
            LeastConstrainedVariable => {
                // take the position with the most possible domain values
                let mut vec = self
                    .slots
                    .iter()
                    .map(|&slot| domains[slot].len())
                    .enumerate()
                    .collect::<Vec<_>>();
                vec.sort_unstable_by(|(_, a), &(_, b)| b.cmp(a));
                let (idx, _) = *vec.first()?;
                let slot = self.slots.remove(idx);
                self.taken_slots.push((idx, slot));

                // dbg!(domains[slot]);
                slot
            }
        };
        // dbg!(slot);

        Some(slot)
    }

    pub fn backtrack(&mut self) -> Option<usize> {
        // dbg!(&self.taken_slots);
        if self.taken_slots.is_empty() {
            return None;
        }

        // remove the current one from the stack
        let (idx, slot) = self.taken_slots.pop().unwrap();
        self.slots.insert(idx, slot);

        // go back to the previous value
        let (idx, slot) = self.taken_slots.pop()?;
        self.slots.insert(idx, slot);

        Some(slot)
    }
}

impl From<Vec<usize>> for EmptySlots {
    fn from(vec: Vec<usize>) -> Self {
        Self {
            taken_slots: Vec::with_capacity(vec.len()),
            slots: vec,
            heuristic: Heuristic::Greedy,
        }
    }
}

impl From<Iter<'_, usize>> for EmptySlots {
    fn from(iter: Iter<usize>) -> Self {
        Self {
            taken_slots: Vec::with_capacity(iter.len()),
            slots: iter.copied().collect(),
            heuristic: Heuristic::Greedy,
        }
    }
}
