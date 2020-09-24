use itertools::Itertools;

/// The character representing a corner or vertex.
const CORNER: char = '+';
/// The character representing a horizontal side or edge.
const HORZ_SIDE: char = '-';
/// The character representing a vertical side of edge.
const VERT_SIDE: char = '|';

/// Extracts the positions of the corners in the given `lines`.
fn corners_pos(lines: &[&str]) -> Vec<(usize, usize)> {
    lines
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_j, chr)| chr == CORNER)
                .map(move |(j, _chr)| (i, j))
        })
        .collect()
}

/// Returns `true` iff the two given points `(i1, j1)` and `(i2, j2)` are linked by a correct edge.
fn are_linked(i1: usize, j1: usize, i2: usize, j2: usize, lines: &[&str]) -> bool {
    if i1 == i2 {
        lines[i1]
            .chars()
            .skip(j1.min(j2))
            .take(j2.max(j1) - j1.min(j2))
            .all(|chr| chr == CORNER || chr == HORZ_SIDE)
    } else if j1 == j2 {
        lines
            .iter()
            .skip(i1.min(i2))
            .take(i2.max(i1) - i1.min(i2))
            .map(|line| line.chars().nth(j1).unwrap())
            .all(|chr| chr == CORNER || chr == VERT_SIDE)
    } else {
        false
    }
}

/// Returns `true` iff the given `corners` form a correct rectangle in the `lines`.
fn is_rectangle(corners: &[(usize, usize)], lines: &[&str]) -> bool {
    (0..corners.len()).all(|i| {
        // Accept consecutive links,
        are_linked(
            corners[i].0,
            corners[i].1,
            corners[(i + 1) % corners.len()].0,
            corners[(i + 1) % corners.len()].1,
            lines,
        )
        // but reject diagonal or flat links.
        && !are_linked(
            corners[i].0,
            corners[i].1,
            corners[(i + 2) % corners.len()].0,
            corners[(i + 2) % corners.len()].1,
            lines,
        )
    })
}

/// Sorts the given four possibly rectangle `corners` by trigonometric order: the result is a
/// vector of the vertices starting from the top-left corner and then going counterclockwise.
fn sort_trigo(corners: &[&(usize, usize)]) -> Vec<(usize, usize)> {
    // Adding coordinates enables sorting by the main axis.
    let (&&top_left, &&bot_right) = corners
        .iter()
        .minmax_by_key(|(i, j)| i + j)
        .into_option()
        .unwrap();
    // Using (j + 1) / (i + 1) as a sorting key selects along the secondary axis:
    // when j is small and i big, j / i is small: the bottom-left corner is the
    // min; when j is big and i is small, j / i is bit: the top-right is the max.
    // +1 on both sides is to avoid equalities and divisions by zero.
    let (&&bot_left, &&top_right) = corners
        .iter()
        .minmax_by_key(|&&&(i, j)| (j as f64 + 1.0) / (i as f64 + 1.0))
        .into_option()
        .unwrap();
    vec![top_left, bot_left, bot_right, top_right]
}

/// Counts and returns the number of correct rectangles graphically represented in the given `lines`.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// assert_eq!(
///     5,
///     rectangles::count(&[
///         "+-----+",
///         "|     |",
///         "+--+--+",
///         "|  |  |",
///         "+--+--+",
///     ]),
/// );
/// ```
///
/// Lots of combinations:
///
/// ```rust
/// assert_eq!(
///     225,
///     rectangles::count(&[
///         "++++++",
///         "++++++",
///         "++++++",
///         "++++++",
///         "++++++",
///         "++++++",
///     ]),
/// );
/// ```
pub fn count(lines: &[&str]) -> u32 {
    corners_pos(lines)
        .iter()
        .combinations(4)
        .map(|cmb| sort_trigo(cmb.as_slice()))
        .filter(|cmb| is_rectangle(cmb.as_slice(), lines))
        .count() as u32
}
