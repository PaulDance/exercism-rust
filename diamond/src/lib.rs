/// Build the top half then append itself reversed to itself.
pub fn get_diamond(dest: char) -> Vec<String> {
    let mut half = ('A'..=dest)
        .enumerate()
        .map(|(i, chr)| {
            let side = " ".repeat(dest as usize - chr as usize);
            let chr_str = chr.to_string();

            side.clone()
                + &if i == 0 {
                    chr_str
                } else {
                    chr_str.clone() + &" ".repeat((2 * i).saturating_sub(1)) + &chr_str
                }
                + &side
        })
        .collect::<Vec<String>>();

    half.append(&mut half.iter().rev().skip(1).cloned().collect());
    half
}
