use std::cmp::max;

#[derive(Debug, Default)]
struct Cubes {
    red: usize,
    green: usize,
    blue: usize
}


// Parse all draws from a game and return the minimum cube counts
#[inline(always)]
fn min_cubes_for_game(game: &str) -> Cubes{
    let mut min:Cubes = Default::default();

    // Parse and iterate through all the draws
    let (_, draws) = game.split_once(": ").unwrap();
    for draw in draws.split("; "){
        draw.split(", ").for_each(|color| {
            let entry = color.split_once(" ").unwrap();
            let value = entry.0.parse().unwrap();
            // Update minimum possible counts
            match entry.1.chars().next().unwrap(){
                'r' => min.red = max(min.red, value),
                'g' => min.green = max(min.green, value),
                'b' => min.blue = max(min.blue, value),
                x => panic!("Unrecognised colour! {x}")
            }
        })
    }
    min
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &str) -> usize {
    input.lines().enumerate().map(|(id,game)|{

        // Calculate the minimum possible cubes for the game
        let min = min_cubes_for_game(game);

        // If the game is possible, return the game ID
        if (min.red <= 12) && (min.green <= 13) && (min.blue <= 14)
        {id + 1} else {0}
        
    }).sum() // Then sum those game IDs
}


#[aoc(day2, part2)]
pub fn solve_part2(input: &str) -> usize {
    input.lines().map(|game|{

        // Calculate the minimum possible cubes for the game
        let min = min_cubes_for_game(game);

        // Return the "Power" of the cubes
        min.red * min.green * min.blue

    }).sum() // Then sum the Power
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