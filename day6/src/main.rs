use common::{run, AoCSolution};
use std::{fs};

struct Solution1 {}
struct Solution2 {}

#[derive(Debug,PartialEq)]
enum Operation {
    ADD,
    MULTIPLY,
    UNKNOWN
}

type Problem = (Vec<i64>, Operation);

impl AoCSolution for Solution1 {
    fn solve(&self, input: &str) -> i64 {
        let table : Vec<Vec<&str>> = input.lines().map(|l| {
            return l.split(" ").filter(|s| {*s != ""} ).collect::<Vec<&str>>(); 
        }).collect();

        let problems : Vec<Problem> = (0..table[0].len()).into_iter().map(|j| {
            let operands : Vec<i64> = (0..table.len()-1).into_iter().map(|i| {
                return table[i][j].parse::<i64>().unwrap();
            }).collect();
            let operation = match table[table.len()-1][j] {
                 "*" => Operation::MULTIPLY,
                 "+" => Operation::ADD,
                 v => panic!("Operation not specified: {v}")
            };
            return (operands,operation);
        }).collect();

        // println!("{}", format!("{:?}", problems));

        let result = problems.into_iter().map(|(operands,operation)| {
            if operation == Operation::MULTIPLY {
                return operands.iter().fold(1, |acc, o| acc * o);
            } else if operation == Operation::ADD {
                return operands.iter().fold(0, |acc, o| acc + o);
            } else {
                panic!("Should not reach here");
            }
        }).sum();

        return result;
    }
}

// type Problem = (i64,i64,i64,&str);

impl AoCSolution for Solution2 {
    fn solve(&self, input: &str) -> i64 {
        let table : Vec<Vec<char>> = input.lines().map(|l| {
            return l.chars().collect();
        }).collect();

        let h = table.len();
        let w = table[0].len();

        let mut problems: Vec<Problem> = vec![(vec![], Operation::UNKNOWN)];

        for i in (0..w).rev() {
            // construct number
            let number_string = (0..h-1).into_iter().fold("".to_string(), |acc, j| {
                return acc + &table[j][i].to_string();
            });

            // add number to current problem
            if let Ok(number) = number_string.trim().parse::<i64>() {
                problems.last_mut().unwrap().0.push(number);
            }

            // specify operation and add new problem if new operand is found
            if table[h-1][i] != ' ' {
                let operation = match table[h-1][i] {
                    '+' => Operation::ADD,
                    '*' => Operation::MULTIPLY,
                    v => panic!("Operation not specified: {v}")
                };
                problems.last_mut().unwrap().1 = operation;
                problems.push((vec![], Operation::UNKNOWN));
            }
        }

        let problem_sum = problems.into_iter().map(|(operands,operation)| {
            if operation == Operation::MULTIPLY {
                return operands.iter().fold(1, |acc, o| acc * o);
            } else if operation == Operation::ADD {
                return operands.iter().fold(0, |acc, o| acc + o);
            } else if operation == Operation::UNKNOWN {
                return 0;
            } else {
                panic!("Should not reach here");
            }
        }).sum();

        return problem_sum;
    }
}

fn main() {
    let solution1 = Solution1 {};
    let solution2 = Solution2 {};
    run(
        "Advent of code day 6", 
        &fs::read_to_string("input.txt").expect("Input Error"), 
        &vec![&solution1 as &dyn AoCSolution, &solution2 as &dyn AoCSolution]
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    
    #[test]
    fn test_part1() {
        let input = String::from(indoc!{"
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "});
        
        let result = (&Solution1 {}).solve(&input);
        assert_eq!(result, 4277556);
    }

    #[test]
    fn test_part2() {
        let input = String::from(indoc!{"
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "});
        
        let result = (&Solution2 {}).solve(&input);
        assert_eq!(result, 3263827);
    }
}


