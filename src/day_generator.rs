use std::{io::Write, path::PathBuf};

use anyhow::Result;
use tracing::debug;

pub struct DayGenerator {
    file_path: String,
}

impl DayGenerator {
    pub fn new(file_path: String) -> Self {
        Self { file_path }
    }

    pub fn generate_day(&self, day: u8, year: u16) -> Result<()> {
        let year_path = PathBuf::from(&self.file_path).join(format!("year_{}", year));

        std::fs::create_dir_all(&year_path)?;

        let mod_year_path = year_path.join("mod.rs");

        let day_path = year_path.join(format!("day_{:02}.rs", day));

        let mut day_file = std::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(day_path)?;

        let year_mod_file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(mod_year_path)?;

        let mut task_mod_file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(PathBuf::from(&self.file_path).join("mod.rs"))?;

        writeln!(&year_mod_file, "pub mod day_{:02};", day)?;

        let mod_template = include_str!("./templates/mod_template.txt");
        let day_template = include_str!("./templates/day_template.txt");

        let mod_day_string = self.get_mod_string();
        let match_day_string = self.get_match_string();

        let composite_formatted = mod_template
            .replace("$MOD_DAYS$", &mod_day_string)
            .replace("$MATCH_DAYS$", &match_day_string);

        day_file.write_all(day_template.as_bytes())?;
        day_file.flush()?;

        task_mod_file.write_all(composite_formatted.as_bytes())?;
        task_mod_file.flush()?;
        Ok(())
    }

    pub fn get_current_day(&self, year: u16) -> Result<u8> {
        let mut next: u8 = 0;

        let year_path = PathBuf::from(&self.file_path).join(format!("year_{}", year));

        let dir_entry = match std::fs::read_dir(&year_path) {
            Ok(dir) => dir,
            Err(_) => return Ok(next),
        };

        for entry in dir_entry
            .into_iter()
            .filter_map(|p| p.ok())
            .filter(|p| !p.file_name().eq_ignore_ascii_case("mod.rs"))
        {
            // Should not be possible
            if entry.file_type().unwrap().is_dir() {
                continue;
            }

            let date: u8 =
                entry.file_name().to_str().unwrap().to_ascii_lowercase()[4..6].parse()?;

            if date > next {
                next = date;
            }
        }

        Ok(next)
    }

    fn get_mod_string(&self) -> String {
        use std::fmt::Write;
        let dir_entry = std::fs::read_dir(&self.file_path).expect("Expected a year entry here");

        let mut mod_string = String::new();

        for entry in dir_entry
            .into_iter()
            .filter(|p| p.as_ref().unwrap().file_type().unwrap().is_dir())
            .map(|p| p.unwrap())
        {
            let file_name = entry.file_name();
            let name = file_name.to_str().unwrap();
            writeln!(mod_string, "mod {};", name).expect("Expected to be able to format string")
        }

        mod_string
    }

    fn get_match_string(&self) -> String {
        use std::fmt::Write;
        let dir_entry = std::fs::read_dir(&self.file_path).expect("Expected a year entry here");

        let mut mod_string = String::new();

        for year_entry in dir_entry
            .into_iter()
            .filter(|p| p.as_ref().unwrap().file_type().unwrap().is_dir())
            .filter_map(|p| p.ok())
        {
            let year: u16 = year_entry
                .file_name()
                .to_str()
                .unwrap()
                .to_ascii_lowercase()[5..]
                .parse()
                .expect("Expected year to be parsable");
            debug!("year: {}", year);

            for day_entry_str in std::fs::read_dir(year_entry.path())
                .expect("Expected a year entry here")
                .filter_map(|p| p.ok())
                .filter(|p| !p.file_name().eq_ignore_ascii_case("mod.rs"))
            {
                let file_name = day_entry_str.file_name();
                debug!("file_name: {:?}", &file_name);
                let day: u8 = file_name.to_str().unwrap().to_ascii_lowercase()[4..6]
                    .parse()
                    .expect("expected to parse date");
                writeln!(
                    mod_string,
                    "({}, {}) => Ok(Box::new(year_{}::day_{:02}::Day::new())),",
                    day, year, year, day
                )
                .expect("Expected to be able to format string")
            }
        }

        mod_string
    }
}
