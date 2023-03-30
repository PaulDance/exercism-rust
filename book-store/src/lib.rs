const DISCOUNT_PRICE: [u32; 6] = [0, 800, 1520, 2160, 2560, 3000];

#[must_use]
pub fn lowest_price(books: &[u32]) -> u32 {
    let mut counter = vec![(0, 1), (0, 2), (0, 3), (0, 4), (0, 5)];

    for book in books {
        counter[*book as usize - 1].0 += 1;
    }

    sort_by_count(&mut counter);
    lowest_price_counter(&counter)
}

/// Finds the lowest price by generating all possible combinations.
fn lowest_price_counter(counter: &[(u32, u8)]) -> u32 {
    let mut price = counter.iter().map(|(c, _)| c).sum::<u32>() * 800;

    for (i, dp) in DISCOUNT_PRICE
        .iter()
        .enumerate()
        .take(counter.iter().filter(|&&(c, _)| c > 0).count() + 1)
        .skip(2)
    {
        let mut new_counter = counter.to_vec();

        for j in new_counter.iter_mut().take(i) {
            j.0 -= 1;
        }

        sort_by_count(&mut new_counter);
        price = price.min(dp + lowest_price_counter(&new_counter));
    }

    price
}

/// Sorts by first key.
fn sort_by_count(counter: &mut [(u32, u8)]) {
    counter.sort_unstable_by(|(a, _), (b, _)| b.cmp(a));
}
