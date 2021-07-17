#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quit_sort() {
        let mut v = vec![3, 2, 1, 4];
        quick_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_next_permutation() {
        let mut v = vec![1, 2, 3];
        next_permutation(&mut v);
        assert_eq!(v, vec![1, 3, 2]);

        v = vec![3, 2, 1];
        next_permutation(&mut v);
        assert_eq!(v, vec![1, 2, 3]);
        
        v = vec![1, 1, 5];
        next_permutation(&mut v);
        assert_eq!(v, vec![1, 5, 1]);

        v = vec![1];
        next_permutation(&mut v);
        assert_eq!(v, vec![1]);

        v = vec![1, 3, 2];
        next_permutation(&mut v);
        assert_eq!(v, vec![2, 1, 3]);
    }
}


pub fn next_permutation(nums: &mut Vec<i32>) {
    if nums.len() < 2 {
        return;
    }

    /*
     * 从右开始往左尝试交换，只有当出现升序序列时进行交换才能实现进位效果
     * 当发现一个升序子序列时，子序列右边即为降序序列
    */
    for index in (1 .. nums.len()).rev() {
        if nums[index] > nums[index - 1] {
            for sub_index in (index..nums.len()).rev() {
                if nums[sub_index] > nums[index - 1] {
                    nums.swap(sub_index, index - 1);
                    break;
                }
            }
            nums[index..].sort();
            return;
        }
    }
    
    nums.sort();
}


fn quick_sort(nums: &mut Vec<i32>) {
    let mut pivot_index = 0;
    let mut right_most = nums.len();
    let mut store_index;
    while pivot_index < nums.len() - 1 {
        store_index = pivot_index + 1;
        for i in pivot_index..right_most {
            if nums[i] < nums[pivot_index] {
                nums[store_index] = nums[i];
                store_index += 1;
            }
        }
        nums.swap(pivot_index, store_index - 1);
        if pivot_index == store_index - 1 {
            pivot_index = store_index;
            right_most = nums.len();
        } else {
            right_most = store_index;
        }
    }
}