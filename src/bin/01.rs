advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {

    let (mut left, mut right) = (Vec::new(), Vec::new());

    for line in input.lines() {
        let nums: Vec<i32> = line.split_whitespace().filter_map(|n| n.parse().ok()).collect();

        (left.push(nums[0]), right.push(nums[1]));
    }

    (left.sort(), right.sort());

    let sum = left.iter().zip(right.iter()).map(|(a,b)| (a - b).abs()).sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<i32> {
    //let test = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";

    let (mut left, mut right) = (Vec::new(), Vec::new());
    let mut sum: i32= 0;

    for line in input.lines() {
        let nums: Vec<i32> = line.split_whitespace().filter_map(|n| n.parse().ok()).collect();

        (left.push(nums[0]), right.push(nums[1]));
    }

    for num_left in left {
        sum += right.iter().filter(|&num_right| *num_right == num_left).sum::<i32>();
    }

    Some(sum)
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
