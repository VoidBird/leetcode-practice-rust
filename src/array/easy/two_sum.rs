use std::collections::HashMap;


pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut record:HashMap<i32, i32> = HashMap::new();
    for (index, num) in nums.iter().enumerate() {
        let other_num = target - num;

        match record.get(&other_num) {
            Some(other_index) => return vec![index as i32, *other_index as i32],
            _ => ()
        }

        record.insert(*num, index as i32);
    }
    vec![]
}