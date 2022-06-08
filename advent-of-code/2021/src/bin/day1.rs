fn get_input() -> &'static str {
    return "199
200
208
210
200
207
240
269
260
263";
}

fn n_increments(x: &Vec<i32>) -> i32 {
    return x
        .iter()
        .fold((0, None), |mut acc, item| {
            if let Some(x) = acc.1 {
                if item > x {
                    acc.0 += 1;
                }
            }
            acc.1 = Some(item);
            return acc;
        })
        .0;
}

fn main() {
    let numbers = get_input()
        .lines()
        .flat_map(str::parse::<i32>)
        .collect::<Vec<i32>>();

    println!("Part 1: {}", n_increments(&numbers));

    let numbers = numbers
        .iter()
        .skip(2)
        .enumerate()
        .map(|(i, _)| -> i32 { numbers[i..=i + 2].iter().sum() })
        .collect::<Vec<i32>>();

    println!("Part 2: {}", n_increments(&numbers));
}
