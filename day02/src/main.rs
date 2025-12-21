use std::fs;

fn get_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Failed to open input.")
}

fn check_vals_safe(levels: Vec<i64>) -> bool {
    let mut dir = 0;
    let mut prev = levels[0];

    for val in &levels[1..] {
        let val = *val;
        match dir {
            0 => {
                if val > prev {
                    dir = 1
                } else {
                    dir = -1
                }
            }
            1 => {
                if prev > val {
                    return false;
                }
            }
            -1 => {
                if val > prev {
                    return false;
                }
            }
            _ => panic!("invalid dir!"),
        }

        let diff = (val - prev).abs();

        if !(1..=3).contains(&diff) {
            return false;
        }
        prev = val;
    }

    true
}

fn check_line_safe(levels: &str) -> bool {
    let levels: Vec<i64> = levels
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse number"))
        .collect();

    if check_vals_safe(levels.clone()) {
        return true;
    };

    for i in 0..levels.len() {
        let mut trimmed = levels.clone();
        trimmed.remove(i);
        if check_vals_safe(trimmed) {
            return true;
        }
    }

    false
}

fn main() {
    let input = get_input("input.txt");

    let mut total: i64 = 0;

    for line in input.lines() {
        if check_line_safe(line) {
            total += 1;
        }
    }

    println!("total: {total}");
}
