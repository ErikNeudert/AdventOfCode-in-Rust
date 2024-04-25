use std::fs::File;
use std::io::{prelude::*, BufReader};
// use std::io::{BufReader};
use std::collections::HashMap;
use std::fmt;
use peak_alloc::PeakAlloc;

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;

const DEBUG_PRINTLN: bool = false;

fn main() -> std::io::Result<()> {
    let file = File::open("res/day5_1.txt")?;
    let reader = BufReader::new(file);
    let lines = reader.lines()
        .map(|line| match line {
            Ok(line) => line,
            Err(e) => panic!("Error reading line: {}", e)
        });
    let almanac: Almanac = parse_almanac(Box::new(lines));

    let mut location_destinations: Vec<Vec<TruncatingRange>> = vec![];
    for (from, len) in almanac.seeds.clone() {
        let to = from + len - 1;
        debug!("calc from-to {}-{}", from, to);
        let dest: Vec<TruncatingRange> = find_destination_range(&almanac, ("seed", "location"), TruncatingRange(from, to));
        debug!("{:?}", dest);
        location_destinations.push(dest);
    }

    println!("location_destinations: {:?}", location_destinations);
    
    let min_location: usize = location_destinations.into_iter()
        .flat_map(|vec| vec.into_iter())
        .map(|range| range.0) //map to from
        .min().expect("Should have found min.");
    println!("min location: {:?}", min_location);


	let current_mem = PEAK_ALLOC.current_usage_as_mb();
	println!("This program currently uses {} MB of RAM.", current_mem);
	println!("The max amount that was used:");
	let peak_mem = PEAK_ALLOC.peak_usage_as_kb();
	println!(" - {} KB", peak_mem);
	let peak_mem = PEAK_ALLOC.peak_usage_as_mb();
	println!(" - {} MB", peak_mem);

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

#[macro_export]
macro_rules! debug {
    ($($x:tt)*) => { if DEBUG_PRINTLN { println!($($x)*); } }
}


//only works single, e.g.
fn find_destination_range(almanac: &Almanac, from_to: (&str, &str), start_range: TruncatingRange) -> Vec<TruncatingRange> {
    let bridges: Vec<(&str, &str)> = find_bridging_maps(almanac, from_to.0, from_to.1);

    let mut current_source_range: Vec<TruncatingRange> = vec![start_range];
    //now iterate all bridges, and find the destination map for it,
    //add all dest/to values
    for (from, to) in bridges {
        let map = match almanac.maps.get(from) {
            Some(map) => map,
            None => panic!("No Map defined for {}", from)
        };
        debug!("briding: {} - {}", from, to);
        //ask the map for each range for all intersections
        let mapped_ranges: Vec<TruncatingRange> = current_source_range.into_iter()
            .map(|range| map.calculate_intersecting_ranges(range))
            .flat_map(|vec| vec.into_iter())
            .collect();
        current_source_range = mapped_ranges;
    }

    current_source_range
}

//truncating ranges can be truncated using AlmanecRanges, resulting in zero to multiple extra ranges
#[derive(PartialEq, Eq)]
struct TruncatingRange(usize, usize);
impl fmt::Debug for TruncatingRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "tr({}, {})", self.0, self.1)
    }
}


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
    fn calculate_intersecting_ranges(&self, input_range: TruncatingRange) -> Vec<TruncatingRange> {
        let mut result: Vec<TruncatingRange> = vec![];
        //check each self.range
        let mut current_unhandled_ranges: Vec<TruncatingRange> = vec![input_range];
        for dest_range in &self.conversion_ranges {
            //for intersection with input.range
            if current_unhandled_ranges.is_empty() {
                break;
            }
            // if let Some((handled_ranges, unhandled_ranges)) = dest_range.truncate_ranges(current_unhandled_ranges) {
            let (mut handled_ranges, unhandled_ranges) = dest_range.truncate_ranges(current_unhandled_ranges);
            //get the intersection split result and append it to the result.
            //handled_ranges contains all resulting TruncatingRanges, mapped from Source to Destination
            result.append(&mut handled_ranges);
            //unhandled_ranges can be empty
            current_unhandled_ranges = unhandled_ranges;
        }
        //at the end, append all unhandled ranges, as their mapped 1-1
        result.append(&mut current_unhandled_ranges);
        return result;
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
    // soil 51 = seed 99xr
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

    
    //cases: i - input, s - this
    //1. ...iiii... equal ->            3-6
    //   ...ssss...                     3-6
    //
    //2. ..iiii.... left shift          2-5
    //   ....ssss..                     4-7
    //
    //3. ....iiii.. right shift         4-7
    //   ..ssss....                     2-5
    //
    //4. ...iiii... total included      3-6
    //   ..ssssss..                     2-7
    //
    //5. ..iiiiii.. partly included     2-7
    //   ...ssss...                     3-6
    //
    //6. .....iii.. not included
    //   ..sss.....
    //
    //7. ..iii..... not included
    //   .....sss..
    //holy shit, it calculated the correct answer in 1h18m xD
    fn truncate_ranges(&self, input_ranges: Vec<TruncatingRange>) -> (Vec<TruncatingRange>, Vec<TruncatingRange>) {
        let mut handled_ranges: Vec<TruncatingRange> = vec![];
        let mut unhandled_ranges: Vec<TruncatingRange> = vec![];

        let diff: i64 = (self.destination_range_start as i64) - (self.source_range_start as i64);
        debug!(" +> diff: {}", diff);
        //subtract 1, as the length is the total length, cause start is inclusive
        let source_range_end = self.source_range_start + self.range_length - 1; 
        for unhandled_range in input_ranges {
            debug!("  unhandled range: {:?}", unhandled_range);
            //get intersection part
            let TruncatingRange(in_from, in_to) = unhandled_range;
            //if one of in_ numbers is inside the source range, 
            if in_from >= self.source_range_start && in_from < source_range_end {
                //case 1, 3, 4
                if in_to >= self.source_range_start && in_to <= source_range_end {
                    debug!(" case 1/4:");
                    //case 1, 4 -> transform range numbers, no splitting
                    let new_range = TruncatingRange(((in_from as i64) + diff) as usize, ((in_to as i64) + diff) as usize);
                    debug!("   - handled:   {:?}", new_range);
                    handled_ranges.push(new_range);
                } else {
                    debug!(" case 3:");
                    //case 3 -> unhandled_range overlapps this.range to the right!
                    //split into two ranges:
                    //handled: in_from - source_range_end (inclusive)
                    let new_range = TruncatingRange(((in_from as i64) + diff) as usize, ((source_range_end as i64) + diff) as usize);
                    let new_unhandled = TruncatingRange(source_range_end + 1, in_to);
                    debug!("   - handled:   {:?}", new_range);
                    debug!("   - unhandled: {:?}", new_unhandled);
                    handled_ranges.push(new_range);
                    //unhandled: source_range_end (exclusive) - in_to
                    //shift + 1 to the right, as the unhandled "from" is exclusive
                    unhandled_ranges.push(new_unhandled);
                }
            } else if in_from <= self.source_range_start && in_to >= self.source_range_start {
                //case 2, 5
                if in_to <= source_range_end {
                    debug!(" case 2:");
                    //case 2 -> unhandled_range overlapps this.range to the left!
                    //split into two ranges:
                    //unhandled: in_from - source_range_start (exclusive)
                    //shift - 1 to the left, as the unhandled "to" is exclusive
                    let new_unhandled = TruncatingRange(in_from, self.source_range_start - 1);
                    let new_range = TruncatingRange(((self.source_range_start as i64) + diff) as usize, ((in_to as i64) + diff) as usize);
                    debug!("   - handled:   {:?}", new_range);
                    debug!("   - unhandled: {:?}", new_unhandled);
                    unhandled_ranges.push(new_unhandled);
                    //handled: source_range_start (inclusive) - in_to
                    handled_ranges.push(new_range);
                } else {
                    debug!(" case 5:");
                    //case 5 -> split into two unhandled, and 1 handled
                    //combination of case 2 and 3:
                    //unhandled: in_from - source_range_start (exclusive)
                    let new_unhandled = TruncatingRange(in_from, self.source_range_start - 1);
                    let new_range = TruncatingRange(((self.source_range_start as i64) + diff) as usize, ((source_range_end as i64) + diff) as usize);
                    let new_unhandled2 = TruncatingRange(source_range_end + 1, in_to);
                    debug!("   - handled:   {:?}", new_range);
                    debug!("   - unhandled: {:?}", new_unhandled);
                    debug!("   - unhandled2:{:?}", new_unhandled2);
                    unhandled_ranges.push(new_unhandled);
                    //handled: source_range_start - source_range_end
                    handled_ranges.push(new_range);
                    //unhandled: source_range_end (exclusive) - in_t
                    unhandled_ranges.push(new_unhandled2);
                }
            } else {
                debug!(" case 6/7:");
                //case 6, 7 -> unhandled, just add to unhandled ranges
                debug!("   - unhandled: {:?}", unhandled_range);
                unhandled_ranges.push(unhandled_range);
            }
        }

        return (handled_ranges, unhandled_ranges);
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
        //    - all in range 53+8 -4:
        //    - applicable water range: 53-60 -> -4 -> 49-56
        //    - e.g. cut 57-69 with 53-60 => 57-60 and 61-69
        //    - 57-60 is mapped via -4
        //    - 61-69 is 1-1 mapped
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
        assert_eq!(TruncatingRange(81, 94), dest[0]);
        let dest: Vec<TruncatingRange> = find_destination_range(&almanac, ("seed", "fertilizer"), TruncatingRange(79, 92));
        assert_eq!(1, dest.len());
        assert_eq!(TruncatingRange(81, 94), dest[0]);
        let dest: Vec<TruncatingRange> = find_destination_range(&almanac, ("seed", "water"), TruncatingRange(79, 92));
        assert_eq!(1, dest.len());
        assert_eq!(TruncatingRange(81, 94), dest[0]);
        
        let dest: Vec<TruncatingRange> = find_destination_range(&almanac, ("seed", "soil"), TruncatingRange(55, 67));
        assert_eq!(1, dest.len());
        assert_eq!(TruncatingRange(57, 69), dest[0]);
        let dest: Vec<TruncatingRange> = find_destination_range(&almanac, ("seed", "fertilizer"), TruncatingRange(55, 67));
        assert_eq!(1, dest.len());
        assert_eq!(TruncatingRange(57, 69), dest[0]);
        let dest: Vec<TruncatingRange> = find_destination_range(&almanac, ("seed", "water"), TruncatingRange(55, 67));
        assert_eq!(2, dest.len());
        assert_eq!(TruncatingRange(53, 56), dest[0]);
        assert_eq!(TruncatingRange(61, 69), dest[1]);
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
    fn test_almanac_range_truncate_ranges() {
        let r1 = AlmanacRange::new(10, 20, 30);
        
        //seed 98 => soil 50
        let (handled_ranges, unhandled_ranges) = r1.truncate_ranges(vec![
            TruncatingRange(10, 15), //not handled
            TruncatingRange(15, 25), //only from 20
            TruncatingRange(25, 45), //complete handled
            TruncatingRange(45, 55), //onlt to 49, e.g. 50 values (20 + 30 value)
            TruncatingRange(55, 65)  //onlt to 49, e.g. 50 values (20 + 30 value)
        ]);
        assert_eq!(4, unhandled_ranges.len());
        assert_eq!(TruncatingRange(10, 15), unhandled_ranges[0]);
        assert_eq!(TruncatingRange(15, 19), unhandled_ranges[1]);
        assert_eq!(TruncatingRange(50, 55), unhandled_ranges[2]);
        assert_eq!(TruncatingRange(55, 65), unhandled_ranges[3]);
        
        //handled ranges direclty translated! -10
        assert_eq!(3, handled_ranges.len());
        println!("{:?}", handled_ranges);
        assert_eq!(TruncatingRange(10, 15), handled_ranges[0]);
        assert_eq!(TruncatingRange(15, 35), handled_ranges[1]);
        assert_eq!(TruncatingRange(35, 39), handled_ranges[2]);

        let (handled_ranges, unhandled_ranges) = r1.truncate_ranges(vec![
            TruncatingRange(10, 60), //2 unhandled, 1 handled
            TruncatingRange(20, 50), //20-49 hanlded, 50 unhandled
            TruncatingRange(20, 49), //complete handled
            TruncatingRange(19, 50), //19 and 50 unhandled, 20-49 handled
            TruncatingRange(30, 30), //only one handled
        ]);
        assert_eq!(5, unhandled_ranges.len());
        assert_eq!(TruncatingRange(10, 19), unhandled_ranges[0]);
        assert_eq!(TruncatingRange(50, 60), unhandled_ranges[1]);
        assert_eq!(TruncatingRange(50, 50), unhandled_ranges[2]);
        assert_eq!(TruncatingRange(19, 19), unhandled_ranges[3]);
        assert_eq!(TruncatingRange(50, 50), unhandled_ranges[4]);
        
        assert_eq!(5, handled_ranges.len());
        assert_eq!(TruncatingRange(10, 39), handled_ranges[0]);
        assert_eq!(TruncatingRange(10, 39), handled_ranges[1]);
        assert_eq!(TruncatingRange(10, 39), handled_ranges[2]);
        assert_eq!(TruncatingRange(10, 39), handled_ranges[3]);
        assert_eq!(TruncatingRange(20, 20), handled_ranges[4]);
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