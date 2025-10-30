#[derive(PartialEq, Debug)]
enum ExecuteState {
    Enabled,
    Disabled,
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Couldn't read input");
    let input = input.trim_ascii_end();

    let mut accum = 0;
    let mut execute_check_start_index = 0;
    let mut execute_state = ExecuteState::Enabled;

    for (i, _) in input.match_indices("mul(") {
        // Check for any do and don't commands betwen where we last checked
        // up to the current mul match
        let execute_check_slice = &input[execute_check_start_index..i];
        if let Some(v) = update_execute_state(execute_check_slice) {
            execute_state = v;
            println!("Execute state changed to: {execute_state:?}");
        }
        if execute_state == ExecuteState::Disabled {
            execute_check_start_index = i;
            continue;
        }

        // End slice for easy printing of the relevent chars
        let end = std::cmp::min(i + 12, input.len());

        let substr = &input[i..end];

        // Check first number
        let substr = &substr[4..];
        println!("{substr}");

        let first_num = match get_next_number(substr) {
            Some(v) => v,
            _ => {
                execute_check_start_index = i;
                continue;
            }
        };
        // substr now starts at ,
        let substr = &substr[first_num.len()..];

        if !is_next_char(substr, ',') {
            execute_check_start_index = i;
            continue;
        }

        // Check second number
        let substr = &substr[1..];
        let second_num = match get_next_number(substr) {
            Some(v) => v,
            _ => {
                execute_check_start_index = i;
                continue;
            }
        };
        // substr now starts at )
        let substr = &substr[second_num.len()..];

        if !is_next_char(substr, ')') {
            execute_check_start_index = i;
            continue;
        }

        println!("Found {first_num} * {second_num}");

        accum += first_num.parse::<u32>().unwrap() * second_num.parse::<u32>().unwrap();
        execute_check_start_index = i;
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

fn update_execute_state(text: &str) -> Option<ExecuteState> {
    let do_index = match text.rmatch_indices("do()").next() {
        Some(v) => {
            let (i, _) = v;
            i as i32
        }
        _ => -1,
    };

    let dont_index = match text.rmatch_indices("don't()").next() {
        Some(v) => {
            let (i, _) = v;
            i as i32
        }
        _ => -1,
    };

    if do_index == -1 && dont_index == -1 {
        return None;
    }

    if do_index > dont_index {
        return Some(ExecuteState::Enabled);
    } else {
        return Some(ExecuteState::Disabled);
    }
}
