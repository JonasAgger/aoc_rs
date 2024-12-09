use anyhow::Result;

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
        let data = generate(&input[0]);
        let data = sort(data);

        let sum: usize = data
            .into_iter()
            .take_while(Option::is_some)
            .enumerate()
            .map(|(idx, val)| idx * val.unwrap_or_default())
            .sum();

        Ok(sum.into())
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        let (data, empties) = generate2(&input[0]);
        let data = sort2(data, empties);

        let sum: usize = data
            .into_iter()
            .enumerate()
            .map(|(idx, val)| idx * val.unwrap_or_default())
            .sum();

        Ok(sum.into())
    }
}

fn get_len(s: &str) -> usize {
    s.as_bytes().iter().map(|&s| (s - b'0') as usize).sum()
}

fn generate(s: &str) -> Vec<Option<usize>> {
    let len = get_len(s);
    let mut data = Vec::with_capacity(len);

    for (idx, len) in s
        .as_bytes()
        .iter()
        .map(|&s| (s - b'0') as usize)
        .enumerate()
    {
        let val = if idx % 2 == 0 {
            Some(idx.checked_div(2).unwrap_or_default())
        } else {
            None
        };

        for _ in 0..len {
            data.push(val);
        }
    }

    data
}

fn sort(mut data: Vec<Option<usize>>) -> Vec<Option<usize>> {
    let mut write = data.iter().position(Option::is_none).unwrap();
    let mut read = data.len() - data.iter().rev().position(Option::is_some).unwrap() - 1;

    while read > write {
        data.swap(write, read);

        let new_write = data[write..].iter().position(Option::is_none).unwrap() + write;
        let new_read = read - data[..read].iter().rev().position(Option::is_some).unwrap() - 1;

        write = new_write;
        read = new_read;
    }

    data
}

struct Empty {
    len: usize,
    idx: usize,
}

impl Empty {
    fn consume(&mut self, size: usize) {
        self.len -= size;
        self.idx += size;
    }

    fn can_fit(&self, size: usize) -> bool {
        self.len >= size
    }
}

fn generate2(s: &str) -> (Vec<Option<usize>>, Vec<Empty>) {
    let len = get_len(s);
    let mut data = Vec::with_capacity(len);
    let mut empties = vec![];

    for (idx, len) in s
        .as_bytes()
        .iter()
        .map(|&s| (s - b'0') as usize)
        .enumerate()
    {
        let val = if idx % 2 == 0 {
            Some(idx.checked_div(2).unwrap_or_default())
        } else {
            empties.push(Empty {
                len,
                idx: data.len(),
            });
            None
        };

        for _ in 0..len {
            data.push(val);
        }
    }

    (data, empties)
}

fn sort2(mut data: Vec<Option<usize>>, mut empties: Vec<Empty>) -> Vec<Option<usize>> {
    let mut read = data.len() - data.iter().rev().position(Option::is_some).unwrap() - 1;
    // keep running until no more empty space to the left
    while empties.first().is_some_and(|element| element.idx < read) {
        let curr = data[read];
        let size_of_current = data[..read]
            .iter()
            .rev()
            .take_while(|&&x| x == curr)
            .count()
            + 1;

        if let Some(idx) = empties
            .iter()
            .position(|x| x.can_fit(size_of_current) && x.idx < read)
        {
            let write_idx = empties[idx].idx;
            empties[idx].consume(size_of_current);

            for i in 0..size_of_current {
                data.swap(write_idx + i, read - i);
            }
        }

        // move read pointer to next number
        read -= size_of_current;
        while data[read].is_none() {
            read -= 1;
        }
    }

    data
}

/*
Continuing the first example, the first few blocks' position multiplied by its file ID number
are 0 * 0 = 0, 1 * 0 = 0, 2 * 9 = 18, 3 * 9 = 27, 4 * 8 = 32,
and so on. In this example, the checksum is the sum of these, 1928
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_len() {
        assert_eq!(get_len("2333133121414131402"), 42);
    }

    #[test]
    fn can_gen() {
        use std::fmt::Write;
        let arr = generate("2333133121414131402");

        let mut s = String::new();

        for x in arr {
            match x {
                Some(d) => write!(&mut s, "{}", d),
                None => write!(&mut s, "."),
            }
            .unwrap();
        }
        assert_eq!(s.as_str(), "00...111...2...333.44.5555.6666.777.888899");
    }

    #[test]
    fn can_sort() {
        use std::fmt::Write;
        let arr = generate("2333133121414131402");

        //TODO sort

        let arr = sort(arr);

        let mut s = String::new();

        for x in arr {
            match x {
                Some(d) => write!(&mut s, "{}", d),
                None => write!(&mut s, "."),
            }
            .unwrap();
        }
        assert_eq!(s.as_str(), "0099811188827773336446555566..............");
    }

    #[test]
    fn can_sort2() {
        use std::fmt::Write;
        let (arr, empty) = generate2("2333133121414131402");

        //TODO sort

        let arr = sort2(arr, empty);

        let mut s = String::new();

        for x in arr {
            match x {
                Some(d) => write!(&mut s, "{}", d),
                None => write!(&mut s, "."),
            }
            .unwrap();
        }
        assert_eq!(s.as_str(), "00992111777.44.333....5555.6666.....8888..");
    }

    #[test]
    fn can_checksum() {
        let arr = generate("2333133121414131402");

        let arr = sort(arr);
        //todo sort, checksum
        let sum: usize = arr
            .into_iter()
            .take_while(Option::is_some)
            .enumerate()
            .map(|(idx, val)| idx * val.unwrap_or_default())
            .sum();
        assert_eq!(sum, 1928)
    }

    #[test]
    fn can_checksum2() {
        let arr: Vec<_> = "00992111777.44.333....5555.6666.....8888.."
            .as_bytes()
            .iter()
            .map(|&x| {
                if x == b'.' {
                    None
                } else {
                    Some((x - b'0') as usize)
                }
            })
            .collect();
        //todo sort, checksum
        let sum: usize = arr
            .into_iter()
            .enumerate()
            .map(|(idx, val)| idx * val.unwrap_or_default())
            .sum();
        assert_eq!(sum, 2858)
    }
}
