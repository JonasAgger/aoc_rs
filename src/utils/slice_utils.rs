use std::fmt::Debug;

pub fn split_chunk_empty(inputs: &[String]) -> Vec<Vec<String>> {
    let mut splits = vec![];

    let mut start = 0;
    for i in 0..inputs.len() {
        if inputs[i].is_empty() {
            splits.push(inputs[start..i].to_vec());
            start = i + 1;
        }
    }

    splits.push(inputs[start..].to_vec());

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

        if self.next().is_none() {
            data
        } else {
            panic!("sequece did not contain a single element");
        }
    }
}
