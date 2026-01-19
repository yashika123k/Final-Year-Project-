use ggez::glam::Vec2;
use rand::Rng;
use crate::config::{INITIAL_ENERGY, SINK};

/// Represents a single sensor node in the Wireless Sensor Network (WSN) simulation.
///
/// This struct contains only the **state** of the node.
/// All protocol-specific logic (LEACH phases, cluster formation, data transmission, etc.)
/// is handled externally.
#[derive(Debug, Clone)]
pub struct Node {
    /// Unique identifier of the node (0..NUM_NODES-1 typically)
    pub id: usize,

    /// Physical position of the node in the deployment area (in meters)
    pub position: Vec2,

    /// Current remaining energy of the node (in Joules)
    pub energy: f64,

    /// Whether the node still has energy (> 0)
    pub is_alive: bool,

    /// Whether this node is currently acting as a Cluster Head in this round
    pub is_cluster_head: bool,

    /// Whether the node is eligible to become a Cluster Head in the current round
    /// (based on LEACH's probabilistic election and rotation rules)
    pub eligible: bool,

    /// Communication range of the node (randomized per node in meters)
    pub transmission_range: f32,

    /// Precomputed Euclidean distance from this node to the sink/base station
    pub distance_to_sink: f32,

    /// ID of the Cluster Head this node belongs to
    /// - `None`        → this node is a Cluster Head itself
    /// - `Some(ch_id)` → this node is a normal member of cluster `ch_id`
    pub cluster_head_id: Option<usize>,

    /// List of member node IDs (only meaningful when this node is a Cluster Head)
    pub cluster_members: Vec<usize>,

    /// List of neighbours node IDs 
    pub neighbours: Vec<usize>
}

impl Node {
    /// Creates a new sensor node with randomized transmission range and precomputed sink distance.
    ///
    /// # Arguments
    /// * `id`       - Unique identifier for the node
    /// * `position` - (x, y) coordinates in the deployment area (meters)
    ///
    /// # Behavior
    /// - Initializes energy to `INITIAL_ENERGY`
    /// - Node starts alive (`is_alive = true`)
    /// - Starts as non-Cluster Head (`is_cluster_head = false`)
    /// - Eligible to become CH in the first round (`eligible = true`)
    /// - Transmission range is randomly chosen between 20–30 meters
    /// - Distance to sink is precomputed for performance
    pub fn new(id: usize, position: Vec2) -> Self {
        let mut rng = rand::rng();

        // Precompute squared distance first (cheaper than sqrt twice)
        let diff = SINK - position;
        let distance_to_sink = diff.length(); 

        Self {
            id,
            position,
            energy: INITIAL_ENERGY,
            is_alive: true,
            is_cluster_head: false,
            eligible: true,
            transmission_range: rng.random_range(20.0..=30.0),
            distance_to_sink,
            cluster_head_id: None,
            cluster_members: Vec::new(),
            neighbours: Vec::new()
        }
    }
}
