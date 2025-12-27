use common::{run, AoCSolution};
use std::{fs};

struct Solution1 {}
struct Solution2 {}

type PaperMap = Vec<Vec<char>>;

fn create_map_from(input: &String) -> PaperMap {
    let map: PaperMap = input.lines().map(|l| {
        let chars = l.chars().collect::<Vec<char>>();
        return chars;
    }).collect();
    
    return map;
}

fn find_accessible_paper_rolls(map: &PaperMap) -> PaperMap {
    
    let mut output = map.clone();

    let neighbours: [(i32,i32);8] = [
        (-1,-1),
        (-1,0),
        (-1,1),
        (0,-1),
        (0,1),
        (1,-1),
        (1,0),
        (1,1)
    ];

    let w = map.len();
    let h = map.len();

    for x in 0..w {
        for y in 0..h {
            if map[x][y] == '@' {
                // check neighbours
                let number_of_occupied_neighbours = neighbours.iter().filter(|n| {
                    let pos = (x as i32 + n.0, y as i32 + n.1);
                    let is_occupied = 
                        pos.0 >= 0 && pos.0 < (w as i32) && 
                        pos.1 >= 0 && pos.1 < (h as i32) && 
                        map[pos.0 as usize][pos.1 as usize] == '@';
                    return is_occupied;
                }).count();

                if number_of_occupied_neighbours < 4 {
                    output[x][y] = 'x';
                }
            }
            
        }
    }

    return output;
}

impl AoCSolution for Solution1 {
    fn solve(&self, input: &str) -> i64 {

        let map = create_map_from(&input.to_string());

        let solved_map = find_accessible_paper_rolls(&map);

        let accessible_rolls_count : usize = solved_map.iter().map(|column| {
            return column.iter().filter(|cell| { **cell == 'x'}).count();
        }).sum();

        return accessible_rolls_count as i64;
    }
}

impl AoCSolution for Solution2 {
    fn solve(&self, input: &str) -> i64 {
        let mut map = create_map_from(&input.to_string());

        let mut removed_rolls = 0;
        
        loop {
            map = find_accessible_paper_rolls(&map);
            
            // count accesbile rolls
            let accessible_rolls_count : usize = map.iter().map(|column| {
                return column.iter().filter(|cell| { **cell == 'x'}).count();
            }).sum();

            // remove rolls
            map = map.iter_mut().map(|column| {
                return column.iter().map(|cell| { 
                    return if *cell == 'x' {'.'} else {*cell}
                }).collect();
            }).collect();

            // add to sum
            removed_rolls += accessible_rolls_count;

            if (accessible_rolls_count == 0) {
                break;
            }
        }
        return removed_rolls as i64;
    }
}

fn main() {
    let solution1 = Solution1 {};
    let solution2 = Solution2 {};
    run(
        "Advent of code day 4", 
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
            ..@@.@@@@.
            @@@.@.@.@@
            @@@@@.@.@@
            @.@@@@..@.
            @@.@@@@.@@
            .@@@@@@@.@
            .@.@.@.@@@
            @.@@@.@@@@
            .@@@@@@@@.
            @.@.@@@.@.
        "});
        
        let result = (&Solution1 {}).solve(&input);
        assert_eq!(result, 13);
    }

     #[test]
    fn test_part1_solve_map() {
        let input = create_map_from(&String::from(indoc!{"
            ..@@.@@@@.
            @@@.@.@.@@
            @@@@@.@.@@
            @.@@@@..@.
            @@.@@@@.@@
            .@@@@@@@.@
            .@.@.@.@@@
            @.@@@.@@@@
            .@@@@@@@@.
            @.@.@@@.@.
        "}));

        let expected = &create_map_from(&String::from(indoc!{"
            ..xx.xx@x.
            x@@.@.@.@@
            @@@@@.x.@@
            @.@@@@..@.
            x@.@@@@.@x
            .@@@@@@@.@
            .@.@.@.@@@
            x.@@@.@@@@
            .@@@@@@@@.
            x.x.@@@.x.
        "}));
        
        let result = find_accessible_paper_rolls(&input);

        for i in 0..input.len() {
            assert_eq!(result[i], expected[i]);
        }
    }

    #[test]
    fn test_part2() {
        let input = String::from(indoc!{"
            ..@@.@@@@.
            @@@.@.@.@@
            @@@@@.@.@@
            @.@@@@..@.
            @@.@@@@.@@
            .@@@@@@@.@
            .@.@.@.@@@
            @.@@@.@@@@
            .@@@@@@@@.
            @.@.@@@.@.
        "});
        
        let result = (&Solution2 {}).solve(&input);
        assert_eq!(result, 43);
    }
}


