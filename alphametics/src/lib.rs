use itertools::Itertools;
use permutohedron::Heap;
use std::collections::HashMap;

struct Formula {
    equation_lhs: Vec<String>,
    equation_rhs: String,
    letters: Vec<char>,
    num_letters: usize,
    first_letters: Vec<char>,
    legend: HashMap<char, u32>,
}

impl Formula {
    pub fn new(text: &str) -> Self {
        let mut equation: Vec<String> =
            text.split("==").map(|str| str.trim().to_string()).collect();
        let equation_rhs = equation.pop().unwrap().to_string();
        let equation_lhs = equation[0]
            .split('+')
            .map(|str| str.trim().to_string())
            .collect::<Vec<_>>();

        let mut letters: Vec<char> = text.chars().filter(|chr| chr.is_alphabetic()).collect();
        let first_letters: Vec<char> = equation_lhs
            .iter()
            .map(|word| word.chars().nth(0).unwrap())
            .collect();
        letters.sort();
        letters.dedup();
        let num_letters = letters.len();
        let mut legend = HashMap::new();

        for &letter in &letters {
            legend.insert(letter, 0);
        }

        Formula {
            equation_lhs,
            equation_rhs,
            letters,
            num_letters,
            first_letters,
            legend,
        }
    }

    fn match_found(&self) -> bool {
        let lhs_result = self.evaluate_list(&self.equation_lhs);
        let rhs = self.evaluate(&self.equation_rhs);

        let solution_length = rhs.to_string().len();
        let expected_num_letters = self.equation_rhs.len();

        lhs_result == rhs && solution_length == expected_num_letters
    }

    fn update_legend(&mut self, digits: &[usize]) {
        self.letters
            .clone()
            .into_iter()
            .zip(digits.into_iter())
            .for_each(|(key, digit)| {
                self.legend.insert(key, *digit as u32);
            });
    }

    fn first_letters_contain_zero(&self) -> bool {
        self.first_letters
            .iter()
            .any(|letter| self.legend[letter] == 0)
    }

    fn evaluate(&self, text: &str) -> u64 {
        text.chars()
            .map(|chr| (*&self.legend[&chr]).to_string())
            .join("")
            .parse::<u64>()
            .unwrap()
    }

    fn evaluate_list(&self, variables: &[String]) -> u64 {
        variables
            .iter()
            .map(|variable| self.evaluate(variable))
            .sum()
    }

    fn find_match(&mut self) -> Option<HashMap<char, u8>> {
        for mut digit_set in (0..10).combinations(self.num_letters) {
            let heap = Heap::new(&mut digit_set);

            for permutation in heap {
                self.update_legend(&permutation);

                if self.first_letters_contain_zero() {
                    continue;
                } else if self.match_found() {
                    return Some(
                        self.letters
                            .clone()
                            .into_iter()
                            .zip(permutation.iter())
                            .map(|(letter, digit)| (letter, *digit as u8))
                            .collect(),
                    );
                }
            }
        }

        None
    }
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut formula = Formula::new(input);
    formula.find_match()
}
