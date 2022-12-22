struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    #[inline]
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    #[inline]
    fn insert(val: i32, mut node: &mut TreeNode) {
        let inode = Box::new(TreeNode::new(val));
        loop {
            if val < node.val {
                if node.left.is_none() {
                    node.left = Some(inode);
                    break;
                }
                node = node.left.as_mut().unwrap();
            } else {
                if node.right.is_none() {
                    node.right = Some(inode);
                    break;
                }
                node = node.right.as_mut().unwrap();
            }
        }
    }

    fn preorder(node: &TreeNode) {
        println!("{}", node.val);
        if let Some(left) = &node.left {
            TreeNode::preorder(left);
        }
        if let Some(right) = &node.right {
            TreeNode::preorder(right);
        }
    }

    fn inorder(node: &TreeNode) {
        if let Some(left) = &node.left {
            TreeNode::inorder(left);
        }
        println!("{}", node.val);
        if let Some(right) = &node.right {
            TreeNode::inorder(right);
        }
    }
}

fn main() {
    let mut root = TreeNode::new(3);
    TreeNode::insert(5, &mut root);
    TreeNode::insert(4, &mut root);
    TreeNode::insert(3, &mut root);
    TreeNode::insert(8, &mut root);
    TreeNode::insert(7, &mut root);
    TreeNode::insert(2, &mut root);
    println!("preorder traversal");
    TreeNode::preorder(&root);
    println!("inorder traversal");
    TreeNode::inorder(&root);
}
