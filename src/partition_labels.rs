pub fn partition_labels(s: String) -> Vec<i32> {
    let mut last = [0; 26];
    for (i, c) in s.bytes().enumerate() {
        last[(c - b'a') as usize] = i;
    }
    let mut start = 0;
    let mut end = 0;
    let mut ans = vec![];
    for (i, c) in s.bytes().enumerate() {
        end = end.max(last[(c - b'a') as usize]);
        if end == i {
            ans.push((end - start + 1) as i32);
            start = i + 1;
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use crate::partition_labels::partition_labels;
    use super::*;

    #[test]
    fn test_partition_labels() {
        let s1 = "ababcbacadefegdehijhklij".to_string();
        let result1 = partition_labels(s1);
        assert_eq!(result1, vec![9, 7, 8]);

        let s2 = "eccbbbbdec".to_string();
        let result2 = partition_labels(s2);
        assert_eq!(result2, vec![10]);

        let s3 = "a".to_string();
        let result3 = partition_labels(s3);
        assert_eq!(result3, vec![1]);

        let s4 = "abac".to_string();
        let result4 = partition_labels(s4);
        assert_eq!(result4, vec![3, 1]);

        let s5 = "aabbcc".to_string();
        let result5 = partition_labels(s5);
        assert_eq!(result5, vec![2, 2, 2]);

        let s6 = "".to_string();
        let result6 = partition_labels(s6);
        assert_eq!(result6, vec![]);
    }
}