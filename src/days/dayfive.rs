use std::{collections::HashMap, fs::File, io::BufReader, ops::Range};

use itertools::Itertools;

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
            string
                .split(" ")
                .into_iter()
                .skip(1)
                .filter_map(|s| s.parse::<i64>().ok())
                .collect_vec()
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
        .map(|&seedvalue| {
            let soil = find_in_map(&ssm, seedvalue);
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
    let mut outputvalue = None;
    for entry in map {
        if input_val >= entry.from_l && input_val <= entry.from_l + entry.range - 1 {
            outputvalue = Some((input_val - entry.from_l) + entry.to_l);
            break;
        }
    }
    outputvalue.unwrap_or(input_val)
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

pub(crate) fn dayfive2(filename: &str) -> std::io::Result<u32> {
    use std::io::BufRead;
    let file = File::open(filename)?;
    let reader: Vec<String> = BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .collect();

    Ok(1)
}

#[test]

fn testmap() {
    let mut rangemap: HashMap<Range<u32>, Range<u32>> = HashMap::new();
    println!("Found {:?}", rangemap.get(&(7..7)));
}
