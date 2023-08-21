const VOID_CHAR: char = ' ';
const MINE_CHAR: char = '*';

/// Produces an iterator over the coordinates of the given point's neighbors.
///
/// The returned pairs are checked against `0` and `*_len` in order to filter out invalid
/// coordinates for corners and edges, that is why `i` and `j` are converted to `i128`s.
fn neighbors(
    i: usize,
    j: usize,
    i_len: usize,
    j_len: usize,
) -> impl Iterator<Item = (usize, usize)> {
    let i = i as i128;
    let j = j as i128;
    vec![
        (i, j + 1),
        (i - 1, j + 1),
        (i - 1, j),
        (i - 1, j - 1),
        (i, j - 1),
        (i + 1, j - 1),
        (i + 1, j),
        (i + 1, j + 1),
    ]
    .into_iter()
    .filter(move |&(ni, nj)| 0 <= ni && ni < i_len as i128 && 0 <= nj && nj < j_len as i128)
    .map(|(ni, nj)| (ni as usize, nj as usize))
}

/// Counts and returns the number of mines among the given point's neighbors.
///
/// Panics if `minefield` is empty.
fn count_mines(i: usize, j: usize, minefield: &[&str]) -> u8 {
    neighbors(i, j, minefield.len(), minefield[0].len())
        .filter(|&(ni, nj)| minefield[ni].chars().nth(nj).unwrap() == MINE_CHAR)
        .count() as u8
}

/// Converts the output of `count_mines` to a character with `0` as `VOID_CHAR`.
fn make_annotation(i: usize, j: usize, minefield: &[&str]) -> char {
    match count_mines(i, j, minefield) {
        0 => VOID_CHAR,
        n => n.to_string().chars().next().unwrap(),
    }
}

/// Builds an annotated minefield following the desired rules.
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    minefield
        .iter()
        .enumerate()
        .map(|(i, s)| {
            s.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    MINE_CHAR => MINE_CHAR,
                    _ => make_annotation(i, j, minefield),
                })
                .collect()
        })
        .collect()
}
