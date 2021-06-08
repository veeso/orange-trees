//! # lemon-tree
//!
//! [lemon-tree](https://github.com/veeso/lemon-tree) is a [tui-realm](https://github.com/veeso/tui-realm) implementation
//! of a treeview component
//!
//! ## Get Started
//!
//! ### Adding `lemon-tree` as dependency
//!
//! ```toml
//! lemon-tree = "0.1.0"
//! ```
//!
//! ## Setup a tree component
//!
//! ```rust,no_run
//! extern crate lemon_tree;
//!
//! ```
//!

#![doc(html_playground_url = "https://play.rust-lang.org")]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/veeso/lemon-tree/main/docs/images/cargo/lemon-tree-128.png"
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/veeso/lemon-tree/main/docs/images/cargo/lemon-tree-512.png"
)]

/**
 * MIT License
 *
 * lemon-tree - Copyright (C) 2021 Christian Visintin
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
// deps
use std::cmp::Ordering;
use std::slice::{Iter, IterMut};

/// ## Tree
///
/// represent the tree data structure inside the component.
/// U: is the type for the node indentifier (must implement PartialEq)
/// T: is the type for the node value
#[derive(Debug)]
pub struct Tree<U, T> {
    root: Node<U, T>,
}

impl<U: PartialEq, T> Tree<U, T> {
    /// ### new
    ///
    /// Instantiates a new `Tree`
    pub fn new(root: Node<U, T>) -> Self {
        Self { root }
    }

    /// ### root
    ///
    /// Returns a reference to the root node
    pub fn root(&self) -> &Node<U, T> {
        &self.root
    }

    /// ### root_mut
    ///
    /// Returns a mutablen reference to the root node
    pub fn root_mut(&mut self) -> &mut Node<U, T> {
        &mut self.root
    }

    /// ### query
    ///
    /// Query tree for a certain node
    pub fn query(&self, id: &U) -> Option<&Node<U, T>> {
        self.root.query(id)
    }

    /// ### query_mut
    ///
    /// Query tree for a certain node and return it as a mutable reference
    pub fn query_mut(&mut self, id: &U) -> Option<&mut Node<U, T>> {
        self.root.query_mut(id)
    }

    /// ### parent
    ///
    /// Get parent node of `id`
    pub fn parent(&self, id: &U) -> Option<&Node<U, T>> {
        self.root().parent(id)
    }

    /// ### siblings
    ///
    /// Get siblings for provided node
    pub fn siblings(&self, id: &U) -> Option<Vec<&U>> {
        self.root().siblings(id)
    }

    /// ### node_by_route
    ///
    /// Get starting from root the node associated to the indexes.
    /// When starting from tree, the first element in route must be `0`
    pub fn node_by_route(&self, route: &[usize]) -> Option<&Node<U, T>> {
        if route.is_empty() {
            None
        } else {
            self.root().node_by_route(&route[1..])
        }
    }

    /// ### route_by_node
    ///
    /// Calculate the route of a node by its id
    pub fn route_by_node(&self, id: &U) -> Option<Vec<usize>> {
        match self.root().route_by_node(id) {
            None => None,
            Some(route) => {
                let mut r: Vec<usize> = vec![0];
                r.extend(route);
                Some(r)
            }
        }
    }
}

/// ## Node
///
/// Describes a node inside the `Tree`
/// U: is the type for the node indentifier (must implement PartialEq)
/// T: is the type for the node value
#[derive(Debug)]
pub struct Node<U, T> {
    id: U,    // Must uniquely identify the node in the tree
    value: T, // The node value
    children: Vec<Node<U, T>>,
}

impl<U: PartialEq, T> Node<U, T> {
    /// ### new
    ///
    /// Instantiates a new `Node`.
    /// In order to use query methods the ID should be unique for each node in the tree
    pub fn new(id: U, value: T) -> Self {
        Self {
            id,
            value,
            children: vec![],
        }
    }

    /// ### id
    ///
    /// Get reference to id
    pub fn id(&self) -> &U {
        &self.id
    }

    /// ### value
    ///
    /// Get reference to node value
    pub fn value(&self) -> &T {
        &self.value
    }

    /// ### children
    ///
    /// Returns a reference to the node's children
    pub fn children(&self) -> &[Node<U, T>] {
        self.children.as_slice()
    }

    /// ### iter
    ///
    /// Returns an iterator over node's children
    pub fn iter(&self) -> Iter<'_, Node<U, T>> {
        self.children.iter()
    }

    /// ### iter_mut
    ///
    /// Returns a mutable iterator over node's children
    pub fn iter_mut(&mut self) -> IterMut<'_, Node<U, T>> {
        self.children.iter_mut()
    }

    /// ### with_children
    ///
    /// Sets Node children
    pub fn with_children(mut self, children: Vec<Node<U, T>>) -> Self {
        self.children = children;
        self
    }

    /// ### with_child
    ///
    /// Create a new child in this Node
    pub fn with_child(mut self, child: Node<U, T>) -> Self {
        self.add_child(child);
        self
    }

    // -- manipulation

    /// ### add_child
    ///
    /// Add a child to the node
    pub fn add_child(&mut self, child: Node<U, T>) {
        self.children.push(child);
    }

    /// ### remove_child
    ///
    /// Remove child from node
    pub fn remove_child(&mut self, id: &U) {
        self.children.retain(|x| x.id() != id);
    }

    /// ### clear
    ///
    /// Clear node children
    pub fn clear(&mut self) {
        self.children.clear();
    }

    /// ### truncate
    ///
    /// Truncate tree at depth.
    /// If depth is `0`, node's children will be cleared
    pub fn truncate(&mut self, depth: usize) {
        if depth == 0 {
            self.children.clear();
        } else {
            self.children.iter_mut().for_each(|x| x.truncate(depth - 1));
        }
    }

    /// ### sort
    ///
    /// Sort node children by predicate
    pub fn sort<F>(&mut self, compare: F)
    where
        F: FnMut(&Node<U, T>, &Node<U, T>) -> Ordering,
    {
        self.children.sort_by(compare);
    }

    // -- query

    /// ### is_leaf
    ///
    /// Returns whether this node is a leaf (which means it has no children)
    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    /// ### query
    ///
    /// Search for `id` inside Node's children (or is itself)
    pub fn query(&self, id: &U) -> Option<&Self> {
        if self.id() == id {
            Some(&self)
        } else {
            // Recurse search
            self.children
                .iter()
                .map(|x| x.query(id))
                .filter(|x| x.is_some())
                .flatten()
                .next()
        }
    }

    /// ### query_mut
    ///
    /// Returns a mutable reference to a Node
    pub fn query_mut(&mut self, id: &U) -> Option<&mut Self> {
        if self.id() == id {
            Some(self)
        } else {
            // Recurse search
            self.children
                .iter_mut()
                .map(|x| x.query_mut(id))
                .filter(|x| x.is_some())
                .flatten()
                .next()
        }
    }

    /// ### count
    ///
    /// Count items in tree
    pub fn count(&self) -> usize {
        self.children.iter().map(|x| x.count()).sum::<usize>() + 1
    }

    /// ### depth
    ///
    /// Calculate the maximum depth of the tree
    pub fn depth(&self) -> usize {
        /// ### depth_r
        ///
        /// Private recursive call for depth
        fn depth_r<U, T>(ptr: &Node<U, T>, depth: usize) -> usize {
            ptr.children
                .iter()
                .map(|x| depth_r(x, depth + 1))
                .max()
                .unwrap_or(depth)
        }
        depth_r(self, 1)
    }

    /// ### parent
    ///
    /// Get parent node of `id`
    pub fn parent(&self, id: &U) -> Option<&Self> {
        match self.route_by_node(id) {
            None => None,
            Some(route) => {
                // Get parent
                if route.is_empty() {
                    None
                } else {
                    self.node_by_route(&route[0..route.len() - 1])
                }
            }
        }
    }

    /// ### siblings
    ///
    /// Get siblings for provided node
    pub fn siblings(&self, id: &U) -> Option<Vec<&U>> {
        self.parent(id).map(|x| {
            x.children
                .iter()
                .filter(|&x| x.id() != id)
                .map(|x| x.id())
                .collect()
        })
    }

    /// ### node_by_route
    ///
    /// Given a vector of indexes, returns the node associated to the route
    pub fn node_by_route(&self, route: &[usize]) -> Option<&Self> {
        if route.is_empty() {
            Some(self)
        } else {
            let next: &Node<U, T> = self.children.get(route[0])?;
            let route = &route[1..];
            next.node_by_route(route)
        }
    }

    /// ### route_by_node
    ///
    /// Calculate the route of a node by its id
    pub fn route_by_node(&self, id: &U) -> Option<Vec<usize>> {
        // Recursive function
        fn route_by_node_r<U: PartialEq, T>(
            node: &Node<U, T>,
            id: &U,
            enumerator: Option<usize>,
            mut route: Vec<usize>,
        ) -> Option<Vec<usize>> {
            if let Some(enumerator) = enumerator {
                route.push(enumerator);
            }
            if node.id() == id {
                // Found!!!
                Some(route)
            } else if node.children.is_empty() {
                // No more children
                route.pop(); // Pop previous entry
                None
            } else {
                // Keep searching
                let mut result: Option<Vec<usize>> = None;
                node.children.iter().enumerate().for_each(|(i, x)| {
                    let this_route: Vec<usize> = route.clone();
                    if let Some(this_route) = route_by_node_r(x, id, Some(i), this_route) {
                        result = Some(this_route);
                    }
                });
                result
            }
        }
        // Call recursive function
        route_by_node_r(self, id, None, Vec::with_capacity(self.depth()))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_tree() {
        // -- Build
        let mut tree: Tree<String, &str> = Tree::new(
            Node::new("/".to_string(), "/")
                .with_child(
                    Node::new("/bin".to_string(), "bin/")
                        .with_child(Node::new("/bin/ls".to_string(), "ls"))
                        .with_child(Node::new("/bin/pwd".to_string(), "pwd")),
                )
                .with_child(
                    Node::new("/home".to_string(), "home/").with_child(
                        Node::new("/home/omar".to_string(), "omar/")
                            .with_child(Node::new("/home/omar/readme.md".to_string(), "readme.md"))
                            .with_child(Node::new(
                                "/home/omar/changelog.md".to_string(),
                                "changelog.md",
                            )),
                    ),
                ),
        );
        let root: &Node<String, &str> = tree.root();
        assert_eq!(root.id(), "/");
        assert_eq!(root.value(), &"/");
        assert_eq!(root.children.len(), 2);
        let bin: &Node<String, &str> = &root.children[0];
        assert_eq!(bin.id(), "/bin");
        assert_eq!(bin.value(), &"bin/");
        assert_eq!(bin.children.len(), 2);
        let bin_ids: Vec<&String> = bin.children.iter().map(|x| x.id()).collect();
        assert_eq!(bin_ids, vec!["/bin/ls", "/bin/pwd"]);
        let home: &Node<String, &str> = &tree.root.children[1];
        assert_eq!(home.id(), "/home");
        assert_eq!(home.value(), &"home/");
        assert_eq!(home.children.len(), 1);
        let omar_home: &Node<String, &str> = &home.children[0];
        let omar_home_ids: Vec<&String> = omar_home.children.iter().map(|x| x.id()).collect();
        assert_eq!(
            omar_home_ids,
            vec!["/home/omar/readme.md", "/home/omar/changelog.md"]
        );
        // count
        assert_eq!(root.count(), 8);
        // depth
        assert_eq!(root.depth(), 4);
        // Children
        assert_eq!(root.children().len(), 2);
        assert_eq!(root.iter().count(), 2);
        // -- Query
        assert_eq!(
            tree.query(&"/home/omar/changelog.md".to_string())
                .unwrap()
                .id(),
            "/home/omar/changelog.md"
        );
        assert!(tree.query(&"ommlar".to_string()).is_none());
        // is leaf
        assert_eq!(
            tree.query(&"/home/omar".to_string()).unwrap().is_leaf(),
            false
        );
        assert_eq!(
            tree.query(&"/home/omar/changelog.md".to_string())
                .unwrap()
                .is_leaf(),
            true
        );
        // parent
        assert!(tree.parent(&"/".to_string()).is_none());
        assert_eq!(
            tree.parent(&"/home/omar/changelog.md".to_string())
                .unwrap()
                .id(),
            "/home/omar"
        );
        assert!(tree.parent(&"/homer".to_string()).is_none());
        // siblings
        assert_eq!(
            tree.siblings(&"/home/omar/changelog.md".to_string())
                .unwrap(),
            vec!["/home/omar/readme.md"]
        );
        assert_eq!(tree.siblings(&"/home/omar".to_string()).unwrap().len(), 0);
        assert!(tree.siblings(&"/homer".to_string()).is_none());
        // Mutable
        let root: &mut Node<String, &str> = tree.root_mut();
        assert_eq!(root.iter_mut().count(), 2);
        // Push node
        tree.query_mut(&"/home/omar".to_string())
            .unwrap()
            .add_child(Node::new("/home/omar/Cargo.toml".to_string(), "Cargo.toml"));
        assert_eq!(
            tree.query(&"/home/omar/Cargo.toml".to_string())
                .unwrap()
                .id(),
            "/home/omar/Cargo.toml"
        );
        // Remove
        tree.query_mut(&"/home/omar".to_string())
            .unwrap()
            .add_child(Node::new("/home/omar/Cargo.lock".to_string(), "Cargo.lock"));
        assert_eq!(
            tree.query(&"/home/omar/Cargo.lock".to_string())
                .unwrap()
                .id(),
            "/home/omar/Cargo.lock"
        );
        tree.query_mut(&"/home/omar".to_string())
            .unwrap()
            .remove_child(&String::from("/home/omar/Cargo.lock"));
        assert!(tree.query(&"/home/omar/Cargo.lock".to_string()).is_none());
        // -- node_by_route
        assert_eq!(
            tree.node_by_route(&[0, 1, 0, 1]).unwrap().id(),
            "/home/omar/changelog.md"
        );
        assert_eq!(
            tree.root().node_by_route(&[1, 0, 1]).unwrap().id(),
            "/home/omar/changelog.md"
        );
        assert!(tree.root().node_by_route(&[1, 0, 3]).is_none());
        // -- Route by node
        assert_eq!(
            tree.route_by_node(&"/home/omar/changelog.md".to_string())
                .unwrap(),
            vec![0, 1, 0, 1]
        );
        assert_eq!(
            tree.root()
                .route_by_node(&"/home/omar/changelog.md".to_string())
                .unwrap(),
            vec![1, 0, 1]
        );
        assert!(tree
            .root()
            .route_by_node(&"ciccio-pasticcio".to_string())
            .is_none());
        // Clear node
        tree.query_mut(&"/home/omar".to_string()).unwrap().clear();
        assert_eq!(
            tree.query(&"/home/omar".to_string())
                .unwrap()
                .children
                .len(),
            0
        );
        // -- With children
        let tree: Tree<String, &str> =
            Tree::new(Node::new("a".to_string(), "a").with_children(vec![
                Node::new("a1".to_string(), "a1"),
                Node::new("a2".to_string(), "a2"),
            ]));
        assert!(tree.query(&"a".to_string()).is_some());
        assert!(tree.query(&"a1".to_string()).is_some());
        assert!(tree.query(&"a2".to_string()).is_some());
        // -- truncate
        let mut tree: Tree<String, &str> = Tree::new(
            Node::new("/".to_string(), "/")
                .with_child(
                    Node::new("/bin".to_string(), "bin/")
                        .with_child(Node::new("/bin/ls".to_string(), "ls"))
                        .with_child(Node::new("/bin/pwd".to_string(), "pwd")),
                )
                .with_child(
                    Node::new("/home".to_string(), "home/").with_child(
                        Node::new("/home/omar".to_string(), "omar/")
                            .with_child(Node::new("/home/omar/readme.md".to_string(), "readme.md"))
                            .with_child(Node::new(
                                "/home/omar/changelog.md".to_string(),
                                "changelog.md",
                            )),
                    ),
                ),
        );
        let root: &mut Node<String, &str> = &mut tree.root;
        root.truncate(1);
        assert_eq!(root.children.len(), 2);
        assert_eq!(root.children[0].children.len(), 0);
        assert_eq!(root.children[0].id(), "/bin");
        assert_eq!(root.children[1].children.len(), 0);
        assert_eq!(root.children[1].id(), "/home");
        // Sort
        let mut tree: Tree<&'static str, usize> = Tree::new(
            Node::new("/", 0)
                .with_child(Node::new("8", 8))
                .with_child(Node::new("7", 7))
                .with_child(Node::new("3", 3))
                .with_child(Node::new("1", 1))
                .with_child(Node::new("2", 2))
                .with_child(Node::new("9", 9))
                .with_child(Node::new("5", 5))
                .with_child(Node::new("4", 4))
                .with_child(Node::new("6", 6)),
        );
        tree.root_mut()
            .sort(|a, b| a.value().partial_cmp(b.value()).unwrap());
        let values: Vec<usize> = tree.root().iter().map(|x| *x.value()).collect();
        assert_eq!(values, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
