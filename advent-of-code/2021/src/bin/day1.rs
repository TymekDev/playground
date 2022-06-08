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

fn main() {
    let result = get_input()
        .lines()
        .flat_map(str::parse::<i32>)
        .collect::<Vec<i32>>()
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

    println!("{}", result);
}
