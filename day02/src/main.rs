// Find the number of lines (reports) which conform to:
// A) Values are either increasing or decreasing
// B) Two adjacent values differ by at least one and at most three

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Couldn't read input.txt");

    // Part One
    let mut num_safe_reports = 0;

    for report in input.lines() {
        let values: Vec<&str> = report.split_ascii_whitespace().collect();

        if is_report_safe(&values) {
            num_safe_reports += 1;
        }
    }
    println!("Part One Solution: {num_safe_reports}");

    // Part Two
    let mut num_safe_reports = 0;

    for report in input.lines() {
        let values: Vec<&str> = report.split_ascii_whitespace().collect();

        let mut report_is_safe = false;
        for i in 0..values.len() {
            let mut mutated_report = values.clone();
            mutated_report.remove(i);

            if is_report_safe(&mutated_report) {
                report_is_safe = true;
                break;
            }
        }
        if report_is_safe {
            num_safe_reports += 1;
        }
    }

    println!("Part Two Solution: {num_safe_reports}");
}

fn is_report_safe(report: &[&str]) -> bool {
    // println!("report: {report}");
    let mut prev_val = -1;
    let mut slope_trend = 0;
    let mut report_safe = true;

    // For each value in a report
    for c in report {
        let v: i32 = c.parse().unwrap();

        // First value
        if prev_val == -1 {
            prev_val = v;
            continue;
        }

        let difference = v - prev_val;
        let difference_abs = difference.abs();
        prev_val = v;
        if difference_abs < 1 || difference_abs > 3 {
            // println!("report unsafe due to a difference of {difference} at {c}");
            report_safe = false;
            break;
        }

        let slope = match difference {
            x if x > 0 => 1,
            x if x < 0 => -1,
            _ => unreachable!(),
        };

        // Second value
        if slope_trend == 0 {
            slope_trend = slope;
            continue;
        }

        if slope != slope_trend {
            // println!("report unsafe due to a varying slope at {c}");
            report_safe = false;
            break;
        }
    }

    if report_safe {
        // println!("report safe");
    }
    // println!("");
    report_safe
}
