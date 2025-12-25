use common::{run, AoCSolution};
use std::{fs, thread::current};

struct Solution1 {}
struct Solution2 {}

static DIAL_STEPS: i64 = 100;

impl AoCSolution for Solution1 {
    fn solve(&self, input: &str) -> i64 {
        
        let rotations = input.lines().map(|l| {
            let number: i64= l[1..].parse().unwrap();
            return if l.starts_with("L") { -1 } else { 1 } * number;
        });
        
        let mut current_position = 50;
        let mut zero_positions = 0;
        for val in rotations {
            current_position = (val + current_position + DIAL_STEPS) % DIAL_STEPS;
            if current_position == 0 {
                zero_positions += 1;
            }
        }
        
        return zero_positions;
    }
}

impl AoCSolution for Solution2 {
    fn solve(&self, input: &str) -> i64 {
        
        let rotations = input.lines().map(|l| {
            let number : i64 = l[1..].parse().unwrap();
            return if l.starts_with("L") { -1 } else { 1 } * number;
        });
        
        let mut current_position = 50;
        let mut zero_crossings = 0;
        for val in rotations {
            print!("c: {current_position}+{val}=");
            current_position += val;
            while current_position < 0 {
                if current_position - val > 0 {
                    zero_crossings += 1;
                }
                current_position += DIAL_STEPS;
                
            }
            while current_position > (DIAL_STEPS-1) {
                current_position -= DIAL_STEPS;
                zero_crossings += 1;
            }
            println!("{current_position} {zero_crossings}");
        }
        return zero_crossings;
    }
}

fn main() {
    let solution1 = Solution1 {};
    let solution2 = Solution2 {};
    run(
        "Advent of code day1", 
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
        let input = String::from(indoc!{"
        L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82
    "});
        
        let result = (&Solution1 {}).solve(&input);
        assert_eq!(result, 3);
    }
    
    #[test]
    fn test_part2_1() {
        let input = String::from(indoc!{"
            L68
            L30
            R48
            L5
            R60
            L55
            L1
            L99
            R14
            L82
        "});

        let result = (&Solution2 {}).solve(&input);
        assert_eq!(result, 6);
    }

    
    
    #[test]
    fn test_part2_2() {
        let input = String::from(indoc!{"
            R1000
        "});
        
        let result = (&Solution2 {}).solve(&input);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_part2_3() {
        let input = String::from(indoc!{"
            L1000
        "});
        
        let result = (&Solution2 {}).solve(&input);
        assert_eq!(result, 10);
    }

     #[test]
    fn test_part2_4() {
        let input = String::from(indoc!{"
            L68
            L30
            R48
            L5
            R60
            L55
            L1
            L99
            R14
            L82
        "});

        assert_eq!((&Solution2 {}).solve(&get_lines(&input,1)), 1);
        assert_eq!((&Solution2 {}).solve(&get_lines(&input,2)), 1);
        assert_eq!((&Solution2 {}).solve(&get_lines(&input,3)), 2);
        assert_eq!((&Solution2 {}).solve(&get_lines(&input,4)), 2);
        assert_eq!((&Solution2 {}).solve(&get_lines(&input,5)), 3);
        assert_eq!((&Solution2 {}).solve(&get_lines(&input,6)), 4);
        assert_eq!((&Solution2 {}).solve(&get_lines(&input,7)), 4);
        assert_eq!((&Solution2 {}).solve(&get_lines(&input,8)), 5);
        assert_eq!((&Solution2 {}).solve(&get_lines(&input,9)), 5);
        assert_eq!((&Solution2 {}).solve(&get_lines(&input,10)), 6);
    }
}


