use std::{marker::PhantomData, ops::DerefMut};

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
}

pub struct PostorderWalk<'lt, N, E, F> {
    completed: bool,
    root: &'lt mut N,
    left_walk: Option<PostorderEdgeWalk<'lt, N, E, F>>,
    right_walk: Option<PostorderEdgeWalk<'lt, N, E, F>>,
}

impl<'lt, N, E, F> PostorderWalk<'lt, N, E, F> 
where 
    N: BinarySearchTreeNode<Wrapper = N, Edge = E>,
    E: DerefMut<Target = N>,
    F: Fn(&N) -> WalkInstruction + Clone,
{
    pub fn new(root: &'lt mut N, instruction_fn: F) -> Self {
        let (left_walk, right_walk) = match (instruction_fn)(root) {
            WalkInstruction::Left => (
                root.detach_left().map(|left| PostorderEdgeWalk::new(left, instruction_fn)),
                None,
            ),
            WalkInstruction::Right => (
                None,
                root.detach_right().map(|right| PostorderEdgeWalk::new(right, instruction_fn)),
            ),
            WalkInstruction::Both => (
                root.detach_left().map(|left| PostorderEdgeWalk::new(left, instruction_fn.clone())),
                root.detach_right().map(|right| PostorderEdgeWalk::new(right, instruction_fn)),
            ),
            WalkInstruction::None => (None, None),
        };

        Self {
            completed: false,
            root,
            left_walk,
            right_walk,
        }
    }
}

#[gat]
impl<'lt, N, E, F> LendingIterator for PostorderWalk<'lt, N, E, F>
where 
    N: BinarySearchTreeNode<Wrapper = N, Edge = E>,
    E: DerefMut<Target = N>,
    F: Fn(&N) -> WalkInstruction,
{
    type Item<'next>
    where 
        Self: 'next,
        = &'next mut N;

    fn next(self: &'_ mut PostorderWalk<'lt, N, E, F>) -> Option<&'_ mut N> {
        if let Some(left_walk) = self.left_walk.as_mut() && let Some(next) = left_walk.next() {
            return Some(next);
        }
        if let Some(right_walk) = self.right_walk.as_mut() && let Some(next) = right_walk.next() {
            return Some(next);
        }
        if !self.completed {
            self.completed = true;
            Some(self.root)
        } else { None }
    }
}

pub struct PostorderEdgeWalk<'lt, N, E, F> {
    completed: bool,
    frontier_stack: Vec<E>, // Edges that we have encountered, but not traversed yet.
    visited_stack: DFSStack<E>, // Edges that we have encountered and traversed, but not reported yet.
    instruction_fn: F,
    _node_ref: PhantomData<&'lt N>,
}

impl<'lt, N, E, F> PostorderEdgeWalk<'lt, N, E, F>
where 
    N: BinarySearchTreeNode<Wrapper = N, Edge = E>,
    E: DerefMut<Target = N>,
    F: Fn(&N) -> WalkInstruction,
{
    pub fn new(root_edge: E, instruction_fn: F) -> Self {
        Self {
            completed: false,
            frontier_stack: vec![root_edge],
            visited_stack: DFSStack::new(),
            instruction_fn,
            _node_ref: PhantomData,
        }
    }
}

#[gat]
impl<'lt, N, E, F> LendingIterator for PostorderEdgeWalk<'lt, N, E, F>
where 
    N: BinarySearchTreeNode<Wrapper = N, Edge = E>,
    E: DerefMut<Target = N>,
    F: Fn(&N) -> WalkInstruction,
{
    type Item<'next>
    where 
        Self: 'next,
        = &'next mut E;

    fn next(self: &'_ mut PostorderEdgeWalk<'lt, N, E, F>) -> Option<&'_ mut E> {
        if self.completed { return None; }

        // Report fully expanded edges first
        if let Some(edge) = self.visited_stack.pop_if_expanded() {
            if let Some((mut parent, mut sides)) = self.visited_stack.pop() {
                let side = sides.pop().unwrap(); // Can unwrap safely: parent is not fully expanded.
                parent.attach_child(side, edge);
                self.visited_stack.push(parent, sides);

                return self.visited_stack.last_edge_mut().and_then(move |parent| parent.get_edge_mut(side));
            } else {
                // Edge is root. Put back on frontier_stack to be able to return a reference to it that lives long enough.
                self.frontier_stack.push(edge);
                self.completed = true;
                return self.frontier_stack.last_mut();
            }
        }

        let next_edge = loop {
            let mut edge = self.frontier_stack.pop()?;

            match (self.instruction_fn)(&edge) {
                WalkInstruction::Left => {
                    if let Some(left) = edge.detach_left() {
                        self.frontier_stack.push(left);
                        self.visited_stack.push(edge, vec![Side::Left]);
                    } else { break edge }
                },
                WalkInstruction::Right => {
                    if let Some(right) = edge.detach_right() {
                        self.frontier_stack.push(right);
                        self.visited_stack.push(edge, vec![Side::Right]);
                    } else { break edge }
                },
                WalkInstruction::Both => {
                    match (edge.detach_left(), edge.detach_right()) {
                        (Some(left), Some(right)) => {
                            self.frontier_stack.push(right);
                            self.frontier_stack.push(left);
                            self.visited_stack.push(edge, vec![Side::Right, Side::Left]);
                        },
                        (Some(left), _) => {
                            self.frontier_stack.push(left);
                            self.visited_stack.push(edge, vec![Side::Left]);
                        },
                        (_, Some(right)) => {
                            self.frontier_stack.push(right);
                            self.visited_stack.push(edge, vec![Side::Right]);
                        },
                        _ => break edge,
                    };
                },
                WalkInstruction::None => break edge,
            }
        };

        // Re-attach the edge to its incident parent before reporting it.
        if let Some((mut parent, mut sides)) = self.visited_stack.pop() {
            let side = sides.pop().unwrap(); // Can unwrap safely: parent is not fully expanded.
            parent.attach_child(side, next_edge);
            self.visited_stack.push(parent, sides);

            self.visited_stack.last_edge_mut().and_then(move |parent| parent.get_edge_mut(side))
        } else {
            // Edge is root. Put back on frontier_stack to be able to return a reference to it that lives long enough.
            self.frontier_stack.push(next_edge);
            self.completed = true;
            self.frontier_stack.last_mut()
        }
    }
}
