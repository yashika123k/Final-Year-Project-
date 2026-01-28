use crate::node::Node;
use crate::config::{E_AGGREGATION, E_ELECTRONICS, E_FREE_SPACE, E_MULTIPATH, THRESHOLD_DISTANCE};


pub(crate) fn transmission_energy(packet_size: f32, distance: f32) -> f32 {
    let mut energy:f32 = packet_size * E_ELECTRONICS;

    if distance <= THRESHOLD_DISTANCE {
        energy += packet_size * E_FREE_SPACE * distance.powi(2);
    }else{
        energy += packet_size * E_MULTIPATH * distance.powi(4);
    }

    energy
}

pub(crate) fn reset_node(node: &mut Node) {

    node.is_cluster_head   = false;
    node.cluster_head_id   = None;
    node.cluster_members.clear();
    
}

pub(crate) fn receive_energy(packet_size: f32) -> f32 {
    packet_size * E_ELECTRONICS
}

pub(crate) fn aggregation_energy(packet_size: f32) -> f32 {
    packet_size * E_AGGREGATION
}

