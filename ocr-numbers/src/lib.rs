/// Array containing grids of characters representing the digits 0 through 9 graphically.
const DIGITS: [[[char; 3]; 4]; 10] = [
    [
        [' ', '_', ' '],
        ['|', ' ', '|'],
        ['|', '_', '|'],
        [' ', ' ', ' '],
    ],
    [
        [' ', ' ', ' '],
        [' ', ' ', '|'],
        [' ', ' ', '|'],
        [' ', ' ', ' '],
    ],
    [
        [' ', '_', ' '],
        [' ', '_', '|'],
        ['|', '_', ' '],
        [' ', ' ', ' '],
    ],
    [
        [' ', '_', ' '],
        [' ', '_', '|'],
        [' ', '_', '|'],
        [' ', ' ', ' '],
    ],
    [
        [' ', ' ', ' '],
        ['|', '_', '|'],
        [' ', ' ', '|'],
        [' ', ' ', ' '],
    ],
    [
        [' ', '_', ' '],
        ['|', '_', ' '],
        [' ', '_', '|'],
        [' ', ' ', ' '],
    ],
    [
        [' ', '_', ' '],
        ['|', '_', ' '],
        ['|', '_', '|'],
        [' ', ' ', ' '],
    ],
    [
        [' ', '_', ' '],
        [' ', ' ', '|'],
        [' ', ' ', '|'],
        [' ', ' ', ' '],
    ],
    [
        [' ', '_', ' '],
        ['|', '_', '|'],
        ['|', '_', '|'],
        [' ', ' ', ' '],
    ],
    [
        [' ', '_', ' '],
        ['|', '_', '|'],
        [' ', '_', '|'],
        [' ', ' ', ' '],
    ],
];

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

/// Converts one graphical digit from the given `grid` at top-left position `(i, j)` into a
/// numerical character if it is correct, `'?'` otherwise.
fn convert_one(grid: &Vec<Vec<char>>, i: usize, j: usize) -> char {
    for num in 0..=9 {
        let mut found = true;

        // Manual equality between `DIGITS[num]` and sub-grid `grid[i..i + 4][j..j + 3]`.
        for line in 0..4 {
            if grid[line + i][j..j + 3] != DIGITS[num][line] {
                found = false;
                break;
            }
        }

        if found {
            return num.to_string().chars().next().unwrap();
        }
    }

    '?'
}

/// Converts a given `input` string containing graphical digits into a string of normal digits.
pub fn convert(input: &str) -> Result<String, Error> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    if grid.len() % 4 != 0 {
        Err(Error::InvalidRowCount(grid.len()))
    } else {
        let mut res = String::new();

        // Line groups.
        for i in (0..grid.len()).step_by(4) {
            if grid[i].len() % 3 != 0 {
                return Err(Error::InvalidColumnCount(grid[i].len()));
            } else {
                if !res.is_empty() {
                    res.push(',');
                }

                // Column groups.
                for j in (0..grid[i].len()).step_by(3) {
                    res.push(convert_one(&grid, i, j));
                }
            }
        }

        Ok(res)
    }
}
