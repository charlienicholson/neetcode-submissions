impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        use std::collections::HashMap;
        let mut occurance_map = HashMap::new();
        //key is our number, value is number from nums

        for &number in nums.iter(){
            if !occurance_map.contains_key(&number) {
                occurance_map.insert(number, 1);
                continue;
            }
            return true;        
        }
        return false;


    }
}
