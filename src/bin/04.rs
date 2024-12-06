use advent_of_code::VecTExt;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    // let test = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX";

    let grid: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    let mas: Vec<_> = "MAS".chars().enumerate().map(|(i,c)|((i + 1) as isize, c)).collect();

    let directions = vec![(1,1),(1,0),(1,-1),(0,1),(0,-1),(-1,1),(-1,0),(-1,-1)];

    let mut counter = 0;

    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == 'X' {
                let (r,c) = (r as isize, c as isize);

                for (dr, dc) in directions.iter() {
                    if mas.iter().all(|(step, char)|{
                        let x = r + dr * step;
                        let y = c + dc * step;

                        grid.iget(x).and_then(|e| e.iget(y)) == Some(char)
                    }) {

                        counter += 1;
                    }
                }
            }
        }
    }

    Some(counter)
    
}

pub fn part_two(input: &str) -> Option<u32> {
    //let test = ".M.S......\n..A..MSMS.\n.M.S.MAA..\n..A.ASMSM.\n.M.S.M....\n..........\nS.S.S.S.S.\n.A.A.A.A..\nM.M.M.M.M.\n..........";
    
    let grid: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut counter = 0;

    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == 'A' {
                let (r,c) = (r as isize, c as isize);

                if let (Some(&tl),Some(&tr),Some(&bl),Some(&br)) = (
                    grid.iget(r-1).and_then(|e| e.iget(c-1)),
                    grid.iget(r+1).and_then(|e| e.iget(c-1)),
                    grid.iget(r-1).and_then(|e| e.iget(c+1)),
                    grid.iget(r+1).and_then(|e| e.iget(c+1))
                ) {
                    let fwd = (tl == 'M' && br == 'S') || (tl == 'S' && br == 'M');
                    let back = (tr == 'M' && bl == 'S') || (tr == 'S' && bl == 'M');

                    if fwd && back {
                        counter += 1;
                    }
                }
            }
        }
    }

    Some(counter)
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
