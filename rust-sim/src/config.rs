
/// Number of sensor nodes deployed in the network.
pub const NUM_NODES: usize = 100;

/// Width of the deployment area (in meters).
pub const AREA_WIDTH: f64 = 100.0;

/// Height of the deployment area (in meters).
pub const AREA_HEIGHT: f64 = 100.0;

// -----------------------------------------------------------------------------
// LEACH protocol parameters
// -----------------------------------------------------------------------------

/// Desired probability of a node becoming a Cluster Head (CH) in LEACH.
///
/// For example, `P = 0.1` means approximately 10% of nodes
/// are expected to become cluster heads in each round.
pub const P: f64 = 0.1;

/// Cluster Head rotation cycle length (in rounds).
///
/// Computed as `1 / P`.
/// Each node should become a CH once every `CYCLE` rounds on average.
pub const CYCLE: i64 = (1.0 / P) as i64;

// -----------------------------------------------------------------------------
// First Order Radio Energy Model parameters
// -----------------------------------------------------------------------------

/// Energy consumed by radio electronics per bit (Joules/bit).
///
/// This cost applies to both transmission and reception.
pub const E_ELEC: f64 = 50e-9;

/// Energy consumed by the transmitter amplifier in the free-space model
/// (Joules/bit/m²).
///
/// Used when the transmission distance is less than the threshold distance `D0`.
pub const E_FS: f64 = 10e-12;

/// Energy consumed by the transmitter amplifier in the multipath fading model
/// (Joules/bit/m⁴).
///
/// Used when the transmission distance is greater than or equal to `D0`.
pub const E_MP: f64 = 0.0013e-12;

/// Initial energy assigned to each sensor node (in Joules).
pub const INITIAL_ENERGY: f64 = 2.0;

/// Size of a data packet transmitted by a sensor node (in bits).
pub const PACKET_SIZE: usize = 4000;

// -----------------------------------------------------------------------------
// Radio propagation threshold
// -----------------------------------------------------------------------------

/// Threshold distance (in meters) that determines which radio propagation
/// model is used.
///
/// - If `d < D0`, the free-space model (`d²`) is used.
/// - If `d >= D0`, the multipath model (`d⁴`) is used.
///
/// Computed from the ratio of `E_FS` and `E_MP`.
pub const D0: f64 = 87.7;

// -----------------------------------------------------------------------------
// Base Station configuration
// -----------------------------------------------------------------------------

/// X-coordinate of the Base Station (in meters).
///
/// Typically placed inside or just outside the deployment area.
pub const BS_X: f64 = 50.0;

/// Y-coordinate of the Base Station (in meters).
///
/// Often placed outside the sensing field to model long-distance transmission.
pub const BS_Y: f64 = 150.0;

// -----------------------------------------------------------------------------
// Simulation control
// -----------------------------------------------------------------------------

/// Maximum number of rounds to run the simulation.
///
/// The simulation may terminate earlier if all nodes die.
pub const MAX_ROUNDS: usize = 2000;

