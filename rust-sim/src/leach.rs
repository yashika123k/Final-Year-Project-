
use rand::Rng;
use crate::node::Node;
use crate::config::*;

fn threshold(round: i64) -> f64{

    let r_mod = round % CYCLE;
    P / (1.0 - P * r_mod as f64)


}


pub fn reset(nodes: &mut[Node]) {
    for node in nodes.iter_mut(){
        node.is_ch = false;
        node.eligible = true;
        node.cluster_id = None;
        node.members.clear();
    } 
}

pub fn build(nodes: &mut[Node], round: i64) {
    
    let threshold = threshold(round);
    let mut rng = rand::rng();

    let mut cluster_heads: Vec<usize> = Vec::new();
    for node in nodes.iter_mut(){
       if node.eligible{
           let rand_prob:f64 = rng.random();
           if cluster_heads.len() > MAX_CH {
               break;
           }
           if rand_prob <= threshold{
               node.is_ch = true;
               node.eligible = false;
               cluster_heads.push(node.id);
           }
       }
    }

    form_clusters(nodes, &cluster_heads);

    for id in 0..nodes.len(){

        if nodes[id].energy < 0.0{
            nodes[id].is_alive = false;
            continue;
        }

        if nodes[id].is_ch {
            let k = nodes[id].members.len() as f64;
            nodes[id].energy -= k * E_ELEC * PACKET_SIZE ;
            nodes[id].energy -= k * E_DA * PACKET_SIZE ;

            let dx = nodes[id].position.x  * PTM_X - BS_X;
            let dy = nodes[id].position.y  * PTM_Y - BS_Y;
            let d2 = dx * dx + dy * dy;

            if d2.sqrt() < D0 {
                nodes[id].energy -= E_ELEC * PACKET_SIZE + E_FS * PACKET_SIZE * d2 as f64;
            } else {
                nodes[id].energy -= E_ELEC * PACKET_SIZE + E_MP * PACKET_SIZE * d2.powi(2) as f64;
            }        
        } 
        else {
            if let Some(ch_id) = nodes[id].cluster_id {

                let dx = nodes[id].position.x  * PTM_X - nodes[ch_id].position.x * PTM_X ;
                let dy = nodes[id].position.y  * PTM_Y - nodes[ch_id].position.y * PTM_Y ;
                let d2 = dx*dx + dy*dy;

                if d2.sqrt() < D0 {
                    nodes[id].energy -= E_ELEC * PACKET_SIZE
                        + E_FS * PACKET_SIZE * d2 as f64;
                } else {
                    nodes[id].energy -= E_ELEC * PACKET_SIZE
                        + E_MP * PACKET_SIZE * d2.powi(2) as f64;
                }
            }
        }
    }
}



fn form_clusters(nodes: &mut [Node], cluster_heads: &[usize]) {
    // (node_id, ch_id)
    let mut assignments: Vec<(usize, usize)> = Vec::new();

   
    for node in nodes.iter() {
        if node.is_ch || !node.is_alive {
            continue;
        }

        let mut min_dist = f32::INFINITY;
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


