/**
 *  Problem 739. Daily Temperature
 *  See: https://leetcode.com/problems/daily-temperatures/
 *
 *  Given an array of integers temperatures represents the daily
 *  temperatures, return an array answer such that answer[i] is the
 *  number of days you have to wait after the ith day to get a warmer
 *  temperature.
 *
 *  If there is no future day for which this is possible, keep
 *  answer[i] == 0 instead.
 */
pub fn run(temperatures: Vec<i32>) -> Vec<i32> {
    vec![]
}

#[cfg(test)]
mod test {
    use super::*;

    #[ignore = "NOT YET IMPLEMENTED"]
    #[test]
    fn it_returns_days_to_wait_for_warmer_day() {
        assert_eq!(
            run(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
    }
}
