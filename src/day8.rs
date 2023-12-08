use std::collections::HashMap;
use num::integer::lcm;

#[aoc(day8, part1)]
pub fn solve_part1(input: &str) -> usize {

    let mut map:HashMap<&str, [&str; 2]> = HashMap::new();

    let mut lines = input.lines();

    let mut directions = lines.next().unwrap().chars().map(|dir| {
        match dir { 'L' => 0, 'R' => 1, _ => panic!("Invalid direction")}
    }).cycle();

    for line in lines.skip(1){
        let key = &line[0..3];
        let left = &line[7..10];
        let right = &line[12..15];

        map.insert(key, [left, right]);
    }

    let mut current_node = "AAA";
    let mut steps = 0;
    while current_node != "ZZZ" {
        let next_dir = directions.next().unwrap();
        current_node = map.get(current_node).unwrap()[next_dir];
        steps += 1
    }
    
    steps
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &str) -> usize {

    let mut map:HashMap<&str, [&str; 2]> = HashMap::new();

    let mut lines = input.lines();

    let mut directions = lines.next().unwrap().chars().map(|dir| {
        match dir { 'L' => 0, 'R' => 1, _ => panic!("Invalid direction")}
    }).cycle();

    for line in lines.skip(1){
        let key = &line[0..3];
        let left = &line[7..10];
        let right = &line[12..15];

        map.insert(key, [left, right]);
    }

    let mut current_nodes = map.keys().filter(|k|{
        k.chars().nth(2) == Some('A')
    }).collect::<Vec<&&str>>();

    let num_nodes = current_nodes.len();

    let mut cycle_start = vec![0; num_nodes];
    let mut cycle_period = vec![0; num_nodes];

    let mut visited_nodes:Vec<Vec<&&str>> = Vec::new();
    (0..num_nodes).for_each(|_| visited_nodes.push(Vec::new()));

    let mut steps = 0;

    while cycle_period.iter().any(|&p| p == 0) {
        let next_dir = directions.next().unwrap();
        steps += 1;

        current_nodes.iter_mut().enumerate().for_each(|(i,node)|{
            *node = &map.get(*node).unwrap()[next_dir];

            if node.ends_with('Z') {
                if cycle_start[i] == 0 {
                    cycle_start[i] = steps;
                } else if cycle_period[i] == 0 {
                    cycle_period[i] = steps - cycle_start[i];
                }
            }
        });
    }

    // I really don't know why these are the same but
    // the solution depends on it
    assert_eq!(cycle_start, cycle_period);

    cycle_period.iter().fold(1, |acc, &n| lcm(acc, n))
}


#[cfg(test)]
mod tests {
    use super::*;

    const FULL_INPUT:&str = include_str!("../input/2023/day8.txt");

    const TEST_INPUT_1:&str = include_str!("../test_input/2023/day8-1.txt");
    const TEST_INPUT_2:&str = include_str!("../test_input/2023/day8-2.txt");
    const TEST_INPUT_3:&str = include_str!("../test_input/2023/day8-3.txt");

    #[test]
    fn part1() {
        assert_eq!(solve_part1(TEST_INPUT_1), 2);
        assert_eq!(solve_part1(TEST_INPUT_2), 6);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(TEST_INPUT_3), 6);
    }

    #[test]
    fn part2_full() {
        solve_part2(FULL_INPUT);
    }
}