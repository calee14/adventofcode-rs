// 2025 was a heartbreak
// solutions for 2025 advent of code

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::read_input::{self, read_input};

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
            i += 1
        }
    }

    println!("{}", result);

    Ok(())
}

pub fn fetch_data_day3() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let data_string = read_input::read_input("data/day3.txt")?;
    Ok(data_string)
}
