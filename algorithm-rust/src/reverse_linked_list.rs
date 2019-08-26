    #[derive(PartialEq, Eq, Clone, Debug)]
    pub struct ListNode {
        pub val: i32,
        pub next: Option<Box<ListNode>>
    }

    impl ListNode {
        #[inline]
        fn new(val: i32) -> Self {
            ListNode {
                next: None,
                val
            }
        }
    }
    impl Solution {
        pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut prev = None;
            let mut cur = head;
            while let Some(mut inner) = cur {
                let next = inner.next.take();
                inner.next = prev;
                prev = Some(inner);
                cur = next;
            }
            prev
        }
    }
