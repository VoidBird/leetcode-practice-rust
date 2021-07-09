use std::usize;


#[test]
fn test_find_median() {
    assert_eq!(find_median_sorted_arrays(vec![1, 3], vec![2]), 2_f64);
    assert_eq!(find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5_f64);
assert_eq!(find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5_f64);
    assert_eq!(find_median_sorted_arrays(vec![], vec![1]), 1_f64);
    assert_eq!(find_median_sorted_arrays(vec![2], vec![]), 2_f64);
}


pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let length = nums1.len() + nums2.len();

    let (median_index, is_odd) = if length % 2 == 0 {
        (length / 2 -1 , false)
    } else {
        ((length + 1) / 2 - 1, true)
    };

    let mut now_index: usize = 0;
    let mut last_value = 0;
    let mut nums_1_index: usize = 0;
    let mut nums_2_index: usize = 0;

    loop {
        let now_value  =  match (nums1.get(nums_1_index), nums2.get(nums_2_index)) {
            (Some(num1), Some(num2)) => {
                if num1 <= num2 {
                    nums_1_index += 1;
                    *num1
                } else {
                    nums_2_index += 1;
                    *num2
                }
            },
            (Some(num1), None) => {
                nums_1_index += 1;
                *num1
            },
            (None, Some(num2)) => {
                nums_2_index += 1;
                *num2
            },
            (None, None) => {
                return last_value as f64;
            }
        };

        if now_index >= median_index + 1 {
            return if is_odd {
                last_value as f64
            } else {
                (last_value + now_value) as f64 / 2_f64
            };
        }

        last_value = now_value;
        now_index += 1
    }
}
