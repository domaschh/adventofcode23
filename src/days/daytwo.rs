use std::{collections::HashMap, fs::File, io::BufReader};

use super::utils::read_file;
pub(crate) fn daytwo1(
    filename: &str,
    red_ct: u32,
    blue_ct: u32,
    green_ct: u32,
) -> std::io::Result<u32> {
    let lines: Vec<String> = read_file(filename)?;

    let game_id_sum: u32 = lines
        .iter()
        .filter_map(|input| {
            let (fst_hlt, snd_hlft) = input.split_once(':')?;
            let (_, num) = fst_hlt.split_once(" ")?;
            let game_id = num.parse::<u32>().ok()?;
            let num_and_cols_str: Vec<&str> = snd_hlft
                .split(";")
                .flat_map(|grab| grab.split(","))
                .flat_map(|a| a.split(" "))
                .filter(|s| !s.is_empty())
                .collect();

            let all = num_and_cols_str.chunks_exact(2).all(|chunk| {
                let number: u32 = chunk
                    .first()
                    .and_then(|&num_str| num_str.parse::<u32>().ok())
                    .unwrap();
                let fits: bool = match chunk[1] {
                    "green" if number > green_ct => false,
                    "green" if number <= green_ct => true,
                    "blue" if number > blue_ct => false,
                    "blue" if number <= blue_ct => true,
                    "red" if number > red_ct => false,
                    "red" if number <= red_ct => true,
                    _ => false,
                };
                fits
            });
            if all {
                Some(game_id)
            } else {
                None
            }
        })
        .sum();

    Ok(game_id_sum)
}

pub(crate) fn daytwo2(filename: &str) -> std::io::Result<u32> {
    let lines = read_file(filename)?;

    let sum_of_min: u32 = lines
        .iter()
        .filter_map(|input| {
            let (_, snd_hlft) = input.split_once(':')?;
            let num_and_cols_str: Vec<&str> = snd_hlft
                .split(";")
                .flat_map(|grab| grab.split(","))
                .flat_map(|a| a.split(" "))
                .filter(|s| !s.is_empty())
                .collect();
            let max_val_map = num_and_cols_str
                .chunks_exact(2)
                .filter_map(|chunk| chunk[0].parse::<u32>().ok().map(|num| (chunk[1], num)))
                .fold(HashMap::new(), |mut acc, (color, num)| {
                    acc.entry(color)
                        .and_modify(|e: &mut u32| *e = (*e).max(num))
                        .or_insert(num);
                    acc
                });
            let mind_red = *max_val_map.get("red").unwrap_or(&0);
            let min_blue = *max_val_map.get("blue").unwrap_or(&0);
            let min_green = *max_val_map.get("green").unwrap_or(&0);

            Some(mind_red * min_blue * min_green)
        })
        .sum();

    Ok(sum_of_min)
}
