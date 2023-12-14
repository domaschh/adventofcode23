use std::cmp::min;

use itertools::Itertools;

fn is_power_of_two(n: usize) -> bool {
    n != 0 && (n & (n - 1) == 0)
}

fn find_symmetry_axis(ints: Vec<usize>, part2: &bool) -> Option<usize> {
    for i in 0..(ints.len() - 1) {
        let ip = i + 1;
        // 0 <= j < i+1 => 0 <= i - j <= i
        // 0 <= j < len-i-1 => i+1 <= i+1+j < len
        let mut valid = true;
        let mut flipped = *part2;
        for j in 0..(min(ip, ints.len() - ip)) {
            if ints[i - j] == ints[ip + j] {
                continue;
            }
            if flipped && is_power_of_two(ints[i - j] ^ ints[ip + j]) {
                flipped = !flipped;
                continue;
            }
            valid = false;
            break;
        }
        if valid && (!part2 || *part2 == !flipped) {
            return Some(ip);
        }
    }
    return None;
}

fn byte_slice_as_num<'a, T: IntoIterator<Item = &'a u8>>(x: T) -> usize {
    let mut num_repr = 0;
    for c in x {
        num_repr <<= 1;
        if *c == (b'#') {
            num_repr |= 1;
        }
    }
    return num_repr;
}

fn find_symmetry(pattern: &Vec<&str>, part2: &bool) -> usize {
    let pattern_smol: Vec<usize> = pattern
        .iter()
        .map(|c| byte_slice_as_num(c.as_bytes().iter()))
        .collect();
    if let Some(x) = find_symmetry_axis(pattern_smol, part2) {
        return 100 * x;
    }

    let v_bytes: Vec<&[u8]> = pattern.iter().map(|c| c.as_bytes()).collect();
    let pattern_smol_transposed: Vec<usize> = (0..(pattern[0].len()))
        .map(|i| byte_slice_as_num(v_bytes.iter().map(|v| &v[i])))
        .collect();

    find_symmetry_axis(pattern_smol_transposed, part2).expect("Not symmetric")
}

pub(crate) fn daythirteen1() -> usize {
    let patterns: Vec<Vec<&str>> = include_str!("../../inputday13.txt")
        .split("\n")
        .group_by(|&s| !s.is_empty())
        .into_iter()
        .filter_map(
            |(key, group)| {
                if key {
                    Some(group.collect())
                } else {
                    None
                }
            },
        )
        .collect();
    patterns.iter().map(|v| find_symmetry(v, &false)).sum()
}

pub(crate) fn daythirteen2() -> usize {
    let patterns: Vec<Vec<&str>> = include_str!("../../inputday13.txt")
        .split("\n")
        .group_by(|&s| !s.is_empty())
        .into_iter()
        .filter_map(
            |(key, group)| {
                if key {
                    Some(group.collect())
                } else {
                    None
                }
            },
        )
        .collect();
    patterns.iter().map(|v| find_symmetry(v, &true)).sum()
}
