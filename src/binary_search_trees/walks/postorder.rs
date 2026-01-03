use std::{fmt::Debug, marker::PhantomData, ops::DerefMut};

use lending_iterator::prelude::*;

use crate::binary_search_trees::binary_search_tree_node::{BinarySearchTreeNode, Side};

use super::WalkInstruction;

struct DFSStack<E>(Vec<(E, Vec<Side>)>);

impl<E> DFSStack<E> {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn push(&mut self, node: E, sides: Vec<Side>) {
        self.0.push((node, sides));
    }

    fn last_edge_mut(&mut self) -> Option<&mut E> {
        if let Some((last, _)) = self.0.last_mut() {
            Some(last)
        } else { None }
    }

    fn pop(&mut self) -> Option<(E, Vec<Side>)> {
        self.0.pop()        
    }

    fn pop_if_expanded(&mut self) -> Option<E> {
        if let Some((_, sides)) = self.0.last() && sides.is_empty() {
            self.pop().map(|(last, _)| last)
        } else { None }
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

pub struct PostorderWalk<'lt, N, E, F> {
    root_state: Option<(&'lt mut N, Vec<Side>)>,
    frontier_stack: Vec<E>, // Edges that we have encountered, but not traversed yet.
    visited_stack: DFSStack<E>, // Edges that we have encountered and traversed, but not reported yet.
    instruction_fn: F,
}

impl<'lt, N, E, F> PostorderWalk<'lt, N, E, F>
where 
    N: BinarySearchTreeNode<Wrapper = N, Edge = E>,
    E: DerefMut<Target = N>,
    F: Fn(&N) -> WalkInstruction,
{
    pub fn new(root_edge: &'lt mut N, instruction_fn: F) -> Self {
        let mut root_state = None;
        let mut frontier_stack = Vec::new();
        match (instruction_fn)(&root_edge) {
            WalkInstruction::Left => {
                if let Some(left) = root_edge.detach_left() {
                    frontier_stack.push(left);
                    root_state = Some((root_edge, vec![Side::Left]));
                }
            },
            WalkInstruction::Right => {
                if let Some(right) = root_edge.detach_right() {
                    frontier_stack.push(right);
                    root_state = Some((root_edge, vec![Side::Right]));
                }
            },
            WalkInstruction::Both => {
                match (root_edge.detach_left(), root_edge.detach_right()) {
                    (Some(left), Some(right)) => {
                        frontier_stack.push(right);
                        frontier_stack.push(left);
                        root_state = Some((root_edge, vec![Side::Right, Side::Left]));
                    },
                    (Some(left), _) => {
                        frontier_stack.push(left);
                        root_state = Some((root_edge, vec![Side::Left]));
                    },
                    (_, Some(right)) => {
                        frontier_stack.push(right);
                        root_state = Some((root_edge, vec![Side::Right]));
                    },
                    _ => (),
                }
            },
            WalkInstruction::None => (),
        };

        Self {
            root_state,
            frontier_stack,
            visited_stack: DFSStack::new(),
            instruction_fn,
        }
    }
}

#[gat]
impl<'lt, N, E, F> LendingIterator for PostorderWalk<'lt, N, E, F>
where 
    N: BinarySearchTreeNode<Wrapper = N, Edge = E, Key: Debug>,
    E: DerefMut<Target = N>,
    F: Fn(&N) -> WalkInstruction,
{
    type Item<'next>
    where 
        Self: 'next,
        = &'next mut N;

    fn next(self: &'_ mut PostorderWalk<'lt, N, E, F>) -> Option<&'_ mut N> {
        if self.frontier_stack.is_empty() && self.visited_stack.is_empty() {
            if let Some((root, _)) = self.root_state.take() {
                return Some(root);
            } else {
                return None;
            }
        }
        // First check for fully expanded nodes, which we report first.
        if let Some(edge) = self.visited_stack.pop_if_expanded() {
            if let Some((mut parent, mut sides)) = self.visited_stack.pop() {
                let side = sides.pop().unwrap(); // Can unwrap safely: parent is not fully expanded.
                parent.attach_child(side, edge);
                self.visited_stack.push(parent, sides);

                return self.visited_stack.last_edge_mut().and_then(move |parent| parent.get_child_node_mut(side));
            } else {
                // Edge is incident to the root node.
                let (root, mut sides) = self.root_state.take().unwrap(); // Can unwrap safely: root edge exists since there is an unreported edge.
                let side = sides.pop().unwrap(); // Can unwrap safely: root edge has unreported children.
                root.attach_child(side, edge);
                self.root_state = Some((root, sides.clone()));

                return self.root_state.as_mut().unwrap().0.get_child_node_mut(side);
            }
        }
        
        // Expand edges on the frontier.
        let current = loop {
            let mut current = self.frontier_stack.pop()?;

            match (self.instruction_fn)(&current) {
                WalkInstruction::Left => {
                    if let Some(left) = current.detach_left() {
                        self.frontier_stack.push(left);
                        self.visited_stack.push(current, vec![Side::Left]);
                    } else { break current }
                },
                WalkInstruction::Right => {
                    if let Some(right) = current.detach_right() {
                        self.frontier_stack.push(right);
                        self.visited_stack.push(current, vec![Side::Right]);
                    } else { break current }
                },
                WalkInstruction::Both => {
                    match (current.detach_left(), current.detach_right()) {
                        (Some(left), Some(right)) => {
                            self.frontier_stack.push(right);
                            self.frontier_stack.push(left);
                            self.visited_stack.push(current, vec![Side::Right, Side::Left]);
                        },
                        (Some(left), _) => {
                            self.frontier_stack.push(left);
                            self.visited_stack.push(current, vec![Side::Left]);
                        },
                        (_, Some(right)) => {
                            self.frontier_stack.push(right);
                            self.visited_stack.push(current, vec![Side::Right]);
                        },
                        _ => break current,
                    };
                },
                WalkInstruction::None => break current,
            }
        };

        // Re-attach the edge to its incident parent node before reporting it.
        if let Some((mut parent, mut sides)) = self.visited_stack.pop() {
            let side = sides.pop().unwrap(); // Can unwrap safely: parent is not fully expanded.
            parent.attach_child(side, current);
            self.visited_stack.push(parent, sides);

            self.visited_stack.last_edge_mut().and_then(move |parent| parent.get_child_node_mut(side))
        } else {
            None
        }
    }
}
