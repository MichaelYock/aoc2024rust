advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    //let test = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";

    let mut safe = 0;

    for line in input.lines() {
        let report: Vec<u32> = line.split_whitespace().filter_map(|n| n.parse().ok()).collect();

        if is_safe(&report) {
            safe += 1;
        }
    }

    Some(safe)
}

pub fn part_two(input: &str) -> Option<u32> {
    let test = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";

    let mut safe = 0;

    for line in input.lines() {
        let report: Vec<u32> = line.split_whitespace().filter_map(|n| n.parse().ok()).collect();

        if is_safe(&report) {
            safe += 1;
            continue;
        }

        dampen(&report, &mut safe);
    }
    
    Some(safe)
}

fn is_safe(report: &[u32]) -> bool {
    let safe_increase = report.windows(2).all(|lvls| (lvls[0] < lvls[1]) && (lvls[1] - lvls[0] <= 3));
    let safe_decrease = report.windows(2).all(|lvls| (lvls[0] > lvls[1]) && (lvls[0] - lvls[1] <= 3));

    safe_decrease || safe_increase
}

fn dampen(report: &[u32], safe: &mut u32) {
    let mut report = report.to_vec();
    for (i, _) in report.clone().iter().enumerate() {
        let removed = report.remove(i);
        
        if is_safe(&report) {
            *safe += 1;
            report.insert(i, removed);
            return
        }
        report.insert(i, removed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
