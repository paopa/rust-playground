struct Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut start = new_interval[0];
        let mut end = new_interval[1];
        let mut is_inserted = false; // check if the new interval is inserted

        for interval in intervals {
            if is_inserted {
                result.push(interval);
                continue;
            }

            // right
            if interval[1] < start {
                result.push(interval);
                continue;
            }

            // left
            if interval[0] > end {
                result.push(vec![start, end]);
                result.push(interval);
                is_inserted = true;
                continue;
            }

            // overlap
            start = i32::min(start, interval[0]);
            end = i32::max(end, interval[1]);
        }

        if !is_inserted {
            result.push(vec![start, end]);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example_1() {
        let intervals = vec![vec![1, 3], vec![6, 9]];
        let new_interval = vec![2, 5];
        let result = Solution::insert(intervals, new_interval);
        assert_eq!(result, vec![vec![1, 5], vec![6, 9]]);
    }
}
