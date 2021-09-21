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
            let sut = Some(Box::new(node4));

            assert_eq!(count_minus(&sut), 2);
        }
    }
}

mod tree {
    struct TreeNode {
        data: i32,
        left: Link,
        right: Link,
    }
    type Link = Option<Box<TreeNode>>;

    fn max(link: &Link) -> Option<i32> {
        if let Some(node) = link {
            let max_data = match (max(&node.left), max(&node.right)) {
                (None, None) => node.data,
                (Some(left), None) => std::cmp::max(left, node.data),
                (None, Some(right)) => std::cmp::max(right, node.data),
                (Some(left), Some(right)) => std::cmp::max(std::cmp::max(left, right), node.data),
            };
            Some(max_data)
        } else {
            None
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_sum_minus() {
            let node1 = TreeNode {
                data: 1,
                left: None,
                right: None,
            };
            let node2 = TreeNode {
                data: 2,
                left: Some(Box::new(node1)),
                right: None,
            };
            let node3 = TreeNode {
                data: 3,
                left: None,
                right: None,
            };
            let node4 = TreeNode {
                data: 0,
                left: Some(Box::new(node3)),
                right: Some(Box::new(node2)),
            };
            let sut = Some(Box::new(node4));
            assert_eq!(max(&sut), Some(3));
        }
    }
}

mod binary_tree {
    struct BinaryTree {
        root: Link,
    }
    struct TreeNode {
        data: i32,
        left: Link,
        right: Link,
    }
    type Link = Option<Box<TreeNode>>;

    impl BinaryTree {
        pub fn leaf_count(&self) -> Option<i32> {
            Self::inner_leaf_count(&self.root)
        }

        fn inner_leaf_count(link: &Link) -> Option<i32> {
            if let Some(node) = link {
                let count = match (
                    Self::inner_leaf_count(&node.left),
                    Self::inner_leaf_count(&node.right),
                ) {
                    (None, None) => 1,
                    (Some(left), None) => left,
                    (None, Some(right)) => right,
                    (Some(left), Some(right)) => left + right,
                };
                Some(count)
            } else {
                None
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_leaf_count() {
            let node1 = TreeNode {
                data: 1,
                left: None,
                right: None,
            };
            let node2 = TreeNode {
                data: 2,
                left: Some(Box::new(node1)),
                right: None,
            };
            let node3 = TreeNode {
                data: 3,
                left: None,
                right: None,
            };
            let node4 = TreeNode {
                data: 0,
                left: Some(Box::new(node3)),
                right: Some(Box::new(node2)),
            };
            let sut = BinaryTree {
                root: Some(Box::new(node4)),
            };
            assert_eq!(sut.leaf_count(), Some(2));
        }
    }
}
