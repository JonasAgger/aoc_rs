use anyhow::Result;
use slice_utils::Single;

use crate::utils::*;

use super::super::AocDay;

pub struct Day {}

impl Day {
    pub fn new() -> Self {
        Self {}
    }
}

impl AocDay for Day {
    fn run_part1(&mut self, input: &[String]) -> Result<AoCResult> {
        let polymers: Vec<_> = input
            .iter()
            .single()
            .as_bytes()
            .iter()
            .map(|&b| Polymer::new(b))
            .collect();

        let mut polymers = Polymers { inner: polymers };

        while polymers.run() {}

        Ok(polymers.inner.len().into())
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        let polymers: Vec<_> = input
            .iter()
            .single()
            .as_bytes()
            .iter()
            .map(|&b| Polymer::new(b))
            .collect();

        let polymers = Polymers { inner: polymers };

        let min_polys = (b'a'..b'z')
            .map(|unit| {
                let mut polymers = polymers.clone();
                polymers.inner.retain(|poly| poly.unit != unit);
                while polymers.run() {}

                polymers.inner.len()
            })
            .min()
            .unwrap();

        Ok(min_polys.into())
    }
}

#[derive(Clone)]
struct Polymers {
    inner: Vec<Polymer>,
}

impl Polymers {
    fn run(&mut self) -> bool {
        let len = self.inner.len() - 1;
        for i in 1..len {
            if self.inner[i].can_react(self.inner[i - 1]) {
                self.inner.remove(i);
                self.inner.remove(i - 1);
                return true;
            }

            if self.inner[i].can_react(self.inner[i + 1]) {
                self.inner.remove(i + 1);
                self.inner.remove(i);
                return true;
            }
        }
        false
    }
}

#[derive(Debug, Clone, Copy)]
struct Polymer {
    polarity: bool,
    unit: u8,
}

impl Polymer {
    fn new(item: u8) -> Self {
        Self {
            polarity: item.is_ascii_uppercase(),
            unit: item.to_ascii_lowercase(),
        }
    }

    fn can_react(&self, other: Polymer) -> bool {
        self.polarity != other.polarity && self.unit == other.unit
    }
}
