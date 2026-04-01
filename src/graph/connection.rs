use crate::XffValue;

/// A directional link between two nodes.
pub struct GraphConnection {
    /// The index of the originating node.
    pub from: u32,
    /// The index of the target node.
    pub to: u32,
    pub metadata: XffValue,
}

impl Default for GraphConnection {
    fn default() -> Self {
        GraphConnection {
            from: 0,
            to: 0,
            metadata: XffValue::default(),
        }
    }
}
