/// Scale error for invalid input.
#[derive(Debug)]
pub enum Error {
    /// When the given tonic exists in neither `SHARPS` nor `FLATS`.
    InvalidTonic,
    /// When the given intervals contain an unrecognized character.
    InvalidInterval,
}

/// The two variants.
const CHROMATIC_SCALES: [[&str; 12]; 2] = [
    [
        "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
    ],
    [
        "A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab",
    ],
];

/// All sharps seperately, majors and then minors, expect for the first two.
const SHARPS: &[&str] = &[
    "C", "a", "G", "D", "A", "E", "B", "F#", "e", "b", "f#", "c#", "g#", "d#",
];

/// All flats seperately, majors and then minors.
const FLATS: &[&str] = &[
    "F", "Bb", "Eb", "Ab", "Db", "Gb", "d", "g", "c", "f", "bb", "eb",
];

/// The desired type.
pub struct Scale {
    /// Pitches as a vector directly.
    pitches: Vec<String>,
}

impl Scale {
    /// Returns the index of the chromatic scale containing the given `tonic`
    /// in the `[SHARPS, FLATS]` couple, otherwise returns an error.
    fn find_scale(tonic: &str) -> Result<usize, Error> {
        [SHARPS, FLATS]
            .iter()
            .enumerate()
            .find(|(_, arr)| arr.contains(&tonic))
            .map(|(i, _)| i)
            .ok_or(Error::InvalidTonic)
    }

    /// Returns the index of the given `tonic` in the given `scale`.
    fn find_tonic(tonic: &str, scale: usize) -> usize {
        // Case insensitive search.
        let tonic = tonic.to_uppercase();
        CHROMATIC_SCALES[scale]
            .iter()
            .enumerate()
            .find(|&(_, note)| note.to_uppercase() == tonic)
            .map(|(i, _)| i)
            .unwrap()
    }

    /// Builds a new scale from the given `tonic` and `intervals`, else an error.
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        let scale = Self::find_scale(tonic)?;
        let mut index = Self::find_tonic(tonic, scale);
        let mut pitches = Vec::new();

        for interval in intervals.chars() {
            pitches.push(CHROMATIC_SCALES[scale][index].to_string());
            index = (index
                + match interval {
                    'm' => Ok(1),
                    'M' => Ok(2),
                    'A' => Ok(3),
                    _ => Err(Error::InvalidInterval),
                }?)
                % 12;
        }

        Ok(Scale { pitches })
    }

    /// Returns the chromatic scale for the given `tonic`.
    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        // Re-use `new` with this little hack.
        Self::new(tonic, "mmmmmmmmmmmm")
    }

    /// Returns the enumeration of pitches comprising the scale.
    pub fn enumerate(&self) -> Vec<String> {
        self.pitches.clone()
    }
}
