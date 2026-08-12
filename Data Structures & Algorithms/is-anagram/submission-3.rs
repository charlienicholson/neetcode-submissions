impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        use std::collections::HashMap;
        if s.len() != t.len() {
            return false;
        }
        //at this point guaranteed s.len == t.len
        let mut s_occurance_map = HashMap::new();
        let mut t_occurance_map = HashMap::new();
        for (index, s_ch) in s.chars().enumerate() {
            s_occurance_map.entry(s_ch).and_modify(|v| *v += 1).or_insert(1);
            let mut t_ch = t.chars().nth(index).unwrap();
            t_occurance_map.entry(t_ch).and_modify(|v| *v += 1).or_insert(1);
        }
        if s_occurance_map == t_occurance_map{
            return true;
        }
        return false;
        
    }
}
