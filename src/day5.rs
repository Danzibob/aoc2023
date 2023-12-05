#[inline(always)]
fn parse_mapping(map: &str) -> Option<(usize, usize, usize)>{
    let mut numbers = [0; 3];
    let mut vals = map
        .split_whitespace()
        .take(4);

    // Take 3 numbers. If we run out, return None
    for i in 0..3 {
        if let Some(val) = vals.next()?.parse::<usize>().ok()
        { numbers[i] = val } else { return None }
    }

    Some((numbers[0], numbers[1], numbers[2]))
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &str) -> usize {
    // "generator"
    let mut file_reader = input.lines();

    // Read seeds into a 
    let mut seeds:Vec<usize> = file_reader
        .next().unwrap()[7..]
        .split(" ").map(|x| x.parse::<usize>().unwrap())
        .collect();
    let mut grow = seeds.clone();

    file_reader.next(); // Consume empty line
    
    while file_reader.next().is_some() { // Consumes title
        while let Some(line) = file_reader.next(){

            // If the line is empty, move on to the next section
            if line == "" {
                // Propegate growth back to original vector
                seeds = grow.clone();
                continue;
            }

            // For every line with a valid mapping
            if let Some(map) = parse_mapping(line) {
                
                // get mapping values & calculate offset
                let (dst, src, rng) = map;
                let offset:isize = dst as isize - src as isize;

                // Apply the mapping to all seeds
                for i in 0..(seeds.len()) {
                    if (seeds[i] >= src) && (seeds[i] < (src + rng)){
                        grow[i] = (seeds[i] as isize + offset) as usize;
                    }
                }
            }
        }
    }
    *grow.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT:&str = include_str!("../test_input/2023/day5.txt");
    const FULL_INPUT:&str = include_str!("../input/2023/day5.txt");

    #[test]
    fn part1() {
        assert_eq!(solve_part1(TEST_INPUT), 35);
    }

    #[test]
    fn part1_full() {
        solve_part1(FULL_INPUT);
    }

    // Too low: 83692320
}