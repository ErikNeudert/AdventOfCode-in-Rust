use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::BTreeMap;
use std::ops::Bound::Included;
use std::iter::Peekable;
use std::io::{self};
use std::iter::Enumerate;
use std::str::Chars;

fn main() -> std::io::Result<()> {
    //read each line,
    //parse anything that is not a .,
    //e.g. numbers, +, *, # etc.
    //safe the coordinates
    //calculate adjacency

    //imageine a grid 8x10
    //ranging from 0x0 to 7x9
    //each number e.g. 467 at 0x0 has adjacent fields,
    //to calculate them we draw a box around that field 

    //The schematic (the sheet 8x10) contains PartNumbers and Symbols
    //each SchematicElement has a (enum) Type PartNumbers or Symbols
    //a length and a coordinate Point (x=column, y=line)

    //add up the IDs of the games that would have been possible, you get 8.
        
    let file = File::open("res/day3_1.txt")?;
    let reader = BufReader::new(file);
    let mut map = BTreeMap::new();

    fill_map_from_text(reader, &mut map);

    //(y, x)
    // map.insert(Point::new(0, 0), "467");
    // map.insert(Point::new(0, 5), "114");
    // map.insert(Point::new(1, 3), "*");
    // map.insert(Point::new(2, 2), "35");
    // map.insert(Point::new(2, 6), "633");
    // map.insert(Point::new(3, 6), "#");
    // map.insert(Point::new(3, 3), "a");
    // map.insert(Point::new(3, 4), "b");
    // map.insert(Point::new(3, 5), "c");
    for (&key, value) in map.range((Included(Point::new(1, 1)), Included(Point::new(3, 4)))) {
        println!("{:?}x{:?}: {:?}", key.y, key.x, value);
    }
    // assert_eq!(Some((&5, &"b")), map.range(4..).next());
    
    Ok(())
}

fn fill_map_from_text(reader: BufReader<File>, map: &mut BTreeMap<Point, Token>) -> Result<(), io::Error> {
    for (y, line) in reader.lines().enumerate() {
        let line: String = match line {
            Ok(line) => line,
            Err(e) => panic!("Error reading line {}", e)
        };
        let mut char_iter: Peekable<Enumerate<Chars>> = line.chars().enumerate().peekable();

        loop {
            let (x, char) = match &char_iter.peek() {
                //it's very important to dereference the usize and char here,
                //as it's a double mutable borrow else, as we'd have a mutable reference to x / idx
                Some((idx, char)) => (*idx, *char),
                None => break
            };
            if char == '.' {
                //skip the dot
                char_iter.next(); 
                continue;
            }

            let token: Token;
            if char.is_numeric() {
                //e.g. 123 in ...123#..
                token = extract_numeric(&mut char_iter);
            } else {
                //e.g. # in ...123#..
                token = extract_symbol(&mut char_iter);
            }
            if let Some(old_value) = map.insert(Point::new(y, x), token) {
                panic!("Duplicate value in map, should not happen! {:?}", old_value);
            }
        }
    }
    Ok(())
}

// fn extract_numeric(iterator: Box<dyn Iterator<Item = (usize, char)>>) -> Token {
fn extract_numeric(iterator: &mut Peekable<Enumerate<Chars>>) -> Token {
    let result: String = String::new();

    Token {
        token_type: TokenType::NUMERIC,
        value: result
    }
}

// fn extract_symbol(iterator: Box<dyn Iterator<Item = (usize, char)>>) -> Token {
fn extract_symbol(iterator: &mut Peekable<Enumerate<Chars>>) -> Token {
    let result: String = String::new();

    Token {
        token_type: TokenType::SYMBOL,
        value: result
    }
}

#[derive(Debug)]
enum TokenType {
    NUMERIC,
    SYMBOL
}
#[derive(Debug)]
struct Token {
    token_type: TokenType,
    value: String
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
struct Point {
    x: usize,
    y: usize
}

impl Point {
    fn new(y: usize, x: usize) -> Self {
        Point {y, x}
    }

    fn surrounding_range(&self, length: usize) -> (Point, Point) {
        (Point::new(self.y - 1, self.x - 1), Point::new(self.y + 1, self.x + length))
    }
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_map_range() {
        let mut map = BTreeMap::new();
        //inserting the following map:
        // 467..114..
        // ...*......
        // ..35..633.
        // ...abc#...
        let interesting_point = Point::new(2, 2);

        map.insert(Point::new(0, 0), "467");
        map.insert(Point::new(0, 5), "114");
        map.insert(Point::new(1, 3), "*");
        map.insert(interesting_point, "35");
        map.insert(Point::new(2, 6), "633");
        map.insert(Point::new(3, 6), "#");
        map.insert(Point::new(3, 3), "a");
        map.insert(Point::new(3, 4), "b");
        map.insert(Point::new(3, 5), "c");

        //now we want all elements surrounding "35"
        let surrounding_range: (Point, Point) = interesting_point.surrounding_range(2);
        assert_eq!(Point::new(1, 1), surrounding_range.0);
        assert_eq!(Point::new(3, 4), surrounding_range.1);

        let map_range: BTreeMap<Point, &str> = map.range((Included(surrounding_range.0), Included(surrounding_range.1)))
            .map(|(ptr, str)| (*ptr, *str)) //not sure why I have to de-reference them here
            .fold(BTreeMap::new(), |mut map, (k, v)| {
                map.insert(k, v);
                return map;
            });

        assert_eq!(Some("35"), map_range.get(&Point::new(2, 2)).copied());
        assert_eq!(Some("*"), map_range.get(&Point::new(1, 3)).copied());
        assert_eq!(Some("a"), map_range.get(&Point::new(3, 3)).copied());
        assert_eq!(Some("b"), map_range.get(&Point::new(3, 4)).copied());
    }
}