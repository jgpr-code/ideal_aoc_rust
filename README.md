# ideal_aoc_rust

Figuring out the ideal Advent of Code project setup in Rust.

## The different setups

### My first setup (project per day)

Initially, I used a single project per day.

Upsides:

- Fast compile time after first compile (first compile needs to compile all the dependencies)

Downsides:

- Fully compiled Advent of Code takes a huge amount of space (>2GB, TODO specify exactly), because of redundantly compiled dependencies
- No overarching program that is able to run & test each day

### The second setup (module per day)

I was annoyed by the downsides and figured out that I can just use one module per day.

Upsides:

- No longer takes as much space (TODO how much?) as dependencies are only compiled once and used throughout the whole crate
- Overarching program is now possible

Downsides:

- Compile times are becoming an issue

### The third setup (workspace with project per day + overarching project)

Still, not everything clear here (TODO)

## The scope of this project

- Enable the 3 - 5 days from Advent of Code 2023 for all the different setups
- Make sure that common dependencies are used there like e.g. regex
- Measure compile time
- Measure memory consumption
