#[derive(Debug, PartialEq)]
pub struct DNA {
    seq: String,
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    seq: String,
}

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        Ok(Self {
            seq: Self::validate_sequence(dna)?,
        })
    }

    pub fn into_rna(self) -> RNA {
        RNA::new(&self.seq.chars().map(Self::nuc_to_rna).collect::<String>()).unwrap()
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

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
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

impl ValidateGenetics for DNA {
    const NUCLEOTIDES: &'static str = "ATCG";
}

impl ValidateGenetics for RNA {
    const NUCLEOTIDES: &'static str = "AUCG";
}
