pub fn smallest_subsequence(s: String) -> String {
    use std::collections::HashMap;
    use std::collections::HashSet;

    let mut left: HashMap<char, i32> = HashMap::new(); // 统计每个字母的出现次数
    for c in s.chars() {
        *left.entry(c).or_insert(0) += 1;
    }

    let mut ans: Vec<char> = Vec::new();
    let mut in_ans: HashSet<char> = HashSet::new();

    for c in s.chars() {
        *left.get_mut(&c).unwrap() -= 1;
        if in_ans.contains(&c) { // ans 中不能有重复字母
            continue;
        }
        while let Some(&last) = ans.last() {
            if c < last && *left.get(&last).unwrap() > 0 {
                ans.pop();
                in_ans.remove(&last);
            } else {
                break;
            }
        }
        ans.push(c); // 把 c 加到 ans 的末尾
        in_ans.insert(c); // 标记 c 在 ans 中
    }

    ans.into_iter().collect()
}

fn main() {
    let s = String::from("cbacdcbc");
    println!("{}", smallest_subsequence(s)); // 输出 "acdb"
}