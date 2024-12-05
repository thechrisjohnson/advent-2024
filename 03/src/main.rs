use std::io::Read;

fn main() {
    let mut input = String::new();
    let _data = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut input).unwrap();

    let reports = get_levels(&input);
    let mut safe_reports = 0;

    // Check each report
    for report in &reports
    {
        if is_safe_report(report) {
            println!("{:?} is safe report!", &report);
            safe_reports += 1;
        } else {
            println!("{:?} is NOT safe report!", &report);
        }
    }
    
    println!("Safe reports: {}", &safe_reports);
}

fn get_levels(list: &str) -> Vec<Vec<i32>> {
    let mut outer = Vec::new();
    for line in list.lines() {
        let mut inner = Vec::new();
        for i in line.split_ascii_whitespace() {
            inner.push(i.parse::<i32>().unwrap());
        }
        outer.push(inner);
    }

    outer
}

fn is_safe_report(level: &Vec<i32>) -> bool {
    let mut direction: Option<Direction> = None;
    let mut current = level.first().unwrap();
    for next in level.iter().skip(1)
    {
        let diff = (current - next).abs();
        if diff < 0 || diff > 3
        {
            return false;
        }

        match &direction {
            Some(d) => 
            {
                if next > current && *d == Direction::Decreasing {
                    return false;
                } else if next < current && *d == Direction::Increasing {
                    return false;
                }
            },
            None => 
            {
                if next > current {
                    direction = Some(Direction::Increasing)
                } else {
                    direction = Some(Direction::Decreasing)
                }
            }
        }

        current = next;
    }

    true
}

#[derive(PartialEq)]
enum Direction
{
    Increasing,
    Decreasing
}
