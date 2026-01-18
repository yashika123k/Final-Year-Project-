use ggez::{Context,ContextBuilder};
use ggez::graphics::{self, Color, DrawMode, DrawParam, Mesh};
use ggez::event::{self,EventHandler};
use ggez::glam::Vec2;

use rust_sim::node::Node;
use rust_sim::leach;
use rust_sim::config::{CYCLE, NUM_NODES, SCREEN_H, SCREEN_W};
use rand::Rng;



struct WSN{
    nodes: Vec<Node>,
    mesh: Mesh,
    round: i64,
    epoch: i64,
    alive: usize
}

impl WSN{
    fn new(ctx: &mut Context ,n_nodes: usize) -> Self{
        
        let mut rng = rand::rng();
        let mut nodes: Vec<Node> = Vec::new();

        for idx in 0..n_nodes{
            let pos_x = rng.random::<f32>() * SCREEN_W;
            let pos_y = rng.random::<f32>() * SCREEN_H;

            let node = Node::new(idx,Vec2::new(pos_x, pos_y));
            nodes.push(node);
        }

        let mesh = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            Vec2::new(0.,0.),
            30.,
            0.1,
            Color::WHITE).unwrap();

        Self{
            nodes,
            mesh,
            round: 0,
            epoch: 0,
            alive: NUM_NODES
        }
    }
}

impl EventHandler for WSN{

    fn update(&mut self, ctx: &mut Context) -> Result<(), ggez::GameError> {

        if ctx.time.check_update_time(2) {

            if self.round % CYCLE == 0 {
                for node in self.nodes.iter_mut(){
                    node.eligible = true;
                }
                println!("EPOCH: {}",self.epoch);
                println!("ALIVE: {}",self.alive);
                self.epoch += 1;
            }

            self.round += 1;
            leach::reset(&mut self.nodes);
            leach::build(&mut self.nodes, self.round, &mut self.alive);

        }
        Ok(())
    }    


    fn draw(&mut self, ctx: &mut Context) -> Result<(), ggez::GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from_rgb(34, 40, 49));

        for node in &self.nodes {
            let color = if node.is_ch && node.is_alive {
                Color::from_rgb(89, 172, 119)

            } else if node.is_alive {
                Color::from_rgb(255, 245, 228)
            } else {
                Color::from_rgb(255, 148, 148)        
            };

            canvas.draw(
                &self.mesh,
                DrawParam::default()
                .dest(node.position)
                .color(color),
            );

        }

        canvas.finish(ctx)?;
        Ok(())
    }
}


fn main(){
    let (mut ctx , event_loop) = ContextBuilder::new("WSN","madhav")
        .window_mode(
            ggez::conf::WindowMode::default()
            .dimensions(SCREEN_W , SCREEN_H ))
        .window_setup(
            ggez::conf::WindowSetup::default()
            .title("WSN")
        )
        .build().unwrap();

    let state = WSN::new(&mut ctx,NUM_NODES);

    event::run(ctx, event_loop, state);


}
