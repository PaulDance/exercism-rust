#[derive(Debug, PartialEq)]
pub struct Dna {
    seq: String,
}

#[derive(Debug, PartialEq)]
pub struct Rna {
    seq: String,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        Ok(Self {
            seq: Self::validate_sequence(dna)?,
        })
    }

    pub fn into_rna(self) -> Rna {
        Rna::new(&self.seq.chars().map(Self::nuc_to_rna).collect::<String>()).unwrap()
    }

    pub fn nuc_to_rna(nucleotide: char) -> char {
        match nucleotide {
            'G' => 'C',
            'C' => 'G',
            'T' => 'A',
            'A' => 'U',
            _ => nucleotide,
        }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        Ok(Self {
            seq: Self::validate_sequence(rna)?,
        })
    }
}

pub trait ValidateGenetics {
    const NUCLEOTIDES: &'static str;

    fn is_valid_nuc(nucleotide: char) -> bool {
        Self::NUCLEOTIDES.contains(nucleotide)
    }

    fn is_invalid_nuc(nucleotide: char) -> bool {
        !Self::is_valid_nuc(nucleotide)
    }

    fn validate_sequence(seq: &str) -> Result<String, usize> {
        match seq.chars().enumerate().find_map(|(i, chr)| {
            if Self::is_invalid_nuc(chr) {
                Some(i)
            } else {
                None
            }
        }) {
            Some(i) => Err(i),
            None => Ok(seq.to_string()),
        }
    }
}

impl ValidateGenetics for Dna {
    const NUCLEOTIDES: &'static str = "ATCG";
}

impl ValidateGenetics for Rna {
    const NUCLEOTIDES: &'static str = "AUCG";
}
