use std::process::exit;
use std::fmt::Debug;

use clap::{Parser};
use itertools::Itertools;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Specifies which part of the solution you want to run
    part: usize
}

pub fn run(title: &str, input: &str, solutions: &[&dyn AoCSolution]) {
    let cli = Cli::parse();
    
    let part = cli.part;
    
    println!("# {title}");
    
    if part == 0 || part > solutions.len() {
        let solutionlist = (1..=solutions.len()).map(|n| n.to_string()).join(", ");
        println!("There is no solution defined for part {part}. Available parts are: [{solutionlist}]");
        exit(0)
    }
    
    println!("Calculating solution for part {part}.");
    
    let result = solutions[part-1].solve(input);
    
    println!("Result: {result}");
}

pub trait AoCSolution {
    fn solve(&self, input: &str) -> i64;
}

pub fn remove_line_breaks(s: &String) -> String {
    return s.chars().filter(|c| !c.is_whitespace()).collect();
}

pub type CharMap = Vec<Vec<char>>;

pub fn create_map_from(input: &String) -> CharMap {
    return  input.lines().map(|l| {
        let chars = l.chars().collect::<Vec<char>>();
        return chars;
    }).collect();
}

pub fn print_map<T: Debug>(map: &Vec<Vec<T>>) {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            print!("{}", format!("{:?}", map[i][j]));
        }
        println!();
    }
}