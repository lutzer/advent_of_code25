use common::{run, AoCSolution};

struct Solution1 {}

impl AoCSolution for Solution1 {
    fn solve(&self, input: &str) -> i64 {
        return 1;
    }
}

fn main() {
    let solution1 = Solution1 {};
    run(
        "Advent of code day1", 
        "test", 
        &vec![&solution1 as &dyn AoCSolution]
    );
}


