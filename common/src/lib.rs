use std::process::exit;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Specifies which part of the solution you want to run
    part: usize
}

pub fn run(title: &str, input: &str, solutions: &[&dyn AoCSolution]) {
  let cli = Cli::parse();

  let part = cli.part;

  if part == 0 || part > solutions.len() {
    println!("There is no solution defined for part {part}");
    exit(0)
  }

  println!("> {title} - Calculating solution for part {part}.");

  let result = solutions[part-1].solve(input);

  println!("> Result: {result}");
}

pub trait AoCSolution {
  fn solve(&self, input: &str) -> i64;
}