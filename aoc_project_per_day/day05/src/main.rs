#![feature(test)]
extern crate test;

use common::anyhow::Result;
use common::{regx, Answer};
use std::collections::{HashSet, VecDeque};
use std::io;

pub fn main() -> Result<()> {
    let stdin = io::read_to_string(io::stdin())?;
    println!("part1: {}", part_one(&stdin)?);
    println!("part2: {}", part_two(&stdin)?);
    Ok(())
}

pub fn part_one(input: &str) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_one(&input)
}

pub fn part_two(input: &str) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_two(&input)
}

// dest range start, source range start, range length
// e.g. 50 98 2 -> 98, 99 -> 50, 51

#[derive(Debug)]
struct Input {
    initial_seeds: Vec<usize>,
    mappings: Vec<Mapping>,
}

impl Input {
    fn map_to_loc(&self, seed_range: (usize, usize), two: bool) -> usize {
        let mut current_range = seed_range;
        let mut last_range = current_range;
        // print!("{:?}->", current_range);
        for mapping in self.mappings.iter() {
            last_range = current_range;
            current_range = self.map_range_tuple(current_range, mapping);
            // print!("{:?}->", current_range);
        }
        // println!();
        if two {
            std::cmp::min(last_range.0, current_range.0)
        } else {
            current_range.0
        }
    }
    fn map_to_multiloc(&self, seed_range: (usize, usize)) -> usize {
        let mut current_ranges = vec![seed_range];
        for mapping in self.mappings.iter() {
            let mut next_ranges = Vec::new();
            for range in current_ranges.iter() {
                next_ranges.append(&mut self.multi_mappings(*range, mapping));
            }
            current_ranges = next_ranges;
        }
        current_ranges.iter().map(|&(a, _)| a).min().unwrap()
    }
    fn multi_mappings(&self, seed_range: (usize, usize), mapping: &Mapping) -> Vec<(usize, usize)> {
        let (_start, len) = seed_range;
        let mut result: HashSet<(usize, usize)> = HashSet::new();
        let mut not_mapped = VecDeque::new();
        not_mapped.push_back(seed_range);
        while let Some(to_map) = not_mapped.pop_front() {
            let mut found = false;
            for r in mapping.ranges.iter() {
                if let Some(i) = self.intersect_ranges(to_map, (r.src, r.len)) {
                    // three things to push, before, inter, after
                    let offset = i.0 - r.src;
                    result.insert((r.dst + offset, i.1)); // this one is mapped
                    let before_start = to_map.0;
                    let before_len = i.0 - to_map.0;
                    if before_len > 0 {
                        not_mapped.push_back((before_start, before_len)) // before intersection
                    }
                    let after_start = i.0 + i.1;
                    let after_len = to_map.0 + to_map.1 - after_start;
                    if after_len > 0 {
                        not_mapped.push_back((after_start, after_len));
                    }
                    found = true
                }
            }
            if !found {
                result.insert(to_map);
            }
        }
        // println!("{:?}", seed_range);
        // println!("{:?}", result);
        let result: Vec<(usize, usize)> = result.into_iter().collect();
        assert_eq!(len, result.iter().map(|(_, l)| l).sum());
        result
    }
    fn map_range_tuple(&self, rt: (usize, usize), mapping: &Mapping) -> (usize, usize) {
        let mut lowest_found = (usize::MAX, 1);
        for r in mapping.ranges.iter() {
            if let Some(i) = self.intersect_ranges(rt, (r.src, r.len)) {
                //assert_eq!(i.1, 1); // thats messed up I know
                if r.dst < lowest_found.0 {
                    let offset = i.0 - r.src;
                    lowest_found = (r.dst + offset, i.1);
                }
            }
        }
        if lowest_found.0 == usize::MAX {
            rt
        } else {
            lowest_found
        }
    }
    fn intersect_ranges(&self, r1: (usize, usize), r2: (usize, usize)) -> Option<(usize, usize)> {
        let (mut r1_start, mut r1_len) = r1;
        let (mut r2_start, mut r2_len) = r2;
        if r1_start > r2_start {
            std::mem::swap(&mut r1_start, &mut r2_start);
            std::mem::swap(&mut r1_len, &mut r2_len);
        }
        if r2_start >= r1_start && r2_start < r1_start + r1_len {
            let start = std::cmp::max(r1_start, r2_start);
            let end = std::cmp::min(r1_start + r1_len, r2_start + r2_len);
            let len = end - start;
            Some((start, len))
        } else {
            None
        }
    }
    fn find_lowest(&self) -> usize {
        let mut lowest = usize::MAX;
        for seed in self.initial_seeds.iter() {
            let mapped = self.map_to_loc((*seed, 1), false);
            // println!("{} -> {}", seed, mapped);
            lowest = std::cmp::min(lowest, mapped);
        }
        lowest
    }
    fn find_lowest_ranges(&self) -> usize {
        let mut lowest = usize::MAX;
        for seed_range in self.initial_seeds.chunks(2) {
            let seed_start = seed_range[0];
            let seed_len = seed_range[1];
            let mapped = self.map_to_multiloc((seed_start, seed_len));
            //println!("{} -> {}", seed, mapped);
            lowest = std::cmp::min(lowest, mapped);
        }
        lowest
    }
}

#[derive(Debug)]
struct Mapping {
    ranges: Vec<RangeMapping>,
}

#[derive(Debug)]
struct RangeMapping {
    src: usize,
    dst: usize,
    len: usize,
}

fn parse_input(input: &str) -> Result<Input> {
    let blocks: Vec<&str> = input.split("\r\n\r\n").collect();
    // println!("{:?}", foo);
    let num_re = regx!(r"\d+");
    let initial_seeds = num_re
        .find_iter(blocks[0])
        .map(|m| m.as_str().parse().unwrap())
        .collect();
    let mut mappings = Vec::new();
    for i in 1..blocks.len() {
        let mut range_mappings = Vec::new();
        let lines = blocks[i].lines();
        for line in lines.skip(1) {
            let range_mapping: Vec<usize> = num_re
                .find_iter(line)
                .map(|m| m.as_str().parse().unwrap())
                .collect();

            range_mappings.push(RangeMapping {
                src: range_mapping[1],
                dst: range_mapping[0],
                len: range_mapping[2],
            });
        }
        mappings.push(Mapping {
            ranges: range_mappings,
        });
    }
    Ok(Input {
        initial_seeds,
        mappings,
    })
}

fn solve_one(input: &Input) -> Result<Answer> {
    // println!("{:?}", input.initial_seeds);
    // println!("{:?}", input.mappings);
    Ok(Answer::Num(input.find_lowest() as i128))
}

fn solve_two(input: &Input) -> Result<Answer> {
    Ok(Answer::Num(input.find_lowest_ranges() as i128))
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::test_utils::*;
    use std::sync::LazyLock;
    use test::Bencher;

    static TEST: LazyLock<String> = local_file!("test.txt");
    static INPUT: LazyLock<String> = local_file!("input.txt");

    #[test]
    fn test_one() -> Result<()> {
        let answer = super::part_one(&TEST)?;
        assert_eq!(answer, Answer::Num(35));
        Ok(())
    }
    #[test]
    fn part_one() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(26273516));
        Ok(())
    }
    #[test]
    fn test_two() -> Result<()> {
        let answer = super::part_two(&TEST)?;
        assert_eq!(answer, Answer::Num(46));
        Ok(())
    }
    #[test]
    fn part_two() -> Result<()> {
        let answer = super::part_two(&INPUT)?;
        assert_eq!(answer, Answer::Num(34039469));
        Ok(())
    }

    #[bench]
    fn bench_part_one(b: &mut Bencher) {
        b.iter(|| part_one())
    }
    #[bench]
    fn bench_part_two(b: &mut Bencher) {
        b.iter(|| part_two())
    }
    #[bench]
    fn bench_solve_two(b: &mut Bencher) {
        let input = parse_input(&INPUT).unwrap();
        b.iter(|| solve_two(&input))
    }
}
