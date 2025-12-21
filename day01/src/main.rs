use std::{fs, path::absolute};

fn get_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Failed to open input.")
}

fn get_vecs(input: &str, v1: &mut Vec<i64>, v2: &mut Vec<i64>) {
    for line in input.lines() {
        let mut vals_it = line.split_whitespace();
        v1.push(vals_it.next().unwrap().parse().unwrap());
        v2.push(vals_it.next().unwrap().parse().unwrap());
    }
}

fn count_matching(nums: &[i64], val: &i64) -> i64 {
    let mut count: i64 = 0;

    for num in nums {
        if num == val {
            count += 1
        };
    }

    count
}

fn main() {
    let input = get_input("input.txt");

    println!("{input}");

    let mut v1: Vec<i64> = Vec::new();
    let mut v2: Vec<i64> = Vec::new();

    get_vecs(&input, &mut v1, &mut v2);

    // v1.sort();
    // v2.sort();

    let mut total: i64 = 0;

    for num in v1 {
        total += num * count_matching(&v2[..], &num);
    }

    println!("total: {total}");
}
