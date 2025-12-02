// 2025 was me bleeding out
// solutions for 2025 advent of code

use crate::read_input::{self};
use std::{collections::HashSet, ops::Deref};

pub fn day1_part1() -> Result<(), Box<dyn std::error::Error>> {
    let data = fetch_data_day1()?;
    let mut pos = 50;
    let mut result = 0;

    for dir in data {
        pos += dir;
        pos %= 100;
        if pos < 0 {
            pos += 100;
        }
        if pos == 0 {
            result += 1;
        }
    }
    println!("{}", result);

    Ok(())
}

pub fn day1_part2() -> Result<(), Box<dyn std::error::Error>> {
    let data = fetch_data_day1()?;
    let mut pos = 50;
    let mut result = 0;

    for dir in data {
        let new_pos = pos + dir;
        if dir > 0 {
            result += (new_pos / 100) - (pos / 100);
            if pos < 0 && new_pos >= 0 {
                result += 1;
            }
        } else if dir < 0 {
            result += (pos / 100) - (new_pos / 100);
            if pos > 0 && new_pos <= 0 {
                result += 1;
            }
        }
        pos = new_pos;
        pos = ((pos % 100) + 100) % 100;
        if pos == 0 && new_pos % 100 != 0 {
            result += 1;
        }
    }
    println!("{}", result);

    Ok(())
}
fn fetch_data_day1() -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let data_string = read_input::read_input("data/2025/day1.txt")?;
    let dirs = data_string
        .iter()
        .map(|s| {
            let first_char = s.chars().next();
            match first_char {
                Some('L') => -s[1..].parse::<i32>().unwrap(),
                Some('R') => s[1..].parse::<i32>().unwrap(),
                _ => 0,
            }
        })
        .collect::<Vec<i32>>();
    Ok(dirs)
}

pub fn day2_part1() -> Result<(), Box<dyn std::error::Error>> {
    let data = fetch_data_day2()?;
    let mut result = 0;
    for range in data {
        for i in range[0]..=range[1] {
            let i_string = i.to_string();
            if i_string.len().is_multiple_of(2)
                && i_string[0..i_string.len() / 2] == i_string[i_string.len() / 2..]
            {
                result += i;
            }
        }
    }
    println!("{}", result);
    Ok(())
}

pub fn day2_part2() -> Result<(), Box<dyn std::error::Error>> {
    let data = fetch_data_day2()?;
    let mut result = 0;
    let mut seen: HashSet<u64> = HashSet::new();

    for range in data {
        for i in range[0]..=range[1] {
            if seen.contains(&i) {
                continue;
            }
            let i_string = i.to_string();
            for j in 1..=i_string.len() / 2 {
                let sub_string = &i_string[..j];
                if i_string.len() % sub_string.len() == 0 {
                    let count = i_string.len() / sub_string.len();
                    if i_string == sub_string.repeat(count) {
                        result += i;
                        seen.insert(i);
                        break;
                    }
                }
            }
        }
    }
    println!("{}", result);
    Ok(())
}

fn fetch_data_day2() -> Result<Vec<Vec<u64>>, Box<dyn std::error::Error>> {
    let data_string = read_input::read_input("data/2025/day2.txt")?;
    let ranges = data_string
        .first()
        .unwrap()
        .split(',')
        .map(|s| {
            s.split('-')
                .map(|num| num.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();
    // ranges
    //     .iter()
    //     .for_each(|v| v.iter().for_each(|s| println!("{}", s)));

    Ok(ranges)
}
