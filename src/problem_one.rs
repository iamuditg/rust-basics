
/*
Correct search result using word grouping
Description
- Given a list a words, group the words that are anagrams

- Tools
- Hashmaps, Nested loops
*/

use std::collections::HashMap;

pub fn search_anagram(words_list: Vec<String>) -> Vec<Vec<String>> {
    let mut word_hash = HashMap::new();
    let mut char_freq = vec![0;26];
    for current_word in words_list {
        for c in current_word.to_lowercase().chars() {
            char_freq[(c as u32 - 'a' as u32) as usize] += 1
        }
        let key = char_freq.into_iter().map(|i|i.to_string()).collect::<String>();
        word_hash.entry(key).or_insert(Vec::new()).push(current_word);
        char_freq = vec![0;26]
    }

    for (key, value) in &word_hash {
        println!("key = {:?} value {:?}", key , value)
    }

    word_hash.into_iter().map(|_,v| v).collect()
}