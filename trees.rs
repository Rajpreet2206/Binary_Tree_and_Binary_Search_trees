//The simple implementation of Binary Tree

pub struct TreeNode<T>{
    data: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

//A definition of binary tree
pub struct BinaryTree<T>{
    root: Option<Box<TreeNode<T>>>,
}

//A definition of BinarySearchTree
pub struct BinarySearchTree<T>{
    root: Option<Box<TreeNode<T>>>,
}

impl<T: std::cmp::PartialOrd + std::fmt::Display>
BinarySearchTree<T>{
    pub fn new() -> Self{
        BinarySearchTree {root: None}
    }
    pub fn insert(&mut self, data: T){
        let root = self.root.take();
        self.root = self.insert_rec(root, data);
    }
    fn insert_rec(&mut self, node: Option<Box<TreeNode<T>>>, data: T) -> Option<Box<TreeNode<T>>>{
        match node{
            Some(mut n) => {
                if n.data < data{
                    n.right = self.insert_rec(n.right, data);
                } else{
                    n.left = self.insert_rec(n.left, data);
                }
                Some(n)
            }
            None => Some(Box::new(TreeNode{data: data, left:None, right: None})),
        }
    }

    pub fn find(&self, val: T) -> bool{
        self.find_rec(&self.root, val)
    }
    fn find_rec(&self, node: &Option<Box<TreeNode<T>>>, val: T) -> bool{
        match node{
            Some(n) => {
                if n.data == val{
                    true
                } else if n.data < val {
                    self.find_rec(&n.right, val)
                } else{
                    self.find_rec(&n.left, val)
                }
            }
            None => false,
        }
    }
    pub fn in_order(&self, node: &Option<Box<TreeNode<T>>>)
    {
        if let Some(n) = node{
            self.in_order(&n.left);
            println!("{}", &n.data);
            self.in_order(&n.right);
        }
    }
}

fn main(){
    let mut BST = BinarySearchTree::new();
    BST.insert(2);
    BST.insert(1);
    BST.insert(3);
    BST.insert(10);
    BST.insert(5);
    BST.insert(2);
    BST.in_order(&BST.root);
    println!("find 2: {}", BST.find(2));
    println!("find 5: {}", BST.find(5));
    println!("find 15: {}", BST.find(15));
}
