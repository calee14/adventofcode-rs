// 2024 was a heartbreak
// solutions for 2024 advent of code

use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    iter,
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

pub fn day6_part1() -> Result<(), Box<dyn std::error::Error>> {
    let mut data_string = fetch_data_day6()?;
    let mut result = 0;
    let mut curr_pos: (i32, i32) = (0, 0);
    let max_row = data_string.len() as i32;
    let max_col = data_string[0].len() as i32;
    let in_range = |r: i32, c: i32| r >= 0 && r < max_row && c >= 0 && c < max_col;
    let dirs: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut curr_dir = 0;

    // Find starting position
    for i in 0..data_string.len() {
        for j in 0..data_string[0].len() {
            if data_string.get(i).unwrap().chars().nth(j).unwrap() == '^' {
                curr_pos = (i as i32, j as i32);
            }
        }
    }

    // Follow rules to find path of gaurd
    while in_range(curr_pos.0, curr_pos.1) {
        let dir = dirs[curr_dir];
        if in_range(curr_pos.0 + dir.0, curr_pos.1 + dir.1) {
            let curr_is_hash = data_string
                .get((curr_pos.0 + dir.0) as usize)
                .unwrap()
                .chars()
                .nth((curr_pos.1 + dir.1) as usize)
                .unwrap()
                == '#';
            if curr_is_hash {
                curr_dir = if curr_dir + 1 < dirs.len() {
                    curr_dir + 1
                } else {
                    0
                };
                continue;
            }
        }
        if let Some(s) = data_string.get_mut(curr_pos.0 as usize) {
            let mut chars: Vec<char> = s.chars().collect();

            if let Some(c) = chars.get_mut(curr_pos.1 as usize)
                && *c != 'X'
            {
                result += 1;
                *c = 'X';
                *s = chars.into_iter().collect();
            }
        }
        curr_pos.0 += dir.0;
        curr_pos.1 += dir.1;
    }

    // data_string.iter(.for_each(|s| {
    //     s.chars().for_each(|c| print!("{}", c));
    //     println!();
    // });
    println!("{}", result);
    Ok(())
}

pub fn day6_part2() -> Result<(), Box<dyn std::error::Error>> {
    let mut data_string = fetch_data_day6()?;
    let mut result = 0;
    let mut curr_pos: (i32, i32) = (0, 0);
    let max_row = data_string.len() as i32;
    let max_col = data_string[0].len() as i32;
    let in_range = |r: i32, c: i32| r >= 0 && r < max_row && c >= 0 && c < max_col;
    let dirs: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut curr_dir = 0;

    // Find starting position
    for i in 0..data_string.len() {
        for j in 0..data_string[0].len() {
            if data_string.get(i).unwrap().chars().nth(j).unwrap() == '^' {
                curr_pos = (i as i32, j as i32);
            }
        }
    }

    // Follow rules to find path of gaurd
    while in_range(curr_pos.0, curr_pos.1) {
        let dir = dirs[curr_dir];
        if in_range(curr_pos.0 + dir.0, curr_pos.1 + dir.1) {
            let curr_is_hash = data_string
                .get((curr_pos.0 + dir.0) as usize)
                .unwrap()
                .chars()
                .nth((curr_pos.1 + dir.1) as usize)
                .unwrap()
                == '#';
            if curr_is_hash {
                curr_dir = if curr_dir + 1 < dirs.len() {
                    curr_dir + 1
                } else {
                    0
                };
                continue;
            }
        }
        if let Some(s) = data_string.get_mut(curr_pos.0 as usize) {
            let mut chars: Vec<char> = s.chars().collect();

            if let Some(c) = chars.get_mut(curr_pos.1 as usize)
                && *c != '^'
            {
                *c = if *c == 'O' || *c == 'W' { 'W' } else { 'X' };
                *s = chars.into_iter().collect();
            }
        }

        curr_pos.0 += dir.0;
        curr_pos.1 += dir.1;

        // Check if we can find a loop
        // 4 turns to make a loop
        let mut temp_pos = curr_pos;
        let temp_start_pos = temp_pos;
        let mut temp_dir = curr_dir;
        let mut has_loop = true;
        let mut temp_reached_start = false;
        let mut iter_count = 0;
        let new_wall_pos = (curr_pos.0 + dir.0, curr_pos.1 + dir.1);

        loop {
            if !has_loop || temp_reached_start {
                break;
            }
            if !in_range(temp_pos.0, temp_pos.1) {
                break;
            }
            temp_dir = if temp_dir + 1 < dirs.len() {
                temp_dir + 1
            } else {
                0
            };

            let mut prev_cell = '.';
            while in_range(temp_pos.0, temp_pos.1) {
                let temp_dir_mod = dirs[temp_dir];
                let next_pos = (temp_pos.0 + temp_dir_mod.0, temp_pos.1 + temp_dir_mod.1);
                if in_range(next_pos.0, next_pos.1) {
                    let next_cell = data_string
                        .get((next_pos.0) as usize)
                        .unwrap()
                        .chars()
                        .nth((next_pos.1) as usize)
                        .unwrap();
                    if iter_count > 0
                        && temp_pos.0 == temp_start_pos.0
                        && temp_pos.1 == temp_start_pos.1
                    {
                        temp_reached_start = true;
                        break;
                    } else if iter_count > 0
                        && temp_pos.0 == temp_start_pos.0
                        && temp_pos.1 == temp_start_pos.1
                        && !has_loop
                    {
                        break;
                    }
                    if next_cell == '#'
                        || next_pos.0 == new_wall_pos.0 && next_pos.1 == new_wall_pos.1
                    {
                        iter_count += 1;
                        if prev_cell == '.' || prev_cell == 'O' {
                            has_loop = false;
                        }
                        break;
                    }
                    prev_cell = next_cell;
                }
                temp_pos.0 += temp_dir_mod.0;
                temp_pos.1 += temp_dir_mod.1;
            }

            iter_count += 1;
        }
        if has_loop
            && temp_reached_start
            && let Some(s) = data_string.get_mut((curr_pos.0 + dir.0) as usize)
        {
            let mut chars: Vec<char> = s.chars().collect();

            if let Some(c) = chars.get_mut((curr_pos.1 + dir.1) as usize)
                && *c != '#'
                && *c != '^'
            {
                result += 1;
                *c = if *c == 'X' { 'W' } else { 'O' };
                *s = chars.into_iter().collect();
            }
        }
    }

    data_string.iter().for_each(|s| {
        s.chars().for_each(|c| print!("{}", c));
        println!();
    });
    println!("{}", result);
    Ok(())
}

fn fetch_data_day6() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let data_string = read_input::read_input("data/day6.txt")?;
    Ok(data_string)
}

fn tree_helper_part1(curr: u64, i: usize, values: &Vec<u64>, target: &u64) -> bool {
    if curr == *target {
        return true;
    }
    if i == values.len() {
        return false;
    }
    tree_helper_part1(curr + values[i], i + 1, values, target)
        || tree_helper_part1(curr * values[i], i + 1, values, target)
}

pub fn day7_part1() -> Result<(), Box<dyn std::error::Error>> {
    let lines = fetch_data_day7()?;
    let mut result = 0;
    for (sum, values) in lines.iter() {
        if tree_helper_part1(0, 0, values, sum) {
            result += sum;
        };
    }
    println!("{}", result);
    Ok(())
}

fn tree_helper_part2(curr: u64, i: usize, values: &Vec<u64>, target: &u64) -> bool {
    if curr == *target && i == values.len() {
        return true;
    }
    if i == values.len() || curr > *target {
        return false;
    }

    let add_result = curr
        .checked_add(values[i])
        .map(|new_curr| tree_helper_part2(new_curr, i + 1, values, target))
        .unwrap_or(false);
    if add_result {
        return true;
    }

    let mul_result = curr
        .checked_mul(values[i])
        .map(|new_curr| tree_helper_part2(new_curr, i + 1, values, target))
        .unwrap_or(false);
    if mul_result {
        return true;
    }

    let digits = values[i].to_string().len() as u32;

    10u64
        .checked_pow(digits)
        .and_then(|multiplier| curr.checked_mul(multiplier))
        .and_then(|shifted| shifted.checked_add(values[i]))
        .map(|new_curr| tree_helper_part2(new_curr, i + 1, values, target))
        .unwrap_or(false)
}

pub fn day7_part2() -> Result<(), Box<dyn std::error::Error>> {
    let lines = fetch_data_day7()?;
    let mut result = 0;
    for (sum, values) in lines.iter() {
        if tree_helper_part2(0, 0, values, sum) {
            result += sum;
        };
    }
    println!("{}", result);
    Ok(())
}

type Day7Result = Result<Vec<(u64, Vec<u64>)>, Box<dyn std::error::Error>>;
fn fetch_data_day7() -> Day7Result {
    let data_string = read_input::read_input("data/day7.txt")?;
    let parsed_data = data_string
        .iter()
        .map(|s| {
            let line_part = s.split(':').collect::<Vec<&str>>();
            let sum = line_part.first().unwrap().parse::<u64>().unwrap();

            let values = line_part[1]
                .strip_prefix(' ')
                .unwrap()
                .split(' ')
                .collect::<Vec<&str>>()
                .iter()
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            (sum, values)
        })
        .collect::<Vec<(u64, Vec<u64>)>>();

    Ok(parsed_data)
}

pub fn day8_part1() -> Result<(), Box<dyn std::error::Error>> {
    let grid = fetch_data_day8()?;
    let in_grid =
        |r: i32, c: i32| r >= 0 && r < grid.len() as i32 && c >= 0 && c < grid[0].len() as i32;

    let mut freqs: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] != '.' {
                freqs
                    .entry(grid[r][c])
                    .or_default()
                    .push((r as i32, c as i32));
            }
        }
    }

    let mut result = 0;
    // Track the antinodes that we have seen
    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    for freq_points in freqs.values() {
        for freq1 in 0..freq_points.len() {
            for freq2 in freq1 + 1..freq_points.len() {
                let dx = freq_points[freq1].0 - freq_points[freq2].0;
                let dy = freq_points[freq1].1 - freq_points[freq2].1;

                let anti1 = (freq_points[freq1].0 + dx, freq_points[freq1].1 + dy);
                if !seen.contains(&anti1)
                    && in_grid(freq_points[freq1].0 + dx, freq_points[freq1].1 + dy)
                {
                    seen.insert((freq_points[freq1].0 + dx, freq_points[freq1].1 + dy));
                    result += 1;
                }
                let anti2 = (freq_points[freq2].0 - dx, freq_points[freq2].1 - dy);
                if !seen.contains(&anti2)
                    && in_grid(freq_points[freq2].0 - dx, freq_points[freq2].1 - dy)
                {
                    seen.insert((freq_points[freq2].0 - dx, freq_points[freq2].1 - dy));
                    result += 1;
                }
            }
        }
    }
    println!("{}", result);
    Ok(())
}

pub fn day8_part2() -> Result<(), Box<dyn std::error::Error>> {
    let grid = fetch_data_day8()?;
    let in_grid =
        |r: i32, c: i32| r >= 0 && r < grid.len() as i32 && c >= 0 && c < grid[0].len() as i32;

    let mut freqs: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] != '.' {
                freqs
                    .entry(grid[r][c])
                    .or_default()
                    .push((r as i32, c as i32));
            }
        }
    }

    // Track the antinodes that we have seen
    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    for freq_points in freqs.values() {
        for &pos in freq_points {
            seen.insert(pos);
        }
        for freq1 in 0..freq_points.len() {
            for freq2 in freq1 + 1..freq_points.len() {
                let dx = freq_points[freq1].0 - freq_points[freq2].0;
                let dy = freq_points[freq1].1 - freq_points[freq2].1;

                let mut anti1 = (freq_points[freq1].0, freq_points[freq1].1);
                loop {
                    anti1.0 += dx;
                    anti1.1 += dy;
                    if in_grid(anti1.0, anti1.1) {
                        seen.insert((anti1.0, anti1.1));
                    } else {
                        break;
                    }
                }

                let mut anti2 = (freq_points[freq2].0, freq_points[freq2].1);
                loop {
                    anti2.0 -= dx;
                    anti2.1 -= dy;
                    if in_grid(anti2.0, anti2.1) {
                        seen.insert((anti2.0, anti2.1));
                    } else {
                        break;
                    }
                }
            }
        }
    }
    println!("{}", seen.len());
    Ok(())
}

fn fetch_data_day8() -> Result<Vec<Vec<char>>, Box<dyn std::error::Error>> {
    let data_string = read_input::read_input("data/day8.txt")?;
    let grid = data_string
        .iter()
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<char>>>();
    Ok(grid)
}

pub fn day9_part1() -> Result<(), Box<dyn std::error::Error>> {
    let data = fetch_data_day9()?;

    let mut file_blocks: Vec<(i32, u32)> = Vec::new();
    let mut num_files = 0;
    data.iter().enumerate().for_each(|(i, val)| {
        if i % 2 == 0 {
            file_blocks.push((num_files, *val));
            num_files += 1;
        } else {
            file_blocks.push((-1, *val));
        }
    });
    let mut result: u64 = 0;
    let mut disk_map_idx = 0;
    while !file_blocks.is_empty() {
        let block = file_blocks.first().unwrap();
        let block_idx = block.0;
        let block_size = block.1;

        if block_idx >= 0 {
            for _ in 0..block_size {
                result += (disk_map_idx * block_idx) as u64;
                disk_map_idx += 1;
            }
            if !file_blocks.is_empty() {
                file_blocks.remove(0);
            }
        } else {
            for _ in 0..block_size {
                // Keep removing empty blocks from the end
                while file_blocks.last().is_some_and(|b| b.1 == 0 || b.0 < 0) {
                    file_blocks.pop();
                }

                if file_blocks.is_empty() {
                    break;
                }
                let last_block: &mut (i32, u32) = file_blocks.last_mut().unwrap();
                result += (disk_map_idx * last_block.0) as u64;
                disk_map_idx += 1;
                last_block.1 -= 1;
            }
            if !file_blocks.is_empty() {
                file_blocks.remove(0);
            }
        }
    }

    println!("{}", result);

    Ok(())
}

pub fn day9_part2() -> Result<(), Box<dyn std::error::Error>> {
    let data = fetch_data_day9()?;

    let mut file_blocks: Vec<(i32, u32)> = Vec::new();
    let mut num_files = 0;

    data.iter().enumerate().for_each(|(i, val)| {
        if i % 2 == 0 {
            file_blocks.push((num_files, *val));
            num_files += 1;
        } else {
            file_blocks.push((-1, *val));
        }
    });

    let mut curr: i32 = file_blocks.len() as i32 - 1;
    while curr >= 0 {
        let (block_id, block_size) = *file_blocks.get(curr as usize).unwrap();

        if block_id >= 0 {
            let mut free_idx: i32 = -1;
            for (i, temp_block) in file_blocks.iter().enumerate() {
                if i > curr as usize {
                    break;
                }

                if temp_block.0 == -1 && temp_block.1 >= block_size {
                    // modify the file block
                    // fileblocks remove element at i and replace with
                    // (block_id, block_size) + (-1, temp_block.1 - )
                    free_idx = i as i32;
                    break;
                }
            }
            if free_idx != -1 {
                // Replace the current block with free space
                file_blocks[curr as usize] = (-1, block_size);

                // Handle the free block we found
                let free_block = file_blocks.remove(free_idx as usize);
                file_blocks.insert(free_idx as usize, (block_id, block_size));

                if free_block.1 - block_size > 0 {
                    // Insert remaining free space after the moved block
                    let new_free_block = (-1, free_block.1 - block_size);
                    file_blocks.insert(free_idx as usize + 1, new_free_block);
                    // curr stays the same because we inserted before it (shifts it right by 1)
                } else {
                    // We used the entire free block, so curr shifts left by 1
                    curr -= 1;
                }
            }
        }
        curr -= 1;
    }

    let mut result: u64 = 0;
    let mut disk_map_idx = 0;

    while !file_blocks.is_empty() {
        let (block_id, block_size) = file_blocks.remove(0);
        if block_id >= 0 {
            for _ in 0..block_size {
                result += disk_map_idx as u64 * block_id as u64;
                disk_map_idx += 1;
            }
        } else {
            disk_map_idx += block_size;
        }
    }

    println!("{}", result);

    Ok(())
}

fn fetch_data_day9() -> Result<Vec<u32>, Box<dyn std::error::Error>> {
    let data_string = read_input::read_input("data/day9.txt")?;
    let grid = data_string
        .first()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    Ok(grid)
}

fn travel_helper(i: usize, j: usize, grid: &Vec<Vec<i32>>, seen: &mut HashSet<(i32, i32)>) -> u32 {
    if grid[i][j] == 9 {
        let pos = (i as i32, j as i32);
        if !seen.contains(&pos) {
            seen.insert(pos);
            return 1;
        } else {
            return 0;
        }
    }
    let in_range =
        |i: i32, j: i32| i >= 0 && i < grid.len() as i32 && j >= 0 && j < grid[0].len() as i32;

    let dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    let mut sub_total = 0;
    for dir in dirs {
        let new_i = i as i32 + dir.0;
        let new_j = j as i32 + dir.1;
        if in_range(new_i, new_j) && grid[i][j] + 1 == grid[new_i as usize][new_j as usize] {
            sub_total += travel_helper(new_i as usize, new_j as usize, grid, seen);
        }
    }
    sub_total
}

pub fn day10_part1() -> Result<(), Box<dyn std::error::Error>> {
    let data = fetch_data_day10()?;
    let mut starts: Vec<(usize, usize)> = Vec::new();
    for i in 0..data.len() {
        for j in 0..data[0].len() {
            if data[i][j] == 0 {
                starts.push((i, j));
            }
        }
    }

    let mut result = 0;
    for s in starts {
        let mut seen = HashSet::new();
        result += travel_helper(s.0, s.1, &data, &mut seen);
    }
    println!("{}", result);
    Ok(())
}
fn fetch_data_day10() -> Result<Vec<Vec<i32>>, Box<dyn std::error::Error>> {
    let data_string = read_input::read_input("data/day10.txt")?;
    let grid = data_string
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).map(|d| d as i32).unwrap_or(-1))
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    Ok(grid)
}
