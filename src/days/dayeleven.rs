use itertools::Itertools;

use super::utils::read_file;
#[derive(Debug)]
struct Coord {
    x: isize,
    y: isize,
}
pub(crate) fn dayeleven1(filepath: &str) -> Result<isize, String> {
    let lines = read_file(filepath).map_err(|_| "reading file")?;
    let mut universe: Vec<Vec<u8>> = lines.iter().map(|a| a.bytes().collect()).collect_vec();
    duplicate_dots_in_place(&mut universe);
    let galaxy_positions = find_hashes(&universe);
    // let nr_of_pairs = num_of_pairs(galaxy_positions.len());
    let mut sum_dist = 0;
    for i in 0..galaxy_positions.len() {
        for j in i + 1..galaxy_positions.len() {
            sum_dist += manhattan_distance(&galaxy_positions[i], &galaxy_positions[j])
        }
    }
    Ok(sum_dist)
}

fn manhattan_distance(p1: &Coord, p2: &Coord) -> isize {
    (p2.x - p1.x).abs() + (p2.y - p1.y).abs()
}

fn find_hashes(grid: &[Vec<u8>]) -> Vec<Coord> {
    grid.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, &ch)| {
                if ch == b'#' {
                    Some(Coord {
                        x: x as isize,
                        y: y as isize,
                    })
                } else {
                    None
                }
            })
        })
        .collect()
}

fn find_galaxies_distorted(grid: &[Vec<u8>]) -> Vec<Coord> {
    let mut empty_rows = vec![true; grid.len()];
    let mut empty_columns = vec![true; grid[0].len()];
    let mut coords = Vec::new();
    let distort_factor = 1000000;
    for (y, row) in grid.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            if ch != b'.' {
                empty_rows[y] = false;
                empty_columns[x] = false;
            }
        }
    }

    for (y, row) in grid.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            if ch == b'#' {
                let mut adjusted_x = x as isize;
                let mut adjusted_y = y as isize;
                for &is_empty in &empty_columns[..x] {
                    if is_empty {
                        adjusted_x += distort_factor - 1;
                    }
                }

                for &is_empty in &empty_rows[..y] {
                    if is_empty {
                        adjusted_y += distort_factor - 1;
                    }
                }

                coords.push(Coord {
                    x: adjusted_x,
                    y: adjusted_y,
                });
            }
        }
    }

    coords
}

pub(crate) fn dayeleven2(filepath: &str) -> Result<isize, String> {
    let lines = read_file(filepath).map_err(|_| "reading file")?;
    let universe: Vec<Vec<u8>> = lines.iter().map(|a| a.bytes().collect()).collect_vec();
    let galaxy_positions = find_galaxies_distorted(&universe);
    let mut sum_dist = 0;
    for i in 0..galaxy_positions.len() {
        for j in i + 1..galaxy_positions.len() {
            sum_dist += manhattan_distance(&galaxy_positions[i], &galaxy_positions[j])
        }
    }
    Ok(sum_dist)
}

fn duplicate_dots_in_place(galaxy: &mut Vec<Vec<u8>>) {
    // Duplicate rows containing only '.'
    let mut i = 0;
    while i < galaxy.len() {
        if galaxy[i].iter().all(|&x| x == b'.') {
            galaxy.insert(i, galaxy[i].clone());
            i += 1;
        }
        i += 1;
    }

    let columns_to_duplicate: Vec<_> = (0..galaxy[0].len())
        .filter(|&col| galaxy.iter().all(|row| row[col] == b'.'))
        .collect();

    for row in galaxy.iter_mut() {
        for &col in columns_to_duplicate.iter().rev() {
            row.insert(col, b'.');
        }
    }
}
