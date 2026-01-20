use std::fs::File;
use std::io::{BufWriter,Write};
use rand::Rng;

use crate::config::*;
use crate::node::Node;

/// Computes the LEACH threshold T(n) for the current round.
fn threshold(round: usize) -> f64 {
    let r_mod = (round % CYCLE_LENGTH) as f64;
    let denom = 1.0 - CH_PROBABILITY * r_mod;
    (CH_PROBABILITY / denom).min(1.0)
}

/// Resets cluster-related state for all nodes at the start of a round.
/// Also resets eligibility at the beginning of each new cycle (epoch).
pub fn reset(nodes: &mut [Node], round: usize) {
    let is_new_cycle = round % CYCLE_LENGTH == 0;

    for node in nodes.iter_mut() {
        node.is_cluster_head   = false;
        node.cluster_head_id   = None;
        node.cluster_members.clear();

        if is_new_cycle {
            node.eligible = node.is_alive;
        }
    }
}

/// Executes one full LEACH round:
/// 1. Cluster Head selection
/// 2. Cluster formation
/// 3. Energy consumption update
pub fn build(nodes: &mut [Node], round: usize, alive_count: &mut usize,writer: &mut BufWriter<File>) {
    // Step 1: Select Cluster Heads
    let t = threshold(round); 
    let mut cluster_heads: Vec<usize> = Vec::new();
    let mut rng = rand::rng();
    // Keep track of energy before the round (for possible debugging / logging later)
    let mut energy_before = vec![0.0; NUM_NODES];

    for i in 0..nodes.len() {

        // Updating dead nodes
        if nodes[i].is_alive && nodes[i].energy <= 0.0 {
            nodes[i].is_alive = false;
            *alive_count -= 1;
            continue;
        }

        if !nodes[i].is_alive {
            continue;
        }

        // Update neighbours → keep only currently alive ones
        let alive_neighbours: Vec<usize> = nodes[i]
            .neighbours
            .iter()
            .filter(|&&nid| nodes[nid].is_alive)
            .copied()
            .collect();

        nodes[i].neighbours = alive_neighbours;

        // Store energy snapshot
        energy_before[i] = nodes[i].energy;

        if !nodes[i].eligible {
            continue;
        }

        let energy_norm = nodes[i].energy / INITIAL_ENERGY; 
        let dist_norm = 1.0 - (nodes[i].distance_to_sink / 707.0) as f64; 
        let neigh_norm = nodes[i].neighbours.len() as f64 / NUM_NODES as f64; 
        let range_norm = (nodes[i].transmission_range/707.0) as f64; 
        let score = 0.4*energy_norm + 0.25*dist_norm + 0.25*neigh_norm + 0.1*range_norm ;

        // Optional early exit once we have enough CHs
        if cluster_heads.len() >= EXPECTED_CLUSTER_HEADS {
            continue;
        }

        if rng.random::<f64>() <= t * score {
            nodes[i].is_cluster_head = true;
            nodes[i].eligible        = false;
            cluster_heads.push(nodes[i].id);
        }
    }

    // Step 2: Form clusters (non-CH nodes join nearest CH)
    form_clusters(nodes, &cluster_heads);

    // Step 3: Simulate energy dissipation for this round
    dissipate_energy(nodes);
    for id in 0..nodes.len(){
        if !nodes[id].is_alive {
            continue;
        }

        let reward = nodes[id].energy - energy_before[id];
        writeln!(writer,"{},{},{},{},{},{:4},{},{},{}",
            id,
            nodes[id].position.x,
            nodes[id].position.y,
            nodes[id].distance_to_sink,
            nodes[id].is_cluster_head,
            nodes[id].energy,
            nodes[id].neighbours.len(),
            nodes[id].transmission_range,
            reward).unwrap();

        
    }
}

/// Simulates energy consumption for all nodes in one round
/// according to the first-order radio model.
fn dissipate_energy(nodes: &mut [Node]) {
    for id in 0..nodes.len() {
        // Skip already dead nodes
        if !nodes[id].is_alive {
            continue;
        }

        if nodes[id].is_cluster_head {
            let num_members = nodes[id].cluster_members.len() as f64;

            // Energy for receiving + aggregating data from members
            nodes[id].energy -= num_members * PACKET_SIZE * (E_ELECTRONICS + E_AGGREGATION);

            // Energy for transmitting aggregated data to sink
            let dist_to_sink = nodes[id].distance_to_sink as f64;
            let tx_energy = E_ELECTRONICS * PACKET_SIZE
                + amplification_energy(PACKET_SIZE, dist_to_sink);

            nodes[id].energy -= tx_energy;
        }
        else if let Some(ch_id) = nodes[id].cluster_head_id {
            // Normal node: transmit to its cluster head
            let ch_pos = nodes[ch_id].position;
            let dist = (nodes[id].position - ch_pos).length() as f64;

            let tx_energy = E_ELECTRONICS * PACKET_SIZE
                + amplification_energy(PACKET_SIZE, dist);

            nodes[id].energy -= tx_energy;
        }


    }
}

/// Returns the energy consumed by the transmitter amplifier
/// depending on distance (free-space vs. multipath model).
#[inline]
fn amplification_energy(packet_size: f64, distance: f64) -> f64 {
    if distance < THRESHOLD_DISTANCE as f64 {
        E_FREE_SPACE * packet_size * distance.powi(2)
    } else {
        E_MULTIPATH * packet_size * distance.powi(4)
    }
}

/// Assigns each non-CH alive node to the nearest cluster head.
fn form_clusters(nodes: &mut [Node], cluster_heads: &[usize]) {
    let mut assignments: Vec<(usize, usize)> = Vec::new(); // (member_id, ch_id)

    for node in nodes.iter() {
        if node.is_cluster_head || !node.is_alive {
            continue;
        }

        let mut min_dist = f32::INFINITY;
        let mut best_ch = None;

        for &ch_id in cluster_heads {
            let dist = (nodes[ch_id].position - node.position).length();
            if dist < min_dist {
                min_dist = dist;
                best_ch = Some(ch_id);
            }
        }

        if let Some(ch_id) = best_ch {
            assignments.push((node.id, ch_id));
        }
    }

    // Apply assignments after all reads are done
    for (member_id, ch_id) in assignments {
        nodes[member_id].cluster_head_id = Some(ch_id);
        nodes[ch_id].cluster_members.push(member_id);
    }
}
