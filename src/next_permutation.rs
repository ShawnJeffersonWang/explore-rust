pub fn next_permutation(nums: &mut Vec<i32>) {
    let n = nums.len();
    if n == 0 {
        return;
    }
    let mut i = n as isize - 2;
    while i >= 0 && nums[i as usize] >= nums[(i + 1) as usize] {
        i -= 1;
    }
    if i >= 0 {
        let mut j = n as isize - 1;
        while j >= 0 && nums[i as usize] >= nums[j as usize] {
            j -= 1;
        }
        nums.swap(i as usize, j as usize);
    }
    reverse(&mut nums[(i + 1) as usize..])
}

fn reverse(a: &mut [i32]) {
    let n = a.len();
    for i in 0..n / 2 {
        a.swap(i, n - 1 - i);
    }
}