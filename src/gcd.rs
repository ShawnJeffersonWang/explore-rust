fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn gcd_iter(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

pub fn find_gcd(nums: Vec<i32>) -> i32 {
    let max_num = *nums.iter().max().unwrap();
    let min_num = *nums.iter().min().unwrap();
    gcd(max_num, min_num)
}