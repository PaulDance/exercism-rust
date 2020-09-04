use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.keys()
        .flat_map(|&score| {
            h[&score]
                .iter()
                .map(move |letter| (letter.to_ascii_lowercase(), score))
        })
        .collect()
}
