use crate::as_ref::as_ref;
use crate::gcd::find_gcd;
use crate::is_some::is_some_demo;
use crate::next_permutation::next_permutation;
use crate::smallest_subsequence::smallest_subsequence;
use crate::unwrap::unwrap_demo;

mod is_some;
mod as_ref;
mod unwrap;
mod middle_node;
mod smallest_subsequence;
mod or_insert;
mod gcd;
mod next_permutation;
mod partition_labels;
mod reverse_list;
mod candy;

fn test_middle_node() {
    // 创建链表 1 -> 2 -> 3 -> 4 -> 5
    let mut node5 = Box::new(crate::middle_node::ListNode::new(5));
    let mut node4 = Box::new(crate::middle_node::ListNode::new(4));
    let mut node3 = Box::new(crate::middle_node::ListNode::new(3));
    let mut node2 = Box::new(crate::middle_node::ListNode::new(2));
    let mut node1 = Box::new(crate::middle_node::ListNode::new(1));

    node4.next = Some(node5);
    node3.next = Some(node4);
    node2.next = Some(node3);
    node1.next = Some(node2);

    let head = Some(node1);

    if let Some(mid) = crate::middle_node::Solution::middle_node(head) {
        println!("The middle node is: {}", mid.val); // 输出: The middle node is: 3
    }
}

fn test_smallest_subsequence() {
    let s = String::from("cbacdcbc");
    println!("{}", smallest_subsequence(s));
}

fn test_find_gcd() {
    let nums1 = vec![2, 5, 6, 9, 10];
    let nums2 = vec![7, 5, 6, 8, 3];
    let nums3 = vec![3, 3];
    println!("{}", find_gcd(nums1));
    println!("{}", find_gcd(nums2));
    println!("{}", find_gcd(nums3));
}

fn test_next_permutation() {
    let mut nums = vec![1, 2, 3];
    println!("{:?}", nums);
    for i in 0..6 {
        next_permutation(&mut nums);
        println!("{:?}", nums);
    }
}

fn main() {
    // test_find_gcd();
    test_next_permutation();
}
