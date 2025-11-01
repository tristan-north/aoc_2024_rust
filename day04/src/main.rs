use std::num;

fn main() {
    let input = std::fs::read_to_string("input_example.txt").expect("Couldn't read input.");

    let input_lines = input.lines().collect::<Vec<&str>>();

    let base_grid = vec![0; input_lines.len()];

    for line in input_lines.iter() {}

    let mut accum = 0;
    for line in input_lines.iter() {
        println!("{line}");

        accum += line.matches("XMAS").count();
        accum += line.matches("SAMX").count();
    }

    // Horizontal

    // Horizontal Backwards

    // Vertically

    // Vertically Backwards

    // First diagonal
    let diagonal = create_diagonals(&input_lines, false);
    for line in &diagonal {
        println!("{:?}", line);
    }

    // First diagonal backwards

    // Second diagonal
    let diagonal = create_diagonals(&input_lines, true);
    for line in &diagonal {
        println!("{:?}", line);
    }

    // Second diagonal backwards

    println!("Part One solution: {accum}");
}

fn create_diagonals(input_lines: &Vec<&str>, other_diagonal: bool) -> Vec<Vec<(char)>> {
    let grid_width = input_lines.len() as i32;
    let mut result = vec![];

    for i in -grid_width + 1..grid_width {
        let mut result_line = vec![];

        let first_coord = (std::cmp::max(0, i), std::cmp::max(0, -i));

        let num_values_in_diag_line = grid_width - i.abs();
        for k in 0..num_values_in_diag_line {
            let x = first_coord.0 + k;
            let mut y = first_coord.1 + k;

            if other_diagonal {
                y = grid_width - y - 1;
            }

            let element = input_lines[y as usize]
                .chars()
                .nth(x as usize)
                .expect("Trying to get char out of bounds");
            result_line.push(element);
        }

        result.push(result_line);
    }

    result
}
