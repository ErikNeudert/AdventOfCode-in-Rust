use std::fs::File;
use std::io::{prelude::*, BufReader};
// use std::io::{BufReader};
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let file = File::open("res/day5_1.txt")?;
    let reader = BufReader::new(file);
    let lines = reader.lines()
        .map(|line| match line {
            Ok(line) => line,
            Err(e) => panic!("Error reading line: {}", e)
        });
    let almanac: Almanac = parse_almanac(Box::new(lines));
    
    //just calculate all valid maps for each map,
    //we could theoretically implement a backtracking algorithm, but won't need it

    //first step is to view ranges differently:
    //view each seed start range independently, and check for each if it reaches the next map layer, ommit if out of range
    //valid ranges from - to
    //initial seeds: 79 - 93
    //accepted seeds: 98 - 99            => this means all seeds inside these ranges might be relevant
    //              : 50 - 97
    //mapped to soil: 50 - 51
    //              : 52 - 99
    //accepted soil : 15 - 51
    //              : 52 - 53
    //              :  0 - 14
    //

    Ok(())
//refactoring to a Range approach.
//  goal is calculating the possible ranges for each 'x-to-y map',
//  and then backtracking from the smallest possible 'location' map value. "
//
//there also seems to be a memory leak, as I am at 6GB ram right now, increasing :D
}
//question:
//get soil nr for my seed:
//98 for 50, 99 for 51
//
// almanac, "seed", "fertilizer", 79);

//only works single, e.g.
fn find_destination_range(almanac: &Almanac, from_to: (&str, &str), source_value: usize) -> Vec<TruncatingRange> {
    let bridges: Vec<(&str, &str)> = find_bridging_maps(almanac, from_to.0, from_to.1);
    //all_numbers corresponds to the values for each parameter, soil, seed etc.
    let mut all_numbers: Vec<(&str, usize)> = vec![(from_to.0, source_value)];

    let mut source_value = source_value;
    //now iterate all bridges, and find the destination map for it,
    //add all dest/to values
    for (from, to) in bridges {
        let map = match almanac.maps.get(from) {
            Some(map) => map,
            None => panic!("No Map defined for {}", from)
        };
        let dst_value = map.calculate_destination(source_value);
        all_numbers.push((to, dst_value));
        source_value = dst_value;
    }

    source_value
}

//truncating ranges can be truncated using AlmanecRanges, resulting in zero to multiple extra ranges
struct TruncatingRange(usize, usize);
//  {
//     from: usize,
//     to: usize //inclusive
// }

// impl TruncatingRange {
//     fn from_to_length(from: usize, length: usize) -> Self {
//         //subtract 1 as it's inclusive
//         Self::new(from, from + length - 1)
//     }
//     fn new(from: usize, to: usize) -> Self {
//         TruncatingRange {
//             from: from,
//             to: to
//         }
//     }
// }

struct Almanac {//almanac manager/handler
    seeds: Vec<(usize, usize)>,
    //maps source to Map providing source -> target ranges
    maps: HashMap<String, AlmanacMap>
    //not sure if Map<str, Vec<AlmanacMap>> would be required, or if these are 1-1 mappings
}

#[derive(Debug)]
struct AlmanacMap {
    from: String,
    to: String,
    conversion_ranges: Vec<AlmanacRange>
}

impl AlmanacMap {
    fn calculate_destination(&self, source: usize) -> usize {
        //if not in any range, return source
        for range in &self.conversion_ranges {
            if let Some(result) = range.maybe_calculate_destination(source) {
                return result;
            }
        }
        return source;
    }
}

#[derive(PartialEq, Eq, Debug)]
struct AlmanacRange {
    // seed-to-soil map:
    // 50 98 2
    // 50 dest / soil
    //equals
    // 98 source / seed
    // soil 50 = seed 98
    // soil 51 = seed 99
    destination_range_start: usize, 
    source_range_start: usize,
    range_length: usize
}

impl AlmanacRange {
    fn new(destination_range_start: usize, source_range_start: usize, range_length: usize) -> Self {
        Self {
            destination_range_start: destination_range_start, 
            source_range_start: source_range_start,
            range_length: range_length
        }
    }

    fn maybe_calculate_destination(&self, source: usize) -> Option<usize> {
        //if not in range, return None
        let source_range_end = self.source_range_start + self.range_length;
        if source >= self.source_range_start && source < source_range_end {
            let destination = source - self.source_range_start + self.destination_range_start;
            Some(destination)
        } else {
            None
        }
    }
}

fn find_bridging_maps<'a>(almanac: &'a Almanac, from: &'a str, to: &'a str) -> Vec<(&'a str, &'a str)> {
    let mut result: Vec<(&str, &str)> = vec![];
    
    let mut from = from;
    loop {
        let map = match almanac.maps.get(from) {
            Some(map) => map,
            None => panic!("Missing Mapping for from: {}", from)
        };
        result.push((&map.from, &map.to));
        if map.to == to {
            break;
        }
        from = &map.to;
    }

    return result;
}


//almanac is the 'newspaper' containing the crop and weather information for farmers
fn parse_almanac<'a>(mut lines: Box<dyn Iterator<Item=String>>) -> Almanac {
    let seeds_line = lines.next().expect("'seeds' line required");
    let seeds: Vec<(usize, usize)> = parse_seed_line(&seeds_line);
    assert_eq!("", lines.next().expect("expected empty line spacing"), "expected empty line");
    let maps: HashMap<String, AlmanacMap> = parse_almanac_maps(lines);
    //first line should contain 

    Almanac {
        seeds: seeds,
        maps: maps
    }
}

fn parse_almanac_maps<'a>(lines: Box<dyn Iterator<Item=String>>) -> HashMap<String, AlmanacMap> {
    let mut res: HashMap<String, AlmanacMap> = HashMap::new();
    //this shitty temp vec is required as the for loop takes ownership of the lines iter, 
    //and I can't just reuse the iter in the nested parse method parse_almanac_map (singular)
    // let mut last_key: Option<(&'a str, &'a str)> = None;
    
    let mut maps: Vec<AlmanacMap> = vec![];

    for line in lines {
        if line.ends_with("map:") {
            let (from, to) = parse_map_name(line);
            let new_map = AlmanacMap {
                from: from,
                to: to,
                conversion_ranges: vec![]
            };
            maps.push(new_map);
            // res.insert((&new_map.from, &new_map.to), new_map);
            // last_key = Some((&from, &to));
        } else if line.is_empty() {
            //new map starts, clear last_key
            // last_key = None;
        } else if line.starts_with(|c: char| c.is_numeric()) {
            //it's a range
            if let Some(map) = maps.last_mut() {
                let range = parse_range(line);
                map.conversion_ranges.push(range);
            } else {
                panic!("There was no last map to add to, line: {}", line);
            }
        } else {
            panic!("Line should end with 'map:' but was '{}'", line);
        }
    }

    for map in maps {
        let from = map.from.clone();
        let prev_val = res.insert(from.clone(), map);
        if prev_val.is_some() {
            panic!("Unhandled case, key '{}' existed already, and was mapped to '{:?}'", from, prev_val.unwrap());
        }
    }

    return res;
}

// fn parse_almanac_map<'a>(lines: Vec<&'a str>, (from, to): (&'a str, &'a str)) -> AlmanacMap<'a> {
//     let mut conversion_ranges: Vec<AlmanacRange> = vec![];

//     for line in lines {
//         if line.is_empty() {
//             break;
//         }
//         conversion_ranges.push(parse_range(line));
//     }

//     return AlmanacMap {
//         from: from,
//         to: to,
//         conversion_ranges: conversion_ranges
//     };
// }

fn parse_range(line: String) -> AlmanacRange {
    let split: Vec<usize> = line.split_whitespace()
        .into_iter()
        .map(|str| str.parse::<usize>().expect(&format!("Could not parse {str}")))//&
        .collect();
    return AlmanacRange::new(split[0], split[1], split[2]);
}

fn parse_map_name(line: String) -> (String, String) {
    let split = line.strip_suffix("map:")
        .expect("line should be suffixed with 'map:'")
        .trim()
        .split_once("-to-")
        .expect("from and to should be separated by '-to-'");

    return (split.0.to_string(), split.1.to_string());
}

fn parse_seed_line(line: &str) -> Vec<(usize, usize)> {
    let mut number_iterator = line.strip_prefix("seeds:").expect("Line should start with 'seeds:'")
        .split_whitespace()
        .into_iter()
        .map(|str| str.parse::<usize>().expect(&format!("Could not parse {str}")))
        .collect::<Vec<usize>>()
        .into_iter();

    let mut ranges: Vec<(usize, usize)> = vec![];
    loop {
        let start = match number_iterator.next() {
            Some(val) => val,
            None => break //no more values
        }; 
        let count = match number_iterator.next() {
            Some(val) => val,
            None => panic!("Seed lines have to be dividable by two!")
        }; 
        ranges.push((start, count));
    }
    return ranges;
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_find_bridging_maps() {
        let input = "seeds: 79 14 55 13\n\
                     \n\
                     seed-to-soil map:\n\
                     50 98 2\n\
                     52 50 48\n\
                     \n\
                     soil-to-fertilizer map:\n\
                     0 15 37\n\
                     37 52 2\n\
                     39 0 15";
        let lines = Box::new(input.split("\n")
            .map(|line| line.to_string()));

        let almanac: Almanac = parse_almanac(lines);

        let bridges = find_bridging_maps(&almanac, "seed", "soil");
        assert_eq!(1, bridges.len());
        assert_eq!(("seed", "soil"), bridges[0]);
        
        let bridges = find_bridging_maps(&almanac, "seed", "fertilizer");
        assert_eq!(2, bridges.len());
        assert_eq!(("seed", "soil"), bridges[0]);
        assert_eq!(("soil", "fertilizer"), bridges[1]);
    }

    #[test]
    fn test_find_destination_range_with_briding() {
        let input = "seeds: 79 14 55 13\n\
                     \n\
                     seed-to-soil map:\n\
                     50 98 2\n\
                     52 50 48\n\
                     \n\
                     soil-to-fertilizer map:\n\
                     0 15 37\n\
                     37 52 2\n\
                     39 0 15\n\
                     \n\
                     fertilizer-to-water map:\n\
                     49 53 8\n\
                     0 11 42\n\
                     42 0 7\n\
                     57 7 4";
        let lines = Box::new(input.split("\n")
            .map(|line| line.to_string()));

        let almanac: Almanac = parse_almanac(lines);

        //difference to before:
        //don't look at each range, instead check for each range if they intersect, and create intersection ranges.

        //calculate the possible output range.
        //e.g. 79 - 92 would result in:
        // - ommitting first range "98 - 99"
        // - truncating range "50 - 97" to "79 - 92" // !! Consider that unmapped would be mapped 1 to 1 !!
        // - mapped to soil range "81 - 94"
        // - mapped to fertilizer "81 - 94"
        // - mapped to water      "81 - 94"
        
        //e.g. 55 - 67 would result in:
        // - ommitting first range "98 - 99"
        // - truncating range "50 - 97" to "55 - 67" // !! Consider that unmapped would be mapped 1 to 1 !!
        // - mapped to soil range "57 - 69"
        // - mapped to fertilizer "57 - 69"
        // all in range 53+8 -4:
        // applicable water range: 53-60 -> -4 -> 49-56
        // e.g. cut 57-69 with 53-60 => 57-60 and 61-69
        // 57-60 is mapped via -4
        // 61-69 is 1-1 mapped
        // - mapped to water ranges! "53 - 56" and "61 - 69"

        //valid ranges from - to
        //initial seeds: 79 - 93
        //accepted seeds: 98 - 99            => this means all seeds inside these ranges might be relevant
        //              : 50 - 97
        //mapped to soil: 50 - 51
        //              : 52 - 99
        //accepted soil : 15 - 51
        //              : 52 - 53
        //              :  0 - 14

        let dest: Vec<TruncatingRange> = find_destination_range(&almanac, ("seed", "soil"), TruncatingRange(79, 92));
        assert_eq!(1, dest.len());
        assert_eq!(TruncatingRange::new(81, 94), dest[0]);
        let dest: Vec<TruncatingRange> = find_destination_range(&almanac, ("seed", "fertilizer"), TruncatingRange(79, 92));
        assert_eq!(1, dest.len());
        assert_eq!(TruncatingRange::new(81, 94), dest[0]);
        let dest: Vec<TruncatingRange> = find_destination_range(&almanac, ("seed", "water"), TruncatingRange(79, 92));
        assert_eq!(1, dest.len());
        assert_eq!(TruncatingRange::new(81, 94), dest[0]);
        
        let dest: Vec<TruncatingRange> = find_destination_range(&almanac, ("seed", "soil"), TruncatingRange(55, 67));
        assert_eq!(1, dest.len());
        assert_eq!(TruncatingRange::new(57, 69), dest[0]);
        let dest: Vec<TruncatingRange> = find_destination_range(&almanac, ("seed", "fertilizer"), TruncatingRange(55, 67));
        assert_eq!(1, dest.len());
        assert_eq!(TruncatingRange::new(57, 69), dest[0]);
        let dest: Vec<TruncatingRange> = find_destination_range(&almanac, ("seed", "water"), TruncatingRange(55, 67));
        assert_eq!(2, dest.len());
        assert_eq!(TruncatingRange::new(53, 56), dest[0]);
        assert_eq!(TruncatingRange::new(61, 69), dest[1]);
    }

    #[test]
    fn test_parse_almanac() {
        let input = "seeds: 79 14 55 13\n\
                    \n\
                    seed-to-soil map:\n\
                    50 98 2\n\
                    52 50 48\n\
                    \n\
                    soil-to-fertilizer map:\n\
                    0 15 37\n\
                    37 52 2\n\
                    39 0 15";
        let lines = Box::new(input.split("\n")
            .map(|line| line.to_string()));
        
        let almanac: Almanac = parse_almanac(lines);
        assert_eq!(2, almanac.seeds.len());
        
        assert_eq!((79, 14), almanac.seeds[0]);
        assert_eq!((55, 13), almanac.seeds[1]);
        
        assert_eq!(2, almanac.maps.len());
        let map1: &AlmanacMap = almanac.maps.get(&"seed".to_string()).unwrap();
        assert_eq!("seed", map1.from);
        assert_eq!("soil", map1.to);
        assert_eq!(2, map1.conversion_ranges.len());
        assert_eq!(AlmanacRange::new(50, 98, 2), map1.conversion_ranges[0]);
        assert_eq!(AlmanacRange::new(52, 50, 48), map1.conversion_ranges[1]);
        
        let map2: &AlmanacMap = almanac.maps.get(&"soil".to_string()).unwrap();
        assert_eq!("soil", map2.from);
        assert_eq!("fertilizer", map2.to);
        assert_eq!(3, map2.conversion_ranges.len());
        assert_eq!(AlmanacRange::new(0, 15, 37), map2.conversion_ranges[0]);
        assert_eq!(AlmanacRange::new(37, 52, 2), map2.conversion_ranges[1]);
        assert_eq!(AlmanacRange::new(39, 0, 15), map2.conversion_ranges[2]);
    }

    #[test]
    fn test_almanac_map_calculate_destination() {
        let r1 = AlmanacRange::new(50, 98, 2);
        let r2 = AlmanacRange::new(52, 50, 48);
        let map = AlmanacMap {
            from: "from".to_string(),
            to: "to".to_string(),
            conversion_ranges: vec![r1, r2]
        };

        assert_eq!(50, map.calculate_destination(98));
        assert_eq!(51, map.calculate_destination(99));
        assert_eq!(55, map.calculate_destination(53));
        assert_eq!(99, map.calculate_destination(97));
        assert_eq!(1, map.calculate_destination(1));
    }

    #[test]
    fn test_almanac_range_maybe_calculate_destination() {
        let r1 = AlmanacRange::new(50, 98, 2);
        let r2 = AlmanacRange::new(52, 50, 48);

        //seed 98 => soil 50
        assert_eq!(Some(50), r1.maybe_calculate_destination(98));
        assert_eq!(Some(51), r1.maybe_calculate_destination(99));
        assert_eq!(None, r1.maybe_calculate_destination(97));

        // 52 50 48
        // "The second line means that the source range starts at 50 and contains 48 
        // values: 50, 51, ..., 96, 97. This corresponds to a destination range 
        // starting at 52 and also containing 48 values: 52, 53, ..., 98, 99. So, seed 
        // number 53 corresponds to soil number 55."
        assert_eq!(Some(55), r2.maybe_calculate_destination(53));
        assert_eq!(Some(99), r2.maybe_calculate_destination(97));
        assert_eq!(None, r2.maybe_calculate_destination(98));
        //None cases handled by the AlmanacMap
    }

    #[test]
    fn test_almanac_range() {
        let r1 = AlmanacRange::new(50, 98, 2);
        let r2 = AlmanacRange::new(52, 50, 48);

        //seed 98 => soil 50
        assert_eq!(Some(50), r1.maybe_calculate_destination(98));
        assert_eq!(Some(51), r1.maybe_calculate_destination(99));
        assert_eq!(None, r1.maybe_calculate_destination(97));

        // 52 50 48
        // "The second line means that the source range starts at 50 and contains 48 
        // values: 50, 51, ..., 96, 97. This corresponds to a destination range 
        // starting at 52 and also containing 48 values: 52, 53, ..., 98, 99. So, seed 
        // number 53 corresponds to soil number 55."
        assert_eq!(Some(55), r2.maybe_calculate_destination(53));
        assert_eq!(Some(99), r2.maybe_calculate_destination(97));
        assert_eq!(None, r2.maybe_calculate_destination(98));
        //None cases handled by the AlmanacMap
    }

    #[test]
    fn test_parse_map_name() {
        let line = "seed-to-soil map:".to_string();
        let (from, to) = parse_map_name(line);

        assert_eq!("seed", from);
        assert_eq!("soil", to);
    }

    #[test]
    fn test_parse_seed_line() {
        let line = "seeds: 79 14 55 13";
        let seeds: Vec<(usize, usize)> = parse_seed_line(line);
        assert_eq!(2, seeds.len());
        
        assert_eq!((79, 14), seeds[0]);
        assert_eq!((55, 13), seeds[1]);
    }
}