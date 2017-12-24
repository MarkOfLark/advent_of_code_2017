extern crate aoch;
extern crate integer_sqrt;

use integer_sqrt::IntegerSquareRoot;
use std::collections::HashMap;

fn main() {
    aoch::advent_of_code(&part_one_solution, &part_two_solution);
}


fn part_one_solution(input_str: &str) -> String {
    let puzzle_input = input_str.parse::<i64>().unwrap();

    // Find the square root of the input to then add 2 to it
    // and square to find the starting corner.
    let side_length = puzzle_input.integer_sqrt() + 1;
    let ring_index = side_length / 2;
    let starting_corner = (side_length + 1).pow(2);

    // Now find which side the puzzle input falls on and then find
    // which index along that side it is.
    let mut distance_along_ring = 0i64;
    for side_counter in 1..4 {
        let corner_below = starting_corner - side_counter * side_length;
        if puzzle_input > corner_below {
            let corner_above = corner_below + side_length;
            distance_along_ring = (puzzle_input - ((corner_above + corner_below) / 2)).abs();
        }
    }
    
    (distance_along_ring + ring_index).to_string()

}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct MemoryAddress {
    x : i64,
    y : i64,
}

impl std::ops::Add for MemoryAddress {
    type Output = MemoryAddress;

    fn add(self, other: MemoryAddress) -> MemoryAddress {
        MemoryAddress {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug)]
struct MemoryFiller {
    location: MemoryAddress,
    heading: [[i64; 2]; 2],
    val : i64,
}

impl MemoryFiller {
    fn new() -> MemoryFiller {
        MemoryFiller {
            location: MemoryAddress{x:0,y:0},
            heading: [[1,0],[0,1]],
            val : 1,
        }
    }

    fn advance(&mut self, mem : &HashMap<MemoryAddress, i64>) {
        // in standard coords the next address is (x+1,y) so we
        // rotate (1,0) using current heading and then add to 
        // current address
        self.location = self.location + MemoryAddress{x:self.heading[0][0],y:self.heading[1][0]};

        // Sum neighboring memory addresses to get the next value
        let neighbor : Vec<(i64,i64)> = vec![(1,1),(0,1),(-1,1),(-1,0)];
        let mut total = 0i64;

        for neighbor_coord in neighbor.iter() {
            let (x,y) = *neighbor_coord;
            match mem.get(&self.memory_address_to(x,y)) {
                Some(val) => total += *val,
                None => ()
            }
        }
        self.val = total;

        // if past the edge of the previous ring then rotate.
        let address_to_left = self.memory_address_to(0,1);
        if !mem.contains_key(&address_to_left) {
            let prev_heading : [[i64;2];2] = self.heading;
            self.heading[0][0] = prev_heading[0][1];
            self.heading[0][1] = -1*prev_heading[0][0];
            self.heading[1][0] = prev_heading[1][1];
            self.heading[1][1] = -1*prev_heading[1][0];
        }
    }

    // X and Y are always relative to standard coords. They get rotated
    // to current heading. The standard Coords are (x,y) where x, and y
    // are as below and heading is assumed to point to (1,0) from (0,0).
    //
    //         Y  (-1,1)       (0,1)       (1,1)
    //        /\
    //        |   (-1,0)       (0,0)       (1,0)
    //        |
    //        |   (-1,-1)      (0,-1)      (1,-1)
    //        |
    //        --------------------------------> X
    //
    fn memory_address_to(&self, x : i64, y : i64) -> MemoryAddress {
        let rel_addr = MemoryAddress{x: x*self.heading[0][0] + y*self.heading[0][1], y: x*self.heading[1][0] + y*self.heading[1][1]};
        rel_addr + self.location
    }
}


fn part_two_solution(input_str: &str) -> String {
    let puzzle_input = input_str.parse::<i64>().unwrap();
    
    let mut memory = HashMap::new();
    let mut filler = MemoryFiller::new();

    while puzzle_input > filler.val {
        memory.insert(filler.location,filler.val);
        filler.advance(&memory);
    }

    filler.val.to_string()
}