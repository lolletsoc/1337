use std::collections::HashMap;
use std::convert::TryFrom;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut numbers = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        numbers
            .entry(*num)
            .and_modify(|list: &mut Vec<usize>| list.push(i))
            .or_insert(vec![i]);
    }

    let vector = numbers
        .iter()
        .find_map(|key_indices| {
            let desired = target - *key_indices.0;
            match numbers.get(&desired) {
                Some(desired_indices) => {
                    let desired_index;
                    if desired == *key_indices.0 {
                        if desired_indices.len() > 1 {
                            desired_index = desired_indices[1];
                        } else {
                            return Option::None;
                        }
                    } else {
                        desired_index = desired_indices[0];
                    }

                    Some(vec![
                        i32::try_from(key_indices.1[0]).unwrap(),
                        i32::try_from(desired_index).unwrap(),
                    ])
                }
                None => Option::None,
            }
        })
        .unwrap();

    vector
}

fn main() {
    println!("{:?}", two_sum(Vec::from([1, 3, 4, 2]), 6));
}
