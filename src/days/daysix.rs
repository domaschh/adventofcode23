use rayon::range;

use super::utils::read_file;

#[derive(Debug, Clone)]
struct RaceToBeat {
    time: i32,
    distance: i32,
}

pub(crate) fn daysix1(filename: &str) -> Result<i32, String> {
    let lines = read_file(filename).map_err(|_| "Reading File")?;
    let races_to_beat = races_from_lines(&lines);
    Ok(races_to_beat
        .map(|race| {
            (0..=race.time)
                .map(|time_held| time_held * (race.time - time_held))
                .filter(|&d_traveled| d_traveled > race.distance)
                .fold(0, |acc, _| acc + 1)
        })
        .product())
}

fn races_from_lines(input: &Vec<String>) -> impl Iterator<Item = RaceToBeat> + '_ {
    let time_iter = input
        .iter()
        .nth(0)
        .unwrap()
        .split(" ")
        .filter(|s| !s.is_empty());
    let distance_iter = input
        .iter()
        .nth(1)
        .unwrap()
        .split(" ")
        .filter(|s| !s.is_empty());

    let races_to_beat =
        time_iter
            .skip(1)
            .zip(distance_iter.skip(1))
            .filter_map(|(t_str, d_str)| {
                Some(RaceToBeat {
                    time: t_str.parse().ok()?,
                    distance: d_str.parse().ok()?,
                })
            });
    races_to_beat
}

pub(crate) fn daysix2(filename: &str) -> Result<i32, String> {
    Ok(1)
}
