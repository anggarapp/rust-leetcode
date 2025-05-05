use std::cmp::min;

pub struct P14;

impl P14 {
    pub fn solve_v1(strings: Vec<String>) -> String {
        let mut string_vecs = strings;
        string_vecs.sort();

        let first_string = &string_vecs[0];
        let last_string = &string_vecs[string_vecs.len() - 1];
        let min_length = min(first_string.len(), last_string.len());

        let mut iteration = 0;

        while iteration < min_length
            && first_string.as_bytes()[iteration] == last_string.as_bytes()[iteration]
        {
            iteration += 1;
        }

        // dbg!(&first_string.as_bytes()[..iteration]);
        String::from_utf8(first_string.as_bytes()[..iteration].to_vec()).unwrap()
    }
}
