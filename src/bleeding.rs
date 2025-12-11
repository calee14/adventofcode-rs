// 2025 was me bleeding out
// solutions for 2025 advent of code

use crate::read_input::{self};
use core::num;
use std::{
    cmp::{Ordering, Reverse, max},
    collections::{BinaryHeap, HashMap, HashSet},
    ops::{Mul, Sub},
};

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

pub fn day3_part1() -> Result<(), Box<dyn std::error::Error>> {
    let data = fetch_data_day3()?;
    let mut result = 0;
    for s in data {
        let mut best_joltage = 0;
        for (i, c1) in s.chars().enumerate() {
            for c2 in s.chars().skip(i + 1) {
                let d1 = c1.to_string().parse::<u32>().unwrap();
                let d2 = c2.to_string().parse::<u32>().unwrap();
                best_joltage = max(best_joltage, d1 * 10 + d2);
            }
        }
        result += best_joltage;
    }
    println!("{}", result);
    Ok(())
}

pub fn day3_part2() -> Result<(), Box<dyn std::error::Error>> {
    let data = fetch_data_day3()?;
    let mut result = 0;
    for s in data {
        let digits: Vec<u64> = s
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as u64))
            .collect();
        let mut stack: Vec<u64> = Vec::new();
        for (i, &digit) in digits.iter().enumerate() {
            // If we have more digits left
            // than space available in the stack
            // then keep popping if the top value is
            // smaller than the incoming value
            while let Some(&top) = stack.last() {
                let remaining_digits = digits.len() - i;
                let can_fill = (stack.len() - 1) + remaining_digits >= 12;
                if digit > top && can_fill {
                    stack.pop();
                } else {
                    break;
                }
            }

            if stack.len() < 12 {
                stack.push(digit);
            }
        }
        let joltage: u64 = stack
            .iter()
            .enumerate()
            .map(|(i, d)| d * 10u64.pow(11u32.saturating_sub(i as u32)))
            .sum();
        result += joltage;
    }
    println!("{}", result);
    Ok(())
}

fn fetch_data_day3() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let data_string = read_input::read_input("data/2025/day3.txt")?;
    Ok(data_string)
}

pub fn day4_part1() -> Result<(), Box<dyn std::error::Error>> {
    let data = fetch_data_day4()?;
    let rows = data.len();
    let cols = data[0].len();
    let dirs: [(i32, i32); 8] = [
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let in_range = |i: i32, j: i32| i >= 0 && j >= 0 && (i as usize) < rows && (j as usize) < cols;
    let mut result = 0;
    for i in 0..data.len() {
        for j in 0..data[0].len() {
            if data[i][j] == '@' {
                let mut count = 0;
                for dir in dirs {
                    let new_i = i as i32 + dir.0;
                    let new_j = j as i32 + dir.1;
                    if in_range(new_i, new_j) && data[new_i as usize][new_j as usize] == '@' {
                        count += 1;
                    }
                }
                if count < 4 {
                    result += 1;
                }
            }
        }
    }
    println!("{}", result);
    Ok(())
}

pub fn day4_part2() -> Result<(), Box<dyn std::error::Error>> {
    let mut data = fetch_data_day4()?;
    let rows = data.len();
    let cols = data[0].len();
    let dirs: [(i32, i32); 8] = [
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let in_range = |i: i32, j: i32| i >= 0 && j >= 0 && (i as usize) < rows && (j as usize) < cols;
    let mut result = 0;
    loop {
        let mut targets: Vec<(usize, usize)> = Vec::new();
        for i in 0..data.len() {
            for j in 0..data[0].len() {
                if data[i][j] == '@' {
                    let mut count = 0;
                    for dir in dirs {
                        let new_i = i as i32 + dir.0;
                        let new_j = j as i32 + dir.1;
                        if in_range(new_i, new_j) && data[new_i as usize][new_j as usize] == '@' {
                            count += 1;
                        }
                    }
                    if count < 4 {
                        result += 1;
                        targets.push((i, j));
                    }
                }
            }
        }
        if targets.is_empty() {
            break;
        }
        for target in targets {
            data[target.0][target.1] = '.';
        }
    }
    println!("{}", result);
    Ok(())
}

fn fetch_data_day4() -> Result<Vec<Vec<char>>, Box<dyn std::error::Error>> {
    let data_string = read_input::read_input("data/2025/day4.txt")?;
    let grid = data_string
        .iter()
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<char>>>();

    Ok(grid)
}

pub fn day5_part1() -> Result<(), Box<dyn std::error::Error>> {
    let (fresh_ranges, product_ids) = fetch_data_day5()?;
    let mut result = 0;
    for product_id in product_ids {
        for range in &fresh_ranges {
            if product_id >= range.0 && product_id <= range.1 {
                result += 1;
                break;
            }
        }
    }
    println!("{}", result);
    Ok(())
}

pub fn day5_part2() -> Result<(), Box<dyn std::error::Error>> {
    let (mut fresh_ranges, product_ids) = fetch_data_day5()?;
    fresh_ranges.sort_by_key(|r| r.0);

    let mut merged_ranges: Vec<(u64, u64)> = Vec::new();
    if let Some(range) = fresh_ranges.first() {
        merged_ranges.push(*range);
    }
    for range in fresh_ranges.iter().skip(1) {
        let last_merge = merged_ranges.last_mut().unwrap();
        if range.0 < last_merge.1 + 1 {
            last_merge.1 = last_merge.1.max(range.1);
        } else {
            merged_ranges.push(*range);
        }
    }
    let mut result = 0;
    for range in merged_ranges {
        result += range.1 - range.0 + 1;
    }
    println!("{}", result);
    Ok(())
}

fn fetch_data_day5() -> Result<(Vec<(u64, u64)>, Vec<u64>), Box<dyn std::error::Error>> {
    let data_string = read_input::read_input("data/2025/day5.txt")?;
    let mut fresh_ranges: Vec<(u64, u64)> = Vec::new();

    let mut curr: usize = 0;
    while !data_string.get(curr).unwrap().is_empty() {
        let range = data_string[curr]
            .split('-')
            .map(|v| v.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        fresh_ranges.push((range[0], range[1]));
        curr += 1;
    }
    let mut product_ids: Vec<u64> = Vec::new();
    for id in data_string.iter().skip(curr + 1) {
        product_ids.push(id.parse::<u64>().unwrap());
    }

    Ok((fresh_ranges, product_ids))
}

pub fn day6_part1() -> Result<(), Box<dyn std::error::Error>> {
    let (numbers, operators) = fetch_data_day6()?;

    let mut result = 0;
    for col in 0..operators.len() {
        let operator = &operators[col];
        result += if operator == "+" {
            let mut answer = 0;
            for row in 0..numbers.len() {
                answer += numbers[row][col];
            }
            answer
        } else {
            let mut answer = 1;
            for row in 0..numbers.len() {
                answer *= numbers[row][col];
            }
            answer
        }
    }
    println!("{}", result);
    Ok(())
}

fn fetch_data_day6() -> Result<(Vec<Vec<i64>>, Vec<String>), Box<dyn std::error::Error>> {
    let data_string = read_input::read_input("data/2025/day6.txt")?;
    let numbers = data_string
        .iter()
        .take(data_string.len() - 1)
        .map(|row| {
            row.split(' ')
                .filter(|s| !s.is_empty())
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();
    let operators = data_string
        .last()
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    Ok((numbers, operators))
}

pub fn day6_part2() -> Result<(), Box<dyn std::error::Error>> {
    let (numbers, operators) = fetch_data_day6_part2()?;

    let mut result = 0;
    for (i, operator) in operators.iter().enumerate() {
        let prob_idx = operators.len() - i - 1;
        result += if operator == "+" {
            numbers[prob_idx].iter().sum()
        } else {
            numbers[prob_idx].iter().product::<i64>()
        };
    }
    println!("{}", result);
    Ok(())
}

fn fetch_data_day6_part2() -> Result<(Vec<Vec<i64>>, Vec<String>), Box<dyn std::error::Error>> {
    let data_string = read_input::read_input("data/2025/day6.txt")?;
    let mut numbers: Vec<Vec<i64>> = Vec::new();
    let mut problem: Vec<i64> = Vec::new();
    for i in (0..data_string[0].len()).rev() {
        let mut num_string = String::new();
        for j in data_string.iter().take(data_string.len() - 1) {
            if let Some(char) = j.chars().nth(i) {
                num_string.push(char);
            }
        }
        // Encoutner full column of empty
        // space. Store problem and work
        // on creating the next one
        if num_string.trim().is_empty() {
            numbers.push(problem.clone());
            problem.clear();
        } else {
            problem.push(num_string.trim().parse::<i64>().unwrap());
        }
    }

    // Push final problem set
    // into vector
    numbers.push(problem);

    // numbers.iter().for_each(|n| {
    //     n.iter().for_each(|s| print!("{} ", s));
    //     println!("");
    // });

    let operators = data_string
        .last()
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    Ok((numbers, operators))
}

fn day7_part1_helper(i: i32, j: i32, data: &mut Vec<Vec<char>>) -> u32 {
    let mut result = 0;

    let rows = data.len();
    let cols = data[0].len();

    let in_range = |i: i32, j: i32| i >= 0 && j >= 0 && (i as usize) < rows && (j as usize) < cols;

    let mut curr_i = i;
    while in_range(curr_i + 1, j)
        && data[curr_i as usize][j as usize] != '^'
        && data[curr_i as usize][j as usize] != '|'
    {
        data[curr_i as usize][j as usize] = '|';
        curr_i += 1;
    }
    if data[curr_i as usize][j as usize] == '^' {
        result += 1;
        if in_range(curr_i, j + 1) && data[curr_i as usize][(j + 1) as usize] != '|' {
            result += day7_part1_helper(curr_i, j + 1, data);
        }
        if in_range(curr_i, j - 1) && data[curr_i as usize][(j - 1) as usize] != '|' {
            result += day7_part1_helper(curr_i, j - 1, data);
        }
    }

    result
}

pub fn day7_part1() -> Result<(), Box<dyn std::error::Error>> {
    let mut data = fetch_data_day7()?;
    let mut start_pos = (0, 0);
    for (i, row) in data.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == 'S' {
                start_pos = (i, j);
                break;
            }
        }
        if start_pos != (0, 0) {
            break;
        }
    }

    let result = day7_part1_helper(start_pos.0 as i32, start_pos.1 as i32, &mut data);
    println!("{}", result);

    Ok(())
}

fn day7_part2_helper(
    i: i32,
    j: i32,
    data: &mut Vec<Vec<char>>,
    memo: &mut HashMap<(i32, i32), u64>,
) -> u64 {
    if let Some(&res) = memo.get(&(i, j)) {
        return res;
    }
    let mut result = 0;

    let rows = data.len();
    let cols = data[0].len();

    let in_range = |i: i32, j: i32| i >= 0 && j >= 0 && (i as usize) < rows && (j as usize) < cols;

    let mut curr_i = i;
    while in_range(curr_i + 1, j) && data[curr_i as usize][j as usize] != '^' {
        curr_i += 1;
    }
    if !in_range(curr_i + 1, j) {
        return 1;
    }

    if data[curr_i as usize][j as usize] == '^' {
        if in_range(curr_i, j + 1) {
            result += day7_part2_helper(curr_i, j + 1, data, memo);
        }
        if in_range(curr_i, j - 1) {
            result += day7_part2_helper(curr_i, j - 1, data, memo);
        }
    }
    memo.insert((i, j), result);
    result
}
pub fn day7_part2() -> Result<(), Box<dyn std::error::Error>> {
    let mut data = fetch_data_day7()?;
    let mut start_pos = (0, 0);
    for (i, row) in data.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == 'S' {
                start_pos = (i, j);
                break;
            }
        }
        if start_pos != (0, 0) {
            break;
        }
    }

    let mut memo = HashMap::new();
    let result = day7_part2_helper(start_pos.0 as i32, start_pos.1 as i32, &mut data, &mut memo);
    println!("{}", result);

    Ok(())
}

fn fetch_data_day7() -> Result<Vec<Vec<char>>, Box<dyn std::error::Error>> {
    let data_string = read_input::read_input("data/2025/day7.txt")?;
    let grid = data_string
        .iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    Ok(grid)
}

pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            rank: vec![0; size],
            size: vec![1; size],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // path compression
        }
        self.parent[x]
    }

    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false; // already in same set
        }

        // union by rank
        match self.rank[root_x].cmp(&self.rank[root_y]) {
            std::cmp::Ordering::Less => {
                self.parent[root_x] = root_y;
                self.size[root_y] += self.size[root_x];
            }
            std::cmp::Ordering::Greater => {
                self.parent[root_y] = root_x;
                self.size[root_x] += self.size[root_y];
            }
            std::cmp::Ordering::Equal => {
                self.parent[root_y] = root_x;
                self.size[root_x] += self.size[root_y];
                self.rank[root_x] += 1;
            }
        }
        true
    }

    // Get the size of the set containing x
    pub fn set_size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        self.size[root]
    }

    pub fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}

#[derive(PartialEq)]
struct MinHeapItem(f64, (usize, usize));

impl Eq for MinHeapItem {}

impl Ord for MinHeapItem {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.partial_cmp(&self.0).unwrap() // reversed for min heap, only compares f64
    }
}

impl PartialOrd for MinHeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn day8_part1() -> Result<(), Box<dyn std::error::Error>> {
    let points = fetch_data_day8()?;

    // Compute distances between all points
    let mut min_dist_heap: BinaryHeap<MinHeapItem> = BinaryHeap::new();
    for (i, p1) in points.iter().enumerate() {
        for (j, p2) in points.iter().enumerate().skip(i + 1) {
            let dist = p1
                .iter()
                .enumerate()
                .map(|(k, val)| (val - p2[k]).powf(2f64))
                .sum::<f64>();
            min_dist_heap.push(MinHeapItem(dist, (i, j)));
        }
    }

    // Create Disjoint set (union find)
    // data structure and put all points
    // into individual circuit (set)
    let mut union_find = UnionFind::new(points.len());

    for _ in 0..1000 {
        if let Some(MinHeapItem(_, (i, j))) = min_dist_heap.pop() {
            if !union_find.connected(i, j) {
                union_find.union(i, j);
            }
        } else {
            break;
        }
    }

    let mut all_sets: HashSet<usize> = HashSet::new();
    for point in 0..points.len() {
        let set = union_find.find(point);
        all_sets.insert(set);
    }

    let mut min_heap: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
    for set in all_sets {
        let set_size = union_find.set_size(set);
        min_heap.push(Reverse(set_size));

        if min_heap.len() > 3 {
            min_heap.pop();
        }
    }
    println!(
        "{}",
        min_heap
            .into_iter()
            .map(|Reverse(val)| val)
            .product::<usize>()
    );
    Ok(())
}

pub fn day8_part2() -> Result<(), Box<dyn std::error::Error>> {
    let points = fetch_data_day8()?;

    // Compute distances between all points
    let mut min_dist_heap: BinaryHeap<MinHeapItem> = BinaryHeap::new();
    for (i, p1) in points.iter().enumerate() {
        for (j, p2) in points.iter().enumerate().skip(i + 1) {
            let dist = p1
                .iter()
                .enumerate()
                .map(|(k, val)| (val - p2[k]).powf(2f64))
                .sum::<f64>();
            min_dist_heap.push(MinHeapItem(dist, (i, j)));
        }
    }

    let mut result = 0.0;
    // Create Disjoint set (union find)
    // data structure and put all points
    // into individual circuit (set)
    let mut union_find = UnionFind::new(points.len());

    while let Some(MinHeapItem(_, (i, j))) = min_dist_heap.pop() {
        if !union_find.connected(i, j) {
            union_find.union(i, j);
            result = points[i].first().unwrap().mul(points[j].first().unwrap());
        }
    }

    let mut all_sets: HashSet<usize> = HashSet::new();
    for point in 0..points.len() {
        let set = union_find.find(point);
        all_sets.insert(set);
    }

    let mut min_heap: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
    for set in all_sets {
        let set_size = union_find.set_size(set);
        min_heap.push(Reverse(set_size));

        if min_heap.len() > 3 {
            min_heap.pop();
        }
    }
    println!(
        "{}",
        min_heap
            .into_iter()
            .map(|Reverse(val)| val)
            .product::<usize>()
    );

    println!("{}", result);
    Ok(())
}

fn fetch_data_day8() -> Result<Vec<Vec<f64>>, Box<dyn std::error::Error>> {
    let data_string = read_input::read_input("data/2025/day8.txt")?;
    let points = data_string
        .iter()
        .map(|s| {
            s.split(',')
                .map(|n| n.parse::<f64>().unwrap())
                .collect::<Vec<f64>>()
        })
        .collect::<Vec<Vec<f64>>>();
    Ok(points)
}

pub fn day9_part1() -> Result<(), Box<dyn std::error::Error>> {
    let data = fetch_data_day9()?;
    let mut result = 0;
    for (i, p1) in data.iter().enumerate() {
        for (_, p2) in data.iter().enumerate().skip(i + 1) {
            let w = p1[0].sub(p2[0]).abs() + 1;
            let h = p1[1].sub(p2[1]).abs() + 1;
            result = max(result, w * h);
        }
    }
    println!("{}", result);
    Ok(())
}

pub fn day9_part2() -> Result<(), Box<dyn std::error::Error>> {
    let data = fetch_data_day9()?;
    let mut result = 0;

    println!("{}", result);
    Ok(())
}

fn fetch_data_day9() -> Result<Vec<Vec<i64>>, Box<dyn std::error::Error>> {
    let data_string = read_input::read_input("data/2025/day9.txt")?;
    let points = data_string
        .iter()
        .map(|s| {
            s.split(',')
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();
    Ok(points)
}
