use itertools::Itertools;
use std::{fs::File, io::BufReader};
pub(crate) fn daytwo1(
    filename: &str,
    red_ct: u32,
    blue_ct: u32,
    green_ct: u32,
) -> std::io::Result<u32> {
    use std::io::BufRead;
    let file = File::open(filename)?;
    let reader: Vec<String> = BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .collect();

    let game_id_sum: u32 = reader
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

#[test]
fn test_delimter() {
    let input = "7 green, 4 blue, 3 red; 4 blue, 10 red, 1 green; 1 blue, 9 red";
    let list: Vec<&str> = input
        .split(";")
        .flat_map(|grab| grab.split(","))
        .flat_map(|a| a.split(" "))
        .filter(|s| !s.is_empty())
        .collect();

    list.chunks_exact(2).for_each(|pair| {
        assert!(pair.len() == 2);
        let count = pair[0].parse::<u32>().unwrap();
    });
}
