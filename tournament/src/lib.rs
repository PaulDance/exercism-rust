use std::cmp::Ordering;
use std::collections::HashMap;

/// Represents a score entry for a team.
#[derive(Default)]
struct TeamEntry {
    pub matches_played: u32,
    pub matches_won: u32,
    pub matches_drawn: u32,
    pub matches_lost: u32,
    pub points: u32,
}

impl TeamEntry {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

/// Extracts a map associating a team's name to its score entry from the given string.
fn team_entries(match_results: &str) -> HashMap<&str, TeamEntry> {
    let mut teams = HashMap::new();

    for line in match_results.lines() {
        let mut iter = line.split(';');
        let t1_name = iter.next().unwrap();
        let t2_name = iter.next().unwrap();
        let result = iter.next().unwrap();

        let mut t1_entry = match teams.remove(t1_name) {
            Some(e) => e,
            None => TeamEntry::new(),
        };

        let mut t2_entry = match teams.remove(t2_name) {
            Some(e) => e,
            None => TeamEntry::new(),
        };

        t1_entry.matches_played += 1;
        t2_entry.matches_played += 1;

        match result {
            "win" => {
                t1_entry.matches_won += 1;
                t1_entry.points += 3;
                t2_entry.matches_lost += 1;
            }
            "draw" => {
                t1_entry.matches_drawn += 1;
                t2_entry.matches_drawn += 1;
                t1_entry.points += 1;
                t2_entry.points += 1;
            }
            "loss" => {
                t2_entry.matches_won += 1;
                t2_entry.points += 3;
                t1_entry.matches_lost += 1;
            }
            _ => panic!("Unknown result: '{}'.", result),
        }

        teams.insert(t1_name, t1_entry);
        teams.insert(t2_name, t2_entry);
    }

    teams
}

/// Transforms the given team map into the desired presentation table as a string.
fn present(teams: HashMap<&str, TeamEntry>) -> String {
    let mut result = "Team                           | MP |  W |  D |  L |  P".to_string();
    let mut teams = teams.into_iter().collect::<Vec<(&str, TeamEntry)>>();
    teams
        .as_mut_slice()
        .sort_unstable_by(|(t1_name, t1_entry), (t2_name, t2_entry)| {
            if t1_entry.points < t2_entry.points
                || t1_entry.points == t2_entry.points && t1_name > t2_name
            {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });

    for (name, entry) in teams.iter() {
        result.push_str(&format!(
            "\n{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            name,
            entry.matches_played,
            entry.matches_won,
            entry.matches_drawn,
            entry.matches_lost,
            entry.points
        ));
    }

    result
}

/// The desired function.
pub fn tally(match_results: &str) -> String {
    present(team_entries(match_results))
}
