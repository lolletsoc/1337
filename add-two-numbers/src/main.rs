#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn add_two_numbers(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut to_carry = 0;

    let mut head = Box::new(ListNode::new(0));
    let mut tail = &mut head;

    for _i in 0..100 {
        let mut passed = false;
        
        let l1_add;
        match l1 {
            Some(ref node) => {
                l1_add = node.val;
                passed = true;
            },
            None => l1_add = 0,
        }

        let l2_add;
        match l2 {
            Some(ref node) => {
                l2_add = node.val;
                passed = true;
            },
            None => l2_add = 0,
        }

        if !passed && to_carry == 0 {
            break;
        }

        let total = l1_add + l2_add + to_carry;
        to_carry = 0;

        let whole = total % 10;
        to_carry += total / 10;

        let new_node = Option::Some(Box::from(ListNode::new(whole)));
        tail.next = new_node;
        tail = tail.next.as_mut().unwrap();

        match l1 {
            Some(node) => l1 = node.next,
            None => l1 = Option::None,
        }

        match l2 {
            Some(node) => l2 = node.next,
            None => l2 = Option::None,
        }
    }

    head.next
}

fn main() {
    let mut l1: ListNode = ListNode::new(2);
    l1.next = Option::Some(Box::from(ListNode::new(4)));

    let mut l2: ListNode = ListNode::new(5);
    l2.next = Option::Some(Box::from(ListNode::new(6)));

    let mut result = add_two_numbers(Option::Some(Box::from(l1)), Option::Some(Box::from(l2)));

    while let Some(ref node) = result {
        println!("{:?}", node.val);
        result = result.unwrap().next
    }

}


// 2 4
// 8 6

// 100