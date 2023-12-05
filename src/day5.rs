#[derive(Clone, Copy)]
struct Mapping {
    src: usize,
    rng: usize,
    end: usize,
    offset: isize
}

impl std::fmt::Debug for Mapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}-{} +{}]", self.src, self.end, self.offset)
    }
}

#[inline(always)]
fn parse_mapping(map: &str) -> Option<Mapping>{
    let mut numbers = [0; 3];
    let mut vals = map
        .split_whitespace()
        .take(4);

    // Take 3 numbers. If we run out, return None
    for num in &mut numbers {
        if let Ok(val) = vals.next()?.parse::<usize>()
        { *num = val } else { return None }
    }

    let offset = numbers[0] as isize - numbers[1] as isize;

    Some(Mapping{
        src: numbers[1],
        rng: numbers[2],
        end: numbers[1] + numbers[2], // Redundant, but pre-computed here
        offset
    })
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut file_reader = input.lines();

    // Read seeds into a 
    let mut seeds:Vec<usize> = file_reader
        .next().unwrap()[7..]
        .split(' ').map(|x| x.parse::<usize>().unwrap())
        .collect();
    let mut grow = seeds.clone();

    file_reader.next(); // Consume empty line
    
    for line in file_reader { // Consumes title

        // If the line is empty, move on to the next section
        if line.is_empty() {
            // Propegate growth back to original vector
            seeds = grow.clone();
            continue;
        }

        // For every line with a valid mapping
        if let Some(map) = parse_mapping(line) {
            // Apply the mapping to all seeds
            for i in 0..(seeds.len()) {
                if (seeds[i] >= map.src) && (seeds[i] < (map.src + map.rng)){
                    grow[i] = (seeds[i] as isize + map.offset) as usize;
                }
            }
        }
    }
    *grow.iter().min().unwrap()
}

use std::cmp::{max, min};

#[derive(Clone)]
struct SeedRange {
    start: usize,
    end: usize
}

impl std::fmt::Debug for SeedRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}-{})", self.start, self.end)
    }
}

fn map_range(range:&SeedRange, map:&Mapping) -> (Option<SeedRange>, Option<SeedRange>,Option<SeedRange>){
    // If the sections don't overlap, return original range
    if (range.start > map.end) || (range.end < map.src){
        return (None, None, None)
    }

    let overlap_start = max(map.src, range.start) as isize + map.offset;
    let overlap_end =   min(map.end, range.end) as isize + map.offset;

    (
        // Before the map
        if range.start < map.src
        {Some(SeedRange {start: range.start, end: map.src})}
        else {None},

        // Overlapping the map
        if overlap_start < overlap_end
        {Some(SeedRange {start: overlap_start as usize, end: overlap_end as usize})}
        else {None},

        // After the map
        if map.end < range.end
        {Some(SeedRange {start: map.end, end: range.end})}
        else {None}
    )
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut file_reader = input.lines();

    // Read values into an iterator numbers
    let mut seed_vals = file_reader
        .next().unwrap()[7..]
        .split(' ').map(|x| x.parse::<usize>().unwrap());

    // Parse the numbers into SeedRanges
    let mut seeds:Vec<SeedRange> = Vec::new();
    while let Some(first_seed) = seed_vals.next(){
        seeds.push(SeedRange {
            start: first_seed,
            end: first_seed + seed_vals.next().unwrap()
        })
    }
    
    // Initialize an empty vec of ranges to grow into
    let mut maps: Vec<Vec<Mapping>> = Vec::new();
    let mut this_mapping: Vec<Mapping> = Vec::new();

    file_reader.next(); // Consume empty line of file
    
    for line in file_reader {

        // If the line is empty, move on to the next section
        if line.is_empty() {
            // Sort and collect the mappings for later use
            this_mapping.sort_unstable_by(|a,b| a.src.cmp(&b.src));
            maps.push(this_mapping);
            this_mapping = Vec::new();
            continue;
        }

        // if a line is valid then add to the mappings list
        if let Some(map) = parse_mapping(line) {
            this_mapping.push(map);
        }   
    }
    maps.push(this_mapping.clone());

    // Vec to track seed "growth" and the current seed's split
    let mut grow:Vec<SeedRange> = Vec::new();
    let mut current_seed:SeedRange;

    // For each set of mapping functions 
    for level in &maps{
        // And for each seed that has "grown"
        for seed in &seeds{
            current_seed = seed.clone();

            let mut grown = false;
            for map in level{
                match map_range(&current_seed, map) {
                    // If there are no matches for this map, grow this seed and go to the next map
                    (None, None, None) => {}, // keep going until we find a match
                    (before, overlap, after) => {
                        if let Some(new_seed) = before  { grow.push(new_seed) }
                        if let Some(new_seed) = overlap { grow.push(new_seed) }
                        if let Some(new_seed) = after {
                            // If there's still room to intersect more maps in this seed range
                            //  update the current seed and continue iterating over maps
                            current_seed.start = new_seed.start;
                        } else {
                            // We have matched and completed the range for this seed
                            //  move on to the next one
                            grown = true;
                            break;
                        }
                    }
                }
            }

            // If the seed hasn't grown at all, we still need to keep it!
            if !grown { grow.push(current_seed.clone()) }
        }
        // Once all maps in this level have been matched,
        //  propegate the growth back to the seed vector
        std::mem::swap(&mut seeds, &mut grow); // Clevel lil vector swap trick
        grow.clear();
    }
    seeds.iter().map(|range| range.start).min().unwrap()
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
    fn part2() {
        assert_eq!(solve_part2(TEST_INPUT), 46);
    }

    #[test]
    fn part1_full() {
        assert_eq!(solve_part1(FULL_INPUT), 107430936);
    }

    #[test]
    fn part2_full() {
        assert_eq!(solve_part2(FULL_INPUT), 23738616);
    }

    // Too low: 83692320
}