use itertools::Itertools;

/// Scoring category.
pub enum Category {
    Ones = 1,
    Twos = 2,
    Threes = 3,
    Fours = 4,
    Fives = 5,
    Sixes = 6,
    FullHouse,
    FourOfAKind,
    LittleStraight = 9,
    BigStraight = 10,
    Choice,
    Yacht,
}

/// Dice throw.
type Dices = [u8; 5];

/// Returns the `dice` score considering only the given `category`.
pub fn score(dices: Dices, category: Category) -> u8 {
    let mut itr = dices.into_iter();

    match category {
        // Use the discriminant to factor all cases together: the face value to
        // count equals the multiplier to get the score.
        cat @ (Category::Ones
        | Category::Twos
        | Category::Threes
        | Category::Fours
        | Category::Fives
        | Category::Sixes) => {
            let cat = cat as u8;
            cat * itr.filter(|&d| d == cat).count() as u8
        }

        // Count occurences and check if there is only two and three then use
        // the fact that `bool as u8` is `0` or `1` to map to another integer
        // by multiplying with the total face sum or `0`.
        Category::FullHouse => {
            let counts = itr.counts();
            counts.values().copied().sorted_unstable().eq(2..=3) as u8
                * counts
                    .into_iter()
                    .map(|(face, count)| face * count as u8)
                    .sum::<u8>()
        }

        // Use counts as well.
        Category::FourOfAKind => itr
            .counts()
            .into_iter()
            .find(|&(_, count)| count >= 4)
            .map_or(0, |(face, _)| 4 * face),

        // Use discriminant and bool trick again to factor all cases together.
        cat @ (Category::LittleStraight | Category::BigStraight) => {
            let cat = cat as u8;
            30 * itr.sorted().eq(cat - 8..=cat - 4) as u8
        }

        // By definition.
        Category::Choice => itr.sum(),

        // Same bool trick.
        Category::Yacht => 50 * itr.all_equal() as u8,
    }
}
