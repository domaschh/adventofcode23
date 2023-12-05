use std::{collections::HashMap, fs::File, io::BufReader};

pub(crate) fn daythree1(filename: &str) -> std::io::Result<u32> {
    use std::io::BufRead;
    let file = File::open(filename)?;
    let lines: Vec<(usize, String)> = BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .enumerate()
        .collect();

    let row_with_nums: Vec<(usize, Vec<_>)> = lines
        .iter()
        .map(|line| {
            let indices = line
                .1
                .match_indices(|c: char| c.is_digit(10))
                .collect::<Vec<_>>();
            let (_, groups) = indices.into_iter().fold(
                (None, Vec::new()),
                |(last_idx, mut groups), (curr_i, digit)| match last_idx {
                    Some((start, end)) if curr_i == end + 1 => {
                        if let Some((start, end_ref, number)) = groups.last_mut() {
                            *end_ref = curr_i;
                            *number = &line.1[*start..=*end_ref];
                        }
                        (Some((start, curr_i)), groups)
                    }
                    _ => {
                        groups.push((curr_i, curr_i, digit));
                        (Some((curr_i, curr_i)), groups)
                    }
                },
            );
            return (line.0, groups);
        })
        .collect();

    let width = lines.get(0).map(|t| t.1.len()).unwrap_or(0);
    let height = lines.len();
    let mut result: u32 = 0;
    for row in row_with_nums {
        let curr_row_index = row.0;
        'nums: for num in row.1 {
            let upperline_str: &str = lines
                .get((curr_row_index as i32 - 1).max(0) as usize)
                .map(|a| &a.1)
                .unwrap();
            let thisline_str: &str = lines.get(curr_row_index).map(|a| &a.1).unwrap();
            let lowerline_str: &str = lines
                .get((curr_row_index + 1).min(height - 1))
                .map(|a| &a.1)
                .unwrap();

            if is_sign(
                &thisline_str
                    [(num.0 as i32 - 1).max(0) as usize..=(num.0 as i32 - 1).max(0) as usize],
            ) || is_sign(&thisline_str[(num.1 + 1).min(width - 1)..=(num.1 + 1).min(width - 1)])
            {
                let found_num = num.2.parse::<u32>().unwrap();
                result += found_num;
                continue 'nums;
            }
            for i in (num.0 as i32 - 1).max(0) as usize..=(num.1 + 1).min(width - 1) {
                if is_sign(&upperline_str[i..=i]) || is_sign(&lowerline_str[i..=i]) {
                    let found_num = num.2.parse::<u32>().unwrap();
                    result += found_num;
                    continue 'nums;
                }
            }
        }
    }
    Ok(result)
}

pub(crate) fn daythree2(filename: &str) -> std::io::Result<u32> {
    use std::io::BufRead;
    let file = File::open(filename)?;
    let lines: Vec<(usize, String)> = BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .enumerate()
        .collect();

    let row_with_nums: Vec<(usize, Vec<_>)> = lines
        .iter()
        .map(|line| {
            let indices = line
                .1
                .match_indices(|c: char| c.is_digit(10))
                .collect::<Vec<_>>();
            let (_, groups) = indices.into_iter().fold(
                (None, Vec::new()),
                |(last_idx, mut groups), (curr_idx, digit)| match last_idx {
                    Some((start, end)) if curr_idx == end + 1 => {
                        if let Some((star_idx, end_idx, number)) = groups.last_mut() {
                            *end_idx = curr_idx;
                            *number = &line.1[*star_idx..=*end_idx];
                        }
                        (Some((start, curr_idx)), groups)
                    }
                    _ => {
                        groups.push((curr_idx, curr_idx, digit));
                        (Some((curr_idx, curr_idx)), groups)
                    }
                },
            );
            return (line.0, groups);
        })
        .collect();

    let width = lines.get(0).map(|t| t.1.len()).unwrap_or(0);
    let height = lines.len();
    let mut gear_ratios: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    //All Rows with numbers
    for row in row_with_nums {
        let curr_row_index = row.0;
        //Numbers and their start and end index
        'nums: for num in row.1 {
            let upperline_str: &str = lines
                .get((curr_row_index as i32 - 1).max(0) as usize)
                .map(|a| &a.1)
                .unwrap();
            let thisline_str: &str = lines.get(curr_row_index).map(|a| &a.1).unwrap();
            let lowerline_str: &str = lines
                .get((curr_row_index + 1).min(height - 1))
                .map(|a| &a.1)
                .unwrap();

            //left
            if is_gear(&thisline_str[(num.1 + 1).min(width - 1)..=(num.1 + 1).min(width - 1)]) {
                let found_num = num.2.parse::<u32>().unwrap();
                gear_ratios
                    .entry((curr_row_index, (num.1 + 1).min(width - 1)))
                    .or_insert_with(Vec::new)
                    .push(found_num);

                continue 'nums;
            }
            //right
            if is_gear(
                &thisline_str
                    [(num.0 as i32 - 1).max(0) as usize..=(num.0 as i32 - 1).max(0) as usize],
            ) {
                let found_num = num.2.parse::<u32>().unwrap();
                gear_ratios
                    .entry((curr_row_index, (num.0 as i32 - 1).max(0) as usize))
                    .or_insert_with(Vec::new)
                    .push(found_num);
                continue 'nums;
            }
            //upperlower
            for i in (num.0 as i32 - 1).max(0) as usize..=(num.1 + 1).min(width - 1) {
                //upper
                if is_gear(&upperline_str[i..=i]) {
                    let found_num = num.2.parse::<u32>().unwrap();
                    gear_ratios
                        .entry(((curr_row_index as i32 - 1).max(0) as usize, i))
                        .or_insert_with(Vec::new)
                        .push(found_num);
                    continue 'nums;
                }

                //lower
                if is_gear(&lowerline_str[i..=i]) {
                    let found_num = num.2.parse::<u32>().unwrap();
                    gear_ratios
                        .entry(((curr_row_index + 1).min(height - 1), i))
                        .or_insert_with(Vec::new)
                        .push(found_num);
                    continue 'nums;
                }
            }
        }
    }

    Ok(gear_ratios
        .values()
        .filter(|vec| vec.len() == 2)
        .map(|v| v.iter().product::<u32>())
        .sum())
}
fn is_sign(input: &str) -> bool {
    match input {
        "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "0" | "." => false,
        _ => true,
    }
}

fn is_gear(input: &str) -> bool {
    match input {
        "*" => true,
        _ => false,
    }
}
