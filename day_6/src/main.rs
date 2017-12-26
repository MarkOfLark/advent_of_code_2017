extern crate aoch;
use std::cmp::Ordering;
use std::fmt;
use std::collections::HashMap;


fn main() {
    aoch::advent_of_code_combined(&common_solution);
}

#[derive(Debug, Eq)]
struct MemoryBank {
    id: u32,
    block_count: u32,
}


impl Ord for MemoryBank {
    fn cmp(&self, other: &MemoryBank) -> Ordering {
        match self.block_count.cmp(&other.block_count) {
            std::cmp::Ordering::Equal   => self.id.cmp(&other.id).reverse(),
            std::cmp::Ordering::Less    => std::cmp::Ordering::Less,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
        }
    }
}


impl PartialOrd for MemoryBank {
    fn partial_cmp(&self, other: &MemoryBank) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for MemoryBank {
    fn eq(&self, other: &MemoryBank) -> bool {
        self.id == other.id && self.block_count == other.block_count
    }
}

struct Memory {
    bank: Vec<MemoryBank>,
}

impl Memory {
    fn get_memory_initial_state(puzzle_input: &str) -> Memory {
        let block_count : Vec<u32> = puzzle_input.split_whitespace()
                                      .map(|s| s.trim())
                                      .filter(|s| !s.is_empty())
                                      .map(|s| s.parse::<u32>().unwrap())
                                      .collect();

        let mut block : Vec<MemoryBank> = Vec::new();
        for b in 0..block_count.len() {
            block.push(MemoryBank{id: b as u32, block_count: block_count[b]});
        }
        Memory {bank: block}
    }

    fn redistribute(&mut self) {
        let mut block_index = 0;
        let mut max_block_count = 0;
        let total_blocks = self.bank.len();
        for index in 0..total_blocks {
            if self.bank[index].block_count > max_block_count {
                block_index = index + 1;
                max_block_count = self.bank[index].block_count;
            }
        }

        let mut blocks_to_be_distributed = self.bank[block_index-1].block_count;
        self.bank[block_index-1].block_count = 0;
        while 0 < blocks_to_be_distributed {
            self.bank[block_index % total_blocks].block_count += 1;
            blocks_to_be_distributed -= 1;
            block_index += 1;
        }
    }
}

impl fmt::Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.bank.iter().fold("".to_string(),|acc, ref bank| acc + &bank.block_count.to_string() + "."))
    }
}


fn common_solution(puzzle_input: &str) -> (String,String) {
    let mut memory = Memory::get_memory_initial_state(puzzle_input);

    let mut redistribute_count = 0u32;

    let mut configurations = HashMap::new();
    loop {
        match configurations.insert(memory.to_string(),redistribute_count) {
            Some(count) => return (redistribute_count.to_string(), (redistribute_count-count).to_string()),
            None => {
                redistribute_count += 1;
                memory.redistribute();
            },
        }
    }
}
