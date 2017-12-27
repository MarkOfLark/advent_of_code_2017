extern crate aoch;
extern crate indextree;
use std::cmp::Ordering;
use std::fmt;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Index;


fn main() {
    aoch::advent_of_code_combined(&common_solution);
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Program {
    name: String,
    weight: u32,
}

fn get_program_tower<'a>(puzzle_input: &str, mut program_tower: &'a mut indextree::Arena<Program>) -> &'a indextree::Node<Program> {
    let mut program_list = HashMap::new();
    let mut node_list = HashMap::new();
    let mut program_name = HashSet::new();

    for line in puzzle_input.lines() {
        let bare_line = line.chars()
            .map(|c: char| match c {
                '('|')'|'-'|'>'|',' => ' ',
                _ => c
            })
            .collect::<String>();

        let token : Vec<&str> = bare_line.split_whitespace()
                             .map(|s| s.trim())
                             .filter(|s| !s.is_empty())
                             .collect();

        let name = token[0].to_string();
        let weight = token[1].parse::<u32>().expect("Weight invalid");
        let mut sub_program = Vec::new();
        for idx in 2..token.len() {
            sub_program.push(token[idx].to_string());
        }

        program_name.insert(name.clone());
        program_list.insert(name.clone(),sub_program);
        node_list.insert(name.clone(),program_tower.new_node(Program{name: name, weight: weight}));
    }

    for (prog,sub_prog) in program_list.iter() {
        match node_list.get(prog) {
            Some(node) => {
                for sub in sub_prog.iter() {
                    match node_list.get(sub) {
                        Some(sub_node) => {
                            node.append(*sub_node, &mut program_tower);
                            program_name.remove(sub);
                        },
                        None => panic!("Failed to get sub node by name"),
                    }
                }
            },
            None => panic!("Failed to get node by name")
        }
    }

    // At this point program_name should only have one item which will be the root node
    if 1 != program_name.len() {
        panic!("Did not find a root node");
    }
    
    let root_name = program_name.iter().take(1).next().expect("Failed to extract root name from program name list");
    match node_list.get(root_name) {
        Some(rn) => program_tower.index(*rn),
        None     => panic!("Failed to return root node"),
    }
}

fn common_solution(puzzle_input: &str) -> (String,String) {
    let mut program_tower = indextree::Arena::new();
    let root = get_program_tower(puzzle_input,&mut program_tower);

    (root.data.name.clone(),"".to_string())
}
