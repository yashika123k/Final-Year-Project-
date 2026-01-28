use glam::Vec2;

// =============================================================================
// Simulation Area & Visualization
// =============================================================================

/// Width of the deployment area (in meters).
pub const AREA_WIDTH: f32 = 500.0;

/// Height of the deployment area (in meters).
pub const AREA_HEIGHT: f32 = 500.0;

/// Screen width in pixels.pub const SCREEN_WIDTH: f32 = 1200.0;
pub const SCREEN_WIDTH: f32 = 1200.0;
/// Screen height in pixels.
pub const SCREEN_HEIGHT: f32 = 720.0;

/// Scaling factor to convert simulation coordinates to screen pixels.
pub const TO_PIXEL_SCALE: Vec2 = Vec2::new(SCREEN_WIDTH / AREA_WIDTH, SCREEN_HEIGHT / AREA_HEIGHT);

/// FPS
pub const FPS: u32 = 10;

/// SENSOR radius
pub const SENSOR_RADIUS:f32 = 10.0;

// =============================================================================
// LEACH Protocol Parameters
// =============================================================================

/// Number of sensor nodes in the network.
pub const NUM_NODES: usize = 100;

/// Desired probability that a node becomes a cluster head in any given round.
pub const CH_PROBABILITY: f32 = 0.1;

/// Expected number of cluster heads per round (ceiled value).
pub const EXPECTED_CLUSTER_HEADS: usize = (NUM_NODES as f32 * CH_PROBABILITY).ceil() as usize;

/// Cluster head rotation cycle length (number of rounds in which each node should become CH once on average).
pub const CYCLE_LENGTH: usize = (1.0 / CH_PROBABILITY) as usize;

// =============================================================================
// First-Order Radio Energy Model Parameters
// =============================================================================

/// Initial energy of each sensor node (in Joules).
pub const INITIAL_ENERGY: f32 = 2.0;

/// Energy dissipated by radio electronics per bit (both TX and RX) in J/bit.
pub const E_ELECTRONICS: f32 = 5e-8;

/// Energy used by the transmitter amplifier in free-space model (J/bit/m²).
pub const E_FREE_SPACE: f32 = 1e-11;

/// Energy used by the transmitter amplifier in multipath model (J/bit/m⁴).
pub const E_MULTIPATH: f32 = 1.3e-15;

/// Energy used for data aggregation per bit per signal in J/bit/signal.
pub const E_AGGREGATION: f32 = 5e-9;

/// Size of a data packet in bits.
pub const PACKET_SIZE: f32 = 4000.0;

// =============================================================================
// Radio Propagation & Threshold
// =============================================================================

/// Threshold distance (in meters) for switching between free-space and multipath models.
/// Computed as sqrt(E_FS / E_MP).
pub const THRESHOLD_DISTANCE: f32 = 87.7;

// =============================================================================
// Base Station (Sink)
// =============================================================================

/// Location of the base station (center of the deployment area).
pub const SINK: Vec2 = Vec2::new(AREA_WIDTH / 2.0, AREA_HEIGHT / 2.0);

// =============================================================================
// Simulation Control
// =============================================================================

/// Maximum number of rounds to simulate (simulation may stop earlier if all nodes die).
pub const MAX_ROUNDS: usize = 2000;
