fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let (mut a, mut b): (Vec<i32>, Vec<i32>) = input
        .lines()
        .filter_map(|line| line.split_once("   "))
        .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
        .unzip();

    a.sort();
    b.sort();

    let accum: i32 = a.iter().zip(b).map(|(a, b)| (a - b).abs()).sum();

    println!("{accum}");

    Ok(())
}
