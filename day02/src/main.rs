// Find the number of lines (reports) which conform to:
// A) Values are either increasing or decreasing
// B) Two adjacent values differ by at least one and at most three

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Couldn't read input.txt");

    let mut num_safe_reports = 0;

    for report in input.lines() {
        println!("report: {report}");
        let mut prev_val = -1;
        let mut slope_trend = 0;
        let mut report_safe = true;

        // For each value in a report
        for c in report.split_ascii_whitespace() {
            let v: i32 = c.parse().unwrap();

            // First value
            if prev_val == -1 {
                prev_val = v;
                continue;
            }

            let difference = (v - prev_val);
            let difference_abs = difference.abs();
            prev_val = v;
            if difference_abs < 1 || difference_abs > 3 {
                println!("report unsafe due to a difference of {difference} at {c}");
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
                println!("report unsafe due to a varying slope at {c}");
                report_safe = false;
                break;
            }
        }

        if report_safe {
            num_safe_reports += 1;
            println!("report safe");
        }
        println!("");
    }

    println!("Numer of safe reports: {num_safe_reports}");
}
