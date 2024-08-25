#![feature(test)]
extern crate test;

mod common;
mod day01;
mod solver;

use solver::Solver;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Opt {
    All,
    Day { day: u8, part: Option<u8> },
    File { day: u8, part: u8, file: PathBuf },
}

fn main() {
    let opt = Opt::from_args();

    let mut solver = Solver::new();
    solver.add(1, 1, day01::part_one);
    solver.add(1, 2, day01::part_two);
    solver.solve(opt);
}
