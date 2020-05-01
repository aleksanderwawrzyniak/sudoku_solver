use super::bit_vec::BitSet;

use crate::bit_vec::DomainOperations;
use std::fmt;
use std::iter::FromIterator;

pub type Domain = BitSet;

static SOLVED_CHARACTER: char = '\u{2714}';
static UNSOLVED_CHARACTER: char = '\u{2718}';

#[derive(Debug, Clone)]
pub struct Sudoku {
    pub board: Vec<u32>,
    pub domains: Vec<Domain>,
    pub empty_slots: Vec<usize>,
    pub board_coords: Vec<(usize, usize)>,
    pub board_rows: Vec<Vec<usize>>,
    pub board_cols: Vec<Vec<usize>>,
    pub board_squares: Vec<Vec<usize>>,
    pub domain_size: usize,
    pub domain: Domain,
}

impl Sudoku {
    pub fn from_flattened(s: &str) -> Self {
        let v: Vec<(usize, u32)> = s
            .chars()
            .enumerate()
            .map(|(i, c)| (i, c.to_digit(36).unwrap_or_default()))
            .collect();

        Self::from_iter(v)
    }
    pub fn print_board(&self) {
        let s: String = self
            .board_rows
            .iter()
            .map(|row| {
                let mut line = String::new();
                row.iter()
                    .for_each(|idx| line.push_str(&format!(" {}", self.board[*idx])));
                line.push('\n');

                line
            })
            .collect();
        println!("{}", s);
    }

    // given coordinates, returns actual index in the array
    pub fn index(row: usize, col: usize) -> usize {
        row * 9 + col
    }

    // given coordinates, return block number
    pub fn square(row: usize, col: usize) -> usize {
        let r = row / 3;
        let c = col / 3;
        r * 3 + c
    }

    // given coordinates, return index inside its block
    pub fn index_in_block(row: usize, col: usize) -> usize {
        let r = row % 3;
        let c = col % 3;
        r * 3 + c
    }

    pub fn get_number(&self, row: usize, col: usize) -> u32 {
        let index = Self::index(row, col);
        self.board[index]
    }

    pub fn set_number(&mut self, row: usize, col: usize, value: u32) -> bool {
        let index = Self::index(row, col);
        self.board[index] = value;
        true
    }

    pub fn is_valid(&self, row: usize, col: usize, value: u32) -> bool {
        let index = Self::index(row, col);
        // C1 - there cannot be any duplicate in the row
        for i in &self.board_rows[row] {
            if i == &index {
                continue;
            }
            if value == self.board[*i] {
                return false;
            }
        }

        // C2 - there cannot be any duplicate in the column
        for i in &self.board_cols[col] {
            if i == &index {
                continue;
            }
            if value == self.board[*i] {
                return false;
            }
        }

        // C3 - There cannot be any duplicate in the square
        let square = Self::square(row, col);
        for i in &self.board_squares[square] {
            if i == &index {
                continue;
            }
            if value == self.board[*i] {
                return false;
            }
        }
        true
    }

    pub fn find_solution(&self, row: usize, col: usize) -> Option<u32> {
        let current_number = self.get_number(row, col);
        // all possibilities failed
        if current_number == 9 {
            return None;
        }
        let start = current_number + 1;
        for i in start..10 {
            if self.is_valid(row, col, i) {
                return Some(i);
            }
        }
        None
    }

    pub fn solve(&mut self) -> (char, u64) {
        let mut i = 0;
        let mut backtrack_counter = 0u64;
        while i < self.empty_slots.len() {
            // println!("================================");
            // self.print_board();
            let (row, col) = self.board_coords[self.empty_slots[i]];
            match self.find_solution(row, col) {
                Some(solution) => {
                    self.set_number(row, col, solution);
                    i += 1;
                }
                None => {
                    // no solution found, reset cell
                    self.set_number(row, col, 0);
                    backtrack_counter += 1;
                    match i.checked_sub(1) {
                        Some(j) => i = j,
                        // TODO: Do not panic here, as it will kill the whole program later
                        None => {
                            println!("No Solution");
                            return (UNSOLVED_CHARACTER, backtrack_counter);
                        }
                    }
                }
            }
        }

        // if self.solved() {
        //     println!("SOLVED");
        //     println!("{}", backtrack_counter);
        //     self.print_board();
        //     println!("\n{}", self);
        // } else {
        //     println!("NOT SOLVED");
        //     println!("{}", backtrack_counter);
        //     self.print_board();
        //     println!("\n{}", self);
        // }
        //
        (SOLVED_CHARACTER, backtrack_counter)
    }

    fn backtrack(
        &mut self,
        version: (Vec<u32>, Vec<Domain>, usize),
        value: u32,
    ) -> Result<usize, usize> {
        self.board = version.0;
        self.domains = version.1;
        self.board[self.empty_slots[version.2]] = 0;
        self.domains[self.empty_slots[version.2]].remove(value);
        if self.domains[self.empty_slots[version.2]].is_empty() {
            return Err(version.2);
        }

        Ok(version.2)
    }

    pub fn solve_fc(&mut self) -> (char, u64) {
        let mut i: usize = 0;
        let mut backtrack_counter = 0u64;
        let mut versions: Vec<(Vec<u32>, Vec<Domain>, usize)> = Vec::new();
        while i < self.empty_slots.len() {
            versions.push((self.board.clone(), self.domains.clone(), i));
            let (row, col) = self.board_coords[self.empty_slots[i]];
            // if value of cell in the index is not 0, then it was previously set during forward
            // checking.
            if self.board[Self::index(row, col)] != 0 {
                i += 1;
                continue;
            }

            // take the next value in the
            match self.domains[self.empty_slots[i]].iter().next() {
                Some(solution) => {
                    self.set_number(row, col, solution);
                    if self.try_update_domains().is_err() {
                        backtrack_counter += 1;
                        let previous_version = versions.pop().unwrap();
                        match self.backtrack(previous_version, solution) {
                            Ok(_) => {}
                            Err(_) => match versions.pop() {
                                Some((previous_board, previous_domains, previous_i)) => match self
                                    .backtrack(
                                        (previous_board, previous_domains, previous_i),
                                        self.board[self.empty_slots[previous_i]],
                                    ) {
                                    Ok(prev_i) => {
                                        i = prev_i;
                                    }
                                    Err(prev_i) => {
                                        i = prev_i;
                                    }
                                },
                                None => {
                                    println!("No Solution");
                                    return (UNSOLVED_CHARACTER, backtrack_counter);
                                }
                            },
                        }
                    } else {
                        i += 1;
                    }
                }
                None => {
                    backtrack_counter += 1;
                    versions.pop();
                    match versions.pop() {
                        Some((previous_board, previous_domains, previous_i)) => {
                            match self.backtrack(
                                (previous_board, previous_domains, previous_i),
                                self.board[self.empty_slots[previous_i]],
                            ) {
                                Ok(prev_i) => {
                                    i = prev_i;
                                }
                                Err(prev_i) => i = prev_i,
                            }
                        }
                        None => {
                            println!("No path back");
                            println!("No Solution");
                            return (UNSOLVED_CHARACTER, backtrack_counter);
                        }
                    }
                }
            }
        }

        // if self.solved() {
        //     println!("SOLVED");
        //     println!("{}", backtrack_counter);
        //     self.print_board();
        //     println!("\n{}", self);
        // } else {
        //     println!("NOT SOLVED");
        //     println!("{}", backtrack_counter);
        //     self.print_board();
        //     println!("\n{}", self);
        // }

        (SOLVED_CHARACTER, backtrack_counter)
    }

    fn try_update_domains(&mut self) -> Result<(), ()> {
        (0..self.board.len())
            .map(|idx| self.try_update_domain(idx))
            .collect::<Result<(), ()>>()?;

        // if there was some value changed, try to update domains once more, if not,
        // there's no need to check it
        if (0..self.board.len())
            .filter_map(|idx| self.try_update_value(idx))
            .next()
            .is_some()
        {
            (0..self.board.len())
                .map(|idx| self.try_update_domain(idx))
                .collect::<Result<(), ()>>()?;
            // self.try_update_domains()?;
        }

        Ok(())
    }

    fn try_update_domain(&mut self, idx: usize) -> Result<(), ()> {
        let (row, col) = self.board_coords[idx];
        let square = Self::square(row, col);
        let mut domain = self.domains[idx];

        self.update_domain(&mut domain, idx, self.board_rows[row].iter())?;
        self.update_domain(&mut domain, idx, self.board_cols[col].iter())?;
        self.update_domain(&mut domain, idx, self.board_squares[square].iter())?;

        self.domains[idx] = domain;

        Ok(())
    }

    fn update_domain<'a, I>(&self, domain: &mut Domain, index: usize, iter: I) -> Result<(), ()>
    where
        I: Iterator<Item = &'a usize>,
    {
        iter.for_each(|&idx| {
            if index != idx {
                domain.remove(self.board[idx]);
            }
        });

        // if no value can be placed, return error, since it is not a good solution
        if domain.is_empty() {
            Err(())
        } else {
            Ok(())
        }
    }

    fn try_update_value(&mut self, idx: usize) -> Option<()> {
        let value = self.domains[idx].next();
        if self.domains[idx].len() == 1 && value != self.board[idx] {
            self.board[idx] = value;
            Some(())
        } else {
            None
        }
    }

    pub fn solved(&self) -> bool {
        for x in 0..self.board_rows.len() {
            for y in 0..self.board_cols.len() {
                if !self.is_valid(x, y, self.get_number(x, y)) {
                    return false;
                }
            }
        }

        true
    }

    pub fn apply_domain(&mut self, domain: Domain) -> &mut Self {
        self.domain = domain;

        self.domains = self
            .board
            .iter()
            .enumerate()
            .map(|(idx, &value)| {
                if value != 0 {
                    let mut domain = Domain::new();
                    domain.insert(value);

                    domain
                } else {
                    self.find_domain(idx)
                }
            })
            .collect();

        self
    }

    fn find_domain(&self, idx: usize) -> Domain {
        // start with full domain
        let mut domain = self.domain;
        let (row, col) = self.board_coords[idx];

        for &val in self
            .get_values(&self.board_rows[row])
            .iter()
            .filter(|v| v != &&0)
        {
            domain.remove(val);
            // println!("removing {} from domain\nnow it is: {:?}", val, &domain);
        }

        for &val in self
            .get_values(&self.board_cols[col])
            .iter()
            .filter(|v| v != &&0)
        {
            domain.remove(val);
            // println!("removing {} from domain\nnow it is: {:?}", val, &domain);
        }

        let square = Self::square(row, col);
        for &val in self
            .get_values(&self.board_squares[square])
            .iter()
            .filter(|v| v != &&0)
        {
            domain.remove(val);
            // println!("removing {} from domain\nnow it is: {:?}", val, &domain);
        }

        domain
    }

    fn get_values(&self, v: &[usize]) -> Vec<u32> {
        v.iter().map(|&idx| self.board[idx]).collect()
    }
}

impl FromIterator<(usize, u32)> for Sudoku {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (usize, u32)>,
    {
        let mut empty_slots: Vec<usize> = Vec::new();
        // 0 means, that the cell is empty
        let board: Vec<u32> = iter
            .into_iter()
            .map(|(idx, val)| {
                if val == 0 {
                    empty_slots.push(idx);
                }
                val
            })
            .collect();

        // the domain size is a square root of the length of the board
        let domain_size = (board.len() as f32).sqrt().floor() as usize;

        let board_coords: Vec<(usize, usize)> = {
            fn coord(index: usize) -> (usize, usize) {
                let row = index / 9;
                let col = index % 9;
                (row, col)
            }
            (0..board.len()).map(coord).collect()
        };

        // what indices are in each col?
        let board_rows: Vec<Vec<usize>> = (0..domain_size)
            .map(|x| {
                (0..domain_size)
                    .map(|y| Self::index(x, y))
                    .collect::<Vec<usize>>()
            })
            .collect();
        // what indices are in each col?
        let board_cols: Vec<Vec<usize>> = (0..domain_size)
            .map(|x| {
                (0..domain_size)
                    .map(|y| Self::index(y, x))
                    .collect::<Vec<usize>>()
            })
            .collect();

        let board_squares = {
            let mut map = vec![vec![0usize; 9]; 9];
            for k in 0..9 {
                for i in 0..9 {
                    let index = Self::index(k, i);
                    let block = Self::square(k, i);
                    let index_in_block = Self::index_in_block(k, i);
                    map[block][index_in_block] = index;
                }
            }
            (0..domain_size).for_each(|x| {
                (0..domain_size).for_each(|y| {
                    map[Self::square(x, y)][Self::index_in_block(x, y)] = Self::index(x, y);
                })
            });
            map
        };

        let domains = board.iter().map(|_| Domain::new()).collect();

        Self {
            board,
            domains,
            empty_slots,
            board_rows,
            board_cols,
            board_coords,
            board_squares,
            domain_size,
            domain: Domain::new(),
        }
    }
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.board.iter().map(|v| v.to_string()).collect::<String>()
        )
    }
}
