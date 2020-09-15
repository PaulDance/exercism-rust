use std::convert::{TryFrom, TryInto};

pub struct Allergies {
    allergens: Vec<Allergen>,
}

#[repr(u32)]
#[derive(Debug, PartialEq, Clone)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

/// `TryFrom<u32>` implementation for `Allergen` where integers are mapped to allergen scores.
///
/// The errors do not contain information, only the unit type is used.
impl TryFrom<u32> for Allergen {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Allergen::Eggs),
            2 => Ok(Allergen::Peanuts),
            4 => Ok(Allergen::Shellfish),
            8 => Ok(Allergen::Strawberries),
            16 => Ok(Allergen::Tomatoes),
            32 => Ok(Allergen::Chocolate),
            64 => Ok(Allergen::Pollen),
            128 => Ok(Allergen::Cats),
            _ => Err(()),
        }
    }
}

impl Allergies {
    /// Builds a new Allergy from the given `score` with allergens computed eagerly.
    pub fn new(score: u32) -> Self {
        let mut allergens = Vec::new();
        let mut rest = score;
        let mut pow = 1;

        // Obtaining the Allergens from the score is a base 2 decomposition.
        while pow <= 128 {
            if rest % 2 != 0 {
                allergens.push(pow.try_into().unwrap());
            }

            rest /= 2;
            pow *= 2;
        }

        Self { allergens }
    }

    /// Returns `true` iff the given `allergen` triggers the Allergy.
    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergens.contains(allergen)
    }

    /// Returns the various `Allergen`s triggering the Allergy.
    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergens.clone()
    }
}
