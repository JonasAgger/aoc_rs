use std::collections::HashMap;

use anyhow::Result;

use crate::utils::{self, *};

use super::super::AocDay;

pub struct Day {}

impl Day {
    pub fn new() -> Self {
        Self {}
    }
}

impl AocDay for Day {
    fn run_part1(&mut self, input: &[String]) -> Result<AoCResult> {
        let mut res = crate::utils::slice_utils::split_chunk_empty(input);

        let pages: Vec<Vec<usize>> = res
            .pop()
            .unwrap()
            .into_iter()
            .map(|s| s.split(',').map(|c| c.number()).collect())
            .collect();

        let mut page_ordering: HashMap<usize, Vec<usize>> = HashMap::new();

        for (before, number) in res
            .pop()
            .unwrap()
            .into_iter()
            .map(|s| utils::split_numbers_by(&s, '|'))
        {
            page_ordering.entry(number).or_default().push(before);
        }

        let mut sum = 0;

        for page in pages {
            if is_valid(&page, &page_ordering) {
                tracing::debug!("valid: {:?}", page);

                let middle = page[page.len() / 2];
                sum += middle;
            } else {
                tracing::debug!("NOT valid: {:?}", page);
            }
        }

        Ok(sum.into())
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        let mut res = crate::utils::slice_utils::split_chunk_empty(input);

        let pages: Vec<Vec<usize>> = res
            .pop()
            .unwrap()
            .into_iter()
            .map(|s| s.split(',').map(|c| c.number()).collect())
            .collect();

        let mut page_ordering: HashMap<usize, Vec<usize>> = HashMap::new();

        for (before, number) in res
            .pop()
            .unwrap()
            .into_iter()
            .map(|s| utils::split_numbers_by(&s, '|'))
        {
            page_ordering.entry(number).or_default().push(before);
        }

        let mut sum = 0;

        for page in pages {
            if is_valid(&page, &page_ordering) {
                continue;
            }
            let page = make_correct_ordering(page, &page_ordering);
            let middle = page[page.len() / 2];
            sum += middle;
        }

        Ok(sum.into())
    }
}

fn is_valid(page: &[usize], page_ordering: &HashMap<usize, Vec<usize>>) -> bool {
    for i in 0..page.len() {
        let current = page[i];

        if let Some(before) = page_ordering.get(&current) {
            for b in before {
                if !page[..i].contains(b) && page.contains(b) {
                    return false;
                }
            }
        }
    }

    return true;
}

fn make_correct_ordering(
    mut old_page: Vec<usize>,
    page_ordering: &HashMap<usize, Vec<usize>>,
) -> Vec<usize> {
    let mut new_page = vec![];

    loop {
        if old_page.len() == 0 {
            return new_page;
        }
        let next_in_line = replace(&mut old_page, page_ordering);
        new_page.push(next_in_line);
    }
}

fn replace(page: &mut Vec<usize>, page_ordering: &HashMap<usize, Vec<usize>>) -> usize {
    for i in 0..page.len() {
        if let Some(before) = page_ordering.get(&page[i]) {
            if !before.iter().any(|x| page.contains(x)) {
                return page.swap_remove(i);
            }
        } else {
            return page.swap_remove(i);
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_valid() {
        let page_ordering = vec![(75, vec![97])].into_iter().collect();

        let page = vec![75, 97];

        assert!(!is_valid(&page, &page_ordering))
    }

    #[test]
    fn check_valid_p2() {
        let page_ordering = vec![(13, vec![61, 29]), (29, vec![61])]
            .into_iter()
            .collect();
        // 61|13
        // 61|29
        // 29|13
        let page = vec![61, 13, 29];
        let page = make_correct_ordering(page, &page_ordering);
        dbg!(&page);
        assert!(!is_valid(&page, &page_ordering))
    }
}
