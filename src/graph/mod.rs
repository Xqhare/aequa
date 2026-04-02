use crate::XffValue;

mod connection;
mod node;

pub use connection::*;
pub use node::*;

// TODO: Before stabilising inside XffValue, remove Error and move to returning None instead. No
// other XffValue errors like this.
#[derive(Debug, PartialEq, Eq)]
pub enum GraphError {
    /// Attempted to create a connection from or to a node that does not exist.
    NodeNotFound(u32),
    /// Attempted to remove the root node (index 0).
    CannotRemoveRoot,
}

impl std::fmt::Display for GraphError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GraphError::NodeNotFound(idx) => write!(f, "Node at index {} not found", idx),
            GraphError::CannotRemoveRoot => write!(f, "The root node (index 0) cannot be removed"),
        }
    }
}

/// The core container for the entire graph.
///
/// ### Performance and Usage Notes:
/// - **Zero Shifting**: Uses a "Free List" (Slot Map) approach to ensure stable indices. Removing elements
///   marks slots as empty for reuse rather than shifting the vector.
/// - **Serialization Focused**: Designed primarily as a data serialization structure. It is highly efficient
///   for in-place reads and mutations, but performance for massive deletion/churn is non-optimal.
/// - **Scale Limits**: Tracking inbound/outbound connections uses `Vec` ($O(\text{degree})$ removal).
///   Performance may degrade for "Super Nodes" with thousands of connections.
/// - **Memory**: Vectors do not shrink automatically. High churn (millions of adds/removes) will maintain
///   memory usage at the "high-water mark."
/// - **Index Recycling**: Indices are reused. Document that an index (e.g., `5`) might point to "Node A"
///   initially, but if deleted, the next `add_node` may assign index `5` to "Node B."
pub struct Graph {
    /// Index 0 is the root / entry node by convention.
    nodes: Vec<Option<GraphNode>>,
    /// All directional links between nodes.
    connections: Vec<Option<GraphConnection>>,
    /// Indices of nodes that have been removed and are available for reuse.
    free_nodes: Vec<u32>,
    /// Indices of connections that have been removed and are available for reuse.
    free_connections: Vec<u32>,
}

impl Default for Graph {
    fn default() -> Self {
        Graph {
            nodes: Vec::new(),
            connections: Vec::new(),
            free_nodes: Vec::new(),
            free_connections: Vec::new(),
        }
    }
}

impl Graph {
    /// Creates a new, empty graph.
    ///
    /// # Example
    /// ```rust
    /// use aequa::graph::Graph;
    /// let graph = Graph::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a new node to the graph and returns its unique index.
    ///
    /// **Convention**: The first node added (which will receive index 0) is considered
    /// the "Root" or "Entry" node of the graph and cannot be removed.
    ///
    /// # Example
    /// ```rust
    /// use aequa::{graph::Graph, XffValue};
    /// let mut graph = Graph::new();
    /// let idx = graph.add_node(XffValue::from("root"), XffValue::Null);
    /// assert_eq!(idx, 0);
    /// ```
    pub fn add_node(&mut self, payload: XffValue, metadata: XffValue) -> u32 {
        let node = GraphNode::new(payload, metadata);
        if let Some(index) = self.free_nodes.pop() {
            self.nodes[index as usize] = Some(node);
            index
        } else {
            let index = self.nodes.len() as u32;
            self.nodes.push(Some(node));
            index
        }
    }

    /// Removes a node from the graph and all its associated connections.
    ///
    /// - **Root Protection**: Attempting to remove index 0 will do nothing.
    /// - **Stability**: Indices of other nodes remain stable.
    ///
    /// # Example
    /// ```rust
    /// use aequa::{graph::Graph, XffValue};
    /// let mut graph = Graph::new();
    /// let n0 = graph.add_node(XffValue::from("root"), XffValue::Null);
    /// let n1 = graph.add_node(XffValue::from("node1"), XffValue::Null);
    /// graph.remove_node(n1);
    /// assert!(!graph.has_node(n1));
    /// ```
    pub fn remove_node(&mut self, index: u32) {
        if index == 0 {
            // Index 0 is the root by convention and cannot be removed.
            return;
        }

        if index as usize >= self.nodes.len() {
            return;
        }

        let node = match self.nodes[index as usize].take() {
            Some(n) => n,
            None => return,
        };

        self.free_nodes.push(index);

        // Cleanup: remove all connections involving this node.
        let inbounds = node.inbound_connections.clone();
        let outbounds = node.outbound_connections.clone();

        for conn_idx in inbounds {
            self.remove_connection(conn_idx);
        }
        for conn_idx in outbounds {
            self.remove_connection(conn_idx);
        }
    }

    /// Adds a connection between two nodes and returns its unique index.
    ///
    /// # Errors
    /// Returns `GraphError::NodeNotFound` if either the `from` or `to` node does not exist.
    ///
    /// # Example
    /// ```rust
    /// use aequa::{graph::Graph, XffValue};
    /// let mut graph = Graph::new();
    /// let n0 = graph.add_node(XffValue::from("n0"), XffValue::Null);
    /// let n1 = graph.add_node(XffValue::from("n1"), XffValue::Null);
    /// let c0 = graph.add_connection(n0, n1, XffValue::Null).unwrap();
    /// ```
    pub fn add_connection(
        &mut self,
        from: u32,
        to: u32,
        metadata: XffValue,
    ) -> Result<u32, GraphError> {
        if !self.has_node(from) {
            return Err(GraphError::NodeNotFound(from));
        }
        if !self.has_node(to) {
            return Err(GraphError::NodeNotFound(to));
        }

        let conn = GraphConnection { from, to, metadata };
        let index = if let Some(idx) = self.free_connections.pop() {
            self.connections[idx as usize] = Some(conn);
            idx
        } else {
            let idx = self.connections.len() as u32;
            self.connections.push(Some(conn));
            idx
        };

        // Update tracking in nodes
        if let Some(from_node) = self.get_node_mut(from) {
            from_node.add_outbound(index);
        }
        if let Some(to_node) = self.get_node_mut(to) {
            to_node.add_inbound(index);
        }

        Ok(index)
    }

    /// Removes a connection from the graph by its index.
    ///
    /// # Example
    /// ```rust
    /// use aequa::{graph::Graph, XffValue};
    /// let mut graph = Graph::new();
    /// let n0 = graph.add_node(XffValue::from("n0"), XffValue::Null);
    /// let n1 = graph.add_node(XffValue::from("n1"), XffValue::Null);
    /// let c0 = graph.add_connection(n0, n1, XffValue::Null).unwrap();
    /// graph.remove_connection(c0);
    /// assert!(graph.get_connection(c0).is_none());
    /// ```
    pub fn remove_connection(&mut self, index: u32) {
        if index as usize >= self.connections.len() {
            return;
        }

        let conn = match self.connections[index as usize].take() {
            Some(c) => c,
            None => return,
        };

        self.free_connections.push(index);

        // Update tracking in nodes
        if let Some(from_node) = self.get_node_mut(conn.from) {
            from_node.remove_outbound(index);
        }
        if let Some(to_node) = self.get_node_mut(conn.to) {
            to_node.remove_inbound(index);
        }
    }

    /// Returns true if a node exists at the given index.
    ///
    /// # Example
    /// ```rust
    /// use aequa::{graph::Graph, XffValue};
    /// let mut graph = Graph::new();
    /// let n0 = graph.add_node(XffValue::from("root"), XffValue::Null);
    /// assert!(graph.has_node(n0));
    /// ```
    pub fn has_node(&self, index: u32) -> bool {
        self.nodes
            .get(index as usize)
            .map_or(false, |n| n.is_some())
    }

    /// Returns an iterator over every node in the graph.
    ///
    /// # Example
    /// ```rust
    /// use aequa::{graph::Graph, XffValue};
    /// let mut graph = Graph::new();
    /// let n0 = graph.add_node(XffValue::from("root"), XffValue::Null);
    /// assert!(graph.get_all_nodes().any(|n| n.metadata == XffValue::Null));
    /// ```
    pub fn get_all_nodes(&self) -> impl Iterator<Item = &GraphNode> {
        self.nodes.iter().filter_map(|n| n.as_ref())
    }

    /// Returns an iterator over every node in the graph.
    ///
    /// # Example
    /// ```rust
    /// use aequa::{graph::Graph, XffValue};
    /// let mut graph = Graph::new();
    /// let n0 = graph.add_node(XffValue::from("root"), XffValue::Null);
    /// assert!(graph.get_all_nodes().any(|n| n.metadata == XffValue::Null));
    /// ```
    pub fn get_all_nodes_mut(&mut self) -> impl Iterator<Item = &mut GraphNode> {
        self.nodes.iter_mut().filter_map(|n| n.as_mut())
    }

    /// Returns an iterator over every node index in the graph.
    ///
    /// # Example
    /// ```rust
    /// use aequa::{graph::Graph, XffValue};
    /// let mut graph = Graph::new();
    /// let n0 = graph.add_node(XffValue::from("root"), XffValue::Null);
    /// assert!(graph.get_all_nodes_indices().any(|n| n == n0));
    /// ```
    pub fn get_all_nodes_indices(&self) -> impl Iterator<Item = u32> {
        self.nodes
            .iter()
            .enumerate()
            .filter_map(|(i, _)| i.try_into().ok())
    }

    /// Returns a reference to a node if it exists.
    ///
    /// # Example
    /// ```rust
    /// use aequa::{graph::Graph, XffValue};
    /// let mut graph = Graph::new();
    /// let n0 = graph.add_node(XffValue::from("root"), XffValue::Null);
    /// assert!(graph.get_node(n0).is_some());
    /// ```
    pub fn get_node(&self, index: u32) -> Option<&GraphNode> {
        self.nodes.get(index as usize).and_then(|n| n.as_ref())
    }

    /// Returns a mutable reference to a node if it exists.
    ///
    /// # Example
    /// ```rust
    /// use aequa::{graph::Graph, XffValue};
    /// let mut graph = Graph::new();
    /// let n0 = graph.add_node(XffValue::from("root"), XffValue::Null);
    /// assert!(graph.get_node_mut(n0).is_some());
    /// ```
    pub fn get_node_mut(&mut self, index: u32) -> Option<&mut GraphNode> {
        self.nodes.get_mut(index as usize).and_then(|n| n.as_mut())
    }

    /// Returns an iterator over every connection in the graph.
    ///
    /// # Example
    /// ```rust
    /// use aequa::{graph::Graph, XffValue};
    /// let mut graph = Graph::new();
    /// let n0 = graph.add_node(XffValue::from("n0"), XffValue::Null);
    /// let n1 = graph.add_node(XffValue::from("n1"), XffValue::Null);
    /// let c0 = graph.add_connection(n0, n1, XffValue::Null).unwrap();
    /// assert!(graph.get_all_connections().any(|c| c.metadata == XffValue::Null));
    /// ```
    pub fn get_all_connections(&self) -> impl Iterator<Item = &GraphConnection> {
        self.connections.iter().filter_map(|c| c.as_ref())
    }

    /// Returns an iterator over every connection in the graph.
    ///
    /// # Example
    /// ```rust
    /// use aequa::{graph::Graph, XffValue};
    /// let mut graph = Graph::new();
    /// let n0 = graph.add_node(XffValue::from("n0"), XffValue::Null);
    /// let n1 = graph.add_node(XffValue::from("n1"), XffValue::Null);
    /// let c0 = graph.add_connection(n0, n1, XffValue::Null).unwrap();
    /// assert!(graph.get_all_connections().any(|c| c.metadata == XffValue::Null));
    /// ```
    pub fn get_all_connections_mut(&mut self) -> impl Iterator<Item = &mut GraphConnection> {
        self.connections.iter_mut().filter_map(|c| c.as_mut())
    }

    /// Returns an iterator over every connection index in the graph.
    ///
    /// # Example
    /// ```rust
    /// use aequa::{graph::Graph, XffValue};
    /// let mut graph = Graph::new();
    /// let n0 = graph.add_node(XffValue::from("n0"), XffValue::Null);
    /// let n1 = graph.add_node(XffValue::from("n1"), XffValue::Null);
    /// let c0 = graph.add_connection(n0, n1, XffValue::Null).unwrap();
    /// assert!(graph.get_all_connections_indices().any(|c| c == c0));
    /// ```
    pub fn get_all_connections_indices(&self) -> impl Iterator<Item = u32> {
        self.connections
            .iter()
            .enumerate()
            .filter_map(|(i, c)| c.as_ref().map(|_| i as u32))
    }

    /// Returns a reference to a connection if it exists.
    ///
    /// # Example
    /// ```rust
    /// use aequa::{graph::Graph, XffValue};
    /// let mut graph = Graph::new();
    /// let n0 = graph.add_node(XffValue::from("n0"), XffValue::Null);
    /// let n1 = graph.add_node(XffValue::from("n1"), XffValue::Null);
    /// let c0 = graph.add_connection(n0, n1, XffValue::Null).unwrap();
    /// assert!(graph.get_connection(c0).is_some());
    /// ```
    pub fn get_connection(&self, index: u32) -> Option<&GraphConnection> {
        self.connections
            .get(index as usize)
            .and_then(|c| c.as_ref())
    }

    /// Returns a mutable reference to a connection if it exists.
    ///
    /// # Example
    /// ```rust
    /// use aequa::{graph::Graph, XffValue};
    /// let mut graph = Graph::new();
    /// let n0 = graph.add_node(XffValue::from("n0"), XffValue::Null);
    /// let n1 = graph.add_node(XffValue::from("n1"), XffValue::Null);
    /// let c0 = graph.add_connection(n0, n1, XffValue::Null).unwrap();
    /// assert!(graph.get_connection_mut(c0).is_some());
    /// ```
    pub fn get_connection_mut(&mut self, index: u32) -> Option<&mut GraphConnection> {
        self.connections
            .get_mut(index as usize)
            .and_then(|c| c.as_mut())
    }

    /// Returns the indices of all nodes that are directly connected from the given node.
    ///
    /// # Example
    /// ```rust
    /// use aequa::{graph::Graph, XffValue};
    /// let mut graph = Graph::new();
    /// let n0 = graph.add_node(XffValue::from("n0"), XffValue::Null);
    /// let n1 = graph.add_node(XffValue::from("n1"), XffValue::Null);
    /// graph.add_connection(n0, n1, XffValue::Null).unwrap();
    /// assert_eq!(graph.get_children(n0), vec![n1]);
    /// ```
    pub fn get_children(&self, node_index: u32) -> Vec<u32> {
        let mut children = Vec::new();
        if let Some(node) = self.get_node(node_index) {
            for &conn_idx in &node.outbound_connections {
                if let Some(conn) = self.get_connection(conn_idx) {
                    children.push(conn.to);
                }
            }
        }
        children
    }

    /// Returns the indices of all nodes that directly connect to the given node.
    ///
    /// # Example
    /// ```rust
    /// use aequa::{graph::Graph, XffValue};
    /// let mut graph = Graph::new();
    /// let n0 = graph.add_node(XffValue::from("n0"), XffValue::Null);
    /// let n1 = graph.add_node(XffValue::from("n1"), XffValue::Null);
    /// graph.add_connection(n0, n1, XffValue::Null).unwrap();
    /// assert_eq!(graph.get_parents(n1), vec![n0]);
    /// ```
    pub fn get_parents(&self, node_index: u32) -> Vec<u32> {
        let mut parents = Vec::new();
        if let Some(node) = self.get_node(node_index) {
            for &conn_idx in &node.inbound_connections {
                if let Some(conn) = self.get_connection(conn_idx) {
                    parents.push(conn.from);
                }
            }
        }
        parents
    }

    /// Performs a Breadth-First Search starting from the given node.
    /// Returns a list of node indices in the order they were visited.
    ///
    /// This is useful for finding all reachable nodes or processing levels in a tech tree.
    ///
    /// # Example
    /// ```rust
    /// use aequa::{graph::Graph, XffValue};
    /// let mut graph = Graph::new();
    /// let n0 = graph.add_node(XffValue::from("0"), XffValue::Null);
    /// let n1 = graph.add_node(XffValue::from("1"), XffValue::Null);
    /// graph.add_connection(n0, n1, XffValue::Null).unwrap();
    /// assert_eq!(graph.traverse_bfs(n0), vec![n0, n1]);
    /// ```
    pub fn traverse_bfs(&self, start_index: u32) -> Vec<u32> {
        let mut visited = Vec::new();
        let mut queue = std::collections::VecDeque::new();

        if !self.has_node(start_index) {
            return visited;
        }

        queue.push_back(start_index);
        let mut seen = std::collections::HashSet::new();
        seen.insert(start_index);

        while let Some(current) = queue.pop_front() {
            visited.push(current);

            for child in self.get_children(current) {
                if !seen.contains(&child) {
                    seen.insert(child);
                    queue.push_back(child);
                }
            }
        }

        visited
    }

    /// Performs a Depth-First Search starting from the given node.
    /// Returns a list of node indices in the order they were visited.
    ///
    /// This is useful for finding all dependencies of a node or exploring deep paths first.
    ///
    /// # Example
    /// ```rust
    /// use aequa::{graph::Graph, XffValue};
    /// let mut graph = Graph::new();
    /// let n0 = graph.add_node(XffValue::from("0"), XffValue::Null);
    /// let n1 = graph.add_node(XffValue::from("1"), XffValue::Null);
    /// graph.add_connection(n0, n1, XffValue::Null).unwrap();
    /// assert_eq!(graph.traverse_dfs(n0), vec![n0, n1]);
    /// ```
    pub fn traverse_dfs(&self, start_index: u32) -> Vec<u32> {
        let mut visited = Vec::new();
        let mut stack = Vec::new();

        if !self.has_node(start_index) {
            return visited;
        }

        stack.push(start_index);
        let mut seen = std::collections::HashSet::new();

        while let Some(current) = stack.pop() {
            if seen.insert(current) {
                visited.push(current);
                // We process children in reverse to maintain a predictable
                // left-to-right order similar to the vector order.
                let mut children = self.get_children(current);
                children.reverse();
                for child in children {
                    if !seen.contains(&child) {
                        stack.push(child);
                    }
                }
            }
        }

        visited
    }

    /// Checks if there is a path from the `from` node to the `to` node.
    ///
    /// # Example
    /// ```rust
    /// use aequa::{graph::Graph, XffValue};
    /// let mut graph = Graph::new();
    /// let n0 = graph.add_node(XffValue::from("0"), XffValue::Null);
    /// let n1 = graph.add_node(XffValue::from("1"), XffValue::Null);
    /// graph.add_connection(n0, n1, XffValue::Null).unwrap();
    /// assert!(graph.is_reachable(n0, n1));
    /// ```
    pub fn is_reachable(&self, from: u32, to: u32) -> bool {
        if from == to {
            return self.has_node(from);
        }

        let mut queue = std::collections::VecDeque::new();
        queue.push_back(from);

        let mut seen = std::collections::HashSet::new();
        seen.insert(from);

        while let Some(current) = queue.pop_front() {
            for child in self.get_children(current) {
                if child == to {
                    return true;
                }
                if seen.insert(child) {
                    queue.push_back(child);
                }
            }
        }
        false
    }

    /// Checks if the graph contains any cycles.
    ///
    /// This uses a recursive DFS approach with three states (unvisited, visiting, visited).
    ///
    /// # Example
    /// ```rust
    /// use aequa::{graph::Graph, XffValue};
    /// let mut graph = Graph::new();
    /// let n0 = graph.add_node(XffValue::from("0"), XffValue::Null);
    /// graph.add_connection(n0, n0, XffValue::Null).unwrap();
    /// assert!(graph.has_cycle());
    /// ```
    pub fn has_cycle(&self) -> bool {
        let mut visited = std::collections::HashSet::new();
        let mut stack = std::collections::HashSet::new();

        for i in 0..self.nodes.len() {
            let idx = i as u32;
            if self.has_node(idx) && !visited.contains(&idx) {
                if self.check_cycle_recursive(idx, &mut visited, &mut stack) {
                    return true;
                }
            }
        }
        false
    }

    fn check_cycle_recursive(
        &self,
        current: u32,
        visited: &mut std::collections::HashSet<u32>,
        stack: &mut std::collections::HashSet<u32>,
    ) -> bool {
        visited.insert(current);
        stack.insert(current);

        for child in self.get_children(current) {
            if !visited.contains(&child) {
                if self.check_cycle_recursive(child, visited, stack) {
                    return true;
                }
            } else if stack.contains(&child) {
                return true;
            }
        }

        stack.remove(&current);
        false
    }

    /// Returns the number of connections pointing to this node.
    ///
    /// # Example
    /// ```rust
    /// use aequa::{graph::Graph, XffValue};
    /// let mut graph = Graph::new();
    /// let n0 = graph.add_node(XffValue::from("0"), XffValue::Null);
    /// assert_eq!(graph.in_degree(n0), Some(0));
    /// ```
    pub fn in_degree(&self, node_index: u32) -> Option<usize> {
        self.get_node(node_index)
            .map(|n| n.inbound_connections.len())
    }

    /// Returns the number of connections originating from this node.
    ///
    /// # Example
    /// ```rust
    /// use aequa::{graph::Graph, XffValue};
    /// let mut graph = Graph::new();
    /// let n0 = graph.add_node(XffValue::from("0"), XffValue::Null);
    /// assert_eq!(graph.out_degree(n0), Some(0));
    /// ```
    pub fn out_degree(&self, node_index: u32) -> Option<usize> {
        self.get_node(node_index)
            .map(|n| n.outbound_connections.len())
    }

    /// Returns true if the node has no outbound connections.
    ///
    /// # Example
    /// ```rust
    /// use aequa::{graph::Graph, XffValue};
    /// let mut graph = Graph::new();
    /// let n0 = graph.add_node(XffValue::from("0"), XffValue::Null);
    /// assert!(graph.is_leaf(n0));
    /// ```
    pub fn is_leaf(&self, node_index: u32) -> bool {
        self.out_degree(node_index) == Some(0)
    }

    /// Returns true if the node has no inbound connections.
    /// Note: This is a structural check and is independent of the index 0 "Root" convention.
    ///
    /// # Example
    /// ```rust
    /// use aequa::{graph::Graph, XffValue};
    /// let mut graph = Graph::new();
    /// let n0 = graph.add_node(XffValue::from("0"), XffValue::Null);
    /// assert!(graph.is_root_node(n0));
    /// ```
    pub fn is_root_node(&self, node_index: u32) -> bool {
        self.in_degree(node_index) == Some(0)
    }

    /// Returns the indices of all connections between two specific nodes.
    ///
    /// # Example
    /// ```rust
    /// use aequa::{graph::Graph, XffValue};
    /// let mut graph = Graph::new();
    /// let n0 = graph.add_node(XffValue::from("n0"), XffValue::Null);
    /// let n1 = graph.add_node(XffValue::from("n1"), XffValue::Null);
    /// let c0 = graph.add_connection(n0, n1, XffValue::Null).unwrap();
    /// assert_eq!(graph.find_connections(n0, n1), vec![c0]);
    /// ```
    pub fn find_connections(&self, from: u32, to: u32) -> Vec<u32> {
        let mut found = Vec::new();
        if let Some(node) = self.get_node(from) {
            for &conn_idx in &node.outbound_connections {
                if let Some(conn) = self.get_connection(conn_idx) {
                    if conn.to == to {
                        found.push(conn_idx);
                    }
                }
            }
        }
        found
    }

    /// Finds the shortest path (in terms of number of edges) between two nodes.
    /// Returns a list of node indices if a path exists.
    ///
    /// # Example
    /// ```rust
    /// use aequa::{graph::Graph, XffValue};
    /// let mut graph = Graph::new();
    /// let n0 = graph.add_node(XffValue::from("0"), XffValue::Null);
    /// let n1 = graph.add_node(XffValue::from("1"), XffValue::Null);
    /// graph.add_connection(n0, n1, XffValue::Null).unwrap();
    /// assert_eq!(graph.find_path(n0, n1), Some(vec![n0, n1]));
    /// ```
    pub fn find_path(&self, from: u32, to: u32) -> Option<Vec<u32>> {
        if !self.has_node(from) || !self.has_node(to) {
            return None;
        }
        if from == to {
            return Some(vec![from]);
        }

        let mut queue = std::collections::VecDeque::new();
        let mut parent_map = std::collections::HashMap::new();

        queue.push_back(from);
        parent_map.insert(from, None);

        while let Some(current) = queue.pop_front() {
            if current == to {
                let mut path = Vec::new();
                let mut curr = Some(to);
                while let Some(node) = curr {
                    path.push(node);
                    curr = parent_map.get(&node).copied().flatten();
                }
                path.reverse();
                return Some(path);
            }

            for child in self.get_children(current) {
                if !parent_map.contains_key(&child) {
                    parent_map.insert(child, Some(current));
                    queue.push_back(child);
                }
            }
        }
        None
    }

    /// Identifies all Strongly Connected Components (SCCs) in the graph.
    /// An SCC is a sub-graph where every node is reachable from every other node in that sub-graph.
    ///
    /// Uses Tarjan's algorithm ($O(V+E)$).
    ///
    /// # Example
    /// ```rust
    /// use aequa::{graph::Graph, XffValue};
    /// let mut graph = Graph::new();
    /// let n0 = graph.add_node(XffValue::from("0"), XffValue::Null);
    /// let sccs = graph.find_sccs();
    /// assert_eq!(sccs.len(), 1);
    /// ```
    pub fn find_sccs(&self) -> Vec<Vec<u32>> {
        let mut index = 0;
        let mut stack = Vec::new();
        let mut on_stack = std::collections::HashSet::new();
        let mut indices = std::collections::HashMap::new();
        let mut lowlink = std::collections::HashMap::new();
        let mut sccs = Vec::new();

        for i in 0..self.nodes.len() {
            let node_idx = i as u32;
            if self.has_node(node_idx) && !indices.contains_key(&node_idx) {
                self.tarjan_visit(
                    node_idx,
                    &mut index,
                    &mut stack,
                    &mut on_stack,
                    &mut indices,
                    &mut lowlink,
                    &mut sccs,
                );
            }
        }
        sccs
    }

    fn tarjan_visit(
        &self,
        u: u32,
        index: &mut usize,
        stack: &mut Vec<u32>,
        on_stack: &mut std::collections::HashSet<u32>,
        indices: &mut std::collections::HashMap<u32, usize>,
        lowlink: &mut std::collections::HashMap<u32, usize>,
        sccs: &mut Vec<Vec<u32>>,
    ) {
        indices.insert(u, *index);
        lowlink.insert(u, *index);
        *index += 1;
        stack.push(u);
        on_stack.insert(u);

        for v in self.get_children(u) {
            if !indices.contains_key(&v) {
                self.tarjan_visit(v, index, stack, on_stack, indices, lowlink, sccs);
                let low_u = lowlink[&u];
                let low_v = lowlink[&v];
                lowlink.insert(u, std::cmp::min(low_u, low_v));
            } else if on_stack.contains(&v) {
                let low_u = lowlink[&u];
                let idx_v = indices[&v];
                lowlink.insert(u, std::cmp::min(low_u, idx_v));
            }
        }

        if lowlink[&u] == indices[&u] {
            let mut component = Vec::new();
            while let Some(w) = stack.pop() {
                on_stack.remove(&w);
                component.push(w);
                if w == u {
                    break;
                }
            }
            sccs.push(component);
        }
    }

    /// Exports the graph into a format compatible with `athena::utils::sorting::topological_sort::kahns`.
    ///
    /// The format is a vector of tuples: `(node_name, dependencies)`.
    /// Node names are derived from the node's `id`, as "{idx}".
    ///
    /// **Note**: Topological sorting is not implemented in `aequa` to avoid dependency cycles.
    /// Use `athena` for this functionality.
    ///
    /// # Example
    /// ```rust
    /// use aequa::{graph::Graph, XffValue};
    /// let mut graph = Graph::new();
    /// let n0 = graph.add_node(XffValue::from("0"), XffValue::Null);
    /// let export = graph.export_for_topological_sort();
    /// assert_eq!(export.len(), 1);
    /// ```
    pub fn export_for_topological_sort(&self) -> Vec<(String, Vec<String>)> {
        let mut export = Vec::new();
        for i in 0..self.nodes.len() {
            let idx = i as u32;
            if self.get_node(idx).is_some() {
                let name = format!("{idx}");
                let parents = self
                    .get_parents(idx)
                    .into_iter()
                    .map(|p_idx| format!("{p_idx}"))
                    .collect();
                export.push((name, parents));
            }
        }
        export
    }
}
