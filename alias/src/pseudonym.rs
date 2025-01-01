use rand::prelude::IndexedRandom;
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;

const PSEUDONYM_FILE_NAME: &str = "pseudonym.json";

#[derive(Deserialize, Debug)]
pub struct Pseudonym {
    pub last_names: Vec<String>,
    pub first_names: Vec<String>,
}

impl Pseudonym {
    pub fn generate(self) -> String {
        let first_name = self.first_names.choose(&mut rand::rng());
        let last_name = self.last_names.choose(&mut rand::rng());

        match (first_name, last_name) {
            (Some(first), Some(last)) => format!("{} {}", first, last),
            _ => "McLovin".to_string(),
        }
    }

    fn fetch_default() -> Pseudonym {
        // It's OK.
        let file = File::open(PSEUDONYM_FILE_NAME).expect("can't open file");
        let reader = BufReader::new(file);
        let names: Pseudonym = serde_json::from_reader(reader).expect("json parse error");

        names
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trying_fetch_default_names() {
        let names = Pseudonym::fetch_default();
        assert_ne!(names.first_names.len(), 0);
        assert_ne!(names.last_names.len(), 0);
    }

    #[test]
    fn generate_name_by_default() {
        let names = Pseudonym::fetch_default();
        assert_ne!(names.generate(), "McLovin");
    }

    #[test]
    fn generate_name_validity() {
        let names = Pseudonym {
            last_names: vec!["Linson".to_string()],
            first_names: vec!["Arnie".to_string()],
        };
        assert_eq!(names.generate(), "Arnie Linson");
    }

    #[test]
    fn generate_name_by_empty() {
        let names = Pseudonym {
            last_names: vec![],
            first_names: vec![],
        };
        assert_eq!(names.generate(), "McLovin");
    }
}
