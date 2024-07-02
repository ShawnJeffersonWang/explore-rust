pub fn candy(ratings: Vec<i32>) -> i32 {
    let mut candies = vec![1; ratings.len()];
    for i in 1..ratings.len() {
        if ratings[i] > ratings[i - 1] {
            candies[i] = candies[i - 1] + 1;
        }
    }
    for i in (0..ratings.len() - 1).rev() {
        if ratings[i] > ratings[i + 1] {
            candies[i] = candies[i].max(candies[i + 1] + 1);
        }
    }
    candies.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_candy() {
        // Case 1: Single child
        assert_eq!(candy(vec![1]), 1);

        // Case 2: Children with the same ratings
        assert_eq!(candy(vec![1, 1, 1]), 3);

        // Case 3: Increasing ratings
        assert_eq!(candy(vec![1, 2, 3]), 6);

        // Case 4: Decreasing ratings
        assert_eq!(candy(vec![3, 2, 1]), 6);

        // Case 5: Peak in the middle
        assert_eq!(candy(vec![1, 2, 3, 2, 1]), 9);

        // Case 6: Valley in the middle
        assert_eq!(candy(vec![3, 2, 1, 2, 3]), 11);

        // Case 7: Random ratings
        assert_eq!(candy(vec![1, 0, 2]), 5);

        // Case 8: Another set of random ratings
        assert_eq!(candy(vec![1, 2, 2]), 4);
    }
}