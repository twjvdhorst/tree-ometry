# Geometric tree data structures in Rust
This repo contains a hobby project for implementing geometric tree data structures in Rust.
The goal is asymptotically optimal implementations using no interior mutability or reference counting.

### Currently implemented trees:
- Dynamic Red-Black trees. These store key-value pairs in a balanced binary search tree.
- Dynamic semigroup Red-Black trees.
These additionally store a value in each node that is calculated by some semigroup computation based on the keys in its subtree.
Semigroup values are updated whenever the tree is updated.
