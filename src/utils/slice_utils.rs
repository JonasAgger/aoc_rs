

pub fn split_chunk_empty(inputs: &[String]) -> Vec<Vec<String>> {
    let mut splits = vec![];

    let mut start = 0;
    for i in 0..inputs.len() {
        if inputs[i].len() == 0 {
            splits.push(inputs[start..i].to_vec());
            start = i + 1;
        }
    }

    splits.push(inputs[start..].to_vec());

    splits
}