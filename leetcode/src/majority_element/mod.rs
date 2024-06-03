struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut candidate = 0;

        for num in nums.iter() {
            if count == 0 {
                candidate = *num;
            }
            count += if *num == candidate { 1 } else { -1 };
            println!("num: {}, count: {}, candidate: {}", num, count, candidate);
        }

        candidate
    }
}

fn main() {
    let nums = vec![2, 2, 1, 1, 1, 2, 2];
    let res = Solution::majority_element(nums);
    println!("{}", res);
    assert_eq!(res, 2);

    let nums = vec![3, 2, 3];
    let res = Solution::majority_element(nums);
    println!("{}", res);
    assert_eq!(res, 3);

    let nums = vec![5, 5, 2, 3, 5, 2, 4, 5, 5, 4, 4, 4, 3, 4, 4];
    let res = Solution::majority_element(nums);
    println!("{}", res);
    // assert_eq!(res, 5);
}
