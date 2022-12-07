use std::cmp::Reverse;
use std::collections::BinaryHeap;
use itertools::{FoldWhile, Itertools};


fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let top_3_sum = include_str!("day01.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .collect::<Vec<_>>()
        .split(|line| line.is_none())
        .map(|group| {
            group
                .iter()
                .map(|item| item.unwrap())
                .sum::<u64>()
        })
        .collect::<BinaryHeap<u64>>()
        .iter()
        .take(3)
        .sum::<u64>();

    println!("TOP 3 SUM");
    println!("{top_3_sum:?}");

    nicklas();
    amos();

    Ok(())
}

fn nicklas() {
    let output = include_str!("day01.txt")
        .split("\n\n")
        .map(|entry| {
            entry
                .lines()
                .map(|calories| calories.parse::<usize>().expect("Failed to parse calories"))
                .into_iter()
                .sum()
        })
        .collect::<BinaryHeap<usize>>()
        .iter()
        .take(3)
        .sum::<usize>()
        .to_string();
    println!("Nicklas");
    println!("{output:?}");
}

fn amos() {
    let answer = include_str!("day01.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .batching(|mut it| (&mut it).map_while(|x| x).sum1::<u64>())
        .map(Reverse)
        .k_smallest(3)
        .map(|x| x.0)
        .sum::<u64>();
    println!("Amos");
    println!("{answer:?}");
}


fn luka_one() {
    let lines = include_str!("day01.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .collect::<Vec<_>>();

    let calories = lines
        .split(|line| line.is_none())
        .map(|group| {
            group
                .iter()
                .map(|item| item.unwrap())
                .sum::<u64>()
        });

    let leader = calories.max().unwrap();

    println!("luka_one");
    println!("{leader:?}");
}