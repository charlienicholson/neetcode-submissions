impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        use std::collections::HashMap;
        
        if s.len() != t.len() {
            return false;
        }
        
        let mut s_characters = HashMap::new();

        for log_character in s.chars() {
            s_characters.entry(log_character).and_modify(|v| *v += 1).or_insert(1);
        }
        for validate_character in t.chars() {
            if let std::collections::hash_map::Entry::Occupied(mut entry) = s_characters.entry(validate_character) {
                if *entry.get() == 1 {
                    entry.remove();
                } 
                else {
                    *entry.get_mut() -= 1;
                }
                continue;
            };
            
            return false;
        }
        return true;
    }
}
