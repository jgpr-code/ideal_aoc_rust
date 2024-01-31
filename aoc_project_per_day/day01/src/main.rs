#![feature(test)]
#![feature(lazy_cell)]
extern crate test;

use anyhow::Result;
use std::io;

#[macro_use]
mod common {
    #[allow(unused_macros)]
    macro_rules! regex {
        ($re:literal) => {{
            static RE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
                // println!("initializing regex {}", $re);
                regex::Regex::new($re).unwrap()
            });
            &RE
        }};
    }

    #[cfg(test)]
    #[macro_use]
    pub mod test_utils {
        use std::fs;
        pub fn read_from_file(filename: &str) -> String {
            println!("reading {}", filename);
            fs::read_to_string(filename)
                .unwrap_or_else(|msg| panic!("error reading {}: {}", filename, msg))
        }

        macro_rules! local_file {
            ($file:literal) => {
                LazyLock::new(|| common::test_utils::read_from_file(&format!("src/{}", $file)))
            };
        }
    }
}

pub fn main() -> Result<()> {
    let stdin = io::read_to_string(io::stdin())?;
    println!("part1: {}", part_one(&stdin)?);
    println!("part2: {}", part_two(&stdin)?);
    Ok(())
}

pub fn part_one(input: &str) -> Result<i128> {
    let input = parse_input(input)?;
    solve_one(&input)
}

pub fn part_two(input: &str) -> Result<i128> {
    let input = parse_input(input)?;
    solve_two(&input)
}

struct Input {
    lines: Vec<String>,
}

fn parse_input(input: &str) -> Result<Input> {
    let lines: Vec<String> = input.lines().map(|l| String::from(l)).collect();
    Ok(Input { lines })
}

fn parse_line(line: &str) -> i128 {
    let mut first = None;
    let mut last = None;
    for c in line.chars() {
        if c.is_digit(10) {
            if first == None {
                first = Some(c);
            }
            last = Some(c);
        }
    }
    let fd = first.unwrap();
    let ld = last.unwrap();
    let ifd = fd.to_digit(10).unwrap() as i128;
    let ild = ld.to_digit(10).unwrap() as i128;
    ifd * 10 + ild
}

fn parse_advanced(line: &str) -> i128 {
    let spelled_digits = vec![
        ("one", "one1one"),
        ("two", "two2two"),
        ("three", "three3three"),
        ("four", "four4four"),
        ("five", "five5five"),
        ("six", "six6six"),
        ("seven", "seven7seven"),
        ("eight", "eight8eight"),
        ("nine", "nine9nine"),
    ];
    let mut line = String::from(line);
    for (spelled, replacement) in spelled_digits {
        line = line.replace(spelled, replacement);
    }
    // println!("{:?}", line);
    let result = parse_line(&line);
    // println!("{:?}", result);
    result
}

fn solve_one(input: &Input) -> Result<i128> {
    let Input { lines } = input;

    let mut sum = 0;
    for line in lines {
        sum += parse_line(line);
    }
    Ok(sum)
}

fn solve_two(input: &Input) -> Result<i128> {
    let Input { lines } = input;

    let mut sum = 0;
    for line in lines {
        sum += parse_advanced(line);
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::LazyLock;
    use test::Bencher;

    static TEST: LazyLock<String> = local_file!("test.txt");
    static TEST2: LazyLock<String> = local_file!("test2.txt");
    static INPUT: LazyLock<String> = local_file!("input.txt");

    #[test]
    fn test_one() -> Result<()> {
        let i128 = super::part_one(&TEST)?;
        assert_eq!(i128, 142);
        Ok(())
    }
    #[test]
    fn part_one() -> Result<()> {
        let i128 = super::part_one(&INPUT)?;
        assert_eq!(i128, 54450);
        Ok(())
    }
    #[test]
    fn test_two() -> Result<()> {
        let i128 = super::part_two(&TEST2)?;
        assert_eq!(i128, 281);
        Ok(())
    }
    #[test]
    fn part_two() -> Result<()> {
        let i128 = super::part_two(&INPUT)?;
        assert_eq!(i128, 54265);
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
}
