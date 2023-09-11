use std::collections::HashMap;
use std::collections::VecDeque;

pub fn length_of_longest_substring(s: String) -> i32 {
    if s.len() == 0 {
        return 0;
    }

    let string_bytes = &s.as_bytes();

    let mut chars_seen: HashMap<u8, VecDeque<usize>> = HashMap::with_capacity(256);

    let mut longest = 1;
    let mut start = 0;
    let mut end = 0;

    longest.max(other)

    let bytes_length = string_bytes.len();

    loop {
        if end > string_bytes.len() - 1 {
            break;
        }

        let next_char = string_bytes[end];
        match chars_seen.get_mut(&next_char) {
            Some(seen_index_vec) => {
                match seen_index_vec.pop_front() {
                    Some(seen_index) => {
                        let new_index = seen_index + 1;
                        if new_index > start {
                            start = new_index;
                        }
                    }
                    None => {}
                }
                seen_index_vec.push_back(end);
                end += 1;
            }
            None => {
                let mut deq = VecDeque::new();
                deq.insert(0, end);
                chars_seen.insert(next_char, deq);
                end += 1;
            }
        }

        let length = end - start;
        if length > longest {
            longest = length;
        }
    }

    i32::try_from(longest).unwrap()
}

fn main() {
    println!("{}", length_of_longest_substring("abcabcbb".to_owned()));
}
