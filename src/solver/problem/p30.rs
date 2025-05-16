pub struct P30;

impl P30 {
    pub fn solve_vx(s: String, words: Vec<String>)->Vec<i32> {
        // store all posibilities of word combination in vectors
        // do haystack algorithm to every combination with s and append the results seperate vectors
        vec![0,0]
    }

    pub fn solve_v1(s: String, words: Vec<String>) -> Vec<i32> {
        use std::collections::HashMap;
        let word_len = words[0].len();
        let total = s.len();
        let word_count = words.len();
        let mut result = Vec::new();
        if total == 0 || word_count == 0 || total < word_count*word_len {
            return result;
        }
        let mut word_freq = HashMap::new();
        for w in &words {
            *word_freq.entry(w.to_owned()).or_insert(0) +=1;
        }

        for i in 0..word_len {
            let mut word_seen = HashMap::new();
            let mut j = i;
            let mut count = 0;
            while j <= total - word_len{
                let cur = &s[j..(j + word_len)].to_owned();
                if word_freq.contains_key(cur) {
                    *word_seen.entry(cur.to_owned()).or_insert(0) +=1;
                    count += 1;

                    while  *word_seen.get(cur).unwrap() > *word_freq.get(cur).unwrap() {
                        let start = j - (count -1) * word_len;
                        let left_most = &s[start..start+word_len].to_owned();
                        *word_seen.get_mut(left_most).unwrap() -=1;
                        count -= 1;
                    }
                    if count == word_count {
                        result.push((j- (count - 1) * word_len) as i32);
                    }
                } else {
                    word_seen.clear();
                    count = 0
                }
                j += word_len
            }
        }
        result
    }
    pub fn solve_v2(s: String, words: Vec<String>) -> Vec<i32> {
        use std::collections::{HashMap, VecDeque};
        let word_len = words[0].len();
        let substring_length = words.len() * word_len;
        if s.len() < substring_length {
            return vec![];
        }
        let mut word2index = HashMap::with_capacity(words.len());
        let mut goal_stats = Vec::with_capacity(words.len());
        for word in &words {
            if let Some(&index) = word2index.get(word.as_str()) {
                goal_stats[index] += 1;
            } else {
                let index = word2index.len();
                word2index.insert(word.as_str(), index);
                goal_stats.push(1);
            }
        }
        let mut checked = vec![false; s.len()];
        let mut result = Vec::with_capacity(s.len() - substring_length + 1);
        for i in 0..=s.len() - substring_length {
            if checked[i] {
                continue;
            }
            let mut current_stats = vec![0; words.len()];
            let mut index_queue = VecDeque::with_capacity(words.len());
            let mut right = i;
            while right + word_len <= s.len() {
                checked[right] = true;
                if let Some(&right_word_index) = word2index.get(&s[right..right + word_len]) {
                    index_queue.push_back(right_word_index);
                    let stats = current_stats.get_mut(right_word_index).unwrap();
                    *stats += 1;
                    if *stats > goal_stats[right_word_index] {
                        while let Some(left_word_index) = index_queue.pop_front() {
                            current_stats[left_word_index] -= 1;
                            if left_word_index == right_word_index {
                                break;
                            }
                        }
                    }
                    if index_queue.len() == words.len() {
                        result.push((right + word_len - substring_length) as i32);
                    }
                    right += word_len;
                } else {
                    break;
                }
            }
        }
        result
    }
}