fn main() {
    let input = std::fs::read_to_string("input_example.txt").expect("Couldn't read input");
    let input = input.trim_ascii_end();

    let mut accum = 0;

    for (i, _) in input.match_indices("mul(") {
        let start = i;
        let end = std::cmp::min(i + 12, input.len());

        let substr = &input[start..end];

        // Check first number
        let substr = &substr[4..];
        println!("{substr}");

        let first_num = match get_next_number(substr) {
            Some(v) => v,
            _ => continue,
        };
        // substr now starts at ,
        let substr = &substr[first_num.len()..];

        if !is_next_char(substr, ',') {
            continue;
        }

        // Check second number
        let substr = &substr[1..];
        let second_num = match get_next_number(substr) {
            Some(v) => v,
            _ => continue,
        };
        // substr now starts at )
        let substr = &substr[second_num.len()..];

        if !is_next_char(substr, ')') {
            continue;
        }

        println!("Found {first_num} * {second_num}");

        accum += first_num.parse::<u32>().unwrap() * second_num.parse::<u32>().unwrap();
    }

    println!("\nPart One Solution: {accum}");
    ()
}

fn is_next_char(text: &str, test_char: char) -> bool {
    match text.chars().next() {
        Some(next_char) => next_char == test_char,
        _ => false,
    }
}

fn get_next_number(text: &str) -> Option<String> {
    let first_num = text
        .chars()
        .take_while(char::is_ascii_digit)
        .collect::<String>();

    if first_num.len() == 0 {
        // No initial digit
        return None;
    } else {
        return Some(first_num);
    }
}
