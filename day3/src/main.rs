use common::{run, AoCSolution};
use std::{fs};

struct Solution1 {}
struct Solution2 {}

impl AoCSolution for Solution1 {

    fn solve(&self, input: &str) -> i64 {
        let banks = input.lines().map(|l| {
            return l.chars().map(|c| { c.to_digit(10).unwrap() as u32 });
        });

        let joltages = banks.map( |bank| {
            let vals : Vec<u32> = bank.collect();
            let mut first = (0,0);
            for i in 0..(vals.len()-1) {
                if vals[i] > first.0 {
                    first = (vals[i], i)
                }
            }
            let mut second = (0,0);
            for i in (first.1+1)..vals.len() {
                if vals[i] > second.0 {
                    second = (vals[i], i)
                }
            }

            return first.0 * 10 + second.0;
        });

        return joltages.sum::<u32>() as i64;
    }
}

impl AoCSolution for Solution2 {
    fn solve(&self, input: &str) -> i64 {
        return 0
    }
}

fn main() {
    let solution1 = Solution1 {};
    let solution2 = Solution2 {};
    run(
        "Advent of code day 3", 
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
            987654321111111
            811111111111119
            234234234234278
            818181911112111
        "});
        
        let result = (&Solution1 {}).solve(&input);
        assert_eq!(result, 357);
    }

    #[test]
    fn test_part2() {
        let input = String::from(indoc!{""});
        
        let result = (&Solution2 {}).solve(&input);
        assert_eq!(result, 0);
    }
}


