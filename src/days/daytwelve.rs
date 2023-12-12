use itertools::Itertools;

pub(crate) fn daytwelve1() -> usize {
    let bytes = include_str!("../../inputday12.txt");
    let line_bytes: Vec<&str> = bytes.split(|b| b == '\n').collect_vec();
    let mut sum = 0;
    for line in line_bytes.iter().filter(|l| !l.is_empty()) {
        let (spring_str, num_s) = line.split_once(' ').unwrap();
        let nums = num_s.split(',').map(|a| a.parse::<usize>().unwrap());
        sum += calc_combinations(spring_str, nums)
    }
    sum
}

pub(crate) fn daytwelve2() -> usize {
    let bytes = include_str!("../../inputday12.txt");
    let line_bytes: Vec<&str> = bytes.split(|b| b == '\n').collect_vec();
    let mut sum = 0;
    for line in line_bytes.iter().filter(|l| !l.is_empty()) {
        let (spring_str, num_s) = line.split_once(' ').unwrap();
        let nums = num_s.split(',').map(|a| a.parse::<usize>().unwrap());
        let nums = nums.collect_vec();
        let size = nums.len();
        let spring = std::iter::once(spring_str).cycle().take(5).join("?");
        sum += calc_combinations(&spring, nums.into_iter().cycle().take(size * 5))
    }
    sum
}

fn calc_combinations(spring: &str, nums: impl Iterator<Item = usize>) -> usize {
    let spring = std::iter::once('.').chain(spring.chars()).collect_vec();

    let mut dp = vec![0; spring.len() + 1];
    dp[0] = 1;

    for (i, _) in spring.iter().take_while(|&&c| c != '#').enumerate() {
        dp[i + 1] = 1;
    }

    for num in nums {
        let mut n_dp = vec![0; spring.len() + 1];
        let mut match_len = 0;

        for (i, &curr_c) in spring.iter().enumerate() {
            if matches!(curr_c, '?' | '#') {
                match_len += 1;
            } else {
                match_len = 0;
            }

            if curr_c != '#' {
                n_dp[i + 1] += n_dp[i];
            }

            if match_len >= num && spring[i - num] != '#' {
                n_dp[i + 1] += dp[i - num];
            }
        }

        dp = n_dp;
    }

    *dp.last().unwrap()
}
