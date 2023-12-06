use super::utils::read_file;

#[derive(Debug, Clone)]
struct RaceToBeat {
    time: i32,
    distance: i32,
}

pub(crate) fn daysix1(filename: &str) -> Result<i32, String> {
    let lines = read_file(filename).map_err(|_| "Reading File")?;
    let races_to_beat = races_from_lines(&lines)?;
    Ok(races_to_beat
        .map(|race| {
            (0..=race.time)
                .map(|time_held| time_held * (race.time - time_held))
                .filter(|&d_traveled| d_traveled > race.distance)
                .count() as i32
        })
        .product())
}

fn races_from_lines<'vl>(
    input: &'vl Vec<String>,
) -> Result<impl Iterator<Item = RaceToBeat> + 'vl, String> {
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

pub(crate) fn daysix2(filename: &str) -> Result<i32, String> {
    Ok(1)
}
