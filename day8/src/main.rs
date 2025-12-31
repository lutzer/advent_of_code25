use common::{run, AoCSolution};
use core::f64;
use std::{fs};

struct Solution1 {
    connections : usize
}
struct Solution2 {}

type Vec3D = (i64,i64,i64);

fn eucludian_distance(p1: Vec3D, p2: Vec3D) -> f64 {
    let diff_x = p1.0 - p2.0;
    let diff_y = p1.1 - p2.1;
    let diff_z = p1.2 - p2.2;
    return ((diff_x*diff_x+diff_y*diff_y+diff_z*diff_z) as f64).sqrt();
}

impl AoCSolution for Solution1 {
    fn solve(&self, input: &str) -> i64 {
        let junction_positions = input.lines().map(|l| {
            let splits : Vec<&str> = l.split(",").collect();
            let x = splits[0].parse::<i64>().unwrap();
            let y = splits[1].parse::<i64>().unwrap();
            let z = splits[2].parse::<i64>().unwrap();
            return (x,y,z);
        }).collect::<Vec<Vec3D>>();

        // find shortest conections
        let mut shortest_connections : Vec<(f64, usize, usize)> = vec![(f64::MAX,0,0); self.connections];
        for i in 0..junction_positions.len() {
            for j in i+1..junction_positions.len() {
                let dist = eucludian_distance(junction_positions[i], junction_positions[j]);
                if let Some(index) = shortest_connections.iter().position(|d| { dist < d.0 }) {
                    shortest_connections.insert(index, (dist, i, j));
                    shortest_connections.pop();
                }
            }
        }

        // create circuit list
        let mut circuits : Vec<Vec<usize>> = (0..junction_positions.len()).into_iter()
            .map(|i| { vec![i] }).collect();

        // cmake circuit connections
        for (_,to,from) in shortest_connections {

            // connect two circuits
            let c1 = circuits.iter().find(|c| c.contains(&from)).unwrap().clone();
            let c2 = circuits.iter().find(|c| c.contains(&to)).unwrap().clone();

            // connect them if they are not already in the same circuit
            if c1 != c2 {
                // create new circuit
                let connected_circuit = [c1.clone(),c2.clone()].concat();

                // remove c1 and c2
                circuits = circuits.into_iter()
                    .filter(|c| c != &c1 && c != &c2)
                    .collect();

                // add to end
                circuits.push(connected_circuit);
            }

            
        }

        let mut circuit_sizes = circuits.iter().map(|c| c.len()).collect::<Vec<usize>>();
        circuit_sizes.sort();
        circuit_sizes.reverse();

        return circuit_sizes.iter().take(3).fold(1, |acc, v| { acc * v } ) as i64;
    }
}

impl AoCSolution for Solution2 {
    fn solve(&self, input: &str) -> i64 {
        let junction_positions = input.lines().map(|l| {
            let splits : Vec<&str> = l.split(",").collect();
            let x = splits[0].parse::<i64>().unwrap();
            let y = splits[1].parse::<i64>().unwrap();
            let z = splits[2].parse::<i64>().unwrap();
            return (x,y,z);
        }).collect::<Vec<Vec3D>>();

        // find shortest conections
        let mut distances : Vec<Vec<f64>> = vec![ vec![f64::MAX; junction_positions.len()]; junction_positions.len()];
        for i in 0..junction_positions.len() {
            for j in i+1..junction_positions.len() {
                distances[i][j] = eucludian_distance(junction_positions[i], junction_positions[j]);
            }
        }

        // create circuit list
        let mut circuits : Vec<Vec<usize>> = (0..junction_positions.len()).into_iter()
            .map(|i| { vec![i] }).collect();

        let mut from: usize = 0;
        let mut to: usize = 0;
        while circuits.len() > 1 {

            //get shortest connection
            (_, from, to) = distances.iter().enumerate().fold((f64::MAX,0,0),|acc,(i, row)| {
                return row.iter().enumerate().fold(acc, |acc, (j,&dist)| {
                    return if dist < acc.0 { (dist,i,j) } else { acc };
                })
            });

            // put distance on max
            distances[from][to] = f64::MAX;

            // connect two circuits
            let c1 = circuits.iter().find(|c| c.contains(&from)).unwrap().clone();
            let c2 = circuits.iter().find(|c| c.contains(&to)).unwrap().clone();

            // connect them if they are not already in the same circuit
            if c1 != c2 {
                // create new circuit
                let connected_circuit = [c1.clone(),c2.clone()].concat();

                // remove c1 and c2
                circuits = circuits.into_iter()
                    .filter(|c| c != &c1 && c != &c2)
                    .collect();

                // add to end
                circuits.push(connected_circuit);
            }

        }

        let j1 = junction_positions[from];
        let j2 = junction_positions[to];

        return j1.0 * j2.0;
    }
}

fn main() {
    let solution1 = Solution1 { connections: 1000 };
    let solution2 = Solution2 {};
    run(
        "Advent of code day 8", 
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
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"});
        
        let result = (&Solution1 { connections: 10 }).solve(&input);
        assert_eq!(result, 40);
    }

    #[test]
    fn test_part2() {
        let input = String::from(indoc!{"
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"});
        
        let result = (&Solution2 {}).solve(&input);
        assert_eq!(result, 25272);
    }
}


