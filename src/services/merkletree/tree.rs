extern crate ring;
extern crate rustc_serialize;

use self::ring::digest::{ Algorithm, Digest };
use self::rustc_serialize::{ Encodable, Encoder, Decodable, Decoder, json };

use services::merkletree::hashutils::{ Hashable, HashUtils };

pub use services::merkletree::proof::{
    Proof,
    Lemma,
    Positioned
};

/// Binary Tree where leaves hold a stand-alone value.
#[derive(Clone, Debug, PartialEq)]
pub enum Tree<T> {
    Empty {
        hash: Vec<u8>
    },

    Leaf {
        hash: Vec<u8>,
        value: T
    },

    Node {
        hash: Vec<u8>,
        left: Box<Tree<T>>,
        right: Box<Tree<T>>
    }
}

impl <T> Tree<T> {
    /// Create an empty tree
    pub fn empty(hash: Digest) -> Self {
        Tree::Empty {
            hash: hash.as_ref().into()
        }
    }

    /// Create a new tree
    pub fn new(hash: Digest, value: T) -> Self {
        Tree::Leaf {
            hash: hash.as_ref().into(),
            value: value
        }
    }

    /// Create a new leaf
    pub fn new_leaf(algo: &'static Algorithm, value: T) -> Tree<T>
            where T: Hashable {

        let hash = algo.hash_leaf(&value);
        Tree::new(hash, value)
    }

    /// Returns a hash from the tree.
    pub fn hash(&self) -> &Vec<u8> {
        match *self {
            Tree::Empty { ref hash }    => hash,
            Tree::Leaf { ref hash, .. } => hash,
            Tree::Node { ref hash, .. } => hash
        }
    }

    /// Returns a borrowing iterator over the leaves of the tree.
    pub fn iter(&self) -> LeavesIterator<T> {
        LeavesIterator::new(self)
    }
}

impl<T: AsRef<[u8]> + Encodable> Encodable for Tree<T> {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        match *self {
            Tree::Empty { .. } => {
                s.emit_nil()
            }
            Tree::Node { ref hash, ref left, ref right, .. } => {
                s.emit_struct("node", 3, |s| {
                    try!(s.emit_struct_field("hash", 0, |s| {
                        hash.encode(s)
                    }));
                    try!(s.emit_struct_field("left", 1, |s| {
                        left.encode(s)
                    }));
                    try!(s.emit_struct_field("right", 2, |s| {
                        right.encode(s)
                    }));
                    Ok(())
                })
            }
            Tree::Leaf { ref hash, ref value, .. } => {
                s.emit_struct("leaf", 2, |s| {
                    try!(s.emit_struct_field("hash", 0, |s| {
                        hash.encode(s)
                    }));
                    try!(s.emit_struct_field("value", 1, |s| {
                        value.encode(s)
                    }));
                    Ok(())
                })
            }
        }
    }
}

impl<T: AsRef<[u8]> + Decodable> Decodable for Tree<T> {
    fn decode<D: Decoder>(d: &mut D) -> Result<Tree<T>, D::Error> {
        d.read_struct("node", 3, |d| {
            let hash = try!(d.read_struct_field("hash", 0, |d| { Vec::<u8>::decode(d) }));
            let value = d.read_struct_field("value", 1, |d| { T::decode(d) });
            match value {
                Ok(val) => {
                    Ok(Tree::Leaf{
                        hash: hash,
                        value: val
                    })
                }
                Err(e) => {
                    let left = try!(d.read_struct_field("left", 1, |d| { Tree::<T>::decode(d) }));
                    let right = try!(d.read_struct_field("right", 2, |d| { Tree::<T>::decode(d) }));
                    Ok(Tree::Node{
                        hash: hash,
                        left: Box::new(left),
                        right: Box::new(right)
                    })
                }
            }
        })
    }
}

/// An borrowing iterator over the leaves of a `Tree`.
/// Adapted from http://codereview.stackexchange.com/q/110283.
#[allow(missing_debug_implementations)]
pub struct LeavesIterator<'a, T> where T: 'a {
    current_value: Option<&'a T>,
    right_nodes: Vec<&'a Tree<T>>
}

impl <'a, T> LeavesIterator<'a, T> {

    fn new(root: &'a Tree<T>) -> Self {
        let mut iter = LeavesIterator {
            current_value: None,
            right_nodes: Vec::new()
        };

        iter.add_left(root);

        iter
    }

    fn add_left(&mut self, mut tree: &'a Tree<T>) {
        loop {
            match *tree {
                Tree::Empty { .. } => {
                    self.current_value = None;
                    break;
                },

                Tree::Node { ref left, ref right, .. } => {
                    self.right_nodes.push(right);
                    tree = left;
                },

                Tree::Leaf { ref value, .. } => {
                    self.current_value = Some(value);
                    break;
                }
            }
        }
    }

}

impl <'a, T> Iterator for LeavesIterator<'a, T> {

    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        let result = self.current_value.take();

        if let Some(rest) = self.right_nodes.pop() {
            self.add_left(rest);
        }

        result
    }

}

/// An iterator over the leaves of a `Tree`.
#[allow(missing_debug_implementations)]
pub struct LeavesIntoIterator<T> {
    current_value: Option<T>,
    right_nodes: Vec<Tree<T>>
}

impl <T> LeavesIntoIterator<T> {

    fn new(root: Tree<T>) -> Self {
        let mut iter = LeavesIntoIterator {
            current_value: None,
            right_nodes: Vec::new()
        };

        iter.add_left(root);

        iter
    }

    fn add_left(&mut self, mut tree: Tree<T>) {
        loop {
            match tree {
                Tree::Empty { .. } => {
                    self.current_value = None;
                    break;
                },

                Tree::Node { left, right, .. } => {
                    self.right_nodes.push(*right);
                    tree = *left;
                },

                Tree::Leaf { value, .. } => {
                    self.current_value = Some(value);
                    break;
                }
            }
        }
    }

}

impl <T> Iterator for LeavesIntoIterator<T> {

    type Item = T;

    fn next(&mut self) -> Option<T> {
        let result = self.current_value.take();

        if let Some(rest) = self.right_nodes.pop() {
            self.add_left(rest);
        }

        result
    }

}

impl <T> IntoIterator for Tree<T> {

    type Item     = T;
    type IntoIter = LeavesIntoIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        LeavesIntoIterator::new(self)
    }

}