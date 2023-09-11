pub fn merge_alternately(word1: String, word2: String) -> String {
    let word1_bytes = word1.as_bytes();
    let word2_bytes = word2.as_bytes();

    let w1_len = word1_bytes.len();
    let w2_len = word2_bytes.len();

    let mut result = vec!['a'; w1_len + w2_len];

    let mut curr = 0;
    let mut w1_curr = 0;
    let mut w2_curr = 0;

    for _i in 0..(w1_len + w2_len) {
        if w1_curr < w1_len {
            result[curr] = word1_bytes[w1_curr] as char;
            curr += 1;
            w1_curr += 1;
        }

        if w2_curr < w2_len {
            result[curr] = word2_bytes[w2_curr] as char;
            curr += 1;
            w2_curr += 1;
        }
    }
    String::from_iter(&result[0..w1_len + w2_len])
}

fn main() {
    println!("{}", merge_alternately("hi".to_owned(), "bye".to_owned()));
}
