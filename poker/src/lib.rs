//! If you haven't noticed already, it ended up uglier than I first imagined,
//! especially the comparison implementation for hands. Hope the comments help.

use itertools::Itertools;
use std::cmp::Ordering;

/// Given a list of poker hands, returns a list of those hands which win.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    // Extract hands and save references to the strings.
    let mut built_hands = hands.iter().map(|&hand| Hand::from(hand)).collect_vec();
    // Sort them in reverse order: notice x and y and swapped.
    built_hands.sort_unstable_by(|x, y| y.partial_cmp(x).unwrap_or(Ordering::Equal));

    // Return the equivalent hands, starting from the highest ranks.
    let top = built_hands[0].clone();
    built_hands
        .into_iter()
        .take_while(|hand| hand.partial_cmp(&top) == Some(Ordering::Equal))
        .map(|hand| hand.string)
        .collect()
}

/// Represents a poker card. Adds a bit of abstraction.
#[derive(Debug, Clone, PartialEq)]
struct Card {
    /// The rank of the card as a character. `'0'` means 10.
    rank: char,
    /// The character representing the card's suit.
    suit: char,
}

/// Provides the constructor.
impl From<&str> for Card {
    /// Extracts the expected characters from a two-or-three-character string.
    fn from(value: &str) -> Self {
        let bytes = value.as_bytes();
        Self {
            rank: bytes[bytes.len() - 2] as char,
            suit: bytes[bytes.len() - 1] as char,
        }
    }
}

impl Card {
    /// Converts the card's rank to an integer.
    fn rank_number(&self) -> u8 {
        match self.rank {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            '0' => 10,
            chr => chr.to_digit(10).unwrap() as u8,
        }
    }

    /// Returns `true` iff the two cards have the same rank.
    fn is_same_rank(&self, other: &Self) -> bool {
        self.rank == other.rank
    }
}

/// Implementation on the rank number only.
impl PartialOrd<Self> for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.rank_number().partial_cmp(&other.rank_number())
    }
}

/// Represents a poker hand as a set of cards.
#[derive(Debug, Clone, PartialEq)]
struct Hand<'a> {
    /// The original string slice the hand was extracted from.
    string: &'a str,
    /// The set of cards, sorted by rank.
    cards: Vec<Card>,
}

/// Provides the constructor.
impl<'a> From<&'a str> for Hand<'a> {
    /// Extracts the cards from the given string slice and sorts them by rank.
    fn from(value: &'a str) -> Self {
        let mut cards = value
            .split_whitespace()
            .map(Card::from)
            .collect::<Vec<Card>>();
        cards.sort_unstable_by(|x, y| x.partial_cmp(y).unwrap_or(Ordering::Equal));
        Self {
            string: value,
            cards,
        }
    }
}

/// Most of the categorization logic.
impl<'a> Hand<'a> {
    /// Builds and returns the list of cards making pairs, only one by pair however.
    fn get_pairs(&self) -> Vec<Card> {
        let mut pairs = Vec::new();
        let mut i = 0;

        // Don't step by two or it would split some pairs.
        while i < self.cards.len() - 1 {
            if self.cards[i].is_same_rank(&self.cards[i + 1]) {
                pairs.push(self.cards[i].clone());
                i += 2; // Skip pair.
            } else {
                i += 1;
            }
        }

        pairs
    }

    /// Returns the number of pairs the hand has.
    fn count_pairs(&self) -> u8 {
        self.get_pairs().len() as u8
    }

    /// Builds and returns the list of `(count, rank)` couples from the hand's
    /// kinds, lexicographically sorted, i.e. by count first and then by rank.
    fn get_kinds(&self) -> Vec<(u8, u8)> {
        let mut kinds = Vec::new();
        let mut count = 1;
        let mut last_rank = self.cards[0].rank_number();

        // Simply count the successive series separated by changes in rank.
        for card in self.cards.iter().skip(1) {
            let rank = card.rank_number();

            if rank == last_rank {
                count += 1;
            } else {
                kinds.push((count, last_rank));
                count = 1;
                last_rank = rank;
            }
        }

        // Ensure last one and then sort.
        kinds.push((count, last_rank));
        kinds.sort_unstable();
        kinds
    }

    /// Extracts the kind counts from `get_kinds`'s output.
    fn get_kind_counts(&self) -> Vec<u8> {
        self.get_kinds()
            .into_iter()
            .map(|(count, _)| count)
            .collect()
    }

    /// Extracts the kind ranks from `get_kinds`'s output.
    fn get_kind_ranks(&self) -> Vec<u8> {
        self.get_kinds()
            .into_iter()
            .map(|(_, rank_num)| rank_num)
            .collect()
    }

    /// Returns the count of the most frequent kind.
    fn count_max_kind(&self) -> u8 {
        self.get_kind_counts().into_iter().max().unwrap_or(0)
    }

    /// Returns `true` iff the hand is a pair.
    fn is_one_pair(&self) -> bool {
        self.count_pairs() == 1
    }

    /// Returns `true` iff the hand is a two-pair.
    fn is_two_pair(&self) -> bool {
        self.count_pairs() == 2
    }

    /// Returns `true` iff the hand is a three-of-a-kind.
    fn is_three_of_a_kind(&self) -> bool {
        self.count_max_kind() == 3
    }

    /// Returns `true` iff the hand is a four-of-a-kind.
    fn is_four_of_a_kind(&self) -> bool {
        self.count_max_kind() == 4
    }

    /// Returns `true` iff the hand is a straight.
    fn is_straight(&self) -> bool {
        let mut last_rank = self.cards[0].rank_number();

        for card in self.cards.iter().skip(1) {
            if card.rank != 'A' && card.rank_number() != last_rank + 1 {
                return false;
            } else {
                last_rank += 1;
            }
        }

        true
    }

    /// Returns `true` iff the hand's composition indicates a baby straight, but
    /// does not check if it is actually a straight at all.
    fn is_baby_straight(&self) -> bool {
        self.cards[0].rank == '2' && self.cards.last().unwrap().rank == 'A'
    }

    /// Returns `true` iff the hand is a flush.
    fn is_flush(&self) -> bool {
        self.cards.iter().map(|card| card.suit).all_equal()
    }

    /// Returns `true` iff the hand is a full house.
    fn is_full_house(&self) -> bool {
        self.get_kind_counts() == [2, 3]
    }

    /// Returns `true` iff the hand is a straight flush.
    fn is_straight_flush(&self) -> bool {
        self.is_straight() && self.is_flush()
    }

    /// Returns the hand's category rank, 1 being straight flush and 9 high card.
    fn rank(&self) -> u8 {
        // Find the first check yielding true, going from highest to lowest rank.
        [
            Self::is_straight_flush,
            Self::is_four_of_a_kind,
            Self::is_full_house,
            Self::is_flush,
            Self::is_straight,
            Self::is_three_of_a_kind,
            Self::is_two_pair,
            Self::is_one_pair,
        ]
        .iter()
        .enumerate()
        .find(|(_, check)| check(self))
        .map(|(i, _)| i + 1) // Five-of-a-kind not considered here.
        .unwrap_or(9) as u8 // High card is last resort.
    }
}

/// The implementation for comparing hands.
impl<'a> PartialOrd<Self> for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let my_rank = self.rank();
        let other_rank = other.rank();

        // Compare first by rank:
        match other_rank.partial_cmp(&my_rank) {
            // if they are different, return that comparison;
            cmp if cmp != Some(Ordering::Equal) => cmp,
            // otherwise compare further:
            _ => match my_rank {
                // straights: comparing one extremity card is sufficient
                1 | 5 => {
                    // ... except when aces get involved, then choose carefully
                    // which card to use in order to compare successfully;
                    let (my_cmp_idx, oth_cmp_idx) = if self.is_baby_straight() {
                        if other.is_baby_straight() {
                            return Some(Ordering::Equal);
                        } else {
                            (self.cards.len() - 2, self.cards.len() - 1)
                        }
                    } else if other.is_baby_straight() {
                        (self.cards.len() - 1, self.cards.len() - 2)
                    } else {
                        (0, 0)
                    };
                    // the comparison itself;
                    self.cards[my_cmp_idx]
                        .rank_number()
                        .partial_cmp(&other.cards[oth_cmp_idx].rank_number())
                }
                // the rest: compare by highest-ranking kind;
                2 | 3 | 4 | 6 | 7 | 8 | 9 => self
                    .get_kind_ranks()
                    .into_iter()
                    .rev()
                    .partial_cmp(other.get_kind_ranks().into_iter().rev()),
                // last case should be impossible, just give a default.
                _ => Some(Ordering::Equal),
            },
        }
    }
}
