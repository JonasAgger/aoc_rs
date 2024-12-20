use std::fmt::Debug;

pub fn split_chunk_empty(input: &[String]) -> Vec<Vec<String>> {
    let mut splits = vec![];

    let mut start = 0;
    for i in 0..input.len() {
        if input[i].is_empty() {
            splits.push(input[start..i].to_vec());
            start = i + 1;
        }
    }

    splits.push(input[start..].to_vec());

    splits
}

pub trait GrpBy<T> {
    fn group_by<A: Fn(&T, &T) -> bool>(self, cmp: A) -> impl Iterator<Item = Vec<T>>;
}

impl<T> GrpBy<T> for Vec<T> {
    fn group_by<A: Fn(&T, &T) -> bool>(self, cmp: A) -> impl Iterator<Item = Vec<T>> {
        let mut vecs = vec![];

        for item in self {
            let curr = {
                if vecs.is_empty() {
                    vecs.push(vec![item]);
                    continue;
                } else {
                    let len = vecs.len() - 1;
                    &mut vecs[len]
                }
            };

            if cmp(&curr[0], &item) {
                curr.push(item)
            } else {
                vecs.push(vec![item])
            }
        }

        vecs.into_iter()
    }
}

pub trait Single<T> {
    fn single(self) -> T;
}

impl<U: Iterator<Item = T>, T: Debug> Single<T> for U {
    fn single(mut self) -> T {
        let data = self.next().unwrap();
        let next = self.next();
        if next.is_none() {
            data
        } else {
            let mut data = vec![data, next.unwrap()];
            data.extend(self);
            panic!("sequece did not contain a single element: {:?}", data);
        }
    }
}

pub trait MinMax<T> {
    fn min_max(self) -> (T, T);
}

impl<U: Iterator<Item = T>, T: Ord + Clone> MinMax<T> for U {
    fn min_max(mut self) -> (T, T) {
        let first = self.next().unwrap();
        let mut min = first.clone();
        let mut max = first.clone();

        while let Some(next) = self.next() {
            min = min.min(next.clone());
            max = max.max(next.clone());
        }

        (min, max)
    }
}
