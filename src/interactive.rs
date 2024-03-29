use chrono::Datelike;
use inquire::autocompletion::{Autocomplete, Replacement};
use inquire::error::CustomUserError;
use inquire::{DateSelect, Select, Text};

#[derive(Clone)]
struct HourCompleter {
    hours: Vec<String>,
}

impl HourCompleter {
    fn filter_candidates(&self, input: &str) -> Vec<String> {
        let pattern = input.to_lowercase();

        self.hours
            .clone()
            .into_iter()
            .filter(|s| s.starts_with(&pattern))
            .collect()
    }
}

impl Autocomplete for HourCompleter {
    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, CustomUserError> {
        Ok(self.filter_candidates(input))
    }

    fn get_completion(
        &mut self,
        input: &str,
        highlighted_suggestion: Option<String>,
    ) -> Result<Replacement, CustomUserError> {
        Ok(match highlighted_suggestion {
            Some(suggestion) => Replacement::Some(suggestion),
            None => {
                let list = self.filter_candidates(input);
                if list.is_empty() {
                    Replacement::None
                } else {
                    Replacement::Some(list[0].clone())
                }
            }
        })
    }
}

pub fn birthday_from_prompt() -> (i32, u32, u32, f64, bool) {
    let start = chrono::NaiveDate::from_ymd_opt(1990, 6, 1).unwrap();
    let chosen = DateSelect::new("Your birthday in gregorian calendar: ")
        .with_default(start)
        .prompt()
        .unwrap();

    let year = chosen.year();
    let month = chosen.month();
    let day = chosen.day();
    let mut hour: f64 = 10.0;

    let hc = HourCompleter {
        hours: (1..25).map(|i| i.to_string()).collect(),
    };
    let resp = Text::new("Choose the hour: ")
        .with_autocomplete(hc)
        .prompt();

    match resp {
        Ok(h) => hour = h.parse::<f64>().unwrap(),
        Err(err) => println!("Error retrieving your response: {}", err),
    }

    let mut is_male = false;
    let options: Vec<&str> = vec!["男", "女"];
    let ans = Select::new("The gender ?", options).prompt();
    match ans {
        Ok(g) => is_male = g == "男",
        Err(err) => println!("Error retrieving gender: {}", err),
    }

    println!("公曆生日: {}年{}月{}日{}時", year, month, day, hour);
    (year, month, day, hour, is_male)
}
