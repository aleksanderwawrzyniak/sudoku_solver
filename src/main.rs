mod bitset;
mod board;
mod heuristic;
mod opt;

use rayon::prelude::*;
use structopt::StructOpt;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

use crate::bitset::BitSet;
use board::Sudoku;
use heuristic::Heuristic;
use opt::Opt;

fn main() {
    let opt = Opt::from_args();
    dbg!(&opt);
    println!();

    // let domain: std::collections::HashSet<u32> = (1..10).collect();
    let domain = {
        let mut domain = BitSet::new();
        (1..10).for_each(|val| domain.insert(val));
        domain
    };

    match opt {
        Opt::Load { nth } => {
            let f = File::open("sudoku.csv").unwrap();
            let reader = BufReader::new(f);
            let line = reader.lines().nth(nth as usize).unwrap().unwrap();
            let flat_line = line.split(';').nth(2).unwrap();

            let mut board = Sudoku::from_flattened(flat_line);
            board.print_board(100);
            board.apply_domain(domain);
        }
        Opt::Solve { nth } => {
            let f = File::open("sudoku.csv").unwrap();
            let reader = BufReader::new(f);
            match nth {
                Some(n) => {
                    let line = reader.lines().nth(n as usize).unwrap().unwrap();
                    let (now, result, backtrack_counter, board) = run_solve(line, domain);

                    println!(
                        "{} | {} | {} | {}| {}\n",
                        n, now, result, backtrack_counter, board
                    )
                }
                None => {
                    let results = reader
                        .lines()
                        .collect::<Vec<_>>()
                        .into_par_iter()
                        .enumerate()
                        .map(|(idx, line)| {
                            let line = line.unwrap();
                            // let line = line.split(';').nth(2).unwrap();
                            // let mut board = Sudoku::from_flattened(line);
                            // board.apply_domain(domain);
                            //
                            // let now = Instant::now();
                            // let backtrack_counter = board.solve_fc();
                            // let now = now.elapsed().as_secs_f64();
                            // let board = format!("{}", board);

                            let (now, result, backtrack_counter, board) = run_solve(line, domain);

                            (idx, now, result, backtrack_counter, board)
                        })
                        .collect::<Vec<(usize, f64, char, u64, String)>>();

                    results
                        .into_iter()
                        .for_each(|(idx, now, result, backtrack_counter, board)| {
                            println!(
                                "{} | {} | {} | {}| {}\n",
                                idx, now, result, backtrack_counter, board
                            )
                        });
                }
            }
        }
        Opt::SolveFc { nth } => {
            let f = File::open("sudoku.csv").unwrap();
            let reader = BufReader::new(f);
            match nth {
                Some(n) => {
                    let line = reader.lines().nth(n as usize).unwrap().unwrap();
                    let (now, result, backtrack_counter, board) = run_solve_fc(line, domain);

                    println!(
                        "{} | {} | {} | {}| {}\n",
                        n, now, result, backtrack_counter, board
                    )
                }
                None => {
                    let results = reader
                        .lines()
                        .collect::<Vec<_>>()
                        .into_par_iter()
                        .enumerate()
                        .map(|(idx, line)| {
                            let line = line.unwrap();
                            let line = line.split(';').nth(2).unwrap();
                            let mut board = Sudoku::from_flattened(line);
                            board.apply_domain(domain);
                            let heuristic = Heuristic::Greedy;

                            let now = Instant::now();
                            let (result, backtrack_counter) = board.solve_fc(&heuristic);
                            let now = now.elapsed().as_secs_f64();
                            let board = format!("{}", board);

                            (idx, now, result, backtrack_counter, board)
                        })
                        .collect::<Vec<(usize, f64, char, u64, String)>>();

                    results
                        .into_iter()
                        .for_each(|(idx, now, result, backtrack_counter, board)| {
                            println!(
                                "{} | {} | {} | {}| {}\n",
                                idx, now, result, backtrack_counter, board
                            )
                        });
                }
            }
        }
    }
}

fn run_solve(line: String, domain: BitSet) -> (f64, char, u64, String) {
    let line = line.split(';').nth(2).unwrap();
    let mut board = Sudoku::from_flattened(line);
    board.apply_domain(domain);
    dbg!(&board.domains);
    let v_heuristic = Heuristic::Random;
    let s_heuristic = Heuristic::LeastConstrainedVariable;

    let now = Instant::now();
    let (result, backtrack_counter) = board.solve(&v_heuristic, &s_heuristic);
    let now = now.elapsed().as_secs_f64();
    let board = format!("{}", board);

    (now, result, backtrack_counter, board)
}

fn run_solve_fc(line: String, domain: BitSet) -> (f64, char, u64, String) {
    let line = line.split(';').nth(2).unwrap();
    let mut board = Sudoku::from_flattened(line);
    board.apply_domain(domain);
    let heuristic = Heuristic::Greedy;

    let now = Instant::now();
    let (result, backtrack_counter) = board.solve_fc(&heuristic);
    let now = now.elapsed().as_secs_f64();
    let board = format!("{}", board);

    (now, result, backtrack_counter, board)
}
