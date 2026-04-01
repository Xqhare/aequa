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
            payload: XffValue::Null,
            metadata: XffValue::Null,
            inbound_connections: Vec::new(),
            outbound_connections: Vec::new(),
        }
    }
}

impl GraphNode {
    pub fn new(payload: XffValue, metadata: XffValue) -> Self {
        Self {
            payload,
            metadata,
            inbound_connections: Vec::new(),
            outbound_connections: Vec::new(),
        }
    }

    pub fn add_inbound(&mut self, conn_index: u32) {
        if !self.inbound_connections.contains(&conn_index) {
            self.inbound_connections.push(conn_index);
        }
    }

    pub fn add_outbound(&mut self, conn_index: u32) {
        if !self.outbound_connections.contains(&conn_index) {
            self.outbound_connections.push(conn_index);
        }
    }

    pub fn remove_inbound(&mut self, conn_index: u32) {
        self.inbound_connections.retain(|&x| x != conn_index);
    }

    pub fn remove_outbound(&mut self, conn_index: u32) {
        self.outbound_connections.retain(|&x| x != conn_index);
    }
}
