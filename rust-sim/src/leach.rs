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

pub fn select_cluster_heads(nodes: &mut[Node], round: i64) {
    
    let threshold = threshold(round);
    let mut rng = rand::rng();
    for node in nodes{
       if is_eligible(node, round){
           let rand_prob:f64 = rng.random();

           if rand_prob <= threshold{
               node.is_ch = true;
               node.last_ch_round = round;
           }
       }
    }
}


