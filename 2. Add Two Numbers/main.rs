use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut complement_map: HashMap<i32, i32> = HashMap::new();

        for (index, num) in nums.iter().enumerate() {
            let compliment = target - num;

            if let Some(i) = complement_map.get(&compliment) {
                return vec![i.to_owned(), index as i32];
            } else {
                complement_map.insert(num.clone(), index as i32);
            }
        };

        vec![]
    }
}

