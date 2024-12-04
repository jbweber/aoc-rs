use std::collections::HashSet;

pub fn process(input: &str) -> anyhow::Result<String> {
    let lines = input
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let width = lines[0].len();
    let height = lines.len();

    dbg!(width);
    dbg!(height);
    let grid = Grid {
        grid: lines,
        width,
        height,
    };

    let mut count = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            // START FROM A and look in X shape
            if grid.grid[y][x] == 'A' {
                if !y.checked_sub(1).is_none()
                    && y + 1 < grid.height
                    && !x.checked_sub(1).is_none()
                    && x + 1 < grid.width
                {
                    // up left y-1,x-1
                    let a = grid.grid[y - 1][x - 1];
                    // up right y-1,x+1
                    let b = grid.grid[y - 1][x + 1];
                    // down left y+1,x-1
                    let c = grid.grid[y + 1][x - 1];
                    // down right y+1,x+1
                    let d = grid.grid[y + 1][x + 1];

                    let ul = (a == 'M' && d == 'S');
                    let ur = (b == 'M' && c == 'S');
                    let dl = (c == 'M' && b == 'S');
                    let dr = (d == 'M' && a == 'S');

                    if (ul && ur) || (dl && dr) || (ul && dl) || (ur && dr) {
                        count += 1;
                    }
                }
            }
        }
    }
    dbg!(count);

    Ok(count.to_string())
}

struct Grid {
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!("9", process(input)?);
        Ok(())
    }
}
