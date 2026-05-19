# Geometric tree data structures in Rust
This repo contains a hobby project for implementing geometric tree data structures in Rust.
The goal is asymptotically optimal implementations using no interior mutability or reference counting.
It is currently being prepared for a release as a library.

### Currently implemented trees:
- Dynamic Red-Black trees.
These store key-value pairs in a balanced binary search tree.
Red-Black trees support insertions and deletions.<br>
Insertion time: O(log n).<br>
Deletion time: O(log n).<br>
Space used: O(n).
- Dynamic semigroup Red-Black trees.
These additionally store a value in each node that is calculated by some semigroup computation based on the keys in its subtree.
Semigroup values are updated whenever the tree is updated.<br>
Insertion time: O(T(n) log n), where T(n) is the time taken by a semigroup operation.<br>
Deletion time: O(T(n) log n), where T(n) is the time taken by a semigroup operation.<br>
Space used: O(S(n log n) * n), where S(n) is the space used by a semigroup value computed from n elements.
- Static min/max-Cartesian trees.
These store a sequence in a min/max-heap, such that the sequence is preserved by the inorder ordering of the nodes in the tree.<br>
Construction time: O(n).<br>
Space used: O(n).

### Additional operations:
- Inorder, preorder, and postorder iterators over binary trees, both over shared and exclusive references.
- Cursors that allow for arbitrary movement through the trees. Cursors can return shared and exclusive references to data stored in the nodes.
