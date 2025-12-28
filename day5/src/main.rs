use common::{run, AoCSolution};
use std::{cmp::max, collections::HashSet, fs};

struct Solution1 {}
struct Solution2 {}

type Range = (usize,usize);

fn parse_database(db: &String) -> (Vec<Range>, Vec<usize>) {
    let mut parts = db.split("\n\n").map(|s| s.to_string());

    let ranges = parts.next().unwrap().lines().map(|l| {
        let mut splits = l.split("-");
        let from = splits.next().unwrap().parse::<usize>().unwrap();
        let to = splits.next().unwrap().parse::<usize>().unwrap();
        return (from, to);
    }).collect();

    let ids = parts.next().unwrap().lines().map(|l| {
        return l.parse::<usize>().unwrap();
    }).collect();

    return (ranges, ids);
}

impl AoCSolution for Solution1 {
    fn solve(&self, input: &str) -> i64 {
        let (ranges, ids) = parse_database(&input.to_string());

        let fresh_ingridients : Vec<&usize> = ids.iter().filter(|id| {
            return ranges.iter().any(|range| **id >= (range).0 && **id <= range.1);
        }).collect();

        return fresh_ingridients.iter().count() as i64;
    }
}

impl AoCSolution for Solution2 {
    fn solve(&self, input: &str) -> i64 {
        let (ranges, _) = parse_database(&input.to_string());

        let mut ranges_union : Vec<Range> = vec![];

        for (mut from, mut to) in ranges {
            
            ranges_union = ranges_union.into_iter().filter(|range| {
                if from <= range.0 && to >= range.1 {
                    // from, to overlaps the whole range, delete range
                    return false
                } else {
                    if from >= range.0 && from <= range.1 {
                        // from, to overlaps on the left
                        from = range.1+1;
                    }
                    if to >= range.0 && to <= range.1 {
                        // from, to overlaps on the right
                        to = range.0-1;
                    }
                    return true;
                }
            }).collect();

            if from <= to { 
                ranges_union.push((from, to));
            }
        }

        let count = ranges_union.iter()
            .map(|r| r.1 - r.0 + 1)
            .sum::<usize>();

        return count as i64;
    }
}

fn main() {
    let solution1 = Solution1 {};
    let solution2 = Solution2 {};
    run(
        "Advent of code day 5", 
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
            3-5
            10-14
            16-20
            12-18

            1
            5
            8
            11
            17
            32
        "});
        
        let result = (&Solution1 {}).solve(&input);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part2_1() {
        let input = String::from(indoc!{"
            3-5
            10-14
            16-20
            12-18

            1
            5
            8
            11
            17
            32
        "});
        
        let result = (&Solution2 {}).solve(&input);
        assert_eq!(result, 14);
    }

    #[test]
    fn test_part2_2() {
        let input = String::from(indoc!{"
            3-5
            10-14
            16-20
            12-18
            20-25
            26-30
            1-30

            1
            5
            8
            11
            17
            32
        "});
        
        let result = (&Solution2 {}).solve(&input);
        assert_eq!(result, 30);
    }
}


