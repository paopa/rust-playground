struct Solution;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut points = points;
        let mut result = vec![];

        // 2. sort the points by distance
        // the time complexity is O(nlogn)
        points.sort_by_key(|point| Solution::distance(point));

        // 3. get the first k points
        for i in 0..k {
            result.push(points[i as usize].clone());
        }

        result
    }

    // 1. create a function to calculate the distance
    fn distance(point: &Vec<i32>) -> i32 {
        point[0] * point[0] + point[1] * point[1]
    }
}

mod tests {
    use super::*;
    #[test]
    fn example_1() {
        let points = vec![vec![1, 3], vec![-2, 2]];
        let k = 1;
        let result = vec![vec![-2, 2]];
        assert_eq!(Solution::k_closest(points, k), result);
    }
    #[test]
    fn example_2() {
        let points = vec![vec![3, 3], vec![5, -1], vec![-2, 4]];
        let k = 2;
        let result = vec![vec![3, 3], vec![-2, 4]];
        assert_eq!(Solution::k_closest(points, k), result);
    }
}
