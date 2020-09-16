use std::collections::HashMap;

/// Stores the mapping between codons and their names.
pub struct CodonsInfo<'a> {
    map: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    /// Returns the name associated to the given `codon` if it present, otherwise `None`.
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.map.get(codon).map(|&name| name)
    }

    /// Returns a vector of protein names from the given `rna` strand, `None` if it is invalid.
    pub fn of_rna(&self, rna: &'a str) -> Option<Vec<&'a str>> {
        let mut res = Vec::new();

        // Iterators were meant to be used more at first, but it didn't succed.
        for i in (0..rna.len()).step_by(3) {
            if i + 3 > rna.len() {
                return None;
            } else {
                match self.name_for(&rna[i..i + 3]) {
                    None => {
                        return None;
                    }
                    Some(name) => {
                        if name == "stop codon" {
                            return Some(res);
                        } else {
                            res.push(name);
                        }
                    }
                }
            }
        }

        Some(res)
    }
}

/// Collects the given `pairs` of codons and names into a `CodonsInfo` mapping.
pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo {
        map: pairs.into_iter().collect(),
    }
}
