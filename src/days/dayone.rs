

use super::utils::read_file;

pub(crate) fn dayone1(filename: &str) -> std::io::Result<u32> {
    let lines: Vec<String> = read_file(filename)?;

    let result = lines
        .iter()
        .filter_map(|line| {
            let fst_digit = line.chars().find_map(|char| char.to_digit(10))?;
            let snd_digit = line.chars().rev().find_map(|char| char.to_digit(10))?;

            Some(fst_digit * 10 + snd_digit)
        })
        .sum();

    Ok(result)
}

pub(crate) fn dayone2(filename: &str) -> std::io::Result<u32> {
    let lines: Vec<String> = read_file(filename)?;

    let result = lines
        .iter()
        .filter_map(|line| {
            let fst_digit = line
                .chars()
                .enumerate()
                .find_map(|(i, char)| to_digit_peek(char, i, line))?;
            let snd_digit = line
                .chars()
                .rev()
                .enumerate()
                .find_map(|(i, char)| to_digit_peek_rev(char, i, line))?;

            Some(fst_digit * 10 + snd_digit)
        })
        .sum();

    Ok(result)
}

fn to_digit_peek(c: char, index: usize, line: &str) -> Option<u32> {
    if let Some(digit) = c.to_digit(10) {
        return Some(digit);
    }

    match &line[index..] {
        r if r.starts_with("one") => Some(1),
        r if r.starts_with("two") => Some(2),
        r if r.starts_with("three") => Some(3),
        r if r.starts_with("four") => Some(4),
        r if r.starts_with("five") => Some(5),
        r if r.starts_with("six") => Some(6),
        r if r.starts_with("seven") => Some(7),
        r if r.starts_with("eight") => Some(8),
        r if r.starts_with("nine") => Some(9),
        _ => None,
    }
}

fn to_digit_peek_rev(c: char, index: usize, line: &str) -> Option<u32> {
    if let Some(digit) = c.to_digit(10) {
        return Some(digit);
    }
    match &line[..line.len() - index] {
        r if r.ends_with("one") => Some(1),
        r if r.ends_with("two") => Some(2),
        r if r.ends_with("three") => Some(3),
        r if r.ends_with("four") => Some(4),
        r if r.ends_with("five") => Some(5),
        r if r.ends_with("six") => Some(6),
        r if r.ends_with("seven") => Some(7),
        r if r.ends_with("eight") => Some(8),
        r if r.ends_with("nine") => Some(9),
        _ => None,
    }
}
