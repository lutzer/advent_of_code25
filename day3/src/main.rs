use common::{run, AoCSolution};
use std::{cmp::min, fs};

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

            return (first.0 * 10 + second.0) as i64;
        });

        return joltages.sum::<i64>();
    }
}

static JOLTAGE_LENGTH : usize = 12;

fn find_max_digit(digits: &[u32]) -> (u32, usize) {

    let mut max_val = 0;
    let mut index = 0;
    for i in 0..digits.len() {
        if digits[i] > max_val {
            max_val = digits[i];
            index = i;
        }
    }

    return (max_val, index)
}

fn calculate_joltage(digits: &Vec<u32>, joltage_length : usize) -> i64 {
    let mut selected_vals : Vec<u32> = vec![0; JOLTAGE_LENGTH];
    let mut start_index : usize = 0;
    for i in 0..JOLTAGE_LENGTH {
        let end_index = digits.len() - joltage_length + i + 1;
        let (val,index) = find_max_digit(&digits[start_index..end_index]);
        selected_vals[i] = val;
        start_index += index + 1;
    }
    
    return selected_vals.iter().enumerate().fold(0, |acc, (i, val)| {
        return acc + (*val as i64) * 10i64.pow((selected_vals.len()-1-i) as u32);
    });
}

impl AoCSolution for Solution2 {


    fn solve(&self, input: &str) -> i64 {
        let banks = input.lines().map(|l| {
            return l.chars().map(|c| { c.to_digit(10).unwrap() as u32 });
        });

        let joltages = banks.map( |bank| {
            let digits : Vec<u32> = bank.collect();
            return calculate_joltage(&digits, 12);
        });

        return joltages.sum::<i64>();
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
        let input = String::from(indoc!{"
            987654321111111
            811111111111119
            234234234234278
            818181911112111
        "});
        
        let result = (&Solution2 {}).solve(&input);
        assert_eq!(result, 3121910778619);
    }

    #[test]
    fn test_part2_joltages() {
        let batteries : Vec<u32> = "987654321111111"
            .chars()
            .map(|c| { c.to_digit(10).unwrap() as u32 }).collect();
        assert_eq!(calculate_joltage(&batteries, 12), 987654321111);

        let batteries : Vec<u32> = "811111111111119"
            .chars()
            .map(|c| { c.to_digit(10).unwrap() as u32 }).collect();
        assert_eq!(calculate_joltage(&batteries, 12), 811111111119);

        let batteries : Vec<u32> = "234234234234278"
            .chars()
            .map(|c| { c.to_digit(10).unwrap() as u32 }).collect();
        assert_eq!(calculate_joltage(&batteries, 12), 434234234278);

        let batteries : Vec<u32> = "818181911112111"
            .chars()
            .map(|c| { c.to_digit(10).unwrap() as u32 }).collect();
        assert_eq!(calculate_joltage(&batteries, 12), 888911112111);
    }
}


