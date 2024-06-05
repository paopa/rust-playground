struct Solution;

impl Solution {
    // fabonacci sequence
    // f(n) = f(n-1) + f(n-2)
    // f(1) = 1, f(2) = 2
    // f(3) = f(2) + f(1) = 2 + 1 = 3
    // f(4) = f(3) + f(2) = 3 + 2 = 5
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }

        if n == 2 {
            return 2;
        }

        let mut f1 = 1;
        let mut f2 = 2;
        for _ in 3..=n {
            let f3 = f1 + f2;
            f1 = f2;
            f2 = f3;
        }

        f2
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_climb_stairs() {
        assert_eq!(super::Solution::climb_stairs(2), 2);
        assert_eq!(super::Solution::climb_stairs(3), 3);
    }
}
