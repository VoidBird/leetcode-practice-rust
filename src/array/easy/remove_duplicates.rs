#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(remove_duplicates(&mut vec![1, 1, 2]), 2);
        assert_eq!(remove_duplicates(&mut vec![0,0,1,1,1,2,2,3,3,4]), 5);
    }
}


pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    nums.dedup();
    nums.len() as i32
}

pub fn remove_duplicates2(nums: &mut Vec<i32>) -> i32 {
    let mut index = 0;
    /*
     * 不使用nums.iter()是因为rust规定可变引用只能有一个，它可读可写，同时不能有其他可读引用。
     * 如果使用nums.iter()将会创建一个非可变引用，就会违反规则
     * 同样，使用nums.iter_mut()将会创建一个nums指向的Vec的一个可变引用，
     * 这与nums这个已存在的可变引用冲突
    */
    for i in 0..nums.len(){
        if nums[index] != nums[i] {
            index += 1;
            nums[index]=nums[i]
        }
    }
    (index + 1) as i32
}
