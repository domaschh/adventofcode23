use std::iter;

use itertools::Itertools;

use super::utils::read_file;

pub(crate) fn daynine1(filepath: &str) -> Result<i32, String> {
    let lines = read_file(filepath).map_err(|_| "Error reading file")?;
    let initial_seq = lines.iter().map(|line| {
        line.split_whitespace()
            .filter_map(|num_str| num_str.parse::<i32>().ok())
    });

    let difference_sequences = initial_seq.map(|sequence| last_differences(sequence.collect_vec()));
    let mut sum: i32 = 0;
    for mut sequence in difference_sequences {
        sequence.reverse();
        for i in 1..sequence.len() {
            sequence[i] += sequence[i - 1];
        }

        sum += sequence.last().unwrap();
    }
    Ok(sum)
}
pub(crate) fn daynine2(filepath: &str) -> Result<u32, String> {
    Ok(0)
}
fn last_differences<T>(sequence: Vec<T>) -> Vec<T>
where
    T: std::ops::Sub<Output = T> + Copy + PartialEq + From<u8>,
{
    iter::successors(Some(sequence), |seq| {
        let diff_seq: Vec<T> = seq
            .iter()
            .tuple_windows()
            .map(|(&this, &next)| next - this)
            .collect();
        (!diff_seq.is_empty() && diff_seq.iter().any(|&x| x != T::from(0_u8))).then(|| diff_seq)
    })
    .filter_map(|seq| seq.last().copied())
    .collect()
}
