use crate::as_ref::as_ref;
use crate::is_some::is_some_demo;
use crate::unwrap::unwrap_demo;

mod is_some;
mod as_ref;
mod unwrap;
mod middle_node;

fn main() {
    // is_some_demo();
    // as_ref();
    // unwrap_demo();

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
