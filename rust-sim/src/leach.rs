use std::usize;

use rand::Rng;
use crate::node::Node;
use crate::config::{P,CYCLE};

fn threshold(round: i64) -> f64{
    
    let r_mod = round % CYCLE;
    P / (1.0 - P * r_mod as f64)
    

}


fn is_eligible(node: &Node, round: i64) -> bool {
    let rounds_since_ch = round - node.last_ch_round;

    if node.is_alive && rounds_since_ch >= CYCLE {
        return true;
    }

    false
}

pub fn build(nodes: &mut[Node], round: i64) {
    
    let threshold = threshold(round);
    let mut rng = rand::rng();

    let mut cluster_heads: Vec<usize> = Vec::new();
    for node in nodes.iter_mut(){
       if is_eligible(node, round){
           let rand_prob:f64 = rng.random();

           if rand_prob <= threshold{
               node.is_ch = true;
               node.last_ch_round = round;
               cluster_heads.push(node.id);
           }
       }
    }

    form_clusters(nodes, &cluster_heads);
}



fn form_clusters(nodes: &mut [Node], cluster_heads: &[usize]) {
    // (node_id, ch_id)
    let mut assignments: Vec<(usize, usize)> = Vec::new();

   
    for node in nodes.iter() {
        if node.is_ch || !node.is_alive {
            continue;
        }

        let mut min_dist = 0.0;
        let mut chosen_ch: Option<usize> = None;

        for &ch_id in cluster_heads {
            let ch = &nodes[ch_id];

            let dx = ch.position.x - node.position.x;
            let dy = ch.position.y - node.position.y;
            let dist = dx * dx + dy * dy;

            if dist < min_dist {
                min_dist = dist;
                chosen_ch = Some(ch_id);
            }
        }

        if let Some(ch_id) = chosen_ch {
            assignments.push((node.id, ch_id));
        }
    }

    
    for (node_id, ch_id) in assignments {
        nodes[node_id].cluster_id = Some(ch_id);
        nodes[ch_id].members.push(node_id);
    }
}

