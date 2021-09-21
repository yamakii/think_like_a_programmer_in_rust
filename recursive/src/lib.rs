#![allow(dead_code)]

mod array {
    fn sum(array: &[i32], size: usize) -> i32 {
        let mut total = 0;
        for i in 0..size {
            total += array[i as usize];
        }
        total
    }

    fn sum_recursive(array: &[i32], size: usize) -> i32 {
        if size == 0 {
            0
        } else {
            let last_number = array[size - 1];
            let all_but_last_sum = sum_recursive(array, size - 1);
            all_but_last_sum + last_number
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_sum() {
            assert_eq!(sum(&[1, 2, 3], 3), 6);
        }
        #[test]
        fn test_sum_recursive() {
            assert_eq!(sum_recursive(&[1, 2, 3], 3), 6);
        }
    }
}

mod linked_list {
    struct LinkedList(Link);
    struct ListNode {
        data: i32,
        next: Link,
    }
    type Link = Option<Box<ListNode>>;

    fn count_minus(link: &Link) -> u32 {
        if let Some(node) = link {
            let count_but_current = count_minus(&node.next);
            if node.data < 0 {
                count_but_current + 1
            } else {
                count_but_current
            }
        } else {
            0
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_sum_minus() {
            let node1 = ListNode {
                data: 1,
                next: None,
            };
            let node2 = ListNode {
                data: -1,
                next: Some(Box::new(node1)),
            };
            let node3 = ListNode {
                data: -2,
                next: Some(Box::new(node2)),
            };
            let node4 = ListNode {
                data: 2,
                next: Some(Box::new(node3)),
            };
            let sut = LinkedList(Some(Box::new(node4)));

            assert_eq!(count_minus(&sut.0), 2);
        }
    }
}
