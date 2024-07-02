// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            val,
            next: None,
        }
    }
}

pub fn reverse_list1(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut pre = None;
    let mut cur = head;
    while let Some(mut node) = cur {
        let nxt = node.next;
        node.next = pre;
        pre = Some(node);
        cur = nxt;
    }
    pre
}

pub fn reverse_list2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    fn rev(
        mut pre: Option<Box<ListNode>>,
        mut cur: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if let Some(mut node) = cur.take() {
            let next = node.next;
            node.next = pre;
            return rev(Some(node), next);
        }
        return pre;
    }
    return rev(None, head);
}


// Helper function to create a linked list from a vector
fn vec_to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current = None;
    for &val in vec.iter().rev() {
        let mut new_node = Box::new(ListNode::new(val));
        new_node.next = current;
        current = Some(new_node);
    }
    current
}

// Helper function to convert a linked list to a vector
fn list_to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut vec = Vec::new();
    let mut current = head;
    while let Some(node) = current {
        vec.push(node.val);
        current = node.next;
    }
    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_list2() {
        // Test case 1: Empty list
        assert_eq!(reverse_list2(None), None);

        // Test case 2: Single element list
        let list = vec_to_list(vec![1]);
        let expected = vec_to_list(vec![1]);
        assert_eq!(reverse_list2(list), expected);

        // Test case 3: Multiple elements list
        let list = vec_to_list(vec![1, 2, 3, 4, 5]);
        let expected = vec_to_list(vec![5, 4, 3, 2, 1]);
        assert_eq!(reverse_list2(list), expected);
    }
}

// To run the test, use the command: cargo test