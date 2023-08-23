/// Rail fence en/decrypter.
pub struct RailFence {
    /// The symmetric key: the number of rails.
    rails: usize,
}

/// Implementation featuring linear execution in time and space with respect to
/// the input's number of wide characters and with a minimal amount of
/// contiguous memory allocations.
impl RailFence {
    /// Builds a new en/decrypter using the given key.
    pub fn new(rails: u32) -> Self {
        Self {
            rails: rails as usize,
        }
    }

    /// Encodes the given clear`text` into a ciphertext.
    pub fn encode(&self, text: &str) -> String {
        // Convert to vec early to enable index access to wide chars.
        let chars = text.chars().collect::<Vec<_>>();
        // Length is preserved.
        let mut res = String::with_capacity(text.len());

        // Push characters in fence order.
        self.run(chars.len(), |j| res.push(chars[j]));
        res
    }

    /// Decodes the given `cipher`text into a cleartext.
    pub fn decode(&self, cipher: &str) -> String {
        // Convert to vec early to enable index access to wide chars.
        let chars = cipher.chars().collect::<Vec<_>>();
        // Initialize all characters to enable index access.
        let mut res = vec!['\0'; chars.len()];
        // The index in the ciphertext.
        let mut cfr_j = 0;

        // Set characters in cipher order to fence order.
        self.run(chars.len(), |res_j| {
            res[res_j] = chars[cfr_j];
            cfr_j += 1;
        });
        res.into_iter().collect()
    }

    /// Runs the given updage function along the rail path, by lines and valid
    /// columns.
    ///
    /// Parameters:
    ///  * `len`: the length of the text in Unicode code points.
    ///  * `upd_fn`: the function to run at each column iteration, receiving
    ///    the column index as its only argument.
    fn run(&self, len: usize, mut upd_fn: impl FnMut(usize)) {
        // The index difference between the current fence character and the
        // next one on the same line, for even inner loop iterations.
        let mut fst_jmp = 2 * (self.rails - 1);
        // Same, but for odd ones.
        let mut snd_jmp = 0;

        for i in 0..self.rails {
            // Indicates whether the following loop is on an even or odd
            // iteration. Makes it possible to handle jump asymmetry.
            let mut is_fst_jmp = true;
            let mut j = i;

            while j < len {
                upd_fn(j);

                // Update column index by performing the correct jump, which
                // includes avoiding a zero-jump occuring at extremities.
                j += if is_fst_jmp && fst_jmp != 0 || snd_jmp == 0 {
                    fst_jmp
                } else {
                    snd_jmp
                };

                is_fst_jmp = !is_fst_jmp;
            }

            // Don't update during the last iteration to avoid underflow.
            if i < self.rails - 1 {
                fst_jmp -= 2;
                snd_jmp += 2;
            }
        }
    }
}
