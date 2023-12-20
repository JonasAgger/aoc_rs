use std::fmt::Display;

use anyhow::Result;
use tracing::debug;

use crate::utils::*;

use super::super::AocDay;

#[derive(Debug, Clone)]
struct Lens(String, char, usize);

impl Lens {
    fn new(s: &str) -> Self {
        if let Some((p1, p2)) = s.split_once('=') {
            return Self(p1.to_string(), '=', p2.parse().unwrap())
        }

        if let Some((p1, p2)) = s.split_once('-') {
            return Self(p1.to_string(), '-', 0)
        }    
        unreachable!()
    }
}

#[derive(Debug, Clone)]
struct Box {
    items: Vec<Lens>
}

impl Display for Box {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for item in self.items.iter() {
            write!(f, "{}{}{},", item.0, item.1, item.2)?;
        }
        write!(f, "]")
    }
}

impl Box {
    fn new() -> Self {
        Self { items: vec![] }
    }

    fn add(&mut self, lens: Lens) {
        for item in self.items.iter_mut() {
            if item.0 == lens.0 {
                *item = lens;
                return;
            }
        }

        self.items.push(lens);
    }

    fn remove(&mut self, lens: Lens) {
        let res = self.items.iter().enumerate().find(|(_, item)| item.0 == lens.0);

        if let Some((index, _)) = res {
            self.items.remove(index);
        }
    }

    fn any(&self) -> bool {
        self.items.len() > 0
    }

    fn value(&self) -> usize {
        self.items
            .iter()
            .enumerate()
            .map(|(index, item)| (index + 1) * item.2)
            .sum()
    }
}

pub struct Day {}

impl Day {
    pub fn new() -> Self {
        Self {}
    }
}

impl AocDay for Day {
    fn run_part1(&mut self, input: &[String]) -> Result<AoCResult> {

        let inputs = input[0].split(',');

        let mut hashed_value = 0usize;

        for input in inputs {
            hashed_value += hash(input) as usize;
        }

        Ok(hashed_value.into())
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {

        let inputs = input[0].split(',');

        let mut boxes = vec![Box::new(); 256];

        for input in inputs {
            let label = input.split_once(&['-', '=']).unwrap().0;
            let box_index = hash(label) as usize;

            let lens = Lens::new(input);

            match lens.1 {
                '=' => boxes[box_index].add(lens),
                '-' => boxes[box_index].remove(lens),
                _ => unreachable!()
            };
        }

        let mut total_value = 0;
        for (index, hash_box) in boxes.iter().enumerate().filter(|b| b.1.any()) {
            let box_value = (index + 1) * hash_box.value();
            total_value += box_value;
            debug!("Box {} = {} has value = {}", index, hash_box, box_value   );
        }

        Ok(total_value.into())
    }
}

fn hash(input: &str) -> u8 {
    let mut hash = 0u8;
    
    for b in input.as_bytes() {
        hash = hash.overflowing_add(*b).0;
        hash = hash.overflowing_mul(17).0;
    }

    hash
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hash() {
        let input = "HASH";

        let hash_value = hash(input);
        assert_eq!(hash_value, 52)
    }

    #[test]
    fn test_hash2() {
        let input = "cm";

        let hash_value = hash(input);
        assert_eq!(hash_value, 0)
    }

    #[test]
    fn test_hash3() {
        let input = "rn";

        let hash_value = hash(input);
        assert_eq!(hash_value, 0)
    }
}