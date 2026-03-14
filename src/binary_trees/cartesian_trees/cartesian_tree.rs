use std::fmt;

use crate::binary_trees::traits::{BinaryTree, BinaryTreeNode, binary_tree};

pub struct CartesianTree<K, V>(Option<CartesianTreeNode<K, V>>);

pub struct CartesianTreeNode<K, V> {
    key: K,
    value: V,
    left: Box<CartesianTree<K, V>>,
    right: Box<CartesianTree<K, V>>,
}

impl<K, V> CartesianTreeNode<K, V> {
    fn new(key: K, value: V) -> Self {
        Self {
            key,
            value,
            left: Box::new(CartesianTree::new_leaf()),
            right: Box::new(CartesianTree::new_leaf()),
        }
    }
}

impl<K, V> BinaryTreeNode for CartesianTreeNode<K, V> {
    type Tree = CartesianTree<K, V>;

    fn left_subtree(&self) -> &Self::Tree {
        &self.left
    }

    fn right_subtree(&self) -> &Self::Tree {
        &self.right
    }
}

impl<K, V> BinaryTree for CartesianTree<K, V> {
    type Node = CartesianTreeNode<K, V>;

    fn new_leaf() -> Self {
        Self(None)
    }

    fn is_leaf(&self) -> bool {
        self.0.is_none()
    }

    fn root(&self) -> Option<&Self::Node> {
        self.0.as_ref()
    }
}

impl<K, V> FromIterator<(K, V)> for CartesianTree<K, V>
where 
    K: Ord,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        // Maintain the right spine of the tree, as a sequence of to-be-connected nodes.
        let mut spine = Vec::<CartesianTreeNode<_, _>>::new();
        for (key, value) in iter {
            let mut node = CartesianTreeNode::new(key, value);
            let mut left_child = None;
            
            // Find the node on the spine that becomes node's left child.
            while let Some(last) = spine.last() && last.key < node.key {
                let mut last = spine.pop().unwrap();
                last.right = Box::new(CartesianTree(left_child));
                left_child = Some(last);
            }
            node.left = Box::new(CartesianTree(left_child));
            spine.push(node);
        }
        
        // Attach the nodes on the spine to get the final tree.
        let mut root = spine.pop();
        while let Some(mut node) = spine.pop() {
            node.right = Box::new(CartesianTree(root));
            root = Some(node);
        }
        CartesianTree(root)        
    }
}

impl<K, V> fmt::Debug for CartesianTreeNode<K, V>
where
    K: fmt::Debug,
    V: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}, {:?})", &self.key, &self.value)
    }
}

impl<K, V> fmt::Display for CartesianTreeNode<K, V>
where
    K: fmt::Display,
    V: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", &self.key, &self.value)
    }
}

impl<K, V> fmt::Debug for CartesianTree<K, V>
where 
    K: fmt::Debug,
    V: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        binary_tree::debug_binary_tree(self, f)
    }
}
    
impl<K, V> fmt::Display for CartesianTree<K, V>
where 
    K: fmt::Display,
    V: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        binary_tree::display_binary_tree(self, f)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::binary_trees::traits::{BinaryTree, iterable_preorder::IterablePreorder, iterable_inorder::IterableInorder};

    use std::fmt;
    use rand::prelude::*;

    fn assert_max_heap<K, V>(tree: &CartesianTree<K, V>)
    where 
        K: Ord,
    {
        for subtree in tree.preorder_iter() {
            let Some(root) = subtree.root() else { continue; };
            if let Some(left) = root.left.root() {
                assert!(root.key >= left.key);
            }
            if let Some(right) = root.right.root() {
                assert!(root.key >= right.key);
            }
        }
    }

    fn assert_cartesian_tree<K, V>(sequence: Vec<(K, V)>)
    where 
        K: Ord + Clone + fmt::Debug + Eq,
        V: Clone + fmt::Debug + Eq,
    {
        let tree = sequence.clone()
            .into_iter()
            .collect::<CartesianTree<_, _>>();
        assert_max_heap(&tree);
        dbg!(&tree);

        // Assert the sequence is preserved.
        let tree_sequence = tree.inorder_iter()
            .filter_map(|tree| tree.root().map(|root| (root.key.clone(), root.value.clone())))
            .collect::<Vec<_>>();
        for i in 0..sequence.len() {
            assert_eq!(sequence.get(i), tree_sequence.get(i));
        }
    }

    #[test]
    fn test_cartesian_tree() {
        let mut rng = rand::rng();
        for _ in 0..50 {
            let mut sequence = (1..=30).map(|x| (x, ())).collect::<Vec<_>>();
            sequence.shuffle(&mut rng);
            assert_cartesian_tree(sequence);
        }
    }
}
