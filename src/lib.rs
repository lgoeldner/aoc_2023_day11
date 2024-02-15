#[rustfmt::skip]
pub const TEST_DATA: &str = 
"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";
pub struct Map(Vec<Vec<Option<()>>>);
impl From<Vec<Vec<Option<()>>>> for Map {
    fn from(value: Vec<Vec<Option<()>>>) -> Self {
        Self(value)
    }
}
pub struct Part1;

impl Part1 {
    pub fn solve(input: &str) -> usize {
        let parsed = parse(input);
        0
    }
}

fn parse(input: &str) -> Map {
    // for each char in each line, if its a ̀`#`, put Some(()) else None
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => None,
                    '#' => Some(()),
                    _ => panic!("Invalid input"),
                })
                .collect()
        })
        .collect::<Vec<Vec<Option<()>>>>()
        .into()
}
