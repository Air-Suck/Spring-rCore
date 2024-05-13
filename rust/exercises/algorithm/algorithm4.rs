/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

// use std::cmp::Ordering;
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
        //先获取根节点,进入这个分支说明有根节点,需要一直找到为空的时候，在空的位置去建一个节点
        match self.root{
            None=>{
                let Node:Box<TreeNode<T>> = Box::new(TreeNode::<T>::new(value));
                self.root=Some(Node);
            }
            Some(_)=>{
                Self::insertNode(&mut self.root,value);
            }
        }
    }

    //根据节点插入，需要保证传入的节点不空
    fn insertNode(node:&mut Option<Box<TreeNode<T>>>,value:T){
        if let Some(temp)=node{
            //取出来的temp应该是一个指向树节点的指针
            if temp.value<value{
                //说明右边是没有节点的，这个时侯直接插入就好了
                match temp.right{
                    None=>{
                        let Node:Box<TreeNode<T>> = Box::new(TreeNode::<T>::new(value));
                        temp.right=Some(Node);
                    }
                    Some(_)=>{
                        Self::insertNode(&mut temp.right,value);
                    }
                }
            }else if temp.value>value{
                match temp.left{
                    None=>{
                        let Node:Box<TreeNode<T>> = Box::new(TreeNode::<T>::new(value));
                        temp.left=Some(Node);
                    }
                    Some(_)=>{
                        Self::insertNode(&mut temp.left,value);
                    }
                }
            }else{
                //相同的时候就不重复插入了，直接返回就好了
            }
        }
    }

    // Search for a value in the BST
    fn search(&mut self, value: T) -> bool {
        //TODO
        match self.root{
            Some(_)=>{
                Self::searchNode(&mut self.root,value)
            }
            //说明当前节点为空，这个时候就要返回false
            None=>{
                false
            }
        }
    }

    fn searchNode(node:&mut Option<Box<TreeNode<T>>>,value:T)->bool{
        if let Some(temp)=node{
            if temp.value==value{
                true
            }else if temp.value<value{
                Self::searchNode(&mut temp.right,value)
            }else {
                Self::searchNode(&mut temp.left,value)
            }
        }else {
            //此时说明没找到节点，返回false
            false
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


