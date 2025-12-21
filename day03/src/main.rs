use std::{error::Error, fs};

fn get_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Failed to open input.")
}

fn extract_nums(slice: &str) -> Result<(i64, i64), Box<dyn Error>> {
    let mut nums: (i64, i64) = (0, 0);

    if let Some((num1, num2)) = slice.split_once(",") {
        nums.0 = num1.parse()?;
        nums.1 = num2.parse()?;
    } else {
        println!("{slice}");
        return Err("Invalid line format".into());
    };

    Ok(nums)
}

fn set_state(slice: &str, found_idx: usize, state: &mut bool) {
    match (slice.find("do()"), slice.find("don't()")) {
        (Some(on), Some(off)) => {
            println!("do() found");
            println!("don't() found");
            match (found_idx > on, found_idx > off) {
                (true, true) => *state = on > off,
                (true, false) => *state = true,
                (false, true) => *state = false,
                (false, false) => (),
            }
        }
        (Some(on), None) => {
            println!("do() found");
            if found_idx > on {
                println!("Enabling...");
                *state = true;
            }
        }
        (None, Some(off)) => {
            println!("don't() found");
            if found_idx > off {
                println!("Disabling...");
                *state = false;
            }
        }
        (None, None) => println!("No match"),
    }
}

fn get_sums(input: String) -> Vec<(i64, i64)> {
    let mut sums: Vec<(i64, i64)> = Vec::new();
    let mut segment: &str = &input[0..];
    let mut enabled: bool = true;

    while let Some(mut found_idx) = segment.find("mul(") {
        set_state(segment, found_idx, &mut enabled);
        println!("{segment}");
        found_idx += 4;
        if enabled
            && let Some(end_idx) = segment[found_idx..].find(")")
            && let Ok(sum) = extract_nums(&segment[found_idx..(found_idx + end_idx)])
        {
            sums.push(sum)
        };
        segment = &segment[found_idx..];
    }

    sums
}

fn main() {
    // let input = get_input("test.txt");
    let input = get_input("input.txt");

    let sums = get_sums(input);
    let mut total: i64 = 0;

    for (n1, n2) in sums {
        println!("{n1} * {n2}");
        total += n1 * n2;
    }

    println!("total: {total}");
}
