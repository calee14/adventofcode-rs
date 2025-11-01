// 2024 was a heartbreak
// solutions for 2024 advent of code

use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use crate::read_input::{self};

pub fn day1_part1() -> Result<(), Box<dyn std::error::Error>> {
    let (mut v1, mut v2) = fetch_data_day1()?;

    v1.sort();
    v2.sort();

    let mut sum = 0;
    for i in 0..v1.len() {
        sum += (v1.get(i).unwrap() - v2.get(i).unwrap()).abs();
    }

    println!("{}", sum);

    println!("hello there");
    Ok(())
}

pub fn day1_part2() -> Result<(), Box<dyn std::error::Error>> {
    let (v1, v2) = fetch_data_day1()?;

    let mut data_map: HashMap<i32, i32> = HashMap::new();

    v2.iter().for_each(|num| {
        let key = data_map.entry(*num).or_insert(0);
        *key += 1;
    });

    let mut res = 0;
    v1.iter().for_each(|num| {
        res += num * data_map.get(num).unwrap_or(&0);
    });

    println!("{}", res);

    Ok(())
}

fn fetch_data_day1() -> Result<(Vec<i32>, Vec<i32>), Box<dyn std::error::Error>> {
    let data_string = read_input::read_input("data/day1.txt")?;
    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();

    data_string.iter().for_each(|d| {
        let input = d
            .split("   ")
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| s.parse::<i32>().ok().unwrap())
            .collect::<Vec<i32>>();
        v1.push(*input.first().unwrap());
        v2.push(*input.get(1).unwrap());
    });

    Ok((v1, v2))
}

pub fn day2_part1() -> Result<(), Box<dyn std::error::Error>> {
    let v1 = fetch_data_day2()?;
    let mut count = 0;
    v1.iter().for_each(|report| {
        let dir = if report[0] > report[1] { 0 } else { 1 };
        let mut valid = true;
        for (i, level) in report.iter().enumerate().take(report.len() - 1) {
            let next = report.get(i + 1).unwrap();
            if !(level > next && dir == 0 && (level - next).abs() <= 3
                || level < next && dir == 1 && (level - next).abs() <= 3)
            {
                valid = false;
            }
        }
        if valid {
            count += 1;
        }
    });
    println!("{}", count);
    Ok(())
}

pub fn day2_part2() -> Result<(), Box<dyn std::error::Error>> {
    let v1 = fetch_data_day2()?;
    let mut count = 0;
    v1.iter().for_each(|report| {
        for j in 0..report.len() {
            let mut report_ = report.clone();
            report_.remove(j);
            let dir = if report_[0] > report_[1] { 0 } else { 1 };
            let mut valid = true;
            for (i, level) in report_.iter().enumerate().take(report_.len() - 1) {
                let next = report_.get(i + 1).unwrap();
                if !(level > next && dir == 0 && (level - next).abs() <= 3
                    || level < next && dir == 1 && (level - next).abs() <= 3)
                {
                    valid = false;
                }
            }
            if valid {
                count += 1;
                break;
            }
        }
    });
    println!("{}", count);
    Ok(())
}

fn fetch_data_day2() -> Result<Vec<Vec<i32>>, Box<dyn std::error::Error>> {
    let data_string = read_input::read_input("data/day2.txt")?;
    let mut data: Vec<Vec<i32>> = Vec::new();

    data_string.iter().for_each(|d| {
        let line = d
            .split(" ")
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| s.parse::<i32>().ok().unwrap())
            .collect::<Vec<i32>>();

        data.push(line);
    });

    Ok(data)
}

fn is_digit(c: char) -> bool {
    c.is_numeric()
}
//xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
pub fn day3_part1() -> Result<(), Box<dyn std::error::Error>> {
    let v = fetch_data_day3()?;
    let mut result = 0;
    for instr in v.iter() {
        let mut i = 0;
        while i < instr.len() - 4 {
            if instr[i..i + 4] == *"mul(" {
                i += 4;
                let mut num1 = String::from("");
                while i < instr.len() && is_digit(instr.chars().nth(i).unwrap()) && num1.len() <= 3
                {
                    num1.push(instr.chars().nth(i).unwrap());
                    i += 1;
                }
                if i < instr.len() && (num1.is_empty() || instr.chars().nth(i).unwrap() != ',') {
                    continue;
                }
                i += 1;
                let mut num2 = String::from("");
                while i < instr.len() && is_digit(instr.chars().nth(i).unwrap()) && num2.len() <= 3
                {
                    num2.push(instr.chars().nth(i).unwrap());
                    i += 1;
                }
                if i < instr.len() && (num2.is_empty() || instr.chars().nth(i).unwrap() != ')') {
                    continue;
                }
                result += num1.parse::<i32>()? * num2.parse::<i32>()?;
            }
            i += 1;
        }
    }

    println!("{}", result);

    Ok(())
}

pub fn day3_part2() -> Result<(), Box<dyn std::error::Error>> {
    let v = fetch_data_day3()?;
    let mut result = 0;

    let mut doing = true;
    for instr in v.iter() {
        let mut i = 0;
        while i < instr.len() - 4 {
            if instr[i..i + 4] == *"mul(" {
                i += 4;
                let mut num1 = String::from("");
                while i < instr.len() && is_digit(instr.chars().nth(i).unwrap()) && num1.len() <= 3
                {
                    num1.push(instr.chars().nth(i).unwrap());
                    i += 1;
                }
                if i < instr.len() && (num1.is_empty() || instr.chars().nth(i).unwrap() != ',') {
                    continue;
                }
                i += 1;
                let mut num2 = String::from("");
                while i < instr.len() && is_digit(instr.chars().nth(i).unwrap()) && num2.len() <= 3
                {
                    num2.push(instr.chars().nth(i).unwrap());
                    i += 1;
                }
                if i < instr.len() && (num2.is_empty() || instr.chars().nth(i).unwrap() != ')') {
                    continue;
                }
                if doing {
                    result += num1.parse::<i32>()? * num2.parse::<i32>()?;
                }
                continue;
            } else if instr[i..i + 4] == *"do()" {
                doing = true;
                i += 4;
                continue;
            } else if i + 7 < instr.len() && instr[i..i + 7] == *"don't()" {
                doing = false;
                i += 7;
                continue;
            }
            i += 1;
        }
    }

    println!("{}", result);

    Ok(())
}

fn fetch_data_day3() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let data_string = read_input::read_input("data/day3.txt")?;
    Ok(data_string)
}

pub fn day4_part1() -> Result<(), Box<dyn std::error::Error>> {
    let v = fetch_data_day4()?;
    let cols = v.first().unwrap().len() as i32;
    let rows = v.len() as i32;
    let dirs: [(i32, i32); 8] = [
        (0, 1),
        (1, 1),
        (1, 0),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];
    let in_range = |x: i32, y: i32| x >= 0 && x < rows && y >= 0 && y < cols;
    let xmas = "XMAS";
    let mut result = 0;
    for i in 0..cols {
        for j in 0..rows {
            for dir in dirs.iter() {
                let mut i_ = i;
                let mut j_ = j;
                let mut found = true;
                for k in 0..4 {
                    i_ += dir.0 * (if k == 0 { 0 } else { 1 });
                    j_ += dir.1 * (if k == 0 { 0 } else { 1 });
                    if in_range(j_, i_) {
                        let s = v.get(j_ as usize).unwrap();
                        let c = s.chars().nth(i_ as usize).unwrap();
                        if c != xmas.chars().nth(k as usize).unwrap() {
                            found = false;
                        }
                    } else {
                        found = false;
                    }
                }
                if found {
                    result += 1;
                }
            }
        }
    }

    println!("{}", result);
    Ok(())
}

pub fn day4_part2() -> Result<(), Box<dyn std::error::Error>> {
    let v = fetch_data_day4()?;
    let cols = v.first().unwrap().len() as i32;
    let rows = v.len() as i32;
    let in_range = |x: i32, y: i32| x >= 0 && x < rows && y >= 0 && y < cols;
    let char_at = |r: i32, c: i32| v.get(r as usize).unwrap().chars().nth(c as usize).unwrap();
    let mut result = 0;
    for i in 0..cols {
        for j in 0..rows {
            if v.get(j as usize).unwrap().chars().nth(i as usize).unwrap() == 'A' {
                let tl = (i - 1, j - 1);
                let tr = (i + 1, j - 1);
                let bl = (i - 1, j + 1);
                let br = (i + 1, j + 1);
                if in_range(tl.1, tl.0)
                    && in_range(tr.1, tr.0)
                    && in_range(bl.1, bl.0)
                    && in_range(br.1, br.0)
                    && ((char_at(tl.1, tl.0) == 'M'
                        && char_at(tr.1, tr.0) == 'M'
                        && char_at(bl.1, bl.0) == 'S'
                        && char_at(br.1, br.0) == 'S')
                        || (char_at(tl.1, tl.0) == 'M'
                            && char_at(tr.1, tr.0) == 'S'
                            && char_at(bl.1, bl.0) == 'M'
                            && char_at(br.1, br.0) == 'S')
                        || (char_at(tl.1, tl.0) == 'S'
                            && char_at(tr.1, tr.0) == 'S'
                            && char_at(bl.1, bl.0) == 'M'
                            && char_at(br.1, br.0) == 'M')
                        || (char_at(tl.1, tl.0) == 'S'
                            && char_at(tr.1, tr.0) == 'M'
                            && char_at(bl.1, bl.0) == 'S'
                            && char_at(br.1, br.0) == 'M'))
                {
                    result += 1;
                }
            }
        }
    }

    println!("{}", result);
    Ok(())
}

fn fetch_data_day4() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let data_string = read_input::read_input("data/day4.txt")?;
    Ok(data_string)
}

pub fn day5_part1() -> Result<(), Box<dyn std::error::Error>> {
    let (rules, pages) = fetch_data_day5()?;
    let mut result = 0;
    let mut bad_pages: Vec<usize> = Vec::new();
    for (i, page) in pages.iter().enumerate() {
        let mut valid = true;
        for i in 0..page.len() {
            let curr = page[i];
            for (j, comp) in page.iter().enumerate() {
                if j < i {
                    match rules.get(comp) {
                        Some(map) if !map.contains(&curr) => {
                            valid = false;
                            break;
                        }
                        Some(_) => {}
                        None => {
                            valid = false;
                            break;
                        }
                    }
                } else if j > i {
                    match rules.get(&curr) {
                        Some(map) if !map.contains(comp) => {
                            valid = false;
                            break;
                        }
                        Some(_) => {}
                        None => {
                            valid = false;
                            break;
                        }
                    }
                }
            }
        }
        if valid {
            // println!("found {}", page[page.len() / 2]);
            result += page[page.len() / 2]
        } else {
            bad_pages.push(i);
        }
    }
    println!("{}", result);
    // Print the bad pages
    bad_pages.iter().for_each(|p| print!("{},", p));
    println!();
    Ok(())
}

pub fn day5_part2() -> Result<(), Box<dyn std::error::Error>> {
    let (rules, pages) = fetch_data_day5()?;
    let indices: [usize; 96] = [
        1, 3, 4, 8, 11, 13, 15, 16, 18, 22, 23, 24, 27, 28, 32, 36, 37, 38, 39, 43, 45, 47, 49, 50,
        53, 55, 57, 58, 64, 66, 67, 69, 74, 75, 76, 78, 79, 80, 81, 82, 84, 87, 88, 90, 93, 94, 95,
        96, 98, 100, 102, 103, 104, 106, 107, 108, 110, 112, 114, 115, 117, 119, 123, 128, 129,
        130, 132, 133, 135, 139, 142, 143, 147, 149, 152, 154, 156, 158, 160, 161, 162, 163, 164,
        167, 168, 169, 170, 172, 175, 177, 182, 183, 184, 193, 194, 196,
    ];
    let mut result = 0;
    for p_idx in indices.iter() {
        if let Some(page) = pages.get(*p_idx) {
            let mut page_priors: Vec<(i32, i32)> = Vec::new();
            for i in 0..page.len() {
                let curr = page[i];
                let mut prior_count = 0;
                if let Some(map) = rules.get(&curr) {
                    for (j, comp) in page.iter().enumerate() {
                        if j != i && map.contains(comp) {
                            prior_count += 1;
                        }
                    }
                }
                page_priors.push((curr, prior_count));
            }
            // Produce correct ordering after building
            // list that tracks the amount of pages the
            // a certain page comes before
            page_priors.sort_by(|a, b| b.1.cmp(&a.1));
            result += page_priors[page_priors.len() / 2].0;
        }
    }
    println!("{}", result);
    Ok(())
}
type Day5Result = Result<(HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>), Box<dyn std::error::Error>>;

fn fetch_data_day5() -> Day5Result {
    let data_string = read_input::read_input("data/day5.txt")?;
    let mut iter = data_string.iter();
    let mut rules: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut pages: Vec<Vec<i32>> = Vec::new();
    for s in iter.by_ref() {
        if s.is_empty() {
            break;
        }
        let nums = s
            .split('|')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let key = nums[0];
        let val = nums[1];
        rules.entry(key).or_default().insert(val);
    }

    for s in iter.by_ref() {
        let nums = s
            .split(',')
            .map(|f| f.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        pages.push(nums);
    }
    Ok((rules, pages))
}
