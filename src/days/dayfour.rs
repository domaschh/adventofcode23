use std::{collections::HashSet, fs::File, io::BufReader};

pub(crate) fn dayfour1(filename: &str) -> Result<u32, String> {
    use std::io::BufRead;
    let file = File::open(filename).map_err(|_| "Filereadding")?;
    let reader: Vec<String> = BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .collect();

    let totalworth = reader
        .iter()
        .filter_map(|line| {
            let (winning_nums, my_nums) =
                line.split_once(":")?.1.split_once("|").map(|(w, m)| {
                    (
                        w.split(" ")
                            .filter_map(|str| str::parse::<u32>(str).ok())
                            .collect::<HashSet<u32>>(),
                        m.split(" ").filter_map(|str| str::parse::<u32>(str).ok()),
                    )
                })?;
            Some(
                my_nums
                    .filter(|num| winning_nums.contains(&num))
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
