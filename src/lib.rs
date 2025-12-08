#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

extern crate aoc_runner;

#[macro_use]
extern crate aoc_runner_derive;

mod shared;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;

aoc_lib! { year = 2025 }
