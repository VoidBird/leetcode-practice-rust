use std::cmp::max;

#[cfg(test)]
mod tests {
    use super::max_area;
    #[test]
    fn test() {
        assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(max_area(vec![1, 1]), 1);
        assert_eq!(max_area(vec![4, 3, 2, 1, 4]), 16);
        assert_eq!(max_area(vec![1, 2, 1]), 2);
    }
}



pub fn max_area(height: Vec<i32>) -> i32 {
    let mut i: usize = 0;
    let mut j = height.len() - 1;
    let mut area = 0;

    while i != j {
        let h1 = &height[i];
        let h2 = &height[j];
        if h1 >= h2 {
            area = max(h2 * (j - i) as i32, area);
            j -= 1
        } else {
            area = max(h1 * (j - i) as i32, area);
            i += 1
        }
    }
    area
}