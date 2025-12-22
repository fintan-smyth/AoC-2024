use std::fs;

struct Equation {
    result: i64,
    nums: Vec<i64>,
}

enum Ops {
    Add,
    Mult,
    Concat,
}

fn get_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Failed to open input.")
}

fn get_equations(input: String) -> Vec<Equation> {
    let mut equations: Vec<Equation> = Vec::new();

    for line in input.lines() {
        let semi_pos = line.find(":").expect("invalid line format");
        let mut eq = Equation {
            result: line[0..semi_pos].parse().expect("Failed to parse result"),
            nums: Vec::new(),
        };
        for num in line[(semi_pos + 1)..].split_whitespace() {
            eq.nums.push(num.parse().expect("Failed to parse number"));
        }
        equations.push(eq);
    }

    equations
}

fn concat_nums(left: i64, right: i64) -> i64 {
    (left.to_string() + &right.to_string())
        .parse()
        .expect("failed to concat nums")
}

fn find_valid_eq(eq: &Equation, cur_sum: i64, cur_idx: usize, found: &mut bool) {
    if *found {
        return;
    }
    if cur_idx == eq.nums.len() {
        *found = cur_sum == eq.result;
        return;
    }

    let ops = [Ops::Add, Ops::Mult, Ops::Concat];

    for op in ops {
        match op {
            Ops::Add => find_valid_eq(eq, cur_sum + eq.nums[cur_idx], cur_idx + 1, found),
            Ops::Mult => find_valid_eq(eq, cur_sum * eq.nums[cur_idx], cur_idx + 1, found),
            Ops::Concat => find_valid_eq(
                eq,
                concat_nums(cur_sum, eq.nums[cur_idx]),
                cur_idx + 1,
                found,
            ),
        }
    }
}

fn is_valid_eq(eq: &Equation) -> bool {
    let mut is_valid = false;

    find_valid_eq(eq, eq.nums[0], 1, &mut is_valid);

    is_valid
}

fn main() {
    // let input = get_input("test.txt");
    let input = get_input("input.txt");

    let equations = get_equations(input);
    let mut total = 0;

    for eq in equations {
        if is_valid_eq(&eq) {
            total += eq.result;
            println!("{}", eq.result);
        }
    }

    println!("total: {total}");
}
