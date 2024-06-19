#[derive(Debug, Clone)]
pub(crate) struct ListNode {
    pub(crate) val: i32,
    pub(crate) next: Option<Box<ListNode>>,
}

impl ListNode {
    pub(crate) fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast = &head;
        let mut slow = &head;

        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            slow = &slow.as_ref().unwrap().next;
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
        }

        slow.clone()
    }
}

pub(crate) struct Solution;

fn main() {
    // 创建链表 1 -> 2 -> 3 -> 4 -> 5
    let mut node5 = Box::new(ListNode::new(5));
    let mut node4 = Box::new(ListNode::new(4));
    let mut node3 = Box::new(ListNode::new(3));
    let mut node2 = Box::new(ListNode::new(2));
    let mut node1 = Box::new(ListNode::new(1));

    node4.next = Some(node5);
    node3.next = Some(node4);
    node2.next = Some(node3);
    node1.next = Some(node2);

    let head = Some(node1);

    if let Some(mid) = Solution::middle_node(head) {
        println!("The middle node is: {}", mid.val); // 输出: The middle node is: 3
    }
}