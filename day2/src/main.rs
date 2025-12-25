use common::{run, AoCSolution, remove_line_breaks};
use std::{fs};

struct Solution1 {}
struct Solution2 {}

impl Solution1 {
    fn get_invalid_ids_from_range(start: i64, end: i64) -> Vec<i64> {
        let mut invalids : Vec<i64> = vec![];
        for val in start..(end+1) {
            let s = val.to_string();
            if s.len() % 2 > 0 {
                continue;
            }
            // split in the middle
            let (left, right) = s.split_at(s.len() / 2);
            if left == right {
                invalids.push(val)
            }
        }
        return invalids;
    }
}

impl AoCSolution for Solution1 {
    fn solve(&self, input: &str) -> i64 {
        let claned_input = remove_line_breaks(&input.to_string());
        let ranges = claned_input.split(",").map(|range| {
            let parts: Vec<&str> = range.split("-").collect();
            let from : u64 = parts[0].parse().unwrap();
            let to : u64 = parts[1].parse().unwrap();
            return (from, to);
        });

        let mut id_sum : i64 = 0;
        for (from, to) in ranges {
            let invalids = Solution1::get_invalid_ids_from_range(from as i64, to as i64);
            for id in &invalids {
                id_sum += id;
            }
        }
        return id_sum;
    }
}

impl Solution2 {

    fn split_string(s: &str, chunk_size: usize) -> Vec<String> {
        s.chars()
            .collect::<Vec<char>>()
            .chunks(chunk_size)
            .map(|chunk| chunk.iter().collect())
            .collect()
    }

    fn get_invalid_ids_from_range(start: i64, end: i64) -> Vec<i64> {
        let mut invalids : Vec<i64> = vec![];
        'outer: for val in start..(end+1) {
            let s = val.to_string();
           
            for i in 1..(s.len()/2+1) {
                // split in equal parts
                let chunks = Solution2::split_string(&s, i);

                // compare if all chunks are equal
                let first = &chunks[0];
                let is_invalid = chunks.iter().skip(1).all(|chunk| chunk == first);
                if is_invalid {
                    invalids.push(val);
                    continue 'outer; 
                }
            }
            
        }
        return invalids;
    }
}

impl AoCSolution for Solution2 {
    fn solve(&self, input: &str) -> i64 {
        let claned_input = remove_line_breaks(&input.to_string());
        let ranges = claned_input.split(",").map(|range| {
            let parts: Vec<&str> = range.split("-").collect();
            let from : u64 = parts[0].parse().unwrap();
            let to : u64 = parts[1].parse().unwrap();
            return (from, to);
        });

        let mut id_sum : i64 = 0;
        for (from, to) in ranges {
            let invalids = Solution2::get_invalid_ids_from_range(from as i64, to as i64);
            for id in &invalids {
                id_sum += id;
            }
        }
        return id_sum;
    }
}

fn main() {
    let solution1 = Solution1 {};
    let solution2 = Solution2 {};
    run(
        "Advent of code day 2", 
        &fs::read_to_string("input.txt").expect("Input Error"), 
        &vec![&solution1 as &dyn AoCSolution, &solution2 as &dyn AoCSolution]
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_invalid_ids1() {
        assert_eq!(Solution1::get_invalid_ids_from_range(11,22), vec![11,22]);
        assert_eq!(Solution1::get_invalid_ids_from_range(99,115), vec![99]);
        assert_eq!(Solution1::get_invalid_ids_from_range(998,1012), vec![1010]);
        assert_eq!(Solution1::get_invalid_ids_from_range(1188511880,1188511890), vec![1188511885]);
        assert_eq!(Solution1::get_invalid_ids_from_range(222220,222224), vec![222222]);
        assert_eq!(Solution1::get_invalid_ids_from_range(1698522,1698528), vec![]);
        assert_eq!(Solution1::get_invalid_ids_from_range(446443,446449), vec![446446]);
        assert_eq!(Solution1::get_invalid_ids_from_range(38593856,38593862), vec![38593859]);
    }
    
    #[test]
    fn test_part1() {
        let input = String::from(indoc!{"
            11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
            1698522-1698528,446443-446449,38593856-38593862,565653-565659,
            824824821-824824827,2121212118-2121212124"
        });
        
        let result = (&Solution1 {}).solve(&input);
        assert_eq!(result, 1227775554);
    }

     #[test]
    fn test_invalid_ids2() {
        assert_eq!(Solution2::get_invalid_ids_from_range(11,22), vec![11,22]);
        assert_eq!(Solution2::get_invalid_ids_from_range(99,115), vec![99, 111]);
        assert_eq!(Solution2::get_invalid_ids_from_range(998,1012), vec![999, 1010]);
        assert_eq!(Solution2::get_invalid_ids_from_range(1188511880,1188511890), vec![1188511885]);
        assert_eq!(Solution2::get_invalid_ids_from_range(222220,222224), vec![222222]);
        assert_eq!(Solution2::get_invalid_ids_from_range(1698522,1698528), vec![]);
        assert_eq!(Solution2::get_invalid_ids_from_range(446443,446449), vec![446446]);
        assert_eq!(Solution2::get_invalid_ids_from_range(38593856,38593862), vec![38593859]);
        assert_eq!(Solution2::get_invalid_ids_from_range(565653,565659), vec![565656]);
        assert_eq!(Solution2::get_invalid_ids_from_range(824824821,824824827), vec![824824824]);
        assert_eq!(Solution2::get_invalid_ids_from_range(2121212118,2121212124), vec![2121212121]);
    }

    #[test]
    fn test_part2() {
        let input = String::from(indoc!{"
            11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
            1698522-1698528,446443-446449,38593856-38593862,565653-565659,
            824824821-824824827,2121212118-2121212124"
        });
        
        let result = (&Solution2 {}).solve(&input);
        assert_eq!(result, 4174379265);
    }
}


