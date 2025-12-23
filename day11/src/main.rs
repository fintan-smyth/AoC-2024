use std::{collections::{HashMap, hash_map::Entry}, fs};

fn get_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Failed to open input.")
}

fn get_stones(input: String) -> Vec<u64> {
    let mut stones: Vec<u64> = Vec::new();

    for num in input.split_whitespace() {
        stones.push(num.parse().expect("Failed to parse number"));
    }

    stones
}

fn count_digits(mut num: u64) -> u32 {
    let mut count = 0;

    if num == 0 {
        return 1;
    }
    while num > 0 {
        count += 1;
        num /= 10;
    }
    count
}

fn calculate_stone(stone: u64, tick: u64, max: u64, memo: &mut HashMap<(u64, u64), u64>) -> u64 {
    if tick == max {
        return 1;
    }

    if let Entry::Occupied(entry) = memo.entry((stone, tick)) {
        return *entry.get()
    }

    let mut total = 0;
    let digits = count_digits(stone);
    if stone == 0 {
        total += calculate_stone(1, tick + 1, max, memo);
    } else if digits.is_multiple_of(2) {
        let fact = 10u64.pow(digits / 2);
        let high = stone / fact;
        let low = stone % fact;
        total += calculate_stone(high, tick + 1, max, memo);
        total += calculate_stone(low, tick + 1, max, memo);
    } else {
        total += calculate_stone(stone * 2024, tick + 1, max, memo);
    }

    memo.insert((stone, tick), total);
    total
}

fn print_stones(stones: &[u64]) {
    for stone in stones {
        print!("{stone} ");
    }
    println!();
}

fn main() {
    // let input = get_input("test.txt");
    let input = get_input("input.txt");

    let stones = get_stones(input);
    let mut total = 0;
    let mut memo: HashMap<(u64, u64), u64> = HashMap::new();

    print_stones(&stones);

    for stone in stones {
        total += calculate_stone(stone, 0, 75, &mut memo);
    }

    println!("n_stones: {}", total);
}
