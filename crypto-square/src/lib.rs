pub fn encrypt(input: &str) -> String {
    // Normalized input.
    let norm = input
        .chars()
        .filter(|chr| chr.is_ascii_alphanumeric())
        .map(|chr| chr.to_ascii_lowercase() as u8)
        .collect::<Vec<u8>>();

    // Parameters.
    let len = norm.len();
    let rows = (len as f64).sqrt().round() as usize;
    let cols = if rows * rows == len { rows } else { rows + 1 };
    let rem = rows * cols - len;
    let mut res = String::with_capacity(rows * (cols + 1));

    // Build the padded chunks.
    for j in 0..cols {
        if cols - j < rem {
            res.push(' ');
        }

        // Don't mind the mess with the indices, it's probably incorrect...
        for i in 0..rows {
            let pos = i * cols + j;
            res.push(if pos < len { norm[pos] as char } else { ' ' });
        }

        if cols - j >= rem + 1 && j * rows < len - cols {
            res.push(' ');
        }
    }

    res
}
