# Rust Workspaces and the Advent of Code

## About the Advent of Code

The Advent of Code is an annual online advent calendar that runs from December 1st to 25th, featuring daily programming challenges of increasing difficulty.
For each solved challenge, you can earn up to two stars: one for the easy part and another for the harder part, which is only revealed after completing the easy one.
Besides the fun of gathering stars and the sense of accomplishment from earning all 50 stars, the
Advent of Code is also a great place to test out or learn new programming languages.
That's also how I first started learning Rust back in 2020. Since then, I've picked up a few tips on Rust project organization that I'd love to share.

## Rust and the Advent of Code

Since 3 years for me the obvious choice - without any project pressure to use a specific language - was using Rust to do the
Advent of Code challenges. Rust has great speed, is a modern language and has great community support and libraries.
And also compared to using Python it doesn't feel like cheating ;)
As the Advent of Code can be a great place to experiment with things. I have used it in the last X years to test different project setups each year.
It was a nice learning experience that I want to share.

When I initially started using Rust for the Advent of Code my method was simple. Every day I create a new standalone project for this day using Rust's excellent package manager `cargo`. Although this worked well, it was annoying for me to repeat myself every day, that's why I learned about `cargo-generate` and wrote myself a template for that for easier use (see link to cargo-aoc-template TODO). So now everything was fine right? Of course not! In the end I had something like this structure:

(TODO insert file-tree structure picture of project_per_day somehow)

And when I was done and everything was lying around compiled this whole thing took insurmountable amount of disk space (all the dependencies are compiled for every day again).

So some alternative needed to be found. And because the Advent of Code is nothing the you do in complete isolation, I looked around how others organized their code and noticed
that they are using modules instead of new crates. So this is the next approach I tried. In the end I ended up writing my own binary program using a `Solver`  that calls the individually specified days and ended up with something like this:

(TODO inseart file-tree structure of module_per_day)

I was happy with this approach as now the disk space when compiled was much smaller (YEAH). BUT unfortunately I also noticed that this approach has another rather large downside.
Compile times got increasingly worse every day. This is due to the reason how the Rust compiler works. It always considers a crate as the minimal amount of code for compilation.
And because now everything was just in this single crate split up into modules, but still the compiler always had to check every day when only changing a single day.
And this is unacceptable when you are trying to be faster than your competitors. So another solution had to be found:

This is when I read about cargo workspaces.
In the end I now have something like this:

(TODO file-tree structure of workspace approach)

(TODO cargo-aoc-ws-template -> creates workspace with empty days for every day)

So since  3 (4?) years Rust is my go to language for solving the programming tasks of the Advent of Code.
However I noticed that organizing my code changed over time due to various reasons, that I want to take a closer look.

First setup: New day -> New project
The easiest to get started with Rust is just by typing in `cargo new dayXY` every day and then solving the days task at hand in this project.

Pros:

- easy to understand

Cons:

- always the dependencies have to be added every day

Later, I noticed that this setup can be improved by introducing one crate that fixes the dependencies

Second setup: One crate many modules

module approach: one crate considered at a time for compile -> long compile times for 25 days...

The second approach was using a single crate and just have submodules for every day (this approach seemed also very common in fellow Rustaceans doing the Advent of Code).

Third approach: Using a feature of Rust called workspaces that combines both benefits

workspace benefits: cargo commands on all crates, fix dependencies across days, much smaller when compiled due to dependency reuse, no weird reexport of common code necessary (maybe it's not necessary for project per day as well TODO)

## TL;DR Table

| | Disk space | Compile Time | Run/Test All | Dependencies |
| -- | :--: | :--: | :--: | :--: |
| Crate per Day | ❌ | ✅ | ❌ | ❌ |
| Module per Day | ✅ | ❌ | 🆗 | 🆗 |
| Workspace w/ Crate per Day| ✅ | ✅ | ✅ | ✅ |
