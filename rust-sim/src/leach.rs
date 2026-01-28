
use crate::config::PACKET_SIZE;
use crate::simulator::{Protocol,SIMULATOR};
use crate::utils::*;
use rand::Rng; 
use crate::node::Node;

pub struct LEACH{
    threshold: f32,
    ch_probability: f32,
    cycle_length: usize,
}

impl LEACH {
    pub fn new(probability: f32) -> Self{

        Self{
            ch_probability: probability,
            threshold: 0.0, // dummy value
            cycle_length: (1.0/probability) as usize
        }
    }

    fn update_threshold(&mut self, round: usize){

        let r_mod = (round % self.cycle_length) as f32;
        let denom = 1.0 - self.ch_probability * r_mod;
        self.threshold = (self.ch_probability / denom).min(1.0);
    }

    fn form_cluster(wsn: &mut Vec<Node>, cluster_heads: &Vec<usize>){

        for n_id in 0..wsn.len(){
            if (wsn[n_id].is_alive) && (!wsn[n_id].is_cluster_head){

                let mut min_dist = f32::INFINITY;
                let mut nearest_ch: Option<usize> = None;

                for &ch_id in cluster_heads {
                    let dist = (wsn[n_id].position - wsn[ch_id].position).length();
                    if dist < min_dist {
                        min_dist = dist;
                        nearest_ch = Some(ch_id);
                    }
                }

                if let Some(ch) = nearest_ch{
                    wsn[n_id].cluster_head_id = nearest_ch;
                    wsn[ch].cluster_members.push(n_id);
                    wsn[n_id].res_energy -= transmission_energy(PACKET_SIZE,min_dist);
                }
            }
        }
    }
}

impl Protocol for LEACH{

    fn name(&self) -> &'static str {
        "LEACH"
    }

    fn run_round(&mut self, sim: &mut SIMULATOR) {

        self.update_threshold(sim.round);
        let mut rng = rand::rng();
        let mut cluster_heads: Vec<usize> = Vec::new();

        // first pass ( reset , update , select ch)
        for id in 0..sim.wsn.len(){
            reset_node(&mut sim.wsn[id]);
            // reset eligibilty
            if sim.round % self.cycle_length == 0 {
                sim.wsn[id].is_eligible = true;
            }

            // update dead nodes
            if sim.wsn[id].is_alive && sim.wsn[id].res_energy <= 0.0 {
                sim.wsn[id].is_alive = false;
                sim.alive_count -= 1;
                continue;
            }
            
            // cluster head selection
            if (rng.random::<f32>() < self.threshold) && (sim.wsn[id].is_alive && sim.wsn[id].is_eligible) {
                sim.wsn[id].is_cluster_head = true;
                sim.wsn[id].is_eligible = false;
                cluster_heads.push(id);
            }
        }

        // cluster formation and normal node energy dissipation
        LEACH::form_cluster(&mut sim.wsn,&cluster_heads);

        // cluster head energy dissipation
        for &ch_id in cluster_heads.iter(){
            if !sim.wsn[ch_id].is_alive{
                continue;
            }
            let k = sim.wsn[ch_id].cluster_members.len() as f32;
            sim.wsn[ch_id].res_energy -= (receive_energy(PACKET_SIZE) + aggregation_energy(PACKET_SIZE)) * k;
            sim.wsn[ch_id].res_energy -= transmission_energy(PACKET_SIZE,sim.wsn[ch_id].distance_to_sink)
        }

    }

}
