use common::{AoCSolution, CharMap, create_map_from, run};
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
            println!("{}", format!("{:?}", map));
        }

        return split_sum as i64;
    }
}


fn calculate_timelines(map: &CharMap, (x,y): (usize, usize)) -> usize {
        
    // check if coordinates reached the end of map
    if y >= map.len() || x >= map[y].len() {
        return 1;
    }


    if map[y][x] == '^' {
        //split one field left and right
        return calculate_timelines(map, (x+1,y)) + calculate_timelines(map, (x-1,y))
    } else {
        // go straight down
        return calculate_timelines(map, (x,y+1));
    }
}

impl AoCSolution for Solution2 {
    fn solve(&self, input: &str) -> i64 {
        let map = create_map_from(&input.to_string());

        // serach for start
        let start_x = map[0].iter().position(|c| *c == 'S').unwrap();

        //calculate timelines
        let timelines = calculate_timelines(&map, (start_x,0));

        return timelines as i64;
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


