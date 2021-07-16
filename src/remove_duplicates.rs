#[test]
fn test() {
    assert_eq!(remove_duplicates(&mut vec![1, 1, 2]), 2);
    assert_eq!(remove_duplicates(&mut vec![0,0,1,1,1,2,2,3,3,4]), 5);
}

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    nums.dedup();
    nums.len() as i32
}
