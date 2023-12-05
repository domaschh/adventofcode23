use std::fs::File;
use std::io::BufReader;

use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

#[derive(Debug)]
struct Smap {
    from_l: i64,
    to_l: i64,
    range: i64,
}

pub(crate) fn dayfive1(filename: &str) -> Result<i64, String> {
    use std::io::BufRead;
    let file = File::open(filename).map_err(|_| "Sa")?;
    let inputlines: Vec<String> = BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .filter(|l| !l.is_empty())
        .collect();
    let seeds = inputlines
        .first()
        .map(|string| {
            itertools::Itertools::collect_vec(
                string
                    .split(" ")
                    .into_iter()
                    .skip(1)
                    .filter_map(|s| s.parse::<i64>().ok()),
            )
        })
        .ok_or("seedparsing failed")?;
    let mut lastmap = "seeds".to_string();
    let mut ssm: Vec<Smap> = vec![];
    let mut sfm: Vec<Smap> = vec![];
    let mut fwm: Vec<Smap> = vec![];
    let mut wlm: Vec<Smap> = vec![];
    let mut ltm: Vec<Smap> = vec![];
    let mut thm: Vec<Smap> = vec![];
    let mut hlm: Vec<Smap> = vec![];
    for line in inputlines {
        match &*line {
            m @ "seed-to-soil map:" => {
                lastmap = m.to_string();
                continue;
            }
            m @ "soil-to-fertilizer map:" => {
                lastmap = m.to_string();
                continue;
            }
            m @ "fertilizer-to-water map:" => {
                lastmap = m.to_string();
                continue;
            }
            m @ "water-to-light map:" => {
                lastmap = m.to_string();
                continue;
            }
            m @ "light-to-temperature map:" => {
                lastmap = m.to_string();
                continue;
            }
            m @ "temperature-to-humidity map:" => {
                lastmap = m.to_string();
                continue;
            }
            m @ "humidity-to-location map:" => {
                lastmap = m.to_string();
                continue;
            }
            _ => {}
        };
        match &*lastmap {
            "seed-to-soil map:" => ssm.push(parse_line_to_map(&line)),
            "soil-to-fertilizer map:" => sfm.push(parse_line_to_map(&line)),
            "fertilizer-to-water map:" => fwm.push(parse_line_to_map(&line)),
            "water-to-light map:" => wlm.push(parse_line_to_map(&line)),
            "light-to-temperature map:" => ltm.push(parse_line_to_map(&line)),
            "temperature-to-humidity map:" => thm.push(parse_line_to_map(&line)),
            "humidity-to-location map:" => hlm.push(parse_line_to_map(&line)),
            _ => {}
        }
    }

    let min = seeds
        .iter()
        .map(|&seed| {
            let soil = find_in_map(&ssm, seed);
            let fertilizer = find_in_map(&sfm, soil);
            let water = find_in_map(&fwm, fertilizer);
            let light = find_in_map(&wlm, water);
            let temperature = find_in_map(&ltm, light);
            let humidity = find_in_map(&thm, temperature);
            let location = find_in_map(&hlm, humidity);
            location
        })
        .min();

    Ok(min.unwrap_or(0))
}

fn find_in_map(map: &Vec<Smap>, input_val: i64) -> i64 {
    map.iter()
        .find(|entry| input_val >= entry.from_l && input_val <= entry.from_l + entry.range - 1)
        .map(|e| (input_val - e.from_l) + e.to_l)
        .unwrap_or(input_val)
}

fn parse_line_to_map(input: &str) -> Smap {
    let mut spliterator = input.split(" ");
    Smap {
        to_l: spliterator
            .next()
            .and_then(|str| str.parse::<i64>().ok())
            .unwrap_or(0),
        from_l: spliterator
            .next()
            .and_then(|str| str.parse::<i64>().ok())
            .unwrap_or(0),
        range: spliterator
            .next()
            .and_then(|str| str.parse::<i64>().ok())
            .unwrap_or(0),
    }
}

pub(crate) fn dayfive2(filename: &str) -> Result<i64, String> {
    use std::io::BufRead;
    let file = File::open(filename).map_err(|_| "Sa")?;
    let inputlines: Vec<String> = BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .filter(|l| !l.is_empty())
        .collect();
    let seeds: Vec<i64> = inputlines
        .first()
        .map(|string| {
            string
                .split(" ")
                .into_iter()
                .skip(1)
                .filter_map(|s| s.parse::<i64>().ok())
        })
        .ok_or("dasd")?
        .collect();
    let pairs: Vec<_> = seeds
        .chunks_exact(2)
        .map(|chunk| {
            let from = chunk[0];
            let to = chunk[0] + chunk[1];
            [from, to]
        })
        .collect();
    let mut lastmap = "seeds".to_string();
    let mut ssm: Vec<Smap> = vec![];
    let mut sfm: Vec<Smap> = vec![];
    let mut fwm: Vec<Smap> = vec![];
    let mut wlm: Vec<Smap> = vec![];
    let mut ltm: Vec<Smap> = vec![];
    let mut thm: Vec<Smap> = vec![];
    let mut hlm: Vec<Smap> = vec![];
    for line in inputlines {
        match &*line {
            m @ "seed-to-soil map:" => {
                lastmap = m.to_string();
                continue;
            }
            m @ "soil-to-fertilizer map:" => {
                lastmap = m.to_string();
                continue;
            }
            m @ "fertilizer-to-water map:" => {
                lastmap = m.to_string();
                continue;
            }
            m @ "water-to-light map:" => {
                lastmap = m.to_string();
                continue;
            }
            m @ "light-to-temperature map:" => {
                lastmap = m.to_string();
                continue;
            }
            m @ "temperature-to-humidity map:" => {
                lastmap = m.to_string();
                continue;
            }
            m @ "humidity-to-location map:" => {
                lastmap = m.to_string();
                continue;
            }
            _ => {}
        };
        match &*lastmap {
            "seed-to-soil map:" => ssm.push(parse_line_to_map(&line)),
            "soil-to-fertilizer map:" => sfm.push(parse_line_to_map(&line)),
            "fertilizer-to-water map:" => fwm.push(parse_line_to_map(&line)),
            "water-to-light map:" => wlm.push(parse_line_to_map(&line)),
            "light-to-temperature map:" => ltm.push(parse_line_to_map(&line)),
            "temperature-to-humidity map:" => thm.push(parse_line_to_map(&line)),
            "humidity-to-location map:" => hlm.push(parse_line_to_map(&line)),
            _ => {}
        }
    }

    let min = pairs
        .par_iter()
        .flat_map(|&range| {
            (range[0]..range[1])
                .into_par_iter()
                .map(|seed| {
                    let soil = find_in_map(&ssm, seed);
                    let fertilizer = find_in_map(&sfm, soil);
                    let water = find_in_map(&fwm, fertilizer);
                    let light = find_in_map(&wlm, water);
                    let temperature = find_in_map(&ltm, light);
                    let humidity = find_in_map(&thm, temperature);
                    let location = find_in_map(&hlm, humidity);
                    location
                })
                .min()
        })
        .min();

    Ok(min.unwrap_or(0))
}
