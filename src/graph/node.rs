use crate::XffValue;

/// A node in the graph holding a payload and metadata.
pub struct GraphNode {
    /// The payload of the node.
    pub payload: XffValue,
    /// The metadata of the node.
    pub metadata: XffValue,
    /// Indices of connections where this node is the 'to' node.
    pub inbound_connections: Vec<u32>,
    /// Indices of connections where this node is the 'from' node.
    pub outbound_connections: Vec<u32>,
}

impl Default for GraphNode {
    fn default() -> Self {
        GraphNode {
            payload: XffValue::default(),
            metadata: XffValue::default(),
            inbound_connections: Vec::new(),
            outbound_connections: Vec::new(),
        }
    }
}

impl GraphNode {
    /// Creates a new `GraphNode` with the given payload and metadata.
    ///
    /// # Example
    /// ```rust
    /// use aequa::{graph::GraphNode, XffValue};
    /// let node = GraphNode::new(XffValue::from("payload"), XffValue::Null);
    /// ```
    pub fn new(payload: XffValue, metadata: XffValue) -> Self {
        Self {
            payload,
            metadata,
            inbound_connections: Vec::new(),
            outbound_connections: Vec::new(),
        }
    }

    /// Adds an inbound connection index to the node.
    ///
    /// # Example
    /// ```rust
    /// use aequa::{graph::GraphNode, XffValue};
    /// let mut node = GraphNode::default();
    /// node.add_inbound(1);
    /// assert_eq!(node.inbound_connections, vec![1]);
    /// ```
    pub fn add_inbound(&mut self, conn_index: u32) {
        if !self.inbound_connections.contains(&conn_index) {
            self.inbound_connections.push(conn_index);
        }
    }

    /// Adds an outbound connection index to the node.
    ///
    /// # Example
    /// ```rust
    /// use aequa::{graph::GraphNode, XffValue};
    /// let mut node = GraphNode::default();
    /// node.add_outbound(1);
    /// assert_eq!(node.outbound_connections, vec![1]);
    /// ```
    pub fn add_outbound(&mut self, conn_index: u32) {
        if !self.outbound_connections.contains(&conn_index) {
            self.outbound_connections.push(conn_index);
        }
    }

    /// Removes an inbound connection index from the node.
    ///
    /// # Example
    /// ```rust
    /// use aequa::{graph::GraphNode, XffValue};
    /// let mut node = GraphNode::default();
    /// node.add_inbound(1);
    /// node.remove_inbound(1);
    /// assert!(node.inbound_connections.is_empty());
    /// ```
    pub fn remove_inbound(&mut self, conn_index: u32) {
        self.inbound_connections.retain(|&x| x != conn_index);
    }

    /// Removes an outbound connection index from the node.
    ///
    /// # Example
    /// ```rust
    /// use aequa::{graph::GraphNode, XffValue};
    /// let mut node = GraphNode::default();
    /// node.add_outbound(1);
    /// node.remove_outbound(1);
    /// assert!(node.outbound_connections.is_empty());
    /// ```
    pub fn remove_outbound(&mut self, conn_index: u32) {
        self.outbound_connections.retain(|&x| x != conn_index);
    }
}
