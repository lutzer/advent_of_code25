use common::{AoCSolution, CharMap, create_map_from, print_map, run};
use std::{fs};

struct Solution1 {}
struct Solution2 {}

fn run_step(map: &mut CharMap, row: usize) -> usize {
    let mut splits = 0;
    for i in 0..map[row].len() {
        if map[row][i] == 'S' || map[row][i] == '|' {
            // check row below if its empty or has a splitter
            if map[row+1][i] == '.' {
                map[row+1][i] = '|';
            } else if map[row+1][i] == '^' {
                map[row+1][i-1] = '|';
                map[row+1][i+1] = '|';
                splits += 1
            }
        }
    }
    return splits;
}

impl AoCSolution for Solution1 {
    fn solve(&self, input: &str) -> i64 {
        let mut map = create_map_from(&input.to_string());

        let mut split_sum = 0;
        for i in 0..map[0].len()-1 {
            split_sum += run_step(&mut map, i);
        }

        return split_sum as i64;
    }
}


fn calculate_timeline_row(timeline_map: &mut Vec<Vec<usize>>, map: &CharMap, row: usize) {
    for i in 0..map[row].len() {
        if map[row][i] == 'S' || map[row][i] == '|' {
            if row >= map.len()-1 {
                // if its the last row
                timeline_map[row][i] = 1;
            } else if map[row+1][i] == '|' {
                // previous was beam
                timeline_map[row][i] = timeline_map[row+1][i]
            } else if map[row+1][i] == '^' {
                // previous was splitter
                timeline_map[row][i] = timeline_map[row+1][i-1] + timeline_map[row+1][i+1];
            } else {
                panic!("Shouldnt hit anything else than | or ^");
            }
        }
    }
}

impl AoCSolution for Solution2 {
    fn solve(&self, input: &str) -> i64 {
        let mut map = create_map_from(&input.to_string());

        // solve map
        for i in 0..map[0].len() {
            run_step(&mut map, i);
        }

        //calculate timelines
        let mut timeline_map = vec![ vec![ 0; map[0].len()]; map.len() ];
        for i in (0..map[0].len()+1).rev() {
            calculate_timeline_row(&mut timeline_map, &map, i);
        }
        // print_map(&timeline_map);
        // let timelines = calculate_timelines(&map, (start_x,0));

        let timelines_max = &timeline_map[0].iter().max().unwrap();

        return **timelines_max as i64;
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
    
    #[test]
    fn test_part1() {
        let input = String::from(indoc!{"
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"});
        
        let result = (&Solution1 {}).solve(&input);
        assert_eq!(result, 21);
    }

    #[test]
    fn test_part2() {
        let input = String::from(indoc!{"
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"});
        
        let result = (&Solution2 {}).solve(&input);
        assert_eq!(result, 40);
    }
}


