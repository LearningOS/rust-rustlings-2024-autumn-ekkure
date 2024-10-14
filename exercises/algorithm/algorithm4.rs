/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/


use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO
        //let mut rt = self.root.as_mut();
        if let Some(ref mut rt) = self.root{
            rt.insert(value);
        }else{
            self.root = Some(Box::new(TreeNode::new(value)));
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        //true
        if let Some(ref rt)/*也得是ref*/ = self.root{
            return rt.search(value);
        }else{
            return false;
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        // if self == None {
        //     *self = TreeNode::new(value);
        // } else {
        //     match value.cmp(&self.value) {
        //         Ordering::Less => {
        //             self.left.insert(value);
        //         }
        //         Ordering::Greater => {
        //             self.right.insert(value); //b不能这样直接写 
        //                                       //因为right是一个option 没有拆包 不能直接用insert
        //         }
        //     }
        // }
        match value.cmp(&self.value)/*引用借用防止所有权丢失*/{
            Ordering::Less => {
                //self.left.insert(value);
                if let Some(ref mut l)/*必须有ref 可变引用 否则所有权转移到l并死忘*/ = self.left{
                    l.insert(value);
                }else{
                    self.left = Some(Box::new(TreeNode::new(value)));
                }
            }
            Ordering::Greater => {
                //self.right.insert(value);
                if let Some(ref mut r) = self.right{
                    r.insert(value);
                }else{
                    self.right = Some(Box::new(TreeNode::new(value)));
                }
            }
            
            Ordering::Equal => {}
        }
    }
    fn search(&self, value:T) ->bool{
        match value.cmp(&self.value){
            Ordering::Less =>{
                if let Some(ref l) = self.left{
                    return l.search(value);
                }else{
                    return false;
                }
            }
            Ordering::Equal => return true,
            Ordering::Greater =>{
                if let Some(ref r) = self.right{
                    return r.search(value);
                }else{
                    return false;
                }
            }
            /*
            Ordering::Equal => true,
            Ordering::Less => self.left.as_ref().map_or(false, |node| node.search(value)),
            Ordering::Greater => self.right.as_ref().map_or(false, |node| node.search(value)),
             */
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


