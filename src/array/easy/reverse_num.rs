#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(reverse(123), 321);
        assert_eq!(reverse(1534236469), 0);
    }
}



pub fn reverse(x: i32) -> i32 {
    let mut positive = false;
    let mut num = if x < 0 {
        x.abs()
    } else {
        positive = true;
        x
    };

    let mut ret: Vec<i32> = Vec::new();
    while num != 0 {
        ret.push(num % 10);
        num /= 10;
    }
    let mut result = 0;
    for (index, value) in ret.iter().enumerate() {
        if let Some(pow_value) = i32::checked_pow(10, (ret.len() - 1 - index) as u32) {
            if let Some(add_value) = i32::checked_mul(pow_value, *value) {
                if let Some(correct_value) = i32::checked_add(result, add_value) {
                    result = correct_value;
                } else {
                    return 0;
                }
            } else {
                return 0;
            }
        } else {
            return 0;
        }
    }

    if positive {
        result
    } else {
        -result
    }
}
