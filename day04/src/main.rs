fn main() {
    let input = std::fs::read_to_string("input_example.txt").expect("Couldn't read input.");
    let row_length = input.lines().next().unwrap().len();

    let base_grid = vec![0; row_length];

    for line in input.lines() {}

    let mut accum = 0;
    for line in input.lines() {
        println!("{line}");

        accum += line.matches("XMAS").count();
        accum += line.matches("SAMX").count();
    }

    println!("Part One solution: {accum}");
}
