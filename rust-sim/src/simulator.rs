use crate::{config::{SENSOR_RADIUS, TO_PIXEL_SCALE}, node::Node};
use macroquad::prelude::*;

pub trait Protocol{
    fn run_round(&mut self, sim: &mut SIMULATOR);
    fn name(&self) -> &'static str;
} 

pub struct SIMULATOR{
    pub wsn: Vec<Node>,
    pub round: usize,
    pub alive_count: usize
}

impl SIMULATOR{

    pub fn new(width: f32, height: f32, n_nodes: usize) -> Self{
        let wsn = Node::create_wsn(width, height, n_nodes);
        
        Self{
            wsn,
            round:0,
            alive_count:n_nodes
            
        }
    }

    pub fn render(&self){
        for node in self.wsn.iter(){
            let color = if !node.is_alive {
                Color::from_rgba(180, 60, 60,255)    // dead - dark red
            } else if node.is_cluster_head {
                Color::from_rgba(89, 172, 119,255)   // cluster head - green
            } else {
                Color::from_rgba(245, 235, 200,255)  // normal alive node - light yellow
            };
            let position = node.position * TO_PIXEL_SCALE;
            draw_circle(position.x, position.y,SENSOR_RADIUS,color);
        }

    }

    pub fn update<P: Protocol>(&mut self, protocol: &mut P ){
        self.round += 1;
        protocol.run_round(self);
    }
}
