use super::utils::read_file;

#[derive(Debug, Clone)]
struct RaceToBeat {
    time: i64,
    distance: i64,
}

pub(crate) fn daysix1(filename: &str) -> Result<i64, String> {
    let lines = read_file(filename).map_err(|_| "Reading File")?;
    let races_to_beat = races_from_lines(&lines)?;
    Ok(races_to_beat
        .map(|race| {
            (0..=race.time)
                .map(|time_held| time_held * (race.time - time_held))
                .filter(|&d_traveled| d_traveled > race.distance)
                .count() as i64
        })
        .product())
}

pub(crate) fn daysix2(filename: &str) -> Result<i64, String> {
    let lines = read_file(filename).map_err(|_| "Reading File")?;
    let race_to_beat = single_race_from_line(&lines)?;
    let index_of_first_time_beat = (0..=race_to_beat.time)
        .map(|time_held| time_held * (race_to_beat.time - time_held))
        .position(|d_traveled| d_traveled > race_to_beat.distance);

    Ok(index_of_first_time_beat
        .map(|index| race_to_beat.time - 2 * index as i64 + 1)
        .unwrap_or(0)) //no times beat
}

fn races_from_lines<'vlt>(
    input: &'vlt Vec<String>,
) -> Result<impl Iterator<Item = RaceToBeat> + 'vlt, String> {
    let time_str_iter = input
        .iter()
        .nth(0)
        .ok_or("Error reading times")?
        .split(" ")
        .filter(|s| !s.is_empty());
    let distance_str_iter = input
        .iter()
        .nth(1)
        .ok_or("Error reading distances")?
        .split(" ")
        .filter(|s| !s.is_empty());

    let races_to_beat = time_str_iter
        .skip(1)
        .zip(distance_str_iter.skip(1))
        .filter_map(|(t_str, d_str)| {
            Some(RaceToBeat {
                time: t_str.parse().ok()?,
                distance: d_str.parse().ok()?,
            })
        });
    Ok(races_to_beat)
}

fn single_race_from_line(input: &Vec<String>) -> Result<RaceToBeat, String> {
    let time: i64 = input
        .iter()
        .nth(0)
        .ok_or("Error reading times")?
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .skip(1)
        .fold(0, |acc, s| {
            acc * 10i64.pow(s.len() as u32) + s.parse::<i64>().unwrap_or(0)
        });

    let distance: i64 = input
        .iter()
        .nth(1)
        .ok_or("Error reading times")?
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .skip(1)
        .fold(0, |acc, s| {
            acc * 10i64.pow(s.len() as u32) + s.parse::<i64>().unwrap_or(0)
        });

    Ok(RaceToBeat { time, distance })
}
