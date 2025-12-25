use common::{run, AoCSolution};
use std::{fs, thread::current};

struct Solution1 {}
struct Solution2 {}

static DIAL_STEPS: i64 = 100;

impl AoCSolution for Solution1 {
    fn solve(&self, input: &str) -> i64 {
        return 0
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
        "Advent of code day X", 
        &fs::read_to_string("input.txt").expect("Input Error"), 
        &vec![&solution1 as &dyn AoCSolution, &solution2 as &dyn AoCSolution]
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    fn get_lines(s: &String, n: usize ) -> String {
        s.lines().take(n).collect::<Vec<_>>().join("\n")
    }
    
    #[test]
    fn test_part1() {
        let input = String::from(indoc!{""});
        
        let result = (&Solution1 {}).solve(&input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part2() {
        let input = String::from(indoc!{""});
        
        let result = (&Solution2 {}).solve(&input);
        assert_eq!(result, 0);
    }
}


