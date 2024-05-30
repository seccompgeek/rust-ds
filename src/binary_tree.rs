//! Writing a binary tree collection.
//! This is a typical binary tree as learned in the data structures undergrad course.
//! A node will have two children, left an right.
//! The left child's key is smaller than its parent's, while the right child's key is the opposite.
//!
//! Through this, you learn:
//! 1. Ownership
//! 2. Using traits
//! 3. Using the Option type in Rust ==> Recall that a node maybe empty (NULL in C/C++) so how do
//!    we do that in Rust.
//!

use crate::Collection;

pub struct BinaryTree {
    // root:
}

impl BinaryTree {
    // Creates and returns an empty binary tree.
    pub fn new() -> Self {}
}

impl Collection for BinaryTree {}

#[cfg(test)]
mod tests {
    use super::BinaryTree;
    use crate::Collection;
    use rand::Rng;

    // Tests the add function of Binary tree.
    #[test]
    fn test_add() {
        let mut tree = BinaryTree::new();

        for i in 0..100 {
            if 0x1 & i == 0 {
                assert!(tree.add(i));
            } else {
                assert!(tree.add(100 - i));
            }
        }

        for i in 0..100 {
            // we shouldn't be able to readd keys already on tree.
            assert!(!tree.add(i));
        }
    }

    #[test]
    fn test_contains() {
        let mut tree = BinaryTree::new();

        for i in 0..100 {
            assert!(!tree.contains(i));
        }

        for i in 0..100 {
            if i % 2 == 0 {
                tree.add(i);
            } else {
                tree.add(100 - 1);
            }
        }

        for i in 0..100 {
            assert!(tree.contains(i));
        }
    }

    #[test]
    fn test_remove() {
        let mut tree = BinaryTree::new();
        assert!(!tree.remove(5)); //should return false, we have nothing yet.
        assert!(tree.add(5)); // we now have 5 as root.
        assert!(tree.remove(5));
        assert!(!tree.contains(5));
        assert!(tree.add(5));

        tree.add(3);
        tree.add(8);
        tree.add(7);
        tree.add(2);

        assert!(tree.remove(8));
        assert!(!tree.contains(8));
        assert!(tree.contains(7));
        assert!(tree.contains(2));
        assert!(tree.remove(2));

        assert!(tree.remove(5));
        assert!(tree.contains(3));
        assert!(tree.contains(7));
    }
}
