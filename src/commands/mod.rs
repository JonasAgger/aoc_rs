use std::{io::Write, path::PathBuf};

use tracing::{debug, trace};

pub mod bench;
pub mod run;

pub struct InputFetcher {
    data_save_path: String,
    cookie: String,
}

impl InputFetcher {
    const BASE_ADDRESS: &'static str = "https://adventofcode.com";
    const COOKIE_FILE: &'static str = "personal.cookie";

    pub fn new(data_save_path: &str) -> Self {
        let mut fetcher = Self {
            data_save_path: data_save_path.into(),
            cookie: "".into(),
        };

        fetcher.fetch_cookie();

        fetcher
    }

    pub fn fetch(
        &mut self,
        day: u8,
        year: u16,
        test: bool,
        specific_input: &Option<String>,
    ) -> Vec<String> {
        let data_path = PathBuf::from(&self.data_save_path).join(match (test, specific_input) {
            (_, Some(input)) => format!("{}/{:02}_{}.txt", year, day, input),
            (true, None) => format!("{}/{:02}_test.txt", year, day),
            (false, None) => format!("{}/{:02}.txt", year, day),
        });

        debug!("Feting input from {:?}", data_path);
        match (
            std::fs::read_to_string(&data_path),
            specific_input.is_some(),
        ) {
            (Ok(input), _) => {
                trace!("Found file cahce. Fetching from file");
                input.lines().map(|s| s.trim().into()).collect()
            }
            (Err(_), false) => {
                trace!("Did not find file cahce. Fetching from source");
                let input = match test {
                    true => self.fetch_input_test(day, year),
                    false => self.fetch_input(day, year),
                };
                std::fs::create_dir_all(data_path.parent().unwrap())
                    .expect("expected to create all dirs");
                std::fs::write(&data_path, input.join("\n"))
                    .expect("Expected to be able to write inputs");

                input
            }
            (Err(_), true) => panic!(
                "Failed to find specific file: {}/{:02}_{}.txt",
                year,
                day,
                specific_input.clone().unwrap()
            ),
        }
    }

    fn fetch_input(&mut self, day: u8, year: u16) -> Vec<String> {
        let path = format!("/{year}/day/{day}/input");

        debug!("Fetching input from path: '{}'", path);
        let client = reqwest::blocking::Client::new();

        let resp = client
            .get(format!("{}{}", Self::BASE_ADDRESS, &path))
            .header("Cookie", format!("session={}", self.cookie))
            .send()
            .expect("Expected to perform HTTP call");

        resp.text()
            .expect("expected text body")
            .lines()
            .map(|s| s.into())
            .collect()
    }

    fn fetch_input_test(&mut self, day: u8, year: u16) -> Vec<String> {
        let path = format!("/{year}/day/{day}");
        debug!("Fetching test input from path: '{}'", path);

        let client = reqwest::blocking::Client::new();

        let resp = client
            .get(format!("{}{}", Self::BASE_ADDRESS, &path))
            .header("Cookie", format!("session={}", self.cookie))
            .send()
            .expect("Expected to perform HTTP call");

        let text = resp.text().expect("expected text body");

        let start = text.find("<pre><code>");
        let end = text.find("</code></pre>");

        match (start, end) {
            (Some(mut start), Some(end)) => {
                start += "<pre><code>".len();

                // Testing to check if we need to decode html formatted input. Might be needed in somce cases?
                return text[start..end]
                    .lines()
                    .map(|s: &str| html_escape::decode_html_entities(s).into_owned())
                    .collect();
                // return text[start..end].lines().map(|s| s.into()).collect();
            }
            _ => vec![],
        }
    }

    fn fetch_cookie(&mut self) {
        let cookie_path = PathBuf::from(&self.data_save_path).join(Self::COOKIE_FILE);

        match std::fs::read_to_string(&cookie_path) {
            Ok(cookie) => self.cookie = cookie,
            Err(_) => {
                println!(
                    "Could not find personal cookie or the previously entered cookie dident work."
                );
                println!(
                    " Please enter the cookie when searching for input on {}",
                    Self::BASE_ADDRESS
                );
                print!("cookie: ");
                std::io::stdout().lock().flush().unwrap();
                let stdin = std::io::stdin();
                use std::io::BufRead;
                let cookie = stdin.lock().lines().next().unwrap().unwrap();
                self.cookie = cookie;

                std::fs::create_dir_all(cookie_path.parent().unwrap())
                    .expect("expected to create cookie");
                std::fs::write(&cookie_path, &self.cookie)
                    .expect("Expected to be able to write cookie");
            }
        }
    }
}
