extern crate aoc_runner;
#[macro_use]
extern crate aoc_runner_derive;
mod util {
    pub mod grid;
    pub mod point;
    pub mod parse;
}
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
aoc_lib! { year = 2024 }
