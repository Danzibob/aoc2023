use std::cmp::max;

#[derive(Debug, Default)]
struct Cubes {
    red: usize,
    green: usize,
    blue: usize
}

// Parse a single draw from the bag using str.split()
fn parse_draw(draw: &str) -> Cubes {
    let mut counts:Cubes = Default::default();
    draw.split(", ").for_each(|color| {
        let entry = color.split_once(" ").unwrap();
        let value = entry.0.parse::<usize>().unwrap();
        match entry.1.chars().next().unwrap(){
            'r' => counts.red = value,
            'g' => counts.green = value,
            'b' => counts.blue = value,
            x => panic!("Unrecognised colour! {x}")
        }
    });
    counts
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &str) -> usize {
    input.lines().map(|game|{
        let (game_name, draws) = game.split_once(": ").unwrap();
        let mut min:Cubes = Default::default();
        for draw in draws.split("; "){
            let cubes = parse_draw(draw);
            min.red = max(min.red, cubes.red);
            min.green = max(min.green, cubes.green);
            min.blue = max(min.blue, cubes.blue);
        }
        if (min.red > 12) || (min.green > 13) || (min.blue > 14) { 
            0
        } else {
            game_name.split_once(" ").unwrap().1.parse::<usize>().unwrap()
        }
    }).sum()
}


#[aoc(day2, part2)]
pub fn solve_part2(input: &str) -> usize {
    input.lines().map(|game|{
        let (_, draws) = game.split_once(": ").unwrap();
        let mut min:Cubes = Default::default();
        for draw in draws.split("; "){
            let cubes = parse_draw(draw);
            min.red = max(min.red, cubes.red);
            min.green = max(min.green, cubes.green);
            min.blue = max(min.blue, cubes.blue);
        }
        min.red * min.green * min.blue
    }).sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT:&str = include_str!("../test_input/2023/day2.txt");

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(TEST_INPUT), 8);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(TEST_INPUT), 2286);
    }
}