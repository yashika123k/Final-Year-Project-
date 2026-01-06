
use ggez::glam::Vec2;
use crate::config::CYCLE;

/// Represents a single sensor node in the WSN simulation.
///
/// A `Node` stores only the **state** of the sensor:
/// - physical position
/// - energy status
/// - cluster-related metadata
///
/// All protocol logic (LEACH, ML, etc.) is handled outside this struct.
#[derive(Debug, Clone)]
pub struct Node {
    /// Unique identifier of the node
    pub id: usize,

    /// Physical position of the node in the deployment area (meters)
    pub position: Vec2,

    /// Remaining energy of the node (in Joules)
    pub energy: f64,

    /// Whether the node is alive (energy > 0)
    pub is_alive: bool,

    /// Whether the node is a Cluster Head in the current round
    pub is_ch: bool,

    /// The round number in which the node last acted as a Cluster Head
    ///
    /// Initialized to `-CYCLE` so that all nodes are eligible
    /// to become Cluster Heads in the first round.
    pub last_ch_round: i32,

    /// The ID of the Cluster Head this node is associated with
    ///
    /// - `None` if the node is a Cluster Head itself
    /// - `Some(ch_id)` if the node is a normal member
    pub cluster_id: Option<usize>,

    /// List of member node IDs (used only when this node is a Cluster Head)
    pub members: Vec<usize>,
}

impl Node {
    /// Creates a new sensor node with the given parameters.
    ///
    /// # Arguments
    /// * `id` - Unique node identifier
    /// * `position` - Physical location of the node in the network area
    /// * `energy` - Initial energy of the node (Joules)
    ///
    /// # Notes
    /// - The node starts alive
    /// - The node is not a Cluster Head initially
    /// - `last_ch_round` is set to `-CYCLE` to ensure
    ///   eligibility in the first LEACH round
    pub fn new(id: usize, position: Vec2, energy: f64) -> Self {
        Self {
            id,
            position,
            energy,
            is_alive: true,
            is_ch: false,
            last_ch_round: -CYCLE,
            cluster_id: None,
            members: Vec::new(),
        }
    }
}

