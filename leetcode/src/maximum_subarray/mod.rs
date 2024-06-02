struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_sum = nums[0];
        let mut current_sum = if nums[0] > 0 { nums[0] } else { 0 };

        for i in 1..nums.len() {
            let tmp = current_sum + nums[i];
            max_sum = max_sum.max(tmp);
            current_sum = if tmp > 0 { tmp } else { 0 };
        }

        max_sum
    }
}

mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        assert_eq!(Solution::max_sub_array(nums), 6);
    }

    #[test]
    fn example_2() {
        let nums = vec![1];
        assert_eq!(Solution::max_sub_array(nums), 1);
    }

    #[test]
    fn example_3() {
        let nums = vec![5, 4, -1, 7, 8];
        assert_eq!(Solution::max_sub_array(nums), 23);
    }
}
