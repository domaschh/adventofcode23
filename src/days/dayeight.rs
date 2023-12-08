use std::collections::HashMap;

use gcd::Gcd;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use super::utils::read_file;

#[derive(Debug)]
struct Next<'a> {
    left: &'a str,
    right: &'a str,
}
pub(crate) fn dayeight1(filepath: &str) -> Result<i32, String> {
    let lines = read_file(filepath).map_err(|_| "Reading file")?;
    let instr_sequence_str = lines.get(0).ok_or("No first line")?;
    let jumps: HashMap<&str, Next> = lines
        .iter()
        .skip(2)
        .map(|line| {
            let mut split = line.split([' ', '(', ')', ',']);
            (
                split.nth(0).unwrap(),
                Next {
                    left: split.nth(2).unwrap(),
                    right: split.nth(1).unwrap(),
                },
            )
        })
        .collect();
    let (mut curr, mut next) = jumps.get_key_value("AAA").expect("No instruction found");
    let mut count = 0;
    for instruction in instr_sequence_str.chars().cycle() {
        if *curr == "ZZZ" {
            return Ok(count);
        }
        match instruction {
            'L' => {
                (curr, next) = jumps
                    .get_key_value(next.left)
                    .expect("No instruction found")
            }
            'R' => {
                (curr, next) = jumps
                    .get_key_value(next.right)
                    .expect("No instruction found")
            }
            _ => {}
        }
        count += 1;
    }
    Err("Instruction cannot be found something went terribly wrong".to_string())
}
pub(crate) fn dayeight2(filepath: &str) -> Result<u64, String> {
    let lines = read_file(filepath).map_err(|_| "Reading filed")?;
    let instr_sequence_str = lines.get(0).ok_or("No first line")?;
    let jumps: HashMap<&str, Next> = lines
        .iter()
        .skip(2)
        .map(|line| {
            let mut split = line.split([' ', '(', ')', ',']);
            (
                split.nth(0).unwrap(),
                Next {
                    left: split.nth(2).unwrap(),
                    right: split.nth(1).unwrap(),
                },
            )
        })
        .collect();
    let mut elements = jumps
        .keys()
        .zip(jumps.values())
        .filter(|(key, _)| key.ends_with("A"))
        .collect::<Vec<_>>();

    let mut count = 0;
    let mut f_z_encounters: Vec<Option<u64>> = vec![None; elements.len()];
    for instruction in instr_sequence_str.chars().cycle() {
        //Check if each start has already visited some z
        if f_z_encounters.iter().all(|v| v.is_some()) {
            return Ok(f_z_encounters
                .into_iter()
                .flatten()
                .reduce(|acc, v| lcm(acc, v))
                .ok_or("Empty list encountered")?);
        }
        //check if any of the start positions entered a node whch ends with z
        elements.iter().enumerate().for_each(|(i, &(element, _))| {
            if element.ends_with("Z") {
                if let Some(entry) = f_z_encounters.get_mut(i) {
                    if entry.is_none() {
                        *entry = Some(count);
                    }
                }
            }
        });
        //move node forward
        for (val, next) in elements.iter_mut() {
            let key_value_pair = if instruction == 'L' {
                jumps
                    .get_key_value(&next.left)
                    .expect("No instruction found")
            } else {
                jumps
                    .get_key_value(&next.right)
                    .expect("No instruction found")
            };
            *next = key_value_pair.1;
            *val = key_value_pair.0;
        }
        count += 1;
    }
    Ok(count)
}

fn gcd(a: u64, b: u64) -> u64 {
    a.gcd(b)
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}
