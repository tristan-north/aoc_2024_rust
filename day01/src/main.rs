fn main() -> std::io::Result<()> {
    let (a, b) = get_sorted_lists("input.txt")?;

    part1(&a, &b);
    part2(&a, &b);
    Ok(())
}

fn part2(a_list: &Vec<i32>, b_list: &Vec<i32>) {
    let mut accum = 0;
    for a_val in a_list {
        let occurances_in_b = b_list.iter().filter(|&b_val| *b_val == *a_val).count();
        accum += *a_val as usize * occurances_in_b;
    }

    println!("part 2 solution: {accum}");
}

fn part1(a: &Vec<i32>, b: &Vec<i32>) {
    let accum: i32 = a.iter().zip(b).map(|(a, b)| (a - b).abs()).sum();

    println!("part 1 solution: {accum}");
}

fn get_sorted_lists(input_path: &str) -> std::io::Result<(Vec<i32>, Vec<i32>)> {
    let input = std::fs::read_to_string(input_path)?;
    let (mut a, mut b): (Vec<i32>, Vec<i32>) = input
        .lines()
        .filter_map(|line| line.split_once("   "))
        .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
        .unzip();

    a.sort();
    b.sort();

    Ok((a, b))
}
