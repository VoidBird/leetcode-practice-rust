#[cfg(test)]
mod tests {
    use super::three_sum;

    #[test]
    fn test() {
        assert_eq!(three_sum(vec![-1, 0, 1, 2, -1, -4]), vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
        assert_eq!(three_sum(vec![]), Vec::<Vec::<i32>>::new());
        assert_eq!(three_sum(vec![0]), Vec::<Vec::<i32>>::new());
        assert_eq!(three_sum(vec![1, 1, -2]), vec![vec![-2, 1, 1]]);
    }
}


pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    if nums.len() < 3 {
        return vec![];
    }
    let mut result: Vec<Vec<i32>> = vec![];
    let mut nums = nums.clone();
    nums.sort();
    
    for first_index in 0..nums.len()-1 {
        if first_index > 0 && nums[first_index] == nums[first_index - 1] {
            continue;
        }

        if nums[first_index] > 0 {
            // 排序后当第一个数大于0,则后面的数都大于0,不可能存在更多的组合
            break;
        }

        let mut third_index = nums.len() - 1;

        for second_index in (first_index + 1)..nums.len()-1 {
            if second_index == third_index {
                break;
            }

            if second_index > first_index + 1 && nums[second_index] == nums[second_index - 1] {
                continue;
            }

            while second_index < third_index && 
                nums[second_index] + nums[third_index] > -nums[first_index] {
                    third_index -= 1;
            }

            if second_index == third_index {
                break;
            }

            if nums[second_index] + nums[third_index] == -nums[first_index] {
                result.push(vec![nums[first_index], nums[second_index], nums[third_index]]);
            }
        }
    }
    result
}