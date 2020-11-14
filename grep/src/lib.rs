use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Error;

/// Flags as a series of booleans.
#[derive(Debug)]
pub struct Flags {
    line_numbers: bool,
    names_only: bool,
    case_insens: bool,
    invert_match: bool,
    entire_lines: bool,
}

/// Simplistic implementation traversing the slice multiple times.
impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        Self {
            line_numbers: flags.contains(&"-n"),
            names_only: flags.contains(&"-l"),
            case_insens: flags.contains(&"-i"),
            invert_match: flags.contains(&"-v"),
            entire_lines: flags.contains(&"-x"),
        }
    }
}

/// Utter madness completely impossible to maintain.
pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    // The resulting vector to push to.
    let mut res = Vec::new();
    // Whether multiple files are considered or not; used in conditions.
    let multi_files = files.len() > 1;
    // Case sensitive-or-not version of the search pattern.
    let pattern = if flags.case_insens {
        pattern.to_lowercase()
    } else {
        pattern.to_string()
    };

    // Simple loops on file names and lines per file.
    for file_name in files.into_iter().map(|s| s.to_string()) {
        for (i, line) in BufReader::new(File::open(&file_name)?).lines().enumerate() {
            // Return early in case of an error.
            let line = line?;

            // The line used to compare with potential matches.
            let cmp_line = if flags.case_insens {
                line.to_lowercase()
            } else {
                line.clone()
            };

            // The number of matches in the current line.
            let matches = cmp_line
                .as_bytes()
                .windows(if flags.entire_lines {
                    cmp_line.len()
                } else {
                    pattern.len()
                })
                .filter(|&win| win == pattern.as_bytes())
                .count();

            // Yummy.
            if flags.invert_match ^ (matches != 0) {
                if flags.names_only && multi_files {
                    if !res.contains(&file_name) {
                        res.push(file_name.clone());
                    }
                } else {
                    res.push(
                        format!(
                            "{}{}{}",
                            if multi_files || flags.names_only {
                                file_name.clone() + ":"
                            } else {
                                String::new()
                            },
                            if flags.line_numbers
                                && !(flags.names_only && matches == 1 && !multi_files)
                            {
                                i.saturating_add(1).to_string() + ":"
                            } else {
                                String::new()
                            },
                            if flags.names_only && !multi_files {
                                ""
                            } else {
                                &line
                            }
                        )
                        .trim_end_matches(':')
                        .to_string(),
                    );
                }
            }
        }
    }

    Ok(res)
}
