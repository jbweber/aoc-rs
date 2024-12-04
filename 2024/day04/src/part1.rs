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
            if grid.grid[y][x] == 'X' {
                if look_left(&grid, x, y) {
                    dbg!("{},{},{}", y, x, "l");
                    count += 1;
                }

                if look_leftd(&grid, x, y) {
                    dbg!("{},{},{}", y, x, "l");
                    count += 1;
                }

                if look_right(&grid, x, y) {
                    dbg!("{},{},{}", y, x, "r");
                    count += 1;
                }

                if look_rightd(&grid, x, y) {
                    dbg!("{},{},{}", y, x, "r");
                    count += 1;
                }

                if look_up(&grid, x, y) {
                    dbg!("{},{},{}", y, x, "u");
                    count += 1;
                }

                if look_upd(&grid, x, y) {
                    dbg!("{},{},{}", y, x, "u");
                    count += 1;
                }

                if look_down(&grid, x, y) {
                    dbg!("{},{},{}", y, x, "d");
                    count += 1;
                }

                if look_downd(&grid, x, y) {
                    dbg!("{},{},{}", y, x, "d");
                    count += 1;
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

fn look_left(grid: &Grid, x: usize, y: usize) -> bool {
    if !x.checked_sub(3).is_none() {
        if grid.grid[y][x - 1] == 'M' && grid.grid[y][x - 2] == 'A' && grid.grid[y][x - 3] == 'S' {
            return true;
        }
    }
    false
}

fn look_leftd(grid: &Grid, x: usize, y: usize) -> bool {
    if !x.checked_sub(3).is_none() && !y.checked_sub(3).is_none() {
        if grid.grid[y - 1][x - 1] == 'M'
            && grid.grid[y - 2][x - 2] == 'A'
            && grid.grid[y - 3][x - 3] == 'S'
        {
            return true;
        }
    }
    false
}

fn look_right(grid: &Grid, x: usize, y: usize) -> bool {
    if x + 3 < grid.width {
        if grid.grid[y][x + 1] == 'M' && grid.grid[y][x + 2] == 'A' && grid.grid[y][x + 3] == 'S' {
            return true;
        }
    }
    false
}

fn look_rightd(grid: &Grid, x: usize, y: usize) -> bool {
    if x + 3 < grid.width && y + 3 < grid.height {
        if grid.grid[y + 1][x + 1] == 'M'
            && grid.grid[y + 2][x + 2] == 'A'
            && grid.grid[y + 3][x + 3] == 'S'
        {
            return true;
        }
    }
    false
}

fn look_up(grid: &Grid, x: usize, y: usize) -> bool {
    if y + 3 < grid.height {
        if grid.grid[y + 1][x] == 'M' && grid.grid[y + 2][x] == 'A' && grid.grid[y + 3][x] == 'S' {
            return true;
        }
    }
    false
}

fn look_upd(grid: &Grid, x: usize, y: usize) -> bool {
    if y + 3 < grid.height && !x.checked_sub(3).is_none() {
        if grid.grid[y + 1][x - 1] == 'M'
            && grid.grid[y + 2][x - 2] == 'A'
            && grid.grid[y + 3][x - 3] == 'S'
        {
            return true;
        }
    }
    false
}

fn look_down(grid: &Grid, x: usize, y: usize) -> bool {
    if !y.checked_sub(3).is_none() {
        if grid.grid[y - 1][x] == 'M' && grid.grid[y - 2][x] == 'A' && grid.grid[y - 3][x] == 'S' {
            return true;
        }
    }
    false
}

fn look_downd(grid: &Grid, x: usize, y: usize) -> bool {
    if !y.checked_sub(3).is_none() && x + 3 < grid.width {
        if grid.grid[y - 1][x + 1] == 'M'
            && grid.grid[y - 2][x + 2] == 'A'
            && grid.grid[y - 3][x + 3] == 'S'
        {
            return true;
        }
    }
    false
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
        assert_eq!("18", process(input)?);
        Ok(())
    }
}
