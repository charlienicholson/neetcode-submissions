impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        
        let mut transform_index = HashMap::new();
        for (iteration, number) in nums.iter().enumerate() {
            if transform_index.contains_key(number) {
                return vec![transform_index[number], iteration as i32];
            }
            let transformation_value = target - number;
            transform_index.insert(transformation_value, iteration as i32);
        }
        vec![]
    }
}
