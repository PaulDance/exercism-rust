/// Representation of tile side numbers.
pub type Value = u8;
/// Representation of a tile.
pub type Domino = (Value, Value);

/// Builds a new chain for the given set of dominoes or `None` if impossible.
pub fn chain(doms: &[Domino]) -> Option<Vec<Domino>> {
    doms.is_empty().then(Vec::new).or_else(|| {
        // Any can be the first.
        let mut doms = doms.to_vec();
        let dom = doms[0];
        try_swap_chain(&mut doms[1..], dom.1, dom.0).then(|| doms)
    })
}

/// Attempts to build a chain by swapping the elements of `doms` with `start`
/// and `end` side value constraints.
fn try_swap_chain(doms: &mut [Domino], start: Value, end: Value) -> bool {
    // Success if already chained.
    doms.is_empty() && start == end || {
        // Try all to find a fitting one.
        for i in 0..doms.len() {
            // Try flipping the tile.
            if doms[i].1 == start {
                doms[i] = (doms[i].1, doms[i].0);
            } else if doms[i].0 != start {
                // Skip if both sides are invalid.
                continue;
            }

            doms.swap(0, i);
            let dom = doms[0];

            if try_swap_chain(&mut doms[1..], dom.1, end) {
                return true;
            }
        }

        false
    }
}
