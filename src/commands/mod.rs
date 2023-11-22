use std::path::PathBuf;


pub mod run;
pub mod bench;


pub struct InputFetcher {
    data_save_path: String,
    cookie: String
}

impl InputFetcher {
    const BASE_ADDRESS: &'static str = "https://adventofcode.com";
    const COOKIE_FILE: &'static str = "personal.cookie";


    pub fn new(data_save_path: &str) -> Self {
        let mut fetcher = Self {
            data_save_path: data_save_path.into(),
            cookie: "".into()
        };

        fetcher.fetch_cookie();
        
        fetcher
    }


    pub fn fetch(&mut self, day: u8, year: u16, test: bool) -> Vec<String> {
        let data_path = PathBuf::from(&self.data_save_path)
            .join(match test {
                true => format!("{}/{:02}_test.txt", year, day),
                false => format!("{}/{:02}.txt", year, day)
            });
        
        match std::fs::read_to_string(&data_path) {
            Ok(input) => input.lines().map(|s| s.into()).collect(),
            Err(_) => {
                let input = match test {
                    true => self.fetch_input_test(day, year),
                    false => self.fetch_input(day, year),
                };
                std::fs::create_dir_all(&data_path.parent().unwrap()).expect("expected to create all dirs");
                std::fs::write(&data_path, &input.join("\n")).expect("Expected to be able to write inputs");

                input
            },
        }
    }


    fn fetch_input(&mut self, day: u8, year: u16) -> Vec<String> {
        let path = format!("/{year}/day/{day}/input");

        let client = reqwest::blocking::Client::new();
        
        let resp = client.get(format!("{}{}", Self::BASE_ADDRESS, &path))
            .header("Cookie", format!("session={}", self.cookie))
            .send()
            .expect("Expected to perform HTTP call");

        resp.text().expect("expected text body").lines().map(|s| s.into()).collect()
    }

    fn fetch_input_test(&mut self, day: u8, year: u16) -> Vec<String> {
        let path = format!("/{year}/day/{day}");

        let client = reqwest::blocking::Client::new();

        let resp = client.get(format!("{}{}", Self::BASE_ADDRESS, &path))
            .header("Cookie", format!("session={}", self.cookie))
            .send()
            .expect("Expected to perform HTTP call");

        let text = resp.text().expect("expected text body");

        let start = text.find("<pre><code>");
        let end = text.find("</code></pre>");

        if let Some(mut start) = start {
            if let Some(end) = end {

                start += "<pre><code>".len();

                return text[start..end].lines().map(|s| s.into()).collect();
            }
        } 

        vec![]
    }

    fn fetch_cookie(&mut self) {
        let cookie_path = PathBuf::from(&self.data_save_path).join(Self::COOKIE_FILE);

        match std::fs::read_to_string(&cookie_path) {
            Ok(cookie) => self.cookie = cookie,
            Err(_) => {
                println!("Could not find personal cookie or the previously entered cookie dident work.");
                println!(" Please enter the cookie when searching for input on {}", Self::BASE_ADDRESS);
                print!("cookie: ");
                let stdin = std::io::stdin();
                use std::io::BufRead;
                let cookie = stdin.lock().lines().next().unwrap().unwrap();
                self.cookie = cookie;

                std::fs::create_dir_all(&cookie_path.parent().unwrap()).expect("expected to create cookie");
                std::fs::write(&cookie_path, &self.cookie).expect("Expected to be able to write cookie");
            },
        }
    }
}