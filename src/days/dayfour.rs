use std::{
    collections::{HashMap, HashSet},
};

use crate::days::utils::read_file;

pub(crate) fn dayfour1(filename: &str) -> Result<u32, String> {
    let lines: Vec<String> = read_file(filename).map_err(|_| "Error reading file")?;

    let totalworth = lines
        .iter()
        .filter_map(|line| {
            let (winning_nums, my_nums) =
                line.split_once(':')?.1.split_once('|').map(|(w, m)| {
                    (
                        w.split(' ')
                            .filter_map(|str| str::parse::<u32>(str).ok())
                            .collect::<HashSet<u32>>(),
                        m.split(' ').filter_map(|str| str::parse::<u32>(str).ok()),
                    )
                })?;
            Some(
                my_nums
                    .filter(|num| winning_nums.contains(num))
                    .enumerate()
                    .fold(
                        0,
                        |acc, (index, _)| if index == 0 { acc + 1 } else { acc * 2 },
                    ),
            )
        })
        .sum();

    Ok(totalworth)
}

pub(crate) fn dayfour2(filename: &str) -> Result<u32, String> {
    let lines: Vec<String> = read_file(filename).map_err(|_| "Error reading file")?;

    let mut cardmap: HashMap<usize, u32> = (0..lines.len()).map(|i| (i, 1)).collect();
    lines.iter().enumerate().for_each(|(i, line)| {
        let (winning_nums, my_nums) = line
            .split_once(':')
            .unwrap()
            .1
            .split_once('|')
            .map(|(w, m)| {
                (
                    w.split(' ')
                        .filter_map(|str| str::parse::<u32>(str).ok())
                        .collect::<HashSet<u32>>(),
                    m.split(' ').filter_map(|str| str::parse::<u32>(str).ok()),
                )
            })
            .unwrap();
        let matching_nums = my_nums
            .filter(|num| winning_nums.contains(num))
            .fold(0, |acc, _| acc + 1);
        let copycount = *cardmap.get(&i).unwrap_or(&1);
        (i + 1..=i + matching_nums).for_each(|nextrow| {
            cardmap
                .entry(nextrow)
                .and_modify(|val| *val += copycount)
                .or_insert(1);
        })
    });

    Ok(cardmap.values().sum())
}
