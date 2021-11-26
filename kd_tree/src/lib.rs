#![no_std]
#![allow(clippy::too_many_arguments)]
use core::cmp::Ordering;
use core::fmt::Debug;
use num::traits::Num;

pub struct Tree<'a, T, V, const DIM: usize, const MAX_LEVEL: usize> {
    pub nodes: &'a [Node<T, V, DIM>],
}

impl<T, V, const DIM: usize, const MAX_LEVEL: usize> Tree<'static, T, V, DIM, MAX_LEVEL> {
    pub const fn new(nodes: &'static [Node<T, V, DIM>]) -> Tree<'static, T, V, DIM, MAX_LEVEL> {
        Tree { nodes }
    }
}

impl<'a, T: PartialOrd + Num + Copy + Debug, V, const DIM: usize, const MAX_LEVEL: usize>
    Tree<'a, T, V, DIM, MAX_LEVEL>
{
    pub fn search(&self, point: &[T; DIM]) -> &Node<T, V, DIM> {
        let mut node: &Node<T, V, DIM> = self
            .nodes
            .first()
            .expect("This can not fail, expect if the Tree got no nodes: SIZE == 0");
        let mut index = 0usize;
        let mut level = 0usize;
        // Store which child nodes where visited for the Node on the index/level and if it was
        // already compared
        let mut visited: [(Visited, bool); MAX_LEVEL] = [(Visited::None, false); MAX_LEVEL];
        //Initialise best node/distance with root node
        let mut best_distance: T = euclid(&node.val, point);
        let mut best_node: &Node<T, V, DIM> = node;

        loop {
            // Get to leaf based on comparison of only a single dimension per level
            self.search_down(point, &mut node, &mut index, &mut level, &mut visited);

            // Go up until we either reach the top or we go down by one
            // Should we go down by one, we will need to go to the best fit leaf of the current
            // subtree
            self.search_up(
                point,
                &mut node,
                &mut index,
                &mut level,
                &mut visited,
                &mut best_distance,
                &mut best_node,
            );

            // Should we have reached level 0, we are finished
            // as search_up should go down by one should we still need to search a subtree
            if level == 0 {
                break;
            }
        }

        best_node
    }

    fn search_down(
        &'a self,
        point: &[T; DIM],
        node: &mut &'a Node<T, V, DIM>,
        index: &mut usize,
        level: &mut usize,
        visited: &mut [(Visited, bool); MAX_LEVEL],
    ) {
        //Get to leaf node
        loop {
            //Reset Visited and calculated distance for current level
            visited[*level] = (Visited::None, false);

            let dim = *level % DIM;

            //If the left node is not reachable we are at a leaf node
            if left(index) >= self.nodes.len() {
                //Set visited for this level to ALL because there are no more child nodes
                visited[*level].0 = Visited::All;
                return;
            }
            //Decide where to go
            *index = match point[dim].partial_cmp(&node.val[dim]) {
                Some(Ordering::Equal) | Some(Ordering::Less) => left(index),
                Some(Ordering::Greater) => right(index),
                _ => panic!(),
            };

            // If the index is to big we choose the right node
            // Make sure it exists, if not go to the left node instead
            if *index >= self.nodes.len() {
                *index -= 1;
            }

            // Set next node
            *node = self.nodes.get(*index).unwrap();

            // Increase level for next node
            *level += 1;
        }
    }

    fn search_up(
        &'a self,
        point: &[T; DIM],
        node: &mut &'a Node<T, V, DIM>,
        index: &mut usize,
        level: &mut usize,
        visited: &mut [(Visited, bool); MAX_LEVEL],
        best_distance: &mut T,
        best_node: &mut &'a Node<T, V, DIM>,
    ) {
        loop {
            let dim = *level % DIM;

            // Check if current node is a closer node
            if !visited[*level].1 {
                let candidate: T = euclid(&node.val, point);
                if candidate < *best_distance {
                    *best_distance = candidate;
                    *best_node = node;
                }
                visited[*level].1 = true;
            }

            // Determine where to go? Up, BottomLeft or BottomRight
            let dir: Direction = match visited[*level].0 {
                Visited::All => Direction::Up,
                Visited::Left => {
                    // Check if we even can go right
                    // and if its even possible for the right side to be nearer
                    let single_distance = point[dim] - node.val[dim];
                    let single_distance = single_distance * single_distance;
                    if right(index) < self.nodes.len() && single_distance < *best_distance {
                        Direction::Right
                    } else {
                        Direction::Up
                    }
                }
                Visited::Right => {
                    // Check if we even can go left
                    // and if its even possible for the left side to be nearer
                    let single_distance = point[dim] - node.val[dim];
                    let single_distance = single_distance * single_distance;
                    if left(index) < self.nodes.len() && single_distance < *best_distance {
                        Direction::Left
                    } else {
                        Direction::Up
                    }
                }
                _ => panic!("Unexpected state"),
            };

            match dir {
                Direction::Up => {
                    //Stop if we are at level 0
                    if *level == 0 {
                        return;
                    }

                    // Move up
                    let parent_index = parent(index);
                    *level -= 1;
                    // But update the visited status of the parent first
                    // This way we know which children were already visited
                    match visited[*level].0 {
                        Visited::Left | Visited::Right => visited[*level].0 = Visited::All,
                        _ => {
                            if left(&parent_index) == *index {
                                visited[*level].0 = Visited::Left;
                            } else {
                                visited[*level].0 = Visited::Right;
                            }
                        }
                    }
                    *index = parent_index;
                    *node = self.nodes.get(*index).unwrap();
                }
                Direction::Left => {
                    *level += 1;
                    visited[*level].0 = Visited::None;
                    *index = left(index);
                    *node = self.nodes.get(*index).unwrap();
                    return;
                }
                Direction::Right => {
                    *level += 1;
                    visited[*level].0 = Visited::None;
                    *index = right(index);
                    *node = self.nodes.get(*index).unwrap();
                    return;
                }
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Visited {
    None,
    Left,
    Right,
    All,
}

enum Direction {
    Up,
    Left,
    Right,
}

pub fn parent(cur_index: &usize) -> usize {
    (cur_index + 1) / 2 - 1
}

pub fn left(cur_index: &usize) -> usize {
    (cur_index + 1) * 2 - 1
}

pub fn right(cur_index: &usize) -> usize {
    (cur_index + 1) * 2
}

#[derive(Debug)]
pub struct Node<T: Sized, V: Sized, const DIM: usize> {
    val: [T; DIM],
    v: V,
}

impl<T, V, const DIM: usize> Node<T, V, DIM> {
    pub fn val(&self) -> &[T; DIM] {
        &self.val
    }

    pub const fn new(val: [T; DIM], v: V) -> Node<T, V, DIM> {
        Node { val, v }
    }

    pub fn payload(&self) -> &V {
        &self.v
    }
}

pub fn euclid<T, const SIZE: usize>(left: &[T; SIZE], right: &[T; SIZE]) -> T
where
    T: Num + Copy + Debug,
{
    left.iter()
        .zip(right.iter())
        .map(|(x, y)| ((*x) - (*y)) * ((*x) - (*y)))
        .fold(T::zero(), ::core::ops::Add::add)
}
