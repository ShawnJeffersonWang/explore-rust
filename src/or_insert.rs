use std::collections::HashMap;

fn main() {
    let mut map: HashMap<char, i32> = HashMap::new();
    let input = "hello";

    for c in input.chars() {
        // 使用 entry 和 or_insert 更新字符计数
        *map.entry(c).or_insert(0) += 1;
    }

    // 打印结果
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
}